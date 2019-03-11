#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// see http://mongoc.org/libmongoc/current/index.html

use std::os::raw::{c_char, c_int, c_void, c_uint};

use super::bsonc::{bson_t, bson_error_t, bson_oid_t, bson_value_t};

// mongoc_error_domain
pub const MONGOC_ERROR_CLIENT: u32 = 1;
pub const MONGOC_ERROR_STREAM: u32 = 2;
pub const MONGOC_ERROR_PROTOCOL: u32 = 3;
pub const MONGOC_ERROR_CURSOR: u32 = 4;
pub const MONGOC_ERROR_QUERY: u32 = 5;
pub const MONGOC_ERROR_INSERT: u32 = 6;
pub const MONGOC_ERROR_SASL: u32 = 7;
pub const MONGOC_ERROR_BSON: u32 = 8;
pub const MONGOC_ERROR_MATCHER: u32 = 9;
pub const MONGOC_ERROR_NAMESPACE: u32 = 10;
pub const MONGOC_ERROR_COMMAND: u32 = 11;
pub const MONGOC_ERROR_COLLECTION: u32 = 12;
pub const MONGOC_ERROR_GRIDFS: u32 = 13;
pub const MONGOC_ERROR_SCRAM: u32 = 14;
pub const MONGOC_ERROR_SERVER_SELECTION: u32 = 15;
pub const MONGOC_ERROR_WRITE_CONCERN: u32 = 16;
pub const MONGOC_ERROR_SERVER: u32 = 17;
pub const MONGOC_ERROR_TRANSACTION: u32 = 18;

// mongoc_error_code
pub const MONGOC_ERROR_STREAM_INVALID_TYPE: u32 = 1;
pub const MONGOC_ERROR_STREAM_INVALID_STATE: u32 = 2;
pub const MONGOC_ERROR_STREAM_NAME_RESOLUTION: u32 = 3;
pub const MONGOC_ERROR_STREAM_SOCKET: u32 = 4;
pub const MONGOC_ERROR_STREAM_CONNECT: u32 = 5;
pub const MONGOC_ERROR_STREAM_NOT_ESTABLISHED: u32 = 6;
pub const MONGOC_ERROR_CLIENT_NOT_READY: u32 = 7;
pub const MONGOC_ERROR_CLIENT_TOO_BIG: u32 = 8;
pub const MONGOC_ERROR_CLIENT_TOO_SMALL: u32 = 9;
pub const MONGOC_ERROR_CLIENT_GETNONCE: u32 = 10;
pub const MONGOC_ERROR_CLIENT_AUTHENTICATE: u32 = 11;
pub const MONGOC_ERROR_CLIENT_NO_ACCEPTABLE_PEER: u32 = 12;
pub const MONGOC_ERROR_CLIENT_IN_EXHAUST: u32 = 13;
pub const MONGOC_ERROR_PROTOCOL_INVALID_REPLY: u32 = 14;
pub const MONGOC_ERROR_PROTOCOL_BAD_WIRE_VERSION: u32 = 15;
pub const MONGOC_ERROR_CURSOR_INVALID_CURSOR: u32 = 16;
pub const MONGOC_ERROR_QUERY_FAILURE: u32 = 17;
pub const MONGOC_ERROR_BSON_INVALID: u32 = 18;
pub const MONGOC_ERROR_MATCHER_INVALID: u32 = 19;
pub const MONGOC_ERROR_NAMESPACE_INVALID: u32 = 20;
pub const MONGOC_ERROR_NAMESPACE_INVALID_FILTER_TYPE: u32 = 21;
pub const MONGOC_ERROR_COMMAND_INVALID_ARG: u32 = 22;
pub const MONGOC_ERROR_COLLECTION_INSERT_FAILED: u32 = 23;
pub const MONGOC_ERROR_COLLECTION_UPDATE_FAILED: u32 = 24;
pub const MONGOC_ERROR_COLLECTION_DELETE_FAILED: u32 = 25;
pub const MONGOC_ERROR_COLLECTION_DOES_NOT_EXIST: u32 = 26;
pub const MONGOC_ERROR_GRIDFS_INVALID_FILENAME: u32 = 27;
pub const MONGOC_ERROR_SCRAM_NOT_DONE: u32 = 28;
pub const MONGOC_ERROR_SCRAM_PROTOCOL_ERROR: u32 = 29;
pub const MONGOC_ERROR_QUERY_COMMAND_NOT_FOUND: u32 = 59;
pub const MONGOC_ERROR_QUERY_NOT_TAILABLE: u32 = 13051;
pub const MONGOC_ERROR_SERVER_SELECTION_BAD_WIRE_VERSION: u32 = 13052;
pub const MONGOC_ERROR_SERVER_SELECTION_FAILURE: u32 = 13053;
pub const MONGOC_ERROR_SERVER_SELECTION_INVALID_ID: u32 = 13054;
pub const MONGOC_ERROR_GRIDFS_CHUNK_MISSING: u32 = 13055;
pub const MONGOC_ERROR_GRIDFS_PROTOCOL_ERROR: u32 = 13056;
pub const MONGOC_ERROR_PROTOCOL_ERROR: u32 = 17;
pub const MONGOC_ERROR_WRITE_CONCERN_ERROR: u32 = 64;
pub const MONGOC_ERROR_DUPLICATE_KEY: u32 = 11000;
pub const MONGOC_ERROR_CHANGE_STREAM_NO_RESUME_TOKEN: u32 = 11001;
pub const MONGOC_ERROR_CLIENT_SESSION_FAILURE: u32 = 11002;
pub const MONGOC_ERROR_TRANSACTION_INVALID_STATE: u32 = 11003;
pub const MONGOC_ERROR_GRIDFS_CORRUPT: u32 = 11004;

pub const MONGOC_URI_APPNAME: &'static [u8; 8usize] = b"appname\0";
pub const MONGOC_URI_AUTHMECHANISM: &'static [u8; 14usize] = b"authmechanism\0";
pub const MONGOC_URI_AUTHMECHANISMPROPERTIES: &'static [u8; 24usize] = b"authmechanismproperties\0";
pub const MONGOC_URI_AUTHSOURCE: &'static [u8; 11usize] = b"authsource\0";
pub const MONGOC_URI_CANONICALIZEHOSTNAME: &'static [u8; 21usize] = b"canonicalizehostname\0";
pub const MONGOC_URI_CONNECTTIMEOUTMS: &'static [u8; 17usize] = b"connecttimeoutms\0";
pub const MONGOC_URI_COMPRESSORS: &'static [u8; 12usize] = b"compressors\0";
pub const MONGOC_URI_GSSAPISERVICENAME: &'static [u8; 18usize] = b"gssapiservicename\0";
pub const MONGOC_URI_HEARTBEATFREQUENCYMS: &'static [u8; 21usize] = b"heartbeatfrequencyms\0";
pub const MONGOC_URI_JOURNAL: &'static [u8; 8usize] = b"journal\0";
pub const MONGOC_URI_LOCALTHRESHOLDMS: &'static [u8; 17usize] = b"localthresholdms\0";
pub const MONGOC_URI_MAXIDLETIMEMS: &'static [u8; 14usize] = b"maxidletimems\0";
pub const MONGOC_URI_MAXPOOLSIZE: &'static [u8; 12usize] = b"maxpoolsize\0";
pub const MONGOC_URI_MAXSTALENESSSECONDS: &'static [u8; 20usize] = b"maxstalenessseconds\0";
pub const MONGOC_URI_MINPOOLSIZE: &'static [u8; 12usize] = b"minpoolsize\0";
pub const MONGOC_URI_READCONCERNLEVEL: &'static [u8; 17usize] = b"readconcernlevel\0";
pub const MONGOC_URI_READPREFERENCE: &'static [u8; 15usize] = b"readpreference\0";
pub const MONGOC_URI_READPREFERENCETAGS: &'static [u8; 19usize] = b"readpreferencetags\0";
pub const MONGOC_URI_REPLICASET: &'static [u8; 11usize] = b"replicaset\0";
pub const MONGOC_URI_RETRYWRITES: &'static [u8; 12usize] = b"retrywrites\0";
pub const MONGOC_URI_SAFE: &'static [u8; 5usize] = b"safe\0";
pub const MONGOC_URI_SERVERSELECTIONTIMEOUTMS: &'static [u8; 25usize] =
    b"serverselectiontimeoutms\0";
pub const MONGOC_URI_SERVERSELECTIONTRYONCE: &'static [u8; 23usize] = b"serverselectiontryonce\0";
pub const MONGOC_URI_SLAVEOK: &'static [u8; 8usize] = b"slaveok\0";
pub const MONGOC_URI_SOCKETCHECKINTERVALMS: &'static [u8; 22usize] = b"socketcheckintervalms\0";
pub const MONGOC_URI_SOCKETTIMEOUTMS: &'static [u8; 16usize] = b"sockettimeoutms\0";
pub const MONGOC_URI_SSL: &'static [u8; 4usize] = b"ssl\0";
pub const MONGOC_URI_SSLCLIENTCERTIFICATEKEYFILE: &'static [u8; 28usize] =
    b"sslclientcertificatekeyfile\0";
pub const MONGOC_URI_SSLCLIENTCERTIFICATEKEYPASSWORD: &'static [u8; 32usize] =
    b"sslclientcertificatekeypassword\0";
pub const MONGOC_URI_SSLCERTIFICATEAUTHORITYFILE: &'static [u8; 28usize] =
    b"sslcertificateauthorityfile\0";
pub const MONGOC_URI_SSLALLOWINVALIDCERTIFICATES: &'static [u8; 28usize] =
    b"sslallowinvalidcertificates\0";
pub const MONGOC_URI_SSLALLOWINVALIDHOSTNAMES: &'static [u8; 25usize] =
    b"sslallowinvalidhostnames\0";
pub const MONGOC_URI_W: &'static [u8; 2usize] = b"w\0";
pub const MONGOC_URI_WAITQUEUEMULTIPLE: &'static [u8; 18usize] = b"waitqueuemultiple\0";
pub const MONGOC_URI_WAITQUEUETIMEOUTMS: &'static [u8; 19usize] = b"waitqueuetimeoutms\0";
pub const MONGOC_URI_WTIMEOUTMS: &'static [u8; 11usize] = b"wtimeoutms\0";
pub const MONGOC_URI_ZLIBCOMPRESSIONLEVEL: &'static [u8; 21usize] = b"zlibcompressionlevel\0";

