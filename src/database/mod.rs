use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use std::fmt;

use bson::{self, Bson, Document};

use client::MongoClient;
use common::{ReadPreference, WriteConcern, ReadConcern};
use semver::Version;
use error::Result;
use error::Error::{CursorNotFoundError, OperationError, ResponseError, EventListenerError};
use pool::PooledStream;





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

impl Database {
    pub fn open(
        client: MongoClient,
        name: &str,
        read_preference: Option<ReadPreference>,
        read_concern: Option<ReadConcern>,
        write_concern: Option<WriteConcern>
    ) -> Database {
        is_send::<Database>();
        is_sync::<Database>();

        let rp = read_preference.unwrap_or_else(|| client.inner.options.read_preference.clone());
        let rc = read_concern.unwrap_or_else(|| client.inner.options.read_concern.clone());
        let wc = write_concern.unwrap_or_else(|| client.inner.options.write_concern.clone());

        Database {
            inner: Arc::new(DatabaseInner {
                name: name.to_string(),
                client,
                read_preference: rp,
                read_concern: rc,
                write_concern: wc,
            })
        }
    }

    pub fn command(&self, ) {

    }
}
