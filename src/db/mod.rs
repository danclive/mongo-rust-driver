use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use bson::{self, Bson, Document};
use auth::Authenticator;
use client::MongoClient;
use collection::Collection;
use command_type::CommandType;
use common::{ReadPreference, WriteConcern, ReadConcern};
use semver::Version;
use error::{Result, ErrorCode};
use error::Error::{CursorNotFoundError, OperationError, ResponseError, EventListenerError};
use wire_protocol::flags::OpQueryFlags;
use wire_protocol::operations::Message;
use pool::PooledStream;
use apm::{CommandStarted, CommandResult, EventRunner};
use cursor::{Cursor, DEFAULT_BATCH_SIZE};

use self::options::{CreateCollectionOptions, CreateUserOptions, UserInfoOptions};

pub mod options;
pub mod roles;

#[derive(Clone)]
pub struct Database {
    pub inner: Arc<DatabaseInner>
}

pub struct DatabaseInner {
    /// The database name.
    pub name: String,
    /// A reference to the client that spawned this database.
    pub client: MongoClient,
    /// Indicates how a server should be selected for read operations.
    pub read_preference: ReadPreference,
    /// The readConcern query option for replica sets and replica set shards determines
    /// which data to return from a query.
    pub read_concern: ReadConcern,
    /// Describes the guarantees provided by MongoDB when reporting the success of a write
    /// operation.
    pub write_concern: WriteConcern,
}

fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

macro_rules! try_or_emit {
    ($cmd_type:expr, $cmd_name:expr, $req_id:expr, $connstring:expr, $result:expr, $client:expr) =>
    {
        match $result {
            Ok(val) => val,
            Err(e) => {
                if $cmd_type != CommandType::Suppressed {
                    let hook_result = $client.run_completion_hooks(&CommandResult::Failure {
                        duration: 0,
                        command_name: $cmd_name.to_string(),
                        failure: &e,
                        request_id: $req_id as i64,
                        connection_string: $connstring,
                    });

                    if hook_result.is_err() {
                        return Err(EventListenerError(Some(Box::new(e))));
                    }
                }

                return Err(e)
            }
        }
    };
}

impl Database {
    pub fn open(
        client: MongoClient,
        name: &str,
        read_preference: Option<ReadPreference>,
        read_concern: Option<ReadConcern>,
        write_concern: Option<WriteConcern>
    ) -> Database {
        is_send::<MongoClient>();
        is_sync::<MongoClient>();

        let rp = read_preference.unwrap_or_else(|| client.inner.read_preference.clone());
        let rc = read_concern.unwrap_or_else(|| client.inner.read_concern.clone());
        let wc = write_concern.unwrap_or_else(|| client.inner.write_concern.clone());

        Database {
            inner: Arc::new(DatabaseInner {
                name: name.to_string(),
                client: client,
                read_preference: rp,
                read_concern: rc,
                write_concern: wc,
            })
        }
    }

    pub fn auth(&self, user: &str, password: &str) -> Result<()> {
        let authenticator = Authenticator::new(self.clone());
        authenticator.auth(user, password)
    }

    pub fn collection(&self, coll_name: &str) -> Collection {
        Collection::new(
            self.clone(),
            coll_name,
            false,
            Some(self.inner.read_preference.clone()),
            Some(self.inner.read_concern.clone()),
            Some(self.inner.write_concern.clone())
        )
    }

    pub fn collection_with_prefs(
        &self,
        coll_name: &str,
        create: bool,
        read_preference: Option<ReadPreference>,
        read_concern: Option<ReadConcern>,
        write_concern: Option<WriteConcern>
    ) -> Collection {
        Collection::new(
            self.clone(),
            coll_name,
            create,
            read_preference,
            read_concern,
            write_concern
        )
    }

    pub fn get_req_id(&self) -> i32 {
        self.inner.client.get_req_id()
    }

    pub fn command(
        &self,
        command: Document,
        command_type: CommandType,
        read_preference: Option<ReadPreference>
    ) -> Result<Document> {

        let read_preference = read_preference.unwrap_or_else(|| self.inner.read_preference.clone());

        let (stream, slave_ok, send_read_pref) = if command_type.is_write_command() {
            (self.inner.client.acquire_write_stream()?, false, false)
        } else {
            self.inner.client.acquire_stream(read_preference.clone())?
        };

        let new_command = if !send_read_pref {
            command
        } else if command.get("$query").is_some() {
            // Query is already formatted as a $query document; add onto it.
            let mut nq = command;
            nq.insert("$read_preference", Bson::Document(read_preference.to_document()));
            nq
        } else {
            // Convert the query to a $query document.
            let mut nq = doc!{ "$query": command };
            nq.insert("$read_preference", Bson::Document(read_preference.to_document()));
            nq
        };

        Database::command_with_stream(
            self.inner.client.clone(), 
            stream,
            self.inner.name.clone(),
            new_command,
            command_type, slave_ok
        )
    }