pub const MONGOC_DEFAULT_PORT: u32 = 27017;
pub const MONGOC_NAMESPACE_MAX: u32 = 128;
pub const MONGOC_DEFAULT_CONNECTTIMEOUTMS: u32 = 10000;
pub const MONGOC_DEFAULT_SOCKETTIMEOUTMS: u32 = 300000;
pub const MONGOC_ERROR_API_VERSION_LEGACY: u32 = 1;
pub const MONGOC_ERROR_API_VERSION_2: u32 = 2;
pub const MONGOC_HANDSHAKE_APPNAME_MAX: u32 = 128;
pub const MONGOC_LOG_DOMAIN: &'static [u8; 7usize] = b"mongoc\0";
pub const MONGOC_MAJOR_VERSION: u32 = 1;
pub const MONGOC_MINOR_VERSION: u32 = 14;
pub const MONGOC_MICRO_VERSION: u32 = 0;
pub const MONGOC_VERSION_S: &'static [u8; 7usize] = b"1.14.0\0";
pub const MONGOC_VERSION_HEX: u32 = 17694720;

// see http://mongoc.org/libmongoc/current/mongoc_bulk_operation_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_bulk_operation_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_bulk_write_flags_t {
    _unused: [u8; 0],
}
pub type mongoc_bulk_write_flags_t = _mongoc_bulk_write_flags_t;
extern "C" {
    pub fn mongoc_bulk_operation_destroy(bulk: *mut mongoc_bulk_operation_t);
    pub fn mongoc_bulk_operation_execute(
        bulk: *mut mongoc_bulk_operation_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> u32;
    pub fn mongoc_bulk_operation_delete(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
    pub fn mongoc_bulk_operation_delete_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
    pub fn mongoc_bulk_operation_insert(
        bulk: *mut mongoc_bulk_operation_t,
        document: *const bson_t,
    );
    pub fn mongoc_bulk_operation_insert_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_bulk_operation_remove(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
    pub fn mongoc_bulk_operation_remove_many_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_bulk_operation_remove_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
    );
    pub fn mongoc_bulk_operation_remove_one_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_bulk_operation_replace_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        upsert: bool,
    );
    pub fn mongoc_bulk_operation_replace_one_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_bulk_operation_update(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        upsert: bool,
    );
    pub fn mongoc_bulk_operation_update_many_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_bulk_operation_update_one(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        upsert: bool,
    );
    pub fn mongoc_bulk_operation_update_one_with_opts(
        bulk: *mut mongoc_bulk_operation_t,
        selector: *const bson_t,
        document: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_bulk_operation_set_bypass_document_validation(
        bulk: *mut mongoc_bulk_operation_t,
        bypass: bool,
    );
    pub fn mongoc_bulk_operation_new(ordered: bool) -> *mut mongoc_bulk_operation_t;
    pub fn mongoc_bulk_operation_set_write_concern(
        bulk: *mut mongoc_bulk_operation_t,
        write_concern: *const mongoc_write_concern_t,
    );
    pub fn mongoc_bulk_operation_set_database(
        bulk: *mut mongoc_bulk_operation_t,
        database: *const c_char,
    );
    pub fn mongoc_bulk_operation_set_collection(
        bulk: *mut mongoc_bulk_operation_t,
        collection: *const c_char,
    );
    pub fn mongoc_bulk_operation_set_client(
        bulk: *mut mongoc_bulk_operation_t,
        client: *mut c_void,
    );
    pub fn mongoc_bulk_operation_set_client_session(
        bulk: *mut mongoc_bulk_operation_t,
        client_session: *mut mongoc_client_session_t,
    );
    pub fn mongoc_bulk_operation_set_hint(bulk: *mut mongoc_bulk_operation_t, server_id: u32);
    pub fn mongoc_bulk_operation_get_hint(bulk: *const mongoc_bulk_operation_t) -> u32;
    pub fn mongoc_bulk_operation_get_write_concern(
        bulk: *const mongoc_bulk_operation_t,
    ) -> *const mongoc_write_concern_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_change_stream_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_change_stream_t {
    _unused: [u8; 0],
}
pub type mongoc_change_stream_t = _mongoc_change_stream_t;
extern "C" {
    pub fn mongoc_change_stream_destroy(arg1: *mut mongoc_change_stream_t);
    pub fn mongoc_change_stream_next(
        arg1: *mut mongoc_change_stream_t,
        arg2: *mut *const bson_t,
    ) -> bool;
    pub fn mongoc_change_stream_error_document(
        arg1: *const mongoc_change_stream_t,
        arg2: *mut bson_error_t,
        arg3: *mut *const bson_t,
    ) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_client_pool_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_client_pool_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_client_pool_new(uri: *const mongoc_uri_t) -> *mut mongoc_client_pool_t;
    pub fn mongoc_client_pool_destroy(pool: *mut mongoc_client_pool_t);
    pub fn mongoc_client_pool_pop(pool: *mut mongoc_client_pool_t) -> *mut mongoc_client_t;
    pub fn mongoc_client_pool_push(pool: *mut mongoc_client_pool_t, client: *mut mongoc_client_t);
    pub fn mongoc_client_pool_try_pop(pool: *mut mongoc_client_pool_t) -> *mut mongoc_client_t;
    pub fn mongoc_client_pool_max_size(pool: *mut mongoc_client_pool_t, max_pool_size: u32);
    pub fn mongoc_client_pool_min_size(pool: *mut mongoc_client_pool_t, min_pool_size: u32);
    pub fn mongoc_client_pool_set_ssl_opts(
        pool: *mut mongoc_client_pool_t,
        opts: *const mongoc_ssl_opt_t,
    );
    pub fn mongoc_client_pool_set_apm_callbacks(
        pool: *mut mongoc_client_pool_t,
        callbacks: *mut mongoc_apm_callbacks_t,
        context: *mut c_void,
    ) -> bool;
    pub fn mongoc_client_pool_set_error_api(pool: *mut mongoc_client_pool_t, version: i32) -> bool;
    pub fn mongoc_client_pool_set_appname(
        pool: *mut mongoc_client_pool_t,
        appname: *const c_char,
    ) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_client_session_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_client_session_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_client_session_get_client(
        session: *const mongoc_client_session_t,
    ) -> *mut mongoc_client_t;
    pub fn mongoc_client_session_get_opts(
        session: *const mongoc_client_session_t,
    ) -> *const mongoc_session_opt_t;
    pub fn mongoc_client_session_get_lsid(session: *const mongoc_client_session_t)
        -> *const bson_t;
    pub fn mongoc_client_session_get_cluster_time(
        session: *const mongoc_client_session_t,
    ) -> *const bson_t;
    pub fn mongoc_client_session_advance_cluster_time(
        session: *mut mongoc_client_session_t,
        cluster_time: *const bson_t,
    );
    pub fn mongoc_client_session_get_operation_time(
        session: *const mongoc_client_session_t,
        timestamp: *mut u32,
        increment: *mut u32,
    );
    pub fn mongoc_client_session_advance_operation_time(
        session: *mut mongoc_client_session_t,
        timestamp: u32,
        increment: u32,
    );
    pub fn mongoc_client_session_start_transaction(
        session: *mut mongoc_client_session_t,
        opts: *const mongoc_transaction_opt_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_session_in_transaction(session: *const mongoc_client_session_t) -> bool;
    pub fn mongoc_client_session_commit_transaction(
        session: *mut mongoc_client_session_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_session_abort_transaction(
        session: *mut mongoc_client_session_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_session_append(
        client_session: *const mongoc_client_session_t,
        opts: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_session_destroy(session: *mut mongoc_client_session_t);
}

// see http://mongoc.org/libmongoc/current/mongoc_client_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_client_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_client_new(uri_string: *const c_char) -> *mut mongoc_client_t;
    pub fn mongoc_client_new_from_uri(uri: *const mongoc_uri_t) -> *mut mongoc_client_t;
    pub fn mongoc_client_get_uri(client: *const mongoc_client_t) -> *const mongoc_uri_t;
    pub fn mongoc_client_command(
        client: *mut mongoc_client_t,
        db_name: *const c_char,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        query: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_client_kill_cursor(client: *mut mongoc_client_t, cursor_id: i64);
    pub fn mongoc_client_command_simple(
        client: *mut mongoc_client_t,
        db_name: *const c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_read_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_write_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const c_char,
        command: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_read_write_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_command_with_opts(
        client: *mut mongoc_client_t,
        db_name: *const c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_command_simple_with_server_id(
        client: *mut mongoc_client_t,
        db_name: *const c_char,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        server_id: u32,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_destroy(client: *mut mongoc_client_t);
    pub fn mongoc_client_start_session(
        client: *mut mongoc_client_t,
        opts: *const mongoc_session_opt_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_client_session_t;
    pub fn mongoc_client_get_database(
        client: *mut mongoc_client_t,
        name: *const c_char,
    ) -> *mut mongoc_database_t;
    pub fn mongoc_client_get_default_database(
        client: *mut mongoc_client_t,
    ) -> *mut mongoc_database_t;
    pub fn mongoc_client_get_gridfs(
        client: *mut mongoc_client_t,
        db: *const c_char,
        prefix: *const c_char,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_t;
    pub fn mongoc_client_get_collection(
        client: *mut mongoc_client_t,
        db: *const c_char,
        collection: *const c_char,
    ) -> *mut mongoc_collection_t;
    pub fn mongoc_client_get_database_names(
        client: *mut mongoc_client_t,
        error: *mut bson_error_t,
    ) -> *mut *mut c_char;
    pub fn mongoc_client_get_database_names_with_opts(
        client: *mut mongoc_client_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut *mut c_char;
    pub fn mongoc_client_find_databases(
        client: *mut mongoc_client_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_client_find_databases_with_opts(
        client: *mut mongoc_client_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_client_get_server_status(
        client: *mut mongoc_client_t,
        read_prefs: *mut mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_client_get_max_message_size(client: *mut mongoc_client_t) -> i32;
    pub fn mongoc_client_get_max_bson_size(client: *mut mongoc_client_t) -> i32;
    pub fn mongoc_client_get_write_concern(
        client: *const mongoc_client_t,
    ) -> *const mongoc_write_concern_t;
    pub fn mongoc_client_set_write_concern(
        client: *mut mongoc_client_t,
        write_concern: *const mongoc_write_concern_t,
    );
    pub fn mongoc_client_get_read_concern(
        client: *const mongoc_client_t,
    ) -> *const mongoc_read_concern_t;
    pub fn mongoc_client_set_read_concern(
        client: *mut mongoc_client_t,
        read_concern: *const mongoc_read_concern_t,
    );
    pub fn mongoc_client_get_read_prefs(
        client: *const mongoc_client_t,
    ) -> *const mongoc_read_prefs_t;
    pub fn mongoc_client_set_read_prefs(
        client: *mut mongoc_client_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
    pub fn mongoc_client_set_ssl_opts(client: *mut mongoc_client_t, opts: *const mongoc_ssl_opt_t);
    pub fn mongoc_client_set_apm_callbacks(
        client: *mut mongoc_client_t,
        callbacks: *mut mongoc_apm_callbacks_t,
        context: *mut c_void,
    ) -> bool;
    pub fn mongoc_client_get_server_description(
        client: *mut mongoc_client_t,
        server_id: u32,
    ) -> *mut mongoc_server_description_t;
    pub fn mongoc_client_get_server_descriptions(
        client: *const mongoc_client_t,
        n: *mut usize,
    ) -> *mut *mut mongoc_server_description_t;
    pub fn mongoc_server_descriptions_destroy_all(
        sds: *mut *mut mongoc_server_description_t,
        n: usize,
    );
    pub fn mongoc_client_select_server(
        client: *mut mongoc_client_t,
        for_writes: bool,
        prefs: *const mongoc_read_prefs_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_server_description_t;
    pub fn mongoc_client_set_error_api(client: *mut mongoc_client_t, version: i32) -> bool;
    pub fn mongoc_client_set_appname(
        client: *mut mongoc_client_t,
        appname: *const c_char,
    ) -> bool;
    pub fn mongoc_client_watch(
        client: *mut mongoc_client_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_change_stream_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_collection_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_collection_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_collection_aggregate(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_collection_destroy(collection: *mut mongoc_collection_t);
    pub fn mongoc_collection_copy(collection: *mut mongoc_collection_t)
        -> *mut mongoc_collection_t;
    pub fn mongoc_collection_command(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        command: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_collection_read_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_write_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_read_write_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_command_with_opts(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_command_simple(
        collection: *mut mongoc_collection_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_count(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        query: *const bson_t,
        skip: i64,
        limit: i64,
        read_prefs: *const mongoc_read_prefs_t,
        error: *mut bson_error_t,
    ) -> i64;
    pub fn mongoc_collection_count_with_opts(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        query: *const bson_t,
        skip: i64,
        limit: i64,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        error: *mut bson_error_t,
    ) -> i64;
    pub fn mongoc_collection_drop(
        collection: *mut mongoc_collection_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_drop_with_opts(
        collection: *mut mongoc_collection_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_drop_index(
        collection: *mut mongoc_collection_t,
        index_name: *const c_char,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_drop_index_with_opts(
        collection: *mut mongoc_collection_t,
        index_name: *const c_char,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_create_index(
        collection: *mut mongoc_collection_t,
        keys: *const bson_t,
        opt: *const mongoc_index_opt_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_create_index_with_opts(
        collection: *mut mongoc_collection_t,
        keys: *const bson_t,
        opt: *const mongoc_index_opt_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_ensure_index(
        collection: *mut mongoc_collection_t,
        keys: *const bson_t,
        opt: *const mongoc_index_opt_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_find_indexes(
        collection: *mut mongoc_collection_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_collection_find_indexes_with_opts(
        collection: *mut mongoc_collection_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_collection_find(
        collection: *mut mongoc_collection_t,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        query: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_collection_find_with_opts(
        collection: *mut mongoc_collection_t,
        filter: *const bson_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_collection_insert(
        collection: *mut mongoc_collection_t,
        flags: mongoc_insert_flags_t,
        document: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_insert_one(
        collection: *mut mongoc_collection_t,
        document: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_insert_many(
        collection: *mut mongoc_collection_t,
        documents: *mut *const bson_t,
        n_documents: usize,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_insert_bulk(
        collection: *mut mongoc_collection_t,
        flags: mongoc_insert_flags_t,
        documents: *mut *const bson_t,
        n_documents: u32,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_update(
        collection: *mut mongoc_collection_t,
        flags: mongoc_update_flags_t,
        selector: *const bson_t,
        update: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_update_one(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        update: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_update_many(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        update: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_replace_one(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        replacement: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_save(
        collection: *mut mongoc_collection_t,
        document: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_remove(
        collection: *mut mongoc_collection_t,
        flags: mongoc_remove_flags_t,
        selector: *const bson_t,
        write_concern: *const mongoc_write_concern_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_delete_one(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_delete_many(
        collection: *mut mongoc_collection_t,
        selector: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_rename(
        collection: *mut mongoc_collection_t,
        new_db: *const c_char,
        new_name: *const c_char,
        drop_target_before_rename: bool,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_rename_with_opts(
        collection: *mut mongoc_collection_t,
        new_db: *const c_char,
        new_name: *const c_char,
        drop_target_before_rename: bool,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_find_and_modify_with_opts(
        collection: *mut mongoc_collection_t,
        query: *const bson_t,
        opts: *const mongoc_find_and_modify_opts_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_find_and_modify(
        collection: *mut mongoc_collection_t,
        query: *const bson_t,
        sort: *const bson_t,
        update: *const bson_t,
        fields: *const bson_t,
        _remove: bool,
        upsert: bool,
        _new: bool,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_stats(
        collection: *mut mongoc_collection_t,
        options: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_create_bulk_operation(
        collection: *mut mongoc_collection_t,
        ordered: bool,
        write_concern: *const mongoc_write_concern_t,
    ) -> *mut mongoc_bulk_operation_t;
    pub fn mongoc_collection_create_bulk_operation_with_opts(
        collection: *mut mongoc_collection_t,
        opts: *const bson_t,
    ) -> *mut mongoc_bulk_operation_t;
    pub fn mongoc_collection_get_read_prefs(
        collection: *const mongoc_collection_t,
    ) -> *const mongoc_read_prefs_t;
    pub fn mongoc_collection_set_read_prefs(
        collection: *mut mongoc_collection_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
    pub fn mongoc_collection_get_read_concern(
        collection: *const mongoc_collection_t,
    ) -> *const mongoc_read_concern_t;
    pub fn mongoc_collection_set_read_concern(
        collection: *mut mongoc_collection_t,
        read_concern: *const mongoc_read_concern_t,
    );
    pub fn mongoc_collection_get_write_concern(
        collection: *const mongoc_collection_t,
    ) -> *const mongoc_write_concern_t;
    pub fn mongoc_collection_set_write_concern(
        collection: *mut mongoc_collection_t,
        write_concern: *const mongoc_write_concern_t,
    );
    pub fn mongoc_collection_get_name(
        collection: *mut mongoc_collection_t,
    ) -> *const c_char;
    pub fn mongoc_collection_get_last_error(
        collection: *const mongoc_collection_t,
    ) -> *const bson_t;
    pub fn mongoc_collection_keys_to_index_string(
        keys: *const bson_t,
    ) -> *mut c_char;
    pub fn mongoc_collection_validate(
        collection: *mut mongoc_collection_t,
        options: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_collection_watch(
        coll: *const mongoc_collection_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_change_stream_t;
    pub fn mongoc_collection_count_documents(
        coll: *mut mongoc_collection_t,
        filter: *const bson_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> i64;
    pub fn mongoc_collection_estimated_document_count(
        coll: *mut mongoc_collection_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> i64;
}

// see http://mongoc.org/libmongoc/current/mongoc_cursor_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_cursor_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_cursor_clone(cursor: *const mongoc_cursor_t) -> *mut mongoc_cursor_t;
    pub fn mongoc_cursor_destroy(cursor: *mut mongoc_cursor_t);
    pub fn mongoc_cursor_more(cursor: *mut mongoc_cursor_t) -> bool;
    pub fn mongoc_cursor_next(cursor: *mut mongoc_cursor_t, bson: *mut *const bson_t) -> bool;
    pub fn mongoc_cursor_error(cursor: *mut mongoc_cursor_t, error: *mut bson_error_t) -> bool;
    pub fn mongoc_cursor_error_document(
        cursor: *mut mongoc_cursor_t,
        error: *mut bson_error_t,
        doc: *mut *const bson_t,
    ) -> bool;
    pub fn mongoc_cursor_get_host(cursor: *mut mongoc_cursor_t, host: *mut mongoc_host_list_t);
    pub fn mongoc_cursor_is_alive(cursor: *const mongoc_cursor_t) -> bool;
    pub fn mongoc_cursor_current(cursor: *const mongoc_cursor_t) -> *const bson_t;
    pub fn mongoc_cursor_set_batch_size(cursor: *mut mongoc_cursor_t, batch_size: u32);
    pub fn mongoc_cursor_get_batch_size(cursor: *const mongoc_cursor_t) -> u32;
    pub fn mongoc_cursor_set_limit(cursor: *mut mongoc_cursor_t, limit: i64) -> bool;
    pub fn mongoc_cursor_get_limit(cursor: *const mongoc_cursor_t) -> i64;
    pub fn mongoc_cursor_set_hint(cursor: *mut mongoc_cursor_t, server_id: u32) -> bool;
    pub fn mongoc_cursor_get_hint(cursor: *const mongoc_cursor_t) -> u32;
    pub fn mongoc_cursor_get_id(cursor: *const mongoc_cursor_t) -> i64;
    pub fn mongoc_cursor_set_max_await_time_ms(
        cursor: *mut mongoc_cursor_t,
        max_await_time_ms: u32,
    );
    pub fn mongoc_cursor_get_max_await_time_ms(cursor: *const mongoc_cursor_t) -> u32;
    pub fn mongoc_cursor_new_from_command_reply(
        client: *mut mongoc_client_t,
        reply: *mut bson_t,
        server_id: u32,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_cursor_new_from_command_reply_with_opts(
        client: *mut mongoc_client_t,
        reply: *mut bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_database_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_database_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_database_get_name(
        database: *mut mongoc_database_t,
    ) -> *const c_char;
    pub fn mongoc_database_remove_user(
        database: *mut mongoc_database_t,
        username: *const c_char,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_remove_all_users(
        database: *mut mongoc_database_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_add_user(
        database: *mut mongoc_database_t,
        username: *const c_char,
        password: *const c_char,
        roles: *const bson_t,
        custom_data: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_destroy(database: *mut mongoc_database_t);
    pub fn mongoc_database_copy(database: *mut mongoc_database_t) -> *mut mongoc_database_t;
    pub fn mongoc_database_command(
        database: *mut mongoc_database_t,
        flags: mongoc_query_flags_t,
        skip: u32,
        limit: u32,
        batch_size: u32,
        command: *const bson_t,
        fields: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_database_read_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_write_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_read_write_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_command_with_opts(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        opts: *const bson_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_command_simple(
        database: *mut mongoc_database_t,
        command: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        reply: *mut bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_drop(database: *mut mongoc_database_t, error: *mut bson_error_t)
        -> bool;
    pub fn mongoc_database_drop_with_opts(
        database: *mut mongoc_database_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_has_collection(
        database: *mut mongoc_database_t,
        name: *const c_char,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_database_create_collection(
        database: *mut mongoc_database_t,
        name: *const c_char,
        options: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_collection_t;
    pub fn mongoc_database_get_read_prefs(
        database: *const mongoc_database_t,
    ) -> *const mongoc_read_prefs_t;
    pub fn mongoc_database_set_read_prefs(
        database: *mut mongoc_database_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
    pub fn mongoc_database_get_write_concern(
        database: *const mongoc_database_t,
    ) -> *const mongoc_write_concern_t;
    pub fn mongoc_database_set_write_concern(
        database: *mut mongoc_database_t,
        write_concern: *const mongoc_write_concern_t,
    );
    pub fn mongoc_database_get_read_concern(
        database: *const mongoc_database_t,
    ) -> *const mongoc_read_concern_t;
    pub fn mongoc_database_set_read_concern(
        database: *mut mongoc_database_t,
        read_concern: *const mongoc_read_concern_t,
    );
    pub fn mongoc_database_find_collections(
        database: *mut mongoc_database_t,
        filter: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_database_find_collections_with_opts(
        database: *mut mongoc_database_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_database_get_collection_names(
        database: *mut mongoc_database_t,
        error: *mut bson_error_t,
    ) -> *mut *mut c_char;
    pub fn mongoc_database_get_collection_names_with_opts(
        database: *mut mongoc_database_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut *mut c_char;
    pub fn mongoc_database_get_collection(
        database: *mut mongoc_database_t,
        name: *const c_char,
    ) -> *mut mongoc_collection_t;
    pub fn mongoc_database_watch(
        db: *const mongoc_database_t,
        pipeline: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_change_stream_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_find_and_modify_opts_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_find_and_modify_opts_t {
    _unused: [u8; 0],
}

pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_NONE:
    mongoc_find_and_modify_flags_t = 0;
pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_REMOVE:
    mongoc_find_and_modify_flags_t = 1;
pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_UPSERT:
    mongoc_find_and_modify_flags_t = 2;
pub const mongoc_find_and_modify_flags_t_MONGOC_FIND_AND_MODIFY_RETURN_NEW:
    mongoc_find_and_modify_flags_t = 4;
pub type mongoc_find_and_modify_flags_t = u32;

extern "C" {
    pub fn mongoc_find_and_modify_opts_new() -> *mut mongoc_find_and_modify_opts_t;
    pub fn mongoc_find_and_modify_opts_set_sort(
        opts: *mut mongoc_find_and_modify_opts_t,
        sort: *const bson_t,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_get_sort(
        opts: *const mongoc_find_and_modify_opts_t,
        sort: *mut bson_t,
    );
    pub fn mongoc_find_and_modify_opts_set_update(
        opts: *mut mongoc_find_and_modify_opts_t,
        update: *const bson_t,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_get_update(
        opts: *const mongoc_find_and_modify_opts_t,
        update: *mut bson_t,
    );
    pub fn mongoc_find_and_modify_opts_set_fields(
        opts: *mut mongoc_find_and_modify_opts_t,
        fields: *const bson_t,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_get_fields(
        opts: *const mongoc_find_and_modify_opts_t,
        fields: *mut bson_t,
    );
    pub fn mongoc_find_and_modify_opts_set_flags(
        opts: *mut mongoc_find_and_modify_opts_t,
        flags: mongoc_find_and_modify_flags_t,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_get_flags(
        opts: *const mongoc_find_and_modify_opts_t,
    ) -> mongoc_find_and_modify_flags_t;
    pub fn mongoc_find_and_modify_opts_set_bypass_document_validation(
        opts: *mut mongoc_find_and_modify_opts_t,
        bypass: bool,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_get_bypass_document_validation(
        opts: *const mongoc_find_and_modify_opts_t,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_set_max_time_ms(
        opts: *mut mongoc_find_and_modify_opts_t,
        max_time_ms: u32,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_get_max_time_ms(
        opts: *const mongoc_find_and_modify_opts_t,
    ) -> u32;
    pub fn mongoc_find_and_modify_opts_append(
        opts: *mut mongoc_find_and_modify_opts_t,
        extra: *const bson_t,
    ) -> bool;
    pub fn mongoc_find_and_modify_opts_get_extra(
        opts: *const mongoc_find_and_modify_opts_t,
        extra: *mut bson_t,
    );
    pub fn mongoc_find_and_modify_opts_destroy(opts: *mut mongoc_find_and_modify_opts_t);
}

// see http://mongoc.org/libmongoc/current/mongoc_gridfs_file_list_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_file_list_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_file_list_t = _mongoc_gridfs_file_list_t;
extern "C" {
    pub fn mongoc_gridfs_file_list_next(
        list: *mut mongoc_gridfs_file_list_t,
    ) -> *mut mongoc_gridfs_file_t;
    pub fn mongoc_gridfs_file_list_destroy(list: *mut mongoc_gridfs_file_list_t);
    pub fn mongoc_gridfs_file_list_error(
        list: *mut mongoc_gridfs_file_list_t,
        error: *mut bson_error_t,
    ) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_gridfs_file_opt_t.html
pub type mongoc_gridfs_file_opt_t = _mongoc_gridfs_file_opt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_file_opt_t {
    pub md5: *const c_char,
    pub filename: *const c_char,
    pub content_type: *const c_char,
    pub aliases: *const bson_t,
    pub metadata: *const bson_t,
    pub chunk_size: u32,
}

// see http://mongoc.org/libmongoc/current/mongoc_gridfs_file_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_file_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_file_t = _mongoc_gridfs_file_t;

extern "C" {
    pub fn mongoc_gridfs_file_get_md5(
        file: *mut mongoc_gridfs_file_t,
    ) -> *const c_char;
    pub fn mongoc_gridfs_file_set_md5(
        file: *mut mongoc_gridfs_file_t,
        str: *const c_char,
    );
    pub fn mongoc_gridfs_file_get_filename(
        file: *mut mongoc_gridfs_file_t,
    ) -> *const c_char;
    pub fn mongoc_gridfs_file_set_filename(
        file: *mut mongoc_gridfs_file_t,
        str: *const c_char,
    );
    pub fn mongoc_gridfs_file_get_content_type(
        file: *mut mongoc_gridfs_file_t,
    ) -> *const c_char;
    pub fn mongoc_gridfs_file_set_content_type(
        file: *mut mongoc_gridfs_file_t,
        str: *const c_char,
    );
    pub fn mongoc_gridfs_file_get_aliases(file: *mut mongoc_gridfs_file_t) -> *const bson_t;
    pub fn mongoc_gridfs_file_set_aliases(file: *mut mongoc_gridfs_file_t, bson: *const bson_t);
    pub fn mongoc_gridfs_file_get_metadata(file: *mut mongoc_gridfs_file_t) -> *const bson_t;
    pub fn mongoc_gridfs_file_set_metadata(file: *mut mongoc_gridfs_file_t, bson: *const bson_t);
    pub fn mongoc_gridfs_file_get_id(file: *mut mongoc_gridfs_file_t) -> *const bson_value_t;
    pub fn mongoc_gridfs_file_get_length(file: *mut mongoc_gridfs_file_t) -> i64;
    pub fn mongoc_gridfs_file_get_chunk_size(file: *mut mongoc_gridfs_file_t) -> i32;
    pub fn mongoc_gridfs_file_get_upload_date(file: *mut mongoc_gridfs_file_t) -> i64;
    pub fn mongoc_gridfs_file_writev(
        file: *mut mongoc_gridfs_file_t,
        iov: *const mongoc_iovec_t,
        iovcnt: usize,
        timeout_msec: u32,
    ) -> isize;
    pub fn mongoc_gridfs_file_readv(
        file: *mut mongoc_gridfs_file_t,
        iov: *mut mongoc_iovec_t,
        iovcnt: usize,
        min_bytes: usize,
        timeout_msec: u32,
    ) -> isize;
    pub fn mongoc_gridfs_file_seek(
        file: *mut mongoc_gridfs_file_t,
        delta: i64,
        whence: c_int,
    ) -> c_int;
    pub fn mongoc_gridfs_file_tell(file: *mut mongoc_gridfs_file_t) -> u64;
    pub fn mongoc_gridfs_file_set_id(
        file: *mut mongoc_gridfs_file_t,
        id: *const bson_value_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_gridfs_file_save(file: *mut mongoc_gridfs_file_t) -> bool;
    pub fn mongoc_gridfs_file_destroy(file: *mut mongoc_gridfs_file_t);
    pub fn mongoc_gridfs_file_error(
        file: *mut mongoc_gridfs_file_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_gridfs_file_remove(
        file: *mut mongoc_gridfs_file_t,
        error: *mut bson_error_t,
    ) -> bool;
}

extern "C" {
    pub fn mongoc_stream_gridfs_new(file: *mut mongoc_gridfs_file_t) -> *mut mongoc_stream_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_gridfs_bucket_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_bucket_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_bucket_t = _mongoc_gridfs_bucket_t;
extern "C" {
    pub fn mongoc_gridfs_bucket_new(
        db: *mut mongoc_database_t,
        opts: *const bson_t,
        read_prefs: *const mongoc_read_prefs_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_bucket_t;
    pub fn mongoc_gridfs_bucket_open_upload_stream(
        bucket: *mut mongoc_gridfs_bucket_t,
        filename: *const c_char,
        opts: *const bson_t,
        file_id: *mut bson_value_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_stream_t;
    pub fn mongoc_gridfs_bucket_open_upload_stream_with_id(
        bucket: *mut mongoc_gridfs_bucket_t,
        file_id: *const bson_value_t,
        filename: *const c_char,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_stream_t;
    pub fn mongoc_gridfs_bucket_upload_from_stream(
        bucket: *mut mongoc_gridfs_bucket_t,
        filename: *const c_char,
        source: *mut mongoc_stream_t,
        opts: *const bson_t,
        file_id: *mut bson_value_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_gridfs_bucket_upload_from_stream_with_id(
        bucket: *mut mongoc_gridfs_bucket_t,
        file_id: *const bson_value_t,
        filename: *const c_char,
        source: *mut mongoc_stream_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_gridfs_bucket_open_download_stream(
        bucket: *mut mongoc_gridfs_bucket_t,
        file_id: *const bson_value_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_stream_t;
    pub fn mongoc_gridfs_bucket_download_to_stream(
        bucket: *mut mongoc_gridfs_bucket_t,
        file_id: *const bson_value_t,
        destination: *mut mongoc_stream_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_gridfs_bucket_delete_by_id(
        bucket: *mut mongoc_gridfs_bucket_t,
        file_id: *const bson_value_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_gridfs_bucket_find(
        bucket: *mut mongoc_gridfs_bucket_t,
        filter: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_cursor_t;
    pub fn mongoc_gridfs_bucket_stream_error(
        stream: *mut mongoc_stream_t,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_gridfs_bucket_destroy(bucket: *mut mongoc_gridfs_bucket_t);
    pub fn mongoc_gridfs_bucket_abort_upload(stream: *mut mongoc_stream_t) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_gridfs_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_gridfs_t {
    _unused: [u8; 0],
}
pub type mongoc_gridfs_t = _mongoc_gridfs_t;
extern "C" {
    pub fn mongoc_gridfs_create_file_from_stream(
        gridfs: *mut mongoc_gridfs_t,
        stream: *mut mongoc_stream_t,
        opt: *mut mongoc_gridfs_file_opt_t,
    ) -> *mut mongoc_gridfs_file_t;
    pub fn mongoc_gridfs_create_file(
        gridfs: *mut mongoc_gridfs_t,
        opt: *mut mongoc_gridfs_file_opt_t,
    ) -> *mut mongoc_gridfs_file_t;
    pub fn mongoc_gridfs_find(
        gridfs: *mut mongoc_gridfs_t,
        query: *const bson_t,
    ) -> *mut mongoc_gridfs_file_list_t;
    pub fn mongoc_gridfs_find_one(
        gridfs: *mut mongoc_gridfs_t,
        query: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_file_t;
    pub fn mongoc_gridfs_find_with_opts(
        gridfs: *mut mongoc_gridfs_t,
        filter: *const bson_t,
        opts: *const bson_t,
    ) -> *mut mongoc_gridfs_file_list_t;
    pub fn mongoc_gridfs_find_one_with_opts(
        gridfs: *mut mongoc_gridfs_t,
        filter: *const bson_t,
        opts: *const bson_t,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_file_t;
    pub fn mongoc_gridfs_find_one_by_filename(
        gridfs: *mut mongoc_gridfs_t,
        filename: *const c_char,
        error: *mut bson_error_t,
    ) -> *mut mongoc_gridfs_file_t;
    pub fn mongoc_gridfs_drop(gridfs: *mut mongoc_gridfs_t, error: *mut bson_error_t) -> bool;
    pub fn mongoc_gridfs_destroy(gridfs: *mut mongoc_gridfs_t);
    pub fn mongoc_gridfs_get_files(gridfs: *mut mongoc_gridfs_t) -> *mut mongoc_collection_t;
    pub fn mongoc_gridfs_get_chunks(gridfs: *mut mongoc_gridfs_t) -> *mut mongoc_collection_t;
    pub fn mongoc_gridfs_remove_by_filename(
        gridfs: *mut mongoc_gridfs_t,
        filename: *const c_char,
        error: *mut bson_error_t,
    ) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_host_list_t.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mongoc_host_list_t {
    pub next: *mut mongoc_host_list_t,
    pub host: [c_char; 256usize],
    pub host_and_port: [c_char; 262usize],
    pub port: u16,
    pub family: c_int,
    pub padding: [*mut c_void; 4usize]
}

// see http://mongoc.org/libmongoc/current/mongoc_index_opt_geo_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_geo_t {
    pub twod_sphere_version: u8,
    pub twod_bits_precision: u8,
    pub twod_location_min: f64,
    pub twod_location_max: f64,
    pub haystack_bucket_size: f64,
    pub padding: [*mut u8; 32usize],
}

// see http://mongoc.org/libmongoc/current/mongoc_index_opt_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_t {
    pub is_initialized: bool,
    pub background: bool,
    pub unique: bool,
    pub name: *const c_char,
    pub drop_dups: bool,
    pub sparse: bool,
    pub expire_after_seconds: i32,
    pub v: i32,
    pub weights: *const bson_t,
    pub default_language: *const c_char,
    pub language_override: *const c_char,
    pub geo_options: *mut mongoc_index_opt_geo_t,
    pub storage_options: *mut mongoc_index_opt_storage_t,
    pub partial_filter_expression: *const bson_t,
    pub collation: *const bson_t,
    pub padding: [*mut c_void; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_storage_t {
    pub type_: c_int,
}

// see http://mongoc.org/libmongoc/current/mongoc_index_opt_wt_t.html
pub const mongoc_index_storage_opt_type_t_MONGOC_INDEX_STORAGE_OPT_MMAPV1:
    mongoc_index_storage_opt_type_t = 0;
pub const mongoc_index_storage_opt_type_t_MONGOC_INDEX_STORAGE_OPT_WIREDTIGER:
    mongoc_index_storage_opt_type_t = 1;
pub type mongoc_index_storage_opt_type_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_index_opt_wt_t {
    pub base: mongoc_index_opt_storage_t,
    pub config_str: *const c_char,
    pub padding: [*mut c_void; 8usize],
}

// see http://mongoc.org/libmongoc/current/mongoc_insert_flags_t.html
pub const mongoc_insert_flags_t_MONGOC_INSERT_NONE: mongoc_insert_flags_t = 0;
pub const mongoc_insert_flags_t_MONGOC_INSERT_CONTINUE_ON_ERROR: mongoc_insert_flags_t = 1;
pub type mongoc_insert_flags_t = u32;

// see http://mongoc.org/libmongoc/current/mongoc_query_flags_t.html
pub const mongoc_query_flags_t_MONGOC_QUERY_NONE: mongoc_query_flags_t = 0;
pub const mongoc_query_flags_t_MONGOC_QUERY_TAILABLE_CURSOR: mongoc_query_flags_t = 2;
pub const mongoc_query_flags_t_MONGOC_QUERY_SLAVE_OK: mongoc_query_flags_t = 4;
pub const mongoc_query_flags_t_MONGOC_QUERY_OPLOG_REPLAY: mongoc_query_flags_t = 8;
pub const mongoc_query_flags_t_MONGOC_QUERY_NO_CURSOR_TIMEOUT: mongoc_query_flags_t = 16;
pub const mongoc_query_flags_t_MONGOC_QUERY_AWAIT_DATA: mongoc_query_flags_t = 32;
pub const mongoc_query_flags_t_MONGOC_QUERY_EXHAUST: mongoc_query_flags_t = 64;
pub const mongoc_query_flags_t_MONGOC_QUERY_PARTIAL: mongoc_query_flags_t = 128;
pub type mongoc_query_flags_t = u32;

// see http://mongoc.org/libmongoc/current/mongoc_rand.html
extern "C" {
    pub fn mongoc_rand_seed(buf: *const c_void, num: c_int);
}
extern "C" {
    pub fn mongoc_rand_add(
        buf: *const c_void,
        num: c_int,
        entropy: f64,
    );
}
extern "C" {
    pub fn mongoc_rand_status() -> c_int;
}

// see http://mongoc.org/libmongoc/current/mongoc_read_concern_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_read_concern_t {
    _unused: [u8; 0],
}

pub const MONGOC_READ_CONCERN_LEVEL_AVAILABLE: &'static [u8; 10usize] = b"available\0";
pub const MONGOC_READ_CONCERN_LEVEL_LOCAL: &'static [u8; 6usize] = b"local\0";
pub const MONGOC_READ_CONCERN_LEVEL_MAJORITY: &'static [u8; 9usize] = b"majority\0";
pub const MONGOC_READ_CONCERN_LEVEL_LINEARIZABLE: &'static [u8; 13usize] = b"linearizable\0";
pub const MONGOC_READ_CONCERN_LEVEL_SNAPSHOT: &'static [u8; 9usize] = b"snapshot\0";

extern "C" {
    pub fn mongoc_read_concern_new() -> *mut mongoc_read_concern_t;
    pub fn mongoc_read_concern_copy(
        read_concern: *const mongoc_read_concern_t,
    ) -> *mut mongoc_read_concern_t;
    pub fn mongoc_read_concern_destroy(read_concern: *mut mongoc_read_concern_t);
    pub fn mongoc_read_concern_get_level(
        read_concern: *const mongoc_read_concern_t,
    ) -> *const c_char;
    pub fn mongoc_read_concern_set_level(
        read_concern: *mut mongoc_read_concern_t,
        level: *const c_char,
    ) -> bool;
    pub fn mongoc_read_concern_append(
        read_concern: *mut mongoc_read_concern_t,
        doc: *mut bson_t,
    ) -> bool;
    pub fn mongoc_read_concern_is_default(read_concern: *const mongoc_read_concern_t) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_read_mode_t.html
pub const mongoc_read_mode_t_MONGOC_READ_PRIMARY: mongoc_read_mode_t = 1;
pub const mongoc_read_mode_t_MONGOC_READ_SECONDARY: mongoc_read_mode_t = 2;
pub const mongoc_read_mode_t_MONGOC_READ_PRIMARY_PREFERRED: mongoc_read_mode_t = 5;
pub const mongoc_read_mode_t_MONGOC_READ_SECONDARY_PREFERRED: mongoc_read_mode_t = 6;
pub const mongoc_read_mode_t_MONGOC_READ_NEAREST: mongoc_read_mode_t = 10;
pub type mongoc_read_mode_t = u32;

// see http://mongoc.org/libmongoc/current/mongoc_read_prefs_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_read_prefs_t {
    _unused: [u8; 0],
}

pub const MONGOC_READ_PRIMARY: u32 = 1;
pub const MONGOC_READ_SECONDARY: u32 = 2;
pub const MONGOC_READ_PRIMARY_PREFERRED: u32 = 5;
pub const MONGOC_READ_SECONDARY_PREFERRED: u32 = 6;
pub const MONGOC_READ_NEAREST: u32 = 10;

extern "C" {
    pub fn mongoc_read_prefs_new(read_mode: u32) -> *mut mongoc_read_prefs_t;
    pub fn mongoc_read_prefs_copy(
        read_prefs: *const mongoc_read_prefs_t,
    ) -> *mut mongoc_read_prefs_t;
    pub fn mongoc_read_prefs_destroy(read_prefs: *mut mongoc_read_prefs_t);
    pub fn mongoc_read_prefs_get_mode(read_prefs: *const mongoc_read_prefs_t) -> u32;
    pub fn mongoc_read_prefs_set_mode(
        read_prefs: *mut mongoc_read_prefs_t,
        mode: u32,
    );
    pub fn mongoc_read_prefs_get_tags(read_prefs: *const mongoc_read_prefs_t) -> *const bson_t;
    pub fn mongoc_read_prefs_set_tags(read_prefs: *mut mongoc_read_prefs_t, tags: *const bson_t);
    pub fn mongoc_read_prefs_add_tag(read_prefs: *mut mongoc_read_prefs_t, tag: *const bson_t);
    pub fn mongoc_read_prefs_get_max_staleness_seconds(
        read_prefs: *const mongoc_read_prefs_t
    ) -> i64;
    pub fn mongoc_read_prefs_set_max_staleness_seconds(
        read_prefs: *mut mongoc_read_prefs_t,
        max_staleness_seconds: i64
    );
    pub fn mongoc_read_prefs_is_valid(read_prefs: *const mongoc_read_prefs_t) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_remove_flags_t.html
pub const mongoc_remove_flags_t_MONGOC_REMOVE_NONE: mongoc_remove_flags_t = 0;
pub const mongoc_remove_flags_t_MONGOC_REMOVE_SINGLE_REMOVE: mongoc_remove_flags_t = 1;
pub type mongoc_remove_flags_t = u32;

// see http://mongoc.org/libmongoc/current/mongoc_reply_flags_t.html
pub const mongoc_reply_flags_t_MONGOC_REPLY_NONE: mongoc_reply_flags_t = 0;
pub const mongoc_reply_flags_t_MONGOC_REPLY_CURSOR_NOT_FOUND: mongoc_reply_flags_t = 1;
pub const mongoc_reply_flags_t_MONGOC_REPLY_QUERY_FAILURE: mongoc_reply_flags_t = 2;
pub const mongoc_reply_flags_t_MONGOC_REPLY_SHARD_CONFIG_STALE: mongoc_reply_flags_t = 4;
pub const mongoc_reply_flags_t_MONGOC_REPLY_AWAIT_CAPABLE: mongoc_reply_flags_t = 8;
pub type mongoc_reply_flags_t = u32;

// see http://mongoc.org/libmongoc/current/mongoc_server_description_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_server_description_t {
    _unused: [u8; 0],
}
pub type mongoc_server_description_t = _mongoc_server_description_t;
extern "C" {
    pub fn mongoc_server_description_destroy(description: *mut mongoc_server_description_t);
    pub fn mongoc_server_description_new_copy(
        description: *const mongoc_server_description_t,
    ) -> *mut mongoc_server_description_t;
    pub fn mongoc_server_description_id(description: *const mongoc_server_description_t) -> u32;
    pub fn mongoc_server_description_host(
        description: *const mongoc_server_description_t,
    ) -> *mut mongoc_host_list_t;
    pub fn mongoc_server_description_last_update_time(
        description: *const mongoc_server_description_t,
    ) -> i64;
    pub fn mongoc_server_description_round_trip_time(
        description: *const mongoc_server_description_t,
    ) -> i64;
    pub fn mongoc_server_description_type(
        description: *const mongoc_server_description_t,
    ) -> *const c_char;
    pub fn mongoc_server_description_ismaster(
        description: *const mongoc_server_description_t,
    ) -> *const bson_t;
    pub fn mongoc_server_description_compressor_id(
        description: *const mongoc_server_description_t,
    ) -> i32;
}

// see http://mongoc.org/libmongoc/current/mongoc_session_opt_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_session_opt_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_session_opts_new() -> *mut mongoc_session_opt_t;
    pub fn mongoc_session_opts_set_causal_consistency(
        opts: *mut mongoc_session_opt_t,
        causal_consistency: bool,
    );
    pub fn mongoc_session_opts_get_causal_consistency(opts: *const mongoc_session_opt_t) -> bool;
    pub fn mongoc_session_opts_set_default_transaction_opts(
        opts: *mut mongoc_session_opt_t,
        txn_opts: *const mongoc_transaction_opt_t,
    );
    pub fn mongoc_session_opts_get_default_transaction_opts(
        opts: *const mongoc_session_opt_t,
    ) -> *const mongoc_transaction_opt_t;
    pub fn mongoc_session_opts_clone(
        opts: *const mongoc_session_opt_t,
    ) -> *mut mongoc_session_opt_t;
    pub fn mongoc_session_opts_destroy(opts: *mut mongoc_session_opt_t);
}

// see http://mongoc.org/libmongoc/current/mongoc_ssl_opt_t.html
pub type mongoc_ssl_opt_t = _mongoc_ssl_opt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_ssl_opt_t {
    pub pem_file: *const c_char,
    pub pem_pwd: *const c_char,
    pub ca_file: *const c_char,
    pub ca_dir: *const c_char,
    pub crl_file: *const c_char,
    pub weak_cert_validation: bool,
    pub allow_invalid_hostname: bool,
    pub padding: [*mut c_void; 7usize],
}

extern "C" {
    pub fn mongoc_ssl_opt_get_default() -> *const mongoc_ssl_opt_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_stream_socket_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_stream_socket_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_socket_t {
    _unused: [u8; 0],
}

pub type mongoc_socklen_t = c_uint;

extern "C" {
    pub fn mongoc_stream_socket_new(socket: *mut mongoc_socket_t) -> *mut mongoc_stream_t;
}
extern "C" {
    pub fn mongoc_stream_socket_get_socket(
        stream: *mut mongoc_stream_socket_t,
    ) -> *mut mongoc_socket_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_stream_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_stream_poll_t {
    pub stream: *mut mongoc_stream_t,
    pub events: c_int,
    pub revents: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_stream_t {
    pub type_: c_int,
    pub destroy: Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t)>,
    pub close: Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> c_int>,
    pub flush: Option<
        unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> c_int,
    >,
    pub writev: Option<
        unsafe extern "C" fn(
            stream: *mut mongoc_stream_t,
            iov: *mut mongoc_iovec_t,
            iovcnt: usize,
            timeout_msec: i32,
        ) -> isize,
    >,
    pub readv: Option<
        unsafe extern "C" fn(
            stream: *mut mongoc_stream_t,
            iov: *mut mongoc_iovec_t,
            iovcnt: usize,
            min_bytes: usize,
            timeout_msec: i32,
        ) -> isize,
    >,
    pub setsockopt: Option<
        unsafe extern "C" fn(
            stream: *mut mongoc_stream_t,
            level: c_int,
            optname: c_int,
            optval: *mut c_void,
            optlen: mongoc_socklen_t,
        ) -> c_int,
    >,
    pub get_base_stream: Option<
        unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> *mut mongoc_stream_t,
    >,
    pub check_closed: Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> bool>,
    pub poll: Option<
        unsafe extern "C" fn(
            streams: *mut mongoc_stream_poll_t,
            nstreams: usize,
            timeout: i32,
        ) -> isize,
    >,
    pub failed: Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t)>,
    pub timed_out: Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> bool>,
    pub should_retry: Option<unsafe extern "C" fn(stream: *mut mongoc_stream_t) -> bool>,
    pub padding: [*mut c_void; 3usize],
}

extern "C" {
    pub fn mongoc_stream_get_base_stream(stream: *mut mongoc_stream_t) -> *mut mongoc_stream_t;
    pub fn mongoc_stream_get_tls_stream(stream: *mut mongoc_stream_t) -> *mut mongoc_stream_t;
    pub fn mongoc_stream_close(stream: *mut mongoc_stream_t) -> c_int;
    pub fn mongoc_stream_destroy(stream: *mut mongoc_stream_t);
    pub fn mongoc_stream_failed(stream: *mut mongoc_stream_t);
    pub fn mongoc_stream_flush(stream: *mut mongoc_stream_t) -> c_int;
    pub fn mongoc_stream_writev(
        stream: *mut mongoc_stream_t,
        iov: *mut mongoc_iovec_t,
        iovcnt: usize,
        timeout_msec: i32,
    ) -> isize;
    pub fn mongoc_stream_write(
        stream: *mut mongoc_stream_t,
        buf: *mut c_void,
        count: usize,
        timeout_msec: i32,
    ) -> isize;
    pub fn mongoc_stream_readv(
        stream: *mut mongoc_stream_t,
        iov: *mut mongoc_iovec_t,
        iovcnt: usize,
        min_bytes: usize,
        timeout_msec: i32,
    ) -> isize;
    pub fn mongoc_stream_read(
        stream: *mut mongoc_stream_t,
        buf: *mut c_void,
        count: usize,
        min_bytes: usize,
        timeout_msec: i32,
    ) -> isize;
    pub fn mongoc_stream_setsockopt(
        stream: *mut mongoc_stream_t,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: mongoc_socklen_t,
    ) -> c_int;
    pub fn mongoc_stream_check_closed(stream: *mut mongoc_stream_t) -> bool;
    pub fn mongoc_stream_timed_out(stream: *mut mongoc_stream_t) -> bool;
    pub fn mongoc_stream_should_retry(stream: *mut mongoc_stream_t) -> bool;
    pub fn mongoc_stream_poll(
        streams: *mut mongoc_stream_poll_t,
        nstreams: usize,
        timeout: i32,
    ) -> isize;
}

// see http://mongoc.org/libmongoc/current/mongoc_stream_tls_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_stream_tls_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_stream_tls_handshake(
        stream: *mut mongoc_stream_t,
        host: *const c_char,
        timeout_msec: i32,
        events: *mut c_int,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_stream_tls_handshake_block(
        stream: *mut mongoc_stream_t,
        host: *const c_char,
        timeout_msec: i32,
        error: *mut bson_error_t,
    ) -> bool;
    pub fn mongoc_stream_tls_do_handshake(stream: *mut mongoc_stream_t, timeout_msec: i32) -> bool;
    pub fn mongoc_stream_tls_check_cert(
        stream: *mut mongoc_stream_t,
        host: *const c_char,
    ) -> bool;
    pub fn mongoc_stream_tls_new_with_hostname(
        base_stream: *mut mongoc_stream_t,
        host: *const c_char,
        opt: *mut mongoc_ssl_opt_t,
        client: c_int,
    ) -> *mut mongoc_stream_t;
    pub fn mongoc_stream_tls_new(
        base_stream: *mut mongoc_stream_t,
        opt: *mut mongoc_ssl_opt_t,
        client: c_int,
    ) -> *mut mongoc_stream_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_topology_description_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _mongoc_topology_description_t {
    _unused: [u8; 0],
}
pub type mongoc_topology_description_t = _mongoc_topology_description_t;
extern "C" {
    pub fn mongoc_topology_description_has_readable_server(
        td: *mut mongoc_topology_description_t,
        prefs: *const mongoc_read_prefs_t,
    ) -> bool;
    pub fn mongoc_topology_description_has_writable_server(
        td: *mut mongoc_topology_description_t,
    ) -> bool;
    pub fn mongoc_topology_description_type(
        td: *const mongoc_topology_description_t,
    ) -> *const c_char;
    pub fn mongoc_topology_description_get_servers(
        td: *const mongoc_topology_description_t,
        n: *mut usize,
    ) -> *mut *mut mongoc_server_description_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_transaction_opt_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_transaction_opt_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_transaction_opts_new() -> *mut mongoc_transaction_opt_t;
    pub fn mongoc_transaction_opts_clone(
        opts: *const mongoc_transaction_opt_t,
    ) -> *mut mongoc_transaction_opt_t;
    pub fn mongoc_transaction_opts_destroy(opts: *mut mongoc_transaction_opt_t);
    pub fn mongoc_transaction_opts_set_read_concern(
        opts: *mut mongoc_transaction_opt_t,
        read_concern: *const mongoc_read_concern_t,
    );
    pub fn mongoc_transaction_opts_get_read_concern(
        opts: *const mongoc_transaction_opt_t,
    ) -> *const mongoc_read_concern_t;
    pub fn mongoc_transaction_opts_set_write_concern(
        opts: *mut mongoc_transaction_opt_t,
        write_concern: *const mongoc_write_concern_t,
    );
    pub fn mongoc_transaction_opts_get_write_concern(
        opts: *const mongoc_transaction_opt_t,
    ) -> *const mongoc_write_concern_t;
    pub fn mongoc_transaction_opts_set_read_prefs(
        opts: *mut mongoc_transaction_opt_t,
        read_prefs: *const mongoc_read_prefs_t,
    );
    pub fn mongoc_transaction_opts_get_read_prefs(
        opts: *const mongoc_transaction_opt_t,
    ) -> *const mongoc_read_prefs_t;
}

// see http://mongoc.org/libmongoc/current/mongoc_update_flags_t.html
pub const mongoc_update_flags_t_MONGOC_UPDATE_NONE: mongoc_update_flags_t = 0;
pub const mongoc_update_flags_t_MONGOC_UPDATE_UPSERT: mongoc_update_flags_t = 1;
pub const mongoc_update_flags_t_MONGOC_UPDATE_MULTI_UPDATE: mongoc_update_flags_t = 2;
pub type mongoc_update_flags_t = u32;

// see: http://mongoc.org/libmongoc/current/mongoc_uri_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_uri_t {
    _unused: [u8; 0]
}

extern "C" {
    pub fn mongoc_uri_copy(uri: *const mongoc_uri_t) -> *mut mongoc_uri_t;
    pub fn mongoc_uri_destroy(uri: *mut mongoc_uri_t);
    pub fn mongoc_uri_new(uri_string: *const c_char) -> *mut mongoc_uri_t;
    pub fn mongoc_uri_new_with_error(
        uri_string: *const c_char,
        error: *mut bson_error_t
    ) -> *mut mongoc_uri_t;
    pub fn mongoc_uri_new_for_host_port(
        hostname: *const c_char,
        port: u16
    ) -> *mut mongoc_uri_t;
    pub fn mongoc_uri_get_hosts(uri: *const mongoc_uri_t) -> *const mongoc_host_list_t;
    pub fn mongoc_uri_get_service(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_get_database(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_set_database(
        uri: *mut mongoc_uri_t,
        database: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_get_compressors(uri: *const mongoc_uri_t) -> *const bson_t;
    pub fn mongoc_uri_get_options(uri: *const mongoc_uri_t) -> *const bson_t;
    pub fn mongoc_uri_get_password(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_set_password(
        uri: *mut mongoc_uri_t,
        password: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_option_is_int32(key: *const c_char) -> bool;
    pub fn mongoc_uri_option_is_bool(key: *const c_char) -> bool;
    pub fn mongoc_uri_option_is_utf8(key: *const c_char) -> bool;
    pub fn mongoc_uri_get_option_as_int32(
        uri: *const mongoc_uri_t,
        option: *const c_char,
        fallback: i32,
    ) -> i32;
    pub fn mongoc_uri_get_option_as_bool(
        uri: *const mongoc_uri_t,
        option: *const c_char,
        fallback: bool,
    ) -> bool;
    pub fn mongoc_uri_get_option_as_utf8(
        uri: *const mongoc_uri_t,
        option: *const c_char,
        fallback: *const c_char,
    ) -> *const c_char;
    pub fn mongoc_uri_set_option_as_int32(
        uri: *mut mongoc_uri_t,
        option: *const c_char,
        value: i32,
    ) -> bool;
    pub fn mongoc_uri_set_option_as_bool(
        uri: *mut mongoc_uri_t,
        option: *const c_char,
        value: bool,
    ) -> bool;
    pub fn mongoc_uri_set_option_as_utf8(
        uri: *mut mongoc_uri_t,
        option: *const c_char,
        value: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_get_read_prefs(uri: *const mongoc_uri_t) -> *const bson_t;
    pub fn mongoc_uri_get_replica_set(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_get_string(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_get_username(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_set_username(
        uri: *mut mongoc_uri_t,
        username: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_get_credentials(uri: *const mongoc_uri_t) -> *const bson_t;
    pub fn mongoc_uri_get_auth_source(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_set_auth_source(
        uri: *mut mongoc_uri_t,
        value: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_get_appname(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_set_appname(
        uri: *mut mongoc_uri_t,
        value: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_set_compressors(
        uri: *mut mongoc_uri_t,
        value: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_get_auth_mechanism(uri: *const mongoc_uri_t) -> *const c_char;
    pub fn mongoc_uri_set_auth_mechanism(
        uri: *mut mongoc_uri_t,
        value: *const c_char,
    ) -> bool;
    pub fn mongoc_uri_get_mechanism_properties(
        uri: *const mongoc_uri_t,
        properties: *mut bson_t,
    ) -> bool;
    pub fn mongoc_uri_set_mechanism_properties(
        uri: *mut mongoc_uri_t,
        properties: *const bson_t,
    ) -> bool;
    pub fn mongoc_uri_get_ssl(uri: *const mongoc_uri_t) -> bool;
    pub fn mongoc_uri_unescape(
        escaped_string: *const c_char,
    ) -> *mut c_char;
    pub fn mongoc_uri_get_read_prefs_t(uri: *const mongoc_uri_t) -> *const mongoc_read_prefs_t;
    pub fn mongoc_uri_set_read_prefs_t(uri: *mut mongoc_uri_t, prefs: *const mongoc_read_prefs_t);
    pub fn mongoc_uri_get_write_concern(uri: *const mongoc_uri_t) -> *const mongoc_write_concern_t;
    pub fn mongoc_uri_set_write_concern(uri: *mut mongoc_uri_t, wc: *const mongoc_write_concern_t);
    pub fn mongoc_uri_get_read_concern(uri: *const mongoc_uri_t) -> *const mongoc_read_concern_t;
    pub fn mongoc_uri_set_read_concern(uri: *mut mongoc_uri_t, rc: *const mongoc_read_concern_t);
}

// see http://mongoc.org/libmongoc/current/mongoc_version.html
extern "C" {
    pub fn mongoc_get_major_version() -> c_int;
    pub fn mongoc_get_minor_version() -> c_int;
    pub fn mongoc_get_micro_version() -> c_int;
    pub fn mongoc_get_version() -> *const c_char;
    pub fn mongoc_check_version(
        required_major: c_int,
        required_minor: c_int,
        required_micro: c_int,
    ) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_write_concern_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_write_concern_t {
    _unused: [u8; 0],
}

pub const MONGOC_WRITE_CONCERN_W_UNACKNOWLEDGED: u32 = 0;
pub const MONGOC_WRITE_CONCERN_W_ERRORS_IGNORED: i32 = -1;
pub const MONGOC_WRITE_CONCERN_W_DEFAULT: i32 = -2;
pub const MONGOC_WRITE_CONCERN_W_MAJORITY: i32 = -3;
pub const MONGOC_WRITE_CONCERN_W_TAG: i32 = -4;

extern "C" {
    pub fn mongoc_write_concern_new() -> *mut mongoc_write_concern_t;
    pub fn mongoc_write_concern_copy(
        write_concern: *const mongoc_write_concern_t
    ) -> *mut mongoc_write_concern_t;
    pub fn mongoc_write_concern_destroy(write_concern: *mut mongoc_write_concern_t);
    pub fn mongoc_write_concern_get_fsync(write_concern: *const mongoc_write_concern_t) -> bool;
    pub fn mongoc_write_concern_set_fsync(write_concern: *mut mongoc_write_concern_t, fsync_: bool);
    pub fn mongoc_write_concern_get_journal(write_concern: *const mongoc_write_concern_t) -> bool;
    pub fn mongoc_write_concern_journal_is_set(
        write_concern: *const mongoc_write_concern_t,
    ) -> bool;
    pub fn mongoc_write_concern_set_journal(
        write_concern: *mut mongoc_write_concern_t,
        journal: bool,
    );
    pub fn mongoc_write_concern_get_w(write_concern: *const mongoc_write_concern_t) -> i32;
    pub fn mongoc_write_concern_set_w(write_concern: *mut mongoc_write_concern_t, w: i32);
    pub fn mongoc_write_concern_get_wtag(
        write_concern: *const mongoc_write_concern_t,
    ) -> *const c_char;
    pub fn mongoc_write_concern_set_wtag(
        write_concern: *mut mongoc_write_concern_t,
        tag: *const c_char
    );
    pub fn mongoc_write_concern_get_wtimeout(write_concern: *const mongoc_write_concern_t) -> i32;
    pub fn mongoc_write_concern_set_wtimeout(
        write_concern: *mut mongoc_write_concern_t,
        wtimeout_msec: i32
    );
    pub fn mongoc_write_concern_get_wmajority(write_concern: *const mongoc_write_concern_t) -> bool;
    pub fn mongoc_write_concern_set_wmajority(
        write_concern: *mut mongoc_write_concern_t,
        wtimeout_msec: i32
    );
    pub fn mongoc_write_concern_is_acknowledged(
        write_concern: *const mongoc_write_concern_t,
    ) -> bool;
    pub fn mongoc_write_concern_is_valid(write_concern: *const mongoc_write_concern_t) -> bool;
    pub fn mongoc_write_concern_append(
        write_concern: *mut mongoc_write_concern_t,
        doc: *mut bson_t
    ) -> bool;
    pub fn mongoc_write_concern_is_default(write_concern: *const mongoc_write_concern_t) -> bool;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_callbacks_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_callbacks_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_callbacks_new() -> *mut mongoc_apm_callbacks_t;
    pub fn mongoc_apm_callbacks_destroy(callbacks: *mut mongoc_apm_callbacks_t);
    pub fn mongoc_apm_set_command_started_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_command_started_cb_t,
    );
    pub fn mongoc_apm_set_command_succeeded_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_command_succeeded_cb_t,
    );
    pub fn mongoc_apm_set_command_failed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_command_failed_cb_t,
    );
    pub fn mongoc_apm_set_server_changed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_changed_cb_t,
    );
    pub fn mongoc_apm_set_server_opening_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_opening_cb_t,
    );
    pub fn mongoc_apm_set_server_closed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_closed_cb_t,
    );
    pub fn mongoc_apm_set_topology_changed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_topology_changed_cb_t,
    );
    pub fn mongoc_apm_set_topology_opening_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_topology_opening_cb_t,
    );
    pub fn mongoc_apm_set_topology_closed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_topology_closed_cb_t,
    );
    pub fn mongoc_apm_set_server_heartbeat_started_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_heartbeat_started_cb_t,
    );
    pub fn mongoc_apm_set_server_heartbeat_succeeded_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_heartbeat_succeeded_cb_t,
    );
    pub fn mongoc_apm_set_server_heartbeat_failed_cb(
        callbacks: *mut mongoc_apm_callbacks_t,
        cb: mongoc_apm_server_heartbeat_failed_cb_t,
    );
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_command_failed_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_command_failed_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_command_failed_get_duration(event: *const mongoc_apm_command_failed_t)
        -> i64;
    pub fn mongoc_apm_command_failed_get_command_name(
        event: *const mongoc_apm_command_failed_t,
    ) -> *const c_char;
    pub fn mongoc_apm_command_failed_get_error(
        event: *const mongoc_apm_command_failed_t,
        error: *mut bson_error_t,
    );
    pub fn mongoc_apm_command_failed_get_reply(
        event: *const mongoc_apm_command_failed_t,
    ) -> *const bson_t;
    pub fn mongoc_apm_command_failed_get_request_id(
        event: *const mongoc_apm_command_failed_t,
    ) -> i64;
    pub fn mongoc_apm_command_failed_get_operation_id(
        event: *const mongoc_apm_command_failed_t,
    ) -> i64;
    pub fn mongoc_apm_command_failed_get_host(
        event: *const mongoc_apm_command_failed_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_command_failed_get_server_id(
        event: *const mongoc_apm_command_failed_t,
    ) -> u32;
    pub fn mongoc_apm_command_failed_get_context(
        event: *const mongoc_apm_command_failed_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_command_started_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_command_started_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_command_started_get_command(
        event: *const mongoc_apm_command_started_t,
    ) -> *const bson_t;
    pub fn mongoc_apm_command_started_get_database_name(
        event: *const mongoc_apm_command_started_t,
    ) -> *const c_char;
    pub fn mongoc_apm_command_started_get_command_name(
        event: *const mongoc_apm_command_started_t,
    ) -> *const c_char;
    pub fn mongoc_apm_command_started_get_request_id(
        event: *const mongoc_apm_command_started_t,
    ) -> i64;
    pub fn mongoc_apm_command_started_get_operation_id(
        event: *const mongoc_apm_command_started_t,
    ) -> i64;
    pub fn mongoc_apm_command_started_get_host(
        event: *const mongoc_apm_command_started_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_command_started_get_server_id(
        event: *const mongoc_apm_command_started_t,
    ) -> u32;
    pub fn mongoc_apm_command_started_get_context(
        event: *const mongoc_apm_command_started_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_command_succeeded_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_command_succeeded_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_command_succeeded_get_duration(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> i64;
    pub fn mongoc_apm_command_succeeded_get_reply(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *const bson_t;
    pub fn mongoc_apm_command_succeeded_get_command_name(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *const c_char;
    pub fn mongoc_apm_command_succeeded_get_request_id(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> i64;
    pub fn mongoc_apm_command_succeeded_get_operation_id(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> i64;
    pub fn mongoc_apm_command_succeeded_get_host(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_command_succeeded_get_server_id(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> u32;
    pub fn mongoc_apm_command_succeeded_get_context(
        event: *const mongoc_apm_command_succeeded_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_server_changed_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_server_changed_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_server_changed_get_host(
        event: *const mongoc_apm_server_changed_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_server_changed_get_topology_id(
        event: *const mongoc_apm_server_changed_t,
        topology_id: *mut bson_oid_t,
    );
    pub fn mongoc_apm_server_changed_get_previous_description(
        event: *const mongoc_apm_server_changed_t,
    ) -> *const mongoc_server_description_t;
    pub fn mongoc_apm_server_changed_get_new_description(
        event: *const mongoc_apm_server_changed_t,
    ) -> *const mongoc_server_description_t;
    pub fn mongoc_apm_server_changed_get_context(
        event: *const mongoc_apm_server_changed_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_server_closed_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_server_closed_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_server_closed_get_host(
        event: *const mongoc_apm_server_closed_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_server_closed_get_topology_id(
        event: *const mongoc_apm_server_closed_t,
        topology_id: *mut bson_oid_t,
    );
    pub fn mongoc_apm_server_closed_get_context(
        event: *const mongoc_apm_server_closed_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_server_heartbeat_failed_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_server_heartbeat_failed_t {
    _unused: [u8; 0],
}
extern "C" {
    pub fn mongoc_apm_server_heartbeat_failed_get_duration(
        event: *const mongoc_apm_server_heartbeat_failed_t,
    ) -> i64;
    pub fn mongoc_apm_server_heartbeat_failed_get_error(
        event: *const mongoc_apm_server_heartbeat_failed_t,
        error: *mut bson_error_t,
    );
    pub fn mongoc_apm_server_heartbeat_failed_get_host(
        event: *const mongoc_apm_server_heartbeat_failed_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_server_heartbeat_failed_get_context(
        event: *const mongoc_apm_server_heartbeat_failed_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_server_heartbeat_started_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_server_heartbeat_started_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_server_heartbeat_started_get_host(
        event: *const mongoc_apm_server_heartbeat_started_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_server_heartbeat_started_get_context(
        event: *const mongoc_apm_server_heartbeat_started_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_server_heartbeat_succeeded_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_server_heartbeat_succeeded_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_server_heartbeat_succeeded_get_duration(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> i64;
    pub fn mongoc_apm_server_heartbeat_succeeded_get_reply(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> *const bson_t;
    pub fn mongoc_apm_server_heartbeat_succeeded_get_host(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_server_heartbeat_succeeded_get_context(
        event: *const mongoc_apm_server_heartbeat_succeeded_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_server_opening_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_server_opening_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_server_opening_get_host(
        event: *const mongoc_apm_server_opening_t,
    ) -> *const mongoc_host_list_t;
    pub fn mongoc_apm_server_opening_get_topology_id(
        event: *const mongoc_apm_server_opening_t,
        topology_id: *mut bson_oid_t,
    );
    pub fn mongoc_apm_server_opening_get_context(
        event: *const mongoc_apm_server_opening_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_topology_changed_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_topology_changed_t {
    _unused: [u8; 0],
}
extern "C" {
    pub fn mongoc_apm_topology_changed_get_topology_id(
        event: *const mongoc_apm_topology_changed_t,
        topology_id: *mut bson_oid_t,
    );
    pub fn mongoc_apm_topology_changed_get_previous_description(
        event: *const mongoc_apm_topology_changed_t,
    ) -> *const mongoc_topology_description_t;
    pub fn mongoc_apm_topology_changed_get_new_description(
        event: *const mongoc_apm_topology_changed_t,
    ) -> *const mongoc_topology_description_t;
    pub fn mongoc_apm_topology_changed_get_context(
        event: *const mongoc_apm_topology_changed_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_topology_closed_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_topology_closed_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_topology_closed_get_topology_id(
        event: *const mongoc_apm_topology_closed_t,
        topology_id: *mut bson_oid_t,
    );
    pub fn mongoc_apm_topology_closed_get_context(
        event: *const mongoc_apm_topology_closed_t,
    ) -> *mut c_void;
}

// see http://mongoc.org/libmongoc/current/mongoc_apm_topology_opening_t.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mongoc_apm_topology_opening_t {
    _unused: [u8; 0],
}

extern "C" {
    pub fn mongoc_apm_topology_opening_get_topology_id(
        event: *const mongoc_apm_topology_opening_t,
        topology_id: *mut bson_oid_t,
    );
    pub fn mongoc_apm_topology_opening_get_context(
        event: *const mongoc_apm_topology_opening_t,
    ) -> *mut c_void;
}

pub type mongoc_apm_command_started_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_command_started_t)>;
pub type mongoc_apm_command_succeeded_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_command_succeeded_t)>;
pub type mongoc_apm_command_failed_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_command_failed_t)>;
pub type mongoc_apm_server_changed_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_server_changed_t)>;
pub type mongoc_apm_server_opening_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_server_opening_t)>;
pub type mongoc_apm_server_closed_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_server_closed_t)>;
pub type mongoc_apm_topology_changed_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_topology_changed_t)>;
pub type mongoc_apm_topology_opening_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_topology_opening_t)>;
pub type mongoc_apm_topology_closed_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_topology_closed_t)>;
pub type mongoc_apm_server_heartbeat_started_cb_t = Option<
    unsafe extern "C" fn(event: *const mongoc_apm_server_heartbeat_started_t),
>;
pub type mongoc_apm_server_heartbeat_succeeded_cb_t = Option<
    unsafe extern "C" fn(event: *const mongoc_apm_server_heartbeat_succeeded_t),
>;
pub type mongoc_apm_server_heartbeat_failed_cb_t =
    Option<unsafe extern "C" fn(event: *const mongoc_apm_server_heartbeat_failed_t)>;


// util
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

pub type mongoc_iovec_t = iovec;

