use std::ops::Deref;
use std::sync::Arc;

use crate::core::client_pool;
use crate::core::client;

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
        let inner = self.inner.pool.pop();

        Client {
            client: self.clone(),
            inner
        }
    }
}

pub struct Client {
    client: MongoClient,
    inner: client::Client
}

impl Drop for Client {
    fn drop(&mut self) {
        self.client.inner.pool.push(&self.inner)
    }
}

impl Deref for Client {
    type Target = client::Client;

    fn deref(&self) -> &client::Client {
        &self.inner
    }
}