    pub fn command_with_stream(
        client: MongoClient,
        stream: PooledStream,
        name: String,
        command: Document,
        command_type: CommandType,
        slave_ok: bool
    ) -> Result<Document> {

        let flags = OpQueryFlags::empty();

        let new_flags = if slave_ok {
            flags | OpQueryFlags::SLAVE_OK
        } else {
            flags
        };

        let mut stream = stream;
        let socket = stream.get_socket();
        let request_id = client.get_req_id();
        let command_name = command_type.to_str();
        let connstring = format!("{}", socket.get_ref().peer_addr()?);

        let init_time = Instant::now();

        let message = Message::new_query(
            request_id,
            new_flags,
            name.clone() + ".$cmd",
            0,
            -1,
            command.clone(),
            None
        )?;

        if command_type != CommandType::Suppressed {
            let hook_result = client.run_start_hooks(&CommandStarted {
                command: command,
                database_name: name,
                command_name: command_name.to_string(),
                request_id: request_id as i64,
                connection_string: connstring.clone(),
            });

            if hook_result.is_err() {
                return Err(EventListenerError(None));
            }
        }

        try_or_emit!(
            command_type,
            command_name,
            request_id,
            connstring,
            message.write(socket),
            client
        );

        let reply = try_or_emit!(
            command_type,
            command_name,
            request_id,
            connstring,
            Message::read(socket),
            client
        );

        let fin_time = Instant::now();

        let doc = try_or_emit!(
            command_type,
            command_name,
            request_id,
            connstring,
            Database::get_doc_from_reply(reply),
            client
        );
        
        if command_type != CommandType::Suppressed {
            let _hook_result = client.run_completion_hooks(&CommandResult::Success {
                duration: (fin_time - init_time).subsec_nanos() as u64,
                reply: doc.clone(),
                command_name: command_name.to_string(),
                request_id: request_id as i64,
                connection_string: connstring,
            });
        }

        Ok(doc)
    }

    pub fn get_doc_from_reply(reply: Message) -> Result<Document> {
        match reply {
            Message::OpReply { documents: mut docs, .. } => {
                if !docs.is_empty() {

                    let mut doc = docs.remove(0);

                    if let Some(&Bson::Int32(code)) = doc.get("code") {
                        if code != ErrorCode::NamespaceNotFound as i32 {
                            if let Some(&Bson::String(ref msg)) = doc.get("errmsg") {
                                return Err(OperationError(msg.to_string()));
                            }
                        }
                    }

                    if let Some(Bson::Document(cursor)) = doc.remove("cursor") {
                        return Ok(cursor)
                    }

                    return Ok(doc)
                }

                return Ok(Document::new())
            }
            _ => return Err(CursorNotFoundError)
        }
    }

    pub fn list_collections(&self, filter: Option<bson::Document>) -> Result<Cursor> {
        self.list_collections_with_batch_size(filter, DEFAULT_BATCH_SIZE)
    }

    pub fn list_collections_with_batch_size(
        &self,
        filter: Option<bson::Document>,
        batch_size: i32
    ) -> Result<Cursor> {

        let mut list_collections_command = bson::Document::new();

        list_collections_command.insert("listCollections", Bson::Int32(1));

        if filter.is_some() {
            list_collections_command.insert("filter", Bson::Document(filter.unwrap()));
        }

        return Cursor::command(self.clone(), list_collections_command, CommandType::ListCollections, Some(batch_size), None, None)
    }

    pub fn collection_names(&self, filter: Option<bson::Document>) -> Result<Vec<String>> {
        let mut cursor = self.list_collections(filter)?;
        let mut results = vec![];
        loop {
            match cursor.next() {
                Some(Ok(doc)) => {
                    if let Some(&Bson::String(ref name)) = doc.get("name") {
                        results.push(name.to_string());
                    }
                }
                Some(Err(err)) => return Err(err),
                None => return Ok(results),
            }
        }
    }

