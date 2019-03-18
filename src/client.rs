use std::sync::Arc;

use bsonrs::Document;

use crate::core::bsonc::Bsonc;
use crate::core::client_pool;
use crate::core::client;
use crate::core::collection::Collection;

use crate::uri::Uri;
use crate::db::DB;
use crate::error::Result;

#[derive(Clone)]
pub struct MongoClient {
    pub inner: Arc<MongoClientInner>
}

pub struct MongoClientInner {
    pub pool: client_pool::ClientPool
}

impl MongoClient {
    pub fn new(uri: &str) -> Result<MongoClient> {
        let uri = Uri::new_with_error(uri)?;
        let pool = client_pool::ClientPool::new(&uri);

        Ok(MongoClient {
            inner: Arc::new(MongoClientInner {
                pool
            })
        })
    }

    pub fn new_with_uri(uri: &Uri) -> MongoClient {
        let pool = client_pool::ClientPool::new(&uri);

        MongoClient {
            inner: Arc::new(MongoClientInner {
                pool
            })
        }
    }

    pub fn db(&self, name: &str) -> DB {
        DB::new(self.clone(), name)
    }

    pub fn acquire_client(&self) -> Client {
        let client = self.inner.pool.pop();

        Client {
            mongo_client: self.clone(),
            inner: client
        }
    }
}

pub struct Client {
    mongo_client: MongoClient,
    inner: client::Client
}

impl Client {
    #[inline]
    pub fn get_database_names_with_opts(&self, opts: &Document) -> Result<Vec<String>> {
        let opts = Bsonc::from_doc(opts)?;
        self.inner.get_database_names_with_opts(&opts)
    }

    #[inline]
    pub fn get_collection(&self, db: &str, collection: &str) -> Collection {
        self.inner.get_collection(db, collection)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.mongo_client.inner.pool.push(&self.inner)
    }
}

// impl Deref for Client {
//     type Target = client::Client;

//     fn deref(&self) -> &client::Client {
//         &self.inner
//     }
// }
