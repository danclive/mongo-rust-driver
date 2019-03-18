use std::ffi::CStr;
use std::fmt;
use std::error;

use crate::sys::bsonc;
use crate::sys::mongoc;

// see http://mongoc.org/libmongoc/current/errors.html
pub struct MongocError(bsonc::bson_error_t);

impl MongocError {
    pub fn empty() -> Self {
        MongocError(
            bsonc::bson_error_t {
                domain: 0,
                code: 0,
                message: [0; 504]
            }
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.domain == 0 && self.0.code == 0
    }

    pub fn domain(&self) -> MongoErrorDomain {
        match self.0.domain {
            0 => MongoErrorDomain::Blank,
            mongoc::MONGOC_ERROR_CLIENT           => MongoErrorDomain::Client,
            mongoc::MONGOC_ERROR_STREAM           => MongoErrorDomain::Stream,
            mongoc::MONGOC_ERROR_PROTOCOL         => MongoErrorDomain::Protocol,
            mongoc::MONGOC_ERROR_CURSOR           => MongoErrorDomain::Cursor,
            mongoc::MONGOC_ERROR_QUERY            => MongoErrorDomain::Query,
            mongoc::MONGOC_ERROR_INSERT           => MongoErrorDomain::Insert,
            mongoc::MONGOC_ERROR_SASL             => MongoErrorDomain::Sasl,
            mongoc::MONGOC_ERROR_BSON             => MongoErrorDomain::Bson,
            mongoc::MONGOC_ERROR_MATCHER          => MongoErrorDomain::Matcher,
            mongoc::MONGOC_ERROR_NAMESPACE        => MongoErrorDomain::Namespace,
            mongoc::MONGOC_ERROR_COMMAND          => MongoErrorDomain::Command,
            mongoc::MONGOC_ERROR_COLLECTION       => MongoErrorDomain::Collection,
            mongoc::MONGOC_ERROR_GRIDFS           => MongoErrorDomain::Gridfs,
            mongoc::MONGOC_ERROR_SCRAM            => MongoErrorDomain::Scram,
            mongoc::MONGOC_ERROR_SERVER_SELECTION => MongoErrorDomain::ServerSelection,
            mongoc::MONGOC_ERROR_WRITE_CONCERN    => MongoErrorDomain::WriteConcern,
            mongoc::MONGOC_ERROR_SERVER           => MongoErrorDomain::Server,
            mongoc::MONGOC_ERROR_TRANSACTION      => MongoErrorDomain::Transaction,
            _ => MongoErrorDomain::Unknown
        }
    }

    pub fn code(&self) -> MongoErrorCode {
        match self.0.code {
            0                                                  => MongoErrorCode::Blank,
            mongoc::MONGOC_ERROR_STREAM_INVALID_TYPE           => MongoErrorCode::StreamInvalidType,
            mongoc::MONGOC_ERROR_STREAM_INVALID_STATE          => MongoErrorCode::StreamInvalidState,
            mongoc::MONGOC_ERROR_STREAM_NAME_RESOLUTION        => MongoErrorCode::StreamNameResolution,
            mongoc::MONGOC_ERROR_STREAM_SOCKET                 => MongoErrorCode::StreamSocket,
            mongoc::MONGOC_ERROR_STREAM_CONNECT                => MongoErrorCode::StreamConnect,
            mongoc::MONGOC_ERROR_STREAM_NOT_ESTABLISHED        => MongoErrorCode::StreamNotEstablished,
            mongoc::MONGOC_ERROR_CLIENT_NOT_READY              => MongoErrorCode::ClientNotReady,
            mongoc::MONGOC_ERROR_CLIENT_TOO_BIG                => MongoErrorCode::ClientTooBig,
            mongoc::MONGOC_ERROR_CLIENT_TOO_SMALL              => MongoErrorCode::ClientTooSmall,
            mongoc::MONGOC_ERROR_CLIENT_GETNONCE               => MongoErrorCode::ClientGetnonce,
            mongoc::MONGOC_ERROR_CLIENT_AUTHENTICATE           => MongoErrorCode::ClientAuthenticate,
            mongoc::MONGOC_ERROR_CLIENT_NO_ACCEPTABLE_PEER     => MongoErrorCode::ClientNoAcceptablePeer,
            mongoc::MONGOC_ERROR_CLIENT_IN_EXHAUST             => MongoErrorCode::ClientInExhaust,
            mongoc::MONGOC_ERROR_PROTOCOL_INVALID_REPLY        => MongoErrorCode::ProtocolInvalidReply,
            mongoc::MONGOC_ERROR_PROTOCOL_BAD_WIRE_VERSION     => MongoErrorCode::ProtocolBadWireVersion,
            mongoc::MONGOC_ERROR_CURSOR_INVALID_CURSOR         => MongoErrorCode::CursorInvalidCursor,
            mongoc::MONGOC_ERROR_QUERY_FAILURE                 => MongoErrorCode::QueryFailure,
            mongoc::MONGOC_ERROR_BSON_INVALID                  => MongoErrorCode::BsonInvalid,
            mongoc::MONGOC_ERROR_MATCHER_INVALID               => MongoErrorCode::MatcherInvalid,
            mongoc::MONGOC_ERROR_NAMESPACE_INVALID             => MongoErrorCode::NamespaceInvalid,
            mongoc::MONGOC_ERROR_NAMESPACE_INVALID_FILTER_TYPE => MongoErrorCode::NamespaceInvalidFilterType,
            mongoc::MONGOC_ERROR_COMMAND_INVALID_ARG           => MongoErrorCode::CommandInvalidArg,
            mongoc::MONGOC_ERROR_COLLECTION_INSERT_FAILED      => MongoErrorCode::CollectionInsertFailed,
            mongoc::MONGOC_ERROR_COLLECTION_UPDATE_FAILED      => MongoErrorCode::CollectionUpdateFailed,
            mongoc::MONGOC_ERROR_COLLECTION_DELETE_FAILED      => MongoErrorCode::CollectionDeleteFailed,
            mongoc::MONGOC_ERROR_COLLECTION_DOES_NOT_EXIST     => MongoErrorCode::CollectionDoesNotExist,
            mongoc::MONGOC_ERROR_GRIDFS_INVALID_FILENAME       => MongoErrorCode::GridfsInvalidFilename,
            mongoc::MONGOC_ERROR_SCRAM_NOT_DONE                => MongoErrorCode::ScramNotDone,
            mongoc::MONGOC_ERROR_SCRAM_PROTOCOL_ERROR          => MongoErrorCode::ScramProtocolError,
            mongoc::MONGOC_ERROR_QUERY_COMMAND_NOT_FOUND       => MongoErrorCode::QueryCommandNotFound,
            mongoc::MONGOC_ERROR_QUERY_NOT_TAILABLE            => MongoErrorCode::QueryNotTailable,
            mongoc::MONGOC_ERROR_WRITE_CONCERN_ERROR           => MongoErrorCode::WriteConcernError,
            mongoc::MONGOC_ERROR_DUPLICATE_KEY                 => MongoErrorCode::DuplicateKey,
            mongoc::MONGOC_ERROR_CHANGE_STREAM_NO_RESUME_TOKEN => MongoErrorCode::ChangeStreamNoResumeToken,
            mongoc::MONGOC_ERROR_CLIENT_SESSION_FAILURE        => MongoErrorCode::ClientSessionFailure,
            mongoc::MONGOC_ERROR_TRANSACTION_INVALID_STATE     => MongoErrorCode::TransactionIncalidState,
            mongoc::MONGOC_ERROR_GRIDFS_CORRUPT                => MongoErrorCode::GridfsCorrput,
            code                                               => MongoErrorCode::Unknown(code)
        }
    }

    pub fn message(&self) -> String {
        let cstr = unsafe {
            CStr::from_ptr(&self.0.message as *const i8)
        };

        cstr.to_string_lossy().to_string()
    }

    pub fn as_ptr(&self) -> *const bsonc::bson_error_t {
        &self.0
    }

    pub fn as_mut_ptr(&mut self) -> &mut bsonc::bson_error_t {
        &mut self.0
    }
}

impl fmt::Debug for MongocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MongocError: {:?}/{:?} - {}",
            &self.domain(),
            &self.code(),
            &self.message()
        )
    }
}