    pub fn version(&self) -> Result<Version> {
        let doc = doc!{ "buildinfo": 1 };
        let out = self.command(doc, CommandType::BuildInfo, None)?;

        match out.get("version") {
            Some(&Bson::String(ref s)) => {
                match Version::parse(s) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ResponseError(e.description().to_string())),
                }
            }
            _ => Err(ResponseError("No version received from server".to_string())),
        }
    }

    pub fn create_collection(
        &self,
        name: &str,
        options: Option<CreateCollectionOptions>
    ) -> Result<()> {
        let mut doc = doc!{ "create": name };

        if let Some(create_collection_options) = options {
            doc.extend(create_collection_options);
        }

        self.command(doc, CommandType::CreateCollection, None).map(|_| ())
    }

    pub fn create_user(
        &self,
        name: &str,
        password: &str,
        options: Option<CreateUserOptions>
    ) -> Result<()> {
        let mut doc = doc!{
            "createUser": name,
            "pwd": password
        };

        match options {
            Some(user_options) => {
                doc.extend(user_options);
            }
            None => {
                doc.insert("roles", Bson::Array(Vec::new()));
            }
        };

        self.command(doc, CommandType::CreateUser, None).map(|_| ())
    }

    pub fn drop_all_users(&self, write_concern: Option<WriteConcern>) -> Result<(i32)> {
        let mut doc = doc!{ "dropAllUsersFromDatabase": 1 };

        if let Some(concern) = write_concern {
            doc.insert("writeConcern", concern.to_document());
        }

        let response = self.command(doc, CommandType::DropAllUsers, None)?;

        match response.get("n") {
            Some(&Bson::Int32(i)) => Ok(i),
            Some(&Bson::Int64(i)) => Ok(i as i32),
            _ => Err(CursorNotFoundError),
        }
    }

    pub fn drop_collection(&self, name: &str) -> Result<()> {
        let mut spec = bson::Document::new();
        spec.insert("drop", Bson::String(name.to_string()));
        self.command(spec, CommandType::DropCollection, None)?;
        Ok(())
    }

    pub fn drop_database(&self) -> Result<()> {
        let mut spec = bson::Document::new();
        spec.insert("dropDatabase", Bson::Int32(1));
        self.command(spec, CommandType::DropDatabase, None)?;
        Ok(())
    }

    pub fn drop_user(&self, name: &str, write_concern: Option<WriteConcern>) -> Result<()> {
        let mut doc = doc!{ "dropUser": name };

        if let Some(concern) = write_concern {
            doc.insert("writeConcern", concern.to_document());
        }

        self.command(doc, CommandType::DropUser, None).map(|_| ())
    }

    pub fn get_all_users(&self, show_credentials: bool) -> Result<Vec<bson::Document>> {
        let doc = doc!{
            "usersInfo": 1,
            "showCredentials": show_credentials
        };

        let out = self.command(doc, CommandType::GetUsers, None)?;
        let vec = match out.get("users") {
            Some(&Bson::Array(ref vec)) => vec.clone(),
            _ => return Err(CursorNotFoundError),
        };

        let mut users = vec![];

        for bson in vec {
            match bson {
                Bson::Document(doc) => users.push(doc),
                _ => return Err(CursorNotFoundError),
            };
        }

        Ok(users)
    }

    pub fn get_user(&self, user: &str, options: Option<UserInfoOptions>) -> Result<bson::Document> {
        let mut doc = doc!{
            "usersInfo": {
                "user": user,
                "db": (self.inner.name.to_string())
            }
        };

        if let Some(user_info_options) = options {
            doc.extend(user_info_options);
        }

        let out = match self.command(doc, CommandType::GetUser, None) {
            Ok(doc) => doc,
            Err(e) => return Err(e),
        };

        let users = match out.get("users") {
            Some(&Bson::Array(ref v)) => v.clone(),
            _ => return Err(CursorNotFoundError),
        };

        match users.first() {
            Some(&Bson::Document(ref doc)) => Ok(doc.clone()),
            _ => Err(CursorNotFoundError),
        }
    }

    pub fn get_users(
        &self,
        users: Vec<&str>,
        options: Option<UserInfoOptions>
    ) -> Result<Vec<bson::Document>> {
        let vec: Vec<_> = users.into_iter()
            .map(|user| {
                let doc = doc!{
                    "user": user,
                    "db": self.inner.name.to_string()
                };
                Bson::Document(doc)
            })
            .collect();

        let mut doc = doc!{ "usersInfo": vec };

        if let Some(user_info_options) = options {
            doc.extend(user_info_options);
        }

        let out = self.command(doc, CommandType::GetUsers, None)?;
        let vec = match out.get("users") {
            Some(&Bson::Array(ref vec)) => vec.clone(),
            _ => return Err(CursorNotFoundError),
        };

        let mut users = vec![];

        for bson in vec {
            match bson {
                Bson::Document(doc) => users.push(doc),
                _ => return Err(CursorNotFoundError),
            };
        }

        Ok(users)
    }
}
