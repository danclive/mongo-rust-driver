
use std::collections::vec_deque::VecDeque;
use error::Result;
use error::Error;

use bson::{Bson, Document};
use common::ReadPreference;
use database::Database;

// Allows the server to decide the batch size.
pub const DEFAULT_BATCH_SIZE: i32 = 100;

#[derive(Debug)]
pub struct Cursor {
    pub db: Database,
    // The namespace to read and write from.
    pub namespace: String,
    // How many documents to fetch at a given time from the server.
    pub batch_size: i32,
    // Uniquely identifies the cursor being returned by the reply.
    pub cursor_id: i64,
    // Specifies a time limit in milliseconds for processing operations on a cursor
    pub max_time_ms: i64,
    // How many documents have been returned so far.
    pub count: i32,
    // A cache for documents received from the query that have not yet been returned.
    pub buffer: VecDeque<Document>,
    pub read_preference: Option<ReadPreference>,
}

impl Cursor {
    pub fn command(
        db: Database,
        command: Document,
        batch_size: Option<i32>,
        //limit: Option<i32>,
        max_time_ms: Option<i64>,
        read_preference: Option<ReadPreference>,
    ) -> Result<Cursor> {
        let batch_size = batch_size.unwrap_or(DEFAULT_BATCH_SIZE);
        //let limit = limit.unwrap_or_default();
        let max_time_ms = max_time_ms.unwrap_or_default();

        let doc = db.command(command, read_preference.clone())?;

        if let Some(&Bson::Int64(ref id)) = doc.get("id") {
            if let Some(&Bson::String(ref ns)) = doc.get("ns") {
                if let Some(&Bson::Array(ref batch)) = doc.get("firstBatch") {

                    // Extract first batch documents
                    let map: VecDeque<Document> = batch.iter()
                    .filter_map(|bdoc| if let Bson::Document(ref doc) = *bdoc {
                        Some(doc.clone())
                    } else {
                        None
                    }).collect();

                    return Ok(Cursor {
                        db,
                        namespace: ns.to_owned(),
                        batch_size,
                        cursor_id: *id,
                        max_time_ms,
                        count: 0,
                        buffer: map,
                        read_preference,
                    })
                }
            }
        }

        Err(Error::CursorNotFoundError)
    }

    /// Extracts the collection name from the namespace.
    /// If the namespace is invalid, this method will panic.
    pub fn name(&self) -> String {
        match self.namespace.find('.') {
            Some(idx) => {
                self.namespace[
                    self.namespace
                    .char_indices()
                    .nth(idx + 1)
                    .unwrap()
                    .0..
                ].to_string()
            }
            None => {
                // '.' is inserted in Collection::new, so this should only panic due to user error.
                let msg = format!("Invalid namespace specified: '{}'.", self.namespace);
                panic!(msg);
            }
        }
    }

    fn get_more(&mut self) -> Result<()> {
        if self.cursor_id > 0 {
            let get_more_command = doc!{
                "getMore": self.cursor_id,
                "collection": self.name(),
                "batchSize": self.batch_size,
                "maxTimeMS": self.max_time_ms
            };

            let doc = self.db.command(get_more_command, self.read_preference.clone())?;

            if let Some(&Bson::Int64(ref id)) = doc.get("id") {
                self.cursor_id = *id;

                if let Some(&Bson::Array(ref batch)) = doc.get("nextBatch") {
                    for doc in batch {
                        if let Bson::Document(ref doc) = *doc {
                            self.buffer.push_back(doc.clone());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Attempts to read a specified number of BSON documents from the cursor.
    ///
    /// # Arguments
    ///
    /// `n` - The number of documents to read.
    ///
    /// # Return value
    ///
    /// Returns a vector containing the BSON documents that were read.
    pub fn next_n(&mut self, n: i32) -> Result<Vec<Document>> {
        let mut vec = vec![];

        for _ in 0..n {
            let bson_option = self.next();

            match bson_option {
                Some(Ok(bson)) => vec.push(bson),
                Some(Err(err)) => return Err(err),
                None => break,
            };
        }

        Ok(vec)
    }

    /// Attempts to read a batch of BSON documents from the cursor.
    ///
    /// # Return value
    ///
    /// Returns a vector containing the BSON documents that were read.
    pub fn drain_current_batch(&mut self) -> Result<Vec<Document>> {
        if self.buffer.is_empty() {
            self.get_more()?;
        }

        Ok(self.buffer.drain(..).collect())
    }

    /// Checks whether there are any more documents for the cursor to return.
    ///
    /// # Return value
    ///
    /// Returns `true` if the cursor is not yet exhausted, or `false` if it is.
    pub fn has_next(&mut self) -> Result<bool> {
        if self.buffer.is_empty() && self.cursor_id != 0 {
            self.get_more()?;
        }

        Ok(!self.buffer.is_empty())
    }
}

impl Iterator for Cursor {
    type Item = Result<Document>;

    /// Attempts to read a BSON document from the cursor.
    ///
    /// # Return value
    ///
    /// Returns a BSON document if there is another one to return; `None` if
    /// there are no more documents to return; or an Error if the request for
    /// another document fails.
    fn next(&mut self) -> Option<Result<Document>> {
        match self.has_next() {
            Ok(true) => {
                self.count += 1;
                match self.buffer.pop_front() {
                    Some(bson) => Some(Ok(bson)),
                    None => None,
                }
            }
            Ok(false) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        if self.cursor_id > 0 {
            let _ = self.db.command(
                doc!{
                    "killCursors": self.name().clone(),
                    "cursors": [self.cursor_id]
                },
                self.read_preference.clone()
            );
        }
    }
}
