#![cfg_attr(feature = "cargo-clippy", allow(map_entry))]

extern crate linked_hash_map;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate byteorder;
extern crate libc;
extern crate rand;
extern crate data_encoding;
#[macro_use]
extern crate bitflags;
extern crate semver;
extern crate ring;

#[macro_use]
pub mod bson;
pub mod object_id;
pub mod util;
pub mod client;
pub mod stream;
//pub mod command_type;
pub mod common;
pub mod apm;
pub mod topology;
pub mod error;
pub mod pool;
pub mod connstring;
//pub mod cursor;
pub mod database;
//pub mod collection;
//pub mod gridfs;
//pub mod wire_protocol;
//pub mod auth;
pub mod message;
pub mod command;
