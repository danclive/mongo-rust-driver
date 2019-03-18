use std::sync::Arc;

use crate::client::{MongoClient, Client};
use crate::collection::Collection;

#[derive(Clone)]
pub struct DB {
    pub inner: Arc<DBInner>
}

pub struct DBInner {
    pub name: String,
    pub mongo_client: MongoClient
}

impl DB {
    pub fn new(mongo_client: MongoClient, name: &str) -> DB {
        DB {
            inner: Arc::new(DBInner {
                name: name.to_string(),
                mongo_client
            })
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    #[inline]
    pub fn acquire_client(&self) -> Client {
        self.inner.mongo_client.acquire_client()
    }

    #[inline]
    pub fn collection(&self, name: &str) -> Collection {
        Collection::new(self.clone(), name)
    }
}
