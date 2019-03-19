// use std::os::raw::c_char;

// extern "C" {
//     pub fn mongoc_init();
//     pub fn bson_get_monotonic_time() -> i64;
//     pub fn mongoc_get_version() -> *const c_char;
// }

pub mod sys;
pub mod core;
pub mod error;
pub mod uri;
pub mod client;
pub mod db;
pub mod collection;
pub mod cursor;
pub mod session;
pub mod client_session;
pub mod read_preference;

pub use client::{MongoClient, Client};
pub use db::DB;
pub use collection::Collection;
pub use read_preference::{ReadMode, ReadPreference};
pub use cursor::Cursor;

pub mod bson {
    pub use bsonrs::*;
}
