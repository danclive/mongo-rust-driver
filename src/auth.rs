//! Authentication schemes.
use std::str::FromStr;

use ring::{pbkdf2, hmac};
use ring::digest::{self, SHA1, SHA1_OUTPUT_LEN};

use bson::Bson::{self, Binary};
use bson::Document;
use bson::spec::BinarySubtype::Generic;
use command_type::CommandType::Suppressed;
use data_encoding::BASE64;
use db::Database;
use error::Error::{DefaultError, MaliciousServerError, ResponseError};
use error::MaliciousServerErrorType;
use error::Result;
use util::textnonce::TextNonce;
use util::md5;

/// Handles SCRAM-SHA-1 authentication logic.
pub struct Authenticator {
    db: Database,
}

struct InitialData {
    message: String,
    response: String,
    nonce: String,
    conversation_id: Bson,
}

struct AuthData {
    salted_password: Vec<u8>,
    message: String,
    response: Document,
}

impl Authenticator {
    /// Creates a new authenticator.
    pub fn new(db: Database) -> Authenticator {
        Authenticator { db: db }
    }

    /// Authenticates a user-password pair against a database.
    pub fn auth(self, user: &str, password: &str) -> Result<()> {
        let initial_data = self.start(user)?;
        let conversation_id = initial_data.conversation_id.clone();
        let full_password = format!("{}:mongo:{}", user, password);
        let auth_data = self.next(full_password, initial_data)?;

        self.finish(conversation_id, auth_data)
    }

    fn start(&self, user: &str) -> Result<InitialData> {

        let text_nonce = TextNonce::sized(64)?;

        let nonce = format!("{}", text_nonce);
        let message = format!("n={},r={}", user, nonce);
        let bytes = format!("n,,{}", message).into_bytes();

        let start_doc = doc!{
            "saslStart": 1,
            "autoAuthorize": 1,
            "payload": (Generic, bytes),
            "mechanism": "SCRAM-SHA-1"
        };

        let doc = self.db.command(start_doc, Suppressed, None)?;

        let data = match doc.get("payload") {
            Some(&Binary(_, ref payload)) => payload.clone(),
            _ => return Err(ResponseError("Invalid payload returned".to_string())),
        };

        let id = match doc.get("conversationId") {
            Some(bson) => bson.clone(),
            None => return Err(ResponseError("No conversationId returned".to_string())),
        };

        let response = match String::from_utf8(data) {
            Ok(string) => string,
            Err(_) => return Err(ResponseError("Invalid UTF-8 payload returned".to_string())),
        };

        Ok(InitialData {
            message: message,
            response: response,
            nonce: nonce,
            conversation_id: id,
        })
    }