impl fmt::Display for MongocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl error::Error for MongocError {
    fn description(&self) -> &str {
        "Error reported by the underlying Mongo C driver"
    }
}

#[derive(Debug,PartialEq)]
pub enum MongoErrorDomain {
    Blank,
    Client,
    Stream,
    Protocol,
    Cursor,
    Query,
    Insert,
    Sasl,
    Bson,
    Matcher,
    Namespace,
    Command,
    Collection,
    Gridfs,
    Scram,
    ServerSelection,
    WriteConcern,
    Server,
    Transaction,
    Unknown
}

#[derive(Debug,PartialEq)]
pub enum MongoErrorCode {
    Blank,
    StreamInvalidType,
    StreamInvalidState,
    StreamNameResolution,
    StreamSocket,
    StreamConnect,
    StreamNotEstablished,
    ClientNotReady,
    ClientTooBig,
    ClientTooSmall,
    ClientGetnonce,
    ClientAuthenticate,
    ClientNoAcceptablePeer,
    ClientInExhaust,
    ProtocolInvalidReply,
    ProtocolBadWireVersion,
    CursorInvalidCursor,
    QueryFailure,
    BsonInvalid,
    MatcherInvalid,
    NamespaceInvalid,
    NamespaceInvalidFilterType,
    CommandInvalidArg,
    CollectionInsertFailed,
    CollectionUpdateFailed,
    CollectionDeleteFailed,
    CollectionDoesNotExist,
    GridfsInvalidFilename,
    ScramNotDone,
    ScramProtocolError,
    QueryCommandNotFound,
    QueryNotTailable,
    ServerSelectionBadWireVersion,
    ServerSelectionFailure,
    GridfsChunkMissing,
    GridfsProtocolError,
    ProtocolError,
    WriteConcernError,
    DuplicateKey,
    ChangeStreamNoResumeToken,
    ClientSessionFailure,
    TransactionIncalidState,
    GridfsCorrput,
    Unknown(u32)
}
