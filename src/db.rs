use std::sync::Arc;

use crate::client::MongoClient;
use crate::collection::Collection;

#[derive(Clone)]
pub struct DB {
    pub inner: Arc<DBInner>
}

pub struct DBInner {
    pub name: String,
    pub client: MongoClient
}

impl DB {
    pub fn new(client: MongoClient, name: &str) -> DB {
        DB {
            inner: Arc::new(DBInner {
                name: name.to_string(),
                client
            })
        }
    }

    pub fn collection(&self, name: &str) -> Collection {
        Collection::new(self.clone(), name)
    }
}
