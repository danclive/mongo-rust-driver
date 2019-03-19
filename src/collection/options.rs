use bsonrs::Document;
use bsonrs::doc;

pub use crate::core::find_and_modify::FindAndModifyOpts;

#[derive(Clone, Debug, Default)]
pub struct FindOptions {
    pub sort: Option<Document>,
    pub projection: Option<Document>,
    pub hint: Option<String>,
    pub hint_doc: Option<Document>,
    pub skip: Option<i64>,
    pub limit: Option<i64>,
    pub batch_size: Option<i32>,
    pub single_batch: Option<bool>,
    pub comment: Option<String>,
    pub max_time_ms: Option<i64>,
    pub max: Option<Document>,
    pub min: Option<Document>,
    pub return_key: Option<bool>,
    pub show_record_id: Option<bool>,
    pub tailable: Option<bool>,
    pub oplog_replay: Option<bool>,
    pub no_cursor_timeout: Option<bool>,
    pub await_data: Option<bool>,
    pub allow_partial_results: Option<bool>,
    pub collation: Option<Document>,
    pub exhaust: Option<bool>, // Sets the cursor to return all data returned by the query at once rather than splitting the results into batches.
    // pub read_concern: Option<ReadConcern>,
    // pub transaction
    // pub session
}

impl FindOptions {
    /// Creates a new FindOptions struct with default parameters.
    pub fn new() -> Self {
        FindOptions::default()
    }
}

impl Into<Document> for FindOptions {
    fn into(self) -> Document {
        let mut doc = doc!{};

        if let Some(sort) = self.sort {
            doc.insert("sort", sort);
        }

        if let Some(projection) = self.projection {
            doc.insert("projection", projection);
        }

        if let Some(hint) = self.hint_doc {
            doc.insert("hint", hint);
        } else if let Some(hint) = self.hint {
            doc.insert("hint", hint);
        }

        if let Some(skip) = self.skip {
            doc.insert("skip", skip);
        }

        if let Some(limit) = self.limit {
            doc.insert("limit", limit);
        }

        if let Some(batch_size) = self.batch_size {
            doc.insert("batchSize", batch_size);
        }

        if let Some(single_batch) = self.single_batch {
            doc.insert("singleBatch", single_batch);
        }

        if let Some(comment) = self.comment {
            doc.insert("comment", comment);
        }

        if let Some(max_time_ms) = self.max_time_ms {
            doc.insert("maxTimeMS", max_time_ms);
        }

        if let Some(max) = self.max {
            doc.insert("max", max);
        }

        if let Some(min) = self.min {
            doc.insert("min", min);
        }

        if let Some(return_key) = self.return_key {
            doc.insert("returnKey", return_key);
        }

        if let Some(show_record_id) = self.show_record_id {
            doc.insert("showRecordId", show_record_id);
        }

        if let Some(tailable) = self.tailable {
            doc.insert("tailable", tailable);
        }

        if let Some(oplog_replay) = self.oplog_replay {
            doc.insert("oplogReplay", oplog_replay);
        }

        if let Some(no_cursor_timeout) = self.no_cursor_timeout {
            doc.insert("noCursorTimeout", no_cursor_timeout);
        }

        if let Some(await_data) = self.await_data {
            doc.insert("awaitData", await_data);
        }

        if let Some(allow_partial_results) = self.allow_partial_results {
            doc.insert("allowPartialResults", allow_partial_results);
        }

        if let Some(collation) = self.collation {
            doc.insert("collation", collation);
        }

        if let Some(exhaust) = self.exhaust {
            doc.insert("exhaust", exhaust);
        }

        doc
    }
}
