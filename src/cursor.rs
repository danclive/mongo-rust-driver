use bsonrs::Document;

use crate::core::cursor::Cursor as CoreCursor;

use crate::client::Client;
use crate::error::{Result, Error};

pub struct Cursor {
    pub inner: CoreCursor,
    pub client: Client
}

impl Cursor {
    pub fn new(
        core_cursor: CoreCursor,
        client: Client
    ) -> Cursor {

        Cursor {
            inner: core_cursor,
            client
        }
    }

    pub fn current(&self) -> Result<Option<Document>> {
        if let Some(bsonc) = self.inner.current() {
            let doc = bsonc.as_doc()?;
            return Ok(Some(doc))
        }

        return Ok(None)
    }

    pub fn error(&self) -> Option<Error> {
        self.inner.error()
    }

    pub fn error_document(&self) -> Result<Option<(Error, Document)>> {
        if let Some((error, bsonc)) = self.inner.error_document() {
            let doc = bsonc.as_doc();

            std::mem::forget(bsonc); // Don't drop bsonc, otherwise, segmentation fault will happen !

            return Ok(Some((error, doc?)))
        }

        Ok(None)
    }

    pub fn get_batch_size(&self) -> u32 {
        self.inner.get_batch_size()
    }

    pub fn get_hint(&self) -> u32 {
        self.inner.get_hint()
    }

    pub fn get_id(&self) -> i64 {
        self.inner.get_id()
    }

    pub fn get_limit(&self) -> i64 {
        self.inner.get_limit()
    }

    pub fn get_max_await_time_ms(&self) -> u32 {
        self.inner.get_max_await_time_ms()
    }

    pub fn more(&self) -> bool {
        self.inner.more()
    }

    pub fn set_batch_size(&self, size: u32) {
        self.inner.set_batch_size(size)
    }

    pub fn set_hint(&self, server_id: u32) -> bool {
        self.inner.set_hint(server_id)
    }

    pub fn set_limit(&self, limit: i64) -> bool {
        self.inner.set_limit(limit)
    }

    pub fn set_max_await_time_ms(&self, max_await_time_ms: u32) {
        self.inner.set_max_await_time_ms(max_await_time_ms)
    }
}

impl Iterator for Cursor {
    type Item = Result<Document>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.more() {
            return None
        }

        // if let Some(bsonc) = self.inner.next() {
        //     return Some(bsonc.as_doc().map_err(Error::from))
        // }

        // if let Some(error) = self.error() {
        //     return Some(Err(error.into()))
        // }

        // None

        self.inner.next().map(|bsonc| bsonc.as_doc().map_err(Error::from))
            .or(self.error().map(|error| Err(error.into())))
    }
}