    fn next(&self, password: String, initial_data: InitialData) -> Result<AuthData> {
        // Parse out rnonce, salt, and iteration count
        let response: Vec<&str> = initial_data.response.split(",").collect();

        if response.len() != 3 {
            return Err(ResponseError("Invalid response returned".to_string()));
        }

        let rnonce_opt: Option<String> = {
            match response.get(0) {
                Some(r) => {
                    if r.find("r=").is_some() {
                        Some(r[2..].to_string())
                    } else {
                        None
                    }
                }
                None => None
            }
        };

        let salt_opt: Option<String> = {
            match response.get(1) {
                Some(r) => {
                    if r.find("s=").is_some() {
                        Some(r[2..].to_string())
                    } else {
                        None
                    }
                }
                None => None
            }
        };

        let i_opt: Option<u32> = {
            match response.get(2) {
                Some(r) => {
                    if r.find("i=").is_some() {
                        match u32::from_str(&r[2..]) {
                            Ok(result) => Some(result),
                            Err(_) => None
                        }
                    } else {
                        None
                    }
                }
                None => None
            }
        };

        let rnonce_b64 = rnonce_opt.ok_or_else(|| ResponseError("Invalid rnonce returned".to_string()))?;

        // Validate rnonce to make sure server isn't malicious
        if !rnonce_b64.starts_with(&initial_data.nonce) {
            return Err(MaliciousServerError(MaliciousServerErrorType::InvalidRnonce));
        }

        let salt_b64 = salt_opt.ok_or_else(|| ResponseError("Invalid salt returned".to_string()))?;

        let salt = BASE64.decode(salt_b64.as_bytes()).or_else(|e| Err(ResponseError(format!("Invalid base64 salt returned: {}", e))))?;

        let iterations = i_opt.ok_or_else(|| ResponseError("Invalid iteration count returned".to_string()))?;

        // Hash password
        let hashed_password = format!("{:x}", md5::compute(password.as_bytes()));

        // Salt password
        let mut salted_password = vec![0; SHA1_OUTPUT_LEN];
        pbkdf2::derive(&SHA1, iterations, &salt, hashed_password.as_bytes(), &mut salted_password);

        // Compute client key
        let client_key_hmac = hmac::SigningKey::new(&SHA1, &salted_password);
        let signature = hmac::sign(&client_key_hmac, b"Client Key");
        let client_key = signature.as_ref();


        // Hash into stored key
        let actual = digest::digest(&SHA1, &client_key);
        let stored_key = actual.as_ref();


        // Create auth message
        let without_proof = format!("c=biws,r={}", rnonce_b64);
        let auth_message = format!("{},{},{}",
                                   initial_data.message,
                                   initial_data.response,
                                   without_proof);

        // Compute client signature
        let client_signature_hmac = hmac::SigningKey::new(&SHA1, stored_key);
        let signature = hmac::sign(&client_signature_hmac, auth_message.as_bytes());
        let client_signature = signature.as_ref();


        // Sanity check
        if client_key.len() != client_signature.len() {
            return Err(DefaultError("Generated client key and/or client signature is invalid".to_string()));
        }

        // Compute proof by xor'ing key and signature
        let mut proof = vec![];
        for i in 0..client_key.len() {
            proof.push(client_key[i] ^ client_signature[i]);
        }

        // Encode proof and produce the message to send to the server
        let b64_proof = BASE64.encode(&proof);
        let final_message = format!("{},p={}", without_proof, b64_proof);

        let next_doc = doc!{
            "saslContinue": 1,
            "payload": (Generic, final_message.into_bytes()),
            "conversationId": initial_data.conversation_id.clone()
        };

        let response = self.db.command(next_doc, Suppressed, None)?;

        Ok(AuthData {
            salted_password: salted_password,
            message: auth_message,
            response: response,
        })
    }

    fn finish(&self, conversation_id: Bson, auth_data: AuthData) -> Result<()> {
        let final_doc = doc!{
            "saslContinue": 1,
            "payload": (Generic, vec![]),
            "conversationId": conversation_id
        };

        // Compute server key
        let client_key_hmac = hmac::SigningKey::new(&SHA1, &auth_data.salted_password);
        let signature = hmac::sign(&client_key_hmac, b"Server Key");
        let server_key = signature.as_ref();

        // Compute server signature
        let client_signature_hmac = hmac::SigningKey::new(&SHA1, server_key);
        let signature = hmac::sign(&client_signature_hmac, auth_data.message.as_bytes());
        let server_signature = signature.as_ref();

        let mut doc = auth_data.response;

        loop {
            // Verify server signature
            if let Some(&Binary(_, ref payload)) = doc.get("payload") {
                let payload_str = match String::from_utf8(payload.to_vec()) {
                    Ok(string) => string,
                    Err(_) => {
                        return Err(ResponseError("Invalid UTF-8 payload returned".to_string()))
                    }
                };

                let verifier = if payload_str.find("v=").is_some() && payload_str.len() > 2 {
                    &payload_str[2..]
                } else {
                    return Err(MaliciousServerError(MaliciousServerErrorType::NoServerSignature));
                };

                // Check that the signature is valid
                if verifier.ne(&BASE64.encode(&server_signature)) {
                    return Err(MaliciousServerError(MaliciousServerErrorType::InvalidServerSignature));
                }
            }

            doc = self.db.command(final_doc.clone(), Suppressed, None)?;

            if let Some(&Bson::Boolean(true)) = doc.get("done") {
                return Ok(());
            }
        }
    }
}
