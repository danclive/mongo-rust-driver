//! Options for collection-level operations.
use bson::{Bson, Document};
use common::{ReadPreference, WriteConcern, ReadConcern};

use error::Result;
use error::Error::ArgumentError;

/// Describes the type of document to return on write operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReturnDocument {
    Before,
    After,
}

impl ReturnDocument {
    pub fn as_bool(&self) -> bool {
        match *self {
            ReturnDocument::Before => false,
            ReturnDocument::After => true,
        }
    }
}

/// Marker interface for writes that can be batched together.
#[derive(Debug, Clone)]
pub enum WriteModel {
    InsertOne { document: Document },
    DeleteOne { filter: Document },
    DeleteMany { filter: Document },
    ReplaceOne {
        filter: Document,
        replacement: Document,
        upsert: Option<bool>,
    },
    UpdateOne {
        filter: Document,
        update: Document,
        upsert: Option<bool>,
    },
    UpdateMany {
        filter: Document,
        update: Document,
        upsert: Option<bool>,
    },
}

/// Options for aggregation queries.
#[derive(Clone, Debug, Default)]
pub struct AggregateOptions {
    pub allow_disk_use: Option<bool>,
    pub batch_size: Option<i32>,
    pub max_time_ms: Option<i64>,
    pub bypass_document_validation: Option<bool>,
    pub collation: Option<Document>,
    pub hint: Option<String>, // use enum ?
    pub hint_doc: Option<Document>,
    pub comment: Option<String>,
    pub read_concern: Option<ReadConcern>,
    pub read_preference: Option<ReadPreference>,
}

impl AggregateOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<AggregateOptions> for Document {
    fn from(options: AggregateOptions) -> Self {
        let mut document = Document::new();

        if let Some(allow_disk_use) = options.allow_disk_use {
            document.insert("allowDiskUse", allow_disk_use);
        }

        if let Some(batch_size) = options.batch_size {
            document.insert("cursor", doc!{ "batchSize": batch_size });
        } else {
            document.insert("cursor", doc!{});
        }

        if let Some(max_time_ms) = options.max_time_ms {
            document.insert("maxTimeMS", max_time_ms);
        }

        if let Some(bypass_document_validation) = options.bypass_document_validation {
            document.insert("bypassDocumentValidation", bypass_document_validation);
        }

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        if let Some(hint) = options.hint_doc {
            document.insert("hint", hint);
        } else if let Some(hint) = options.hint {
            document.insert("hint", hint);
        }

        if let Some(comment) = options.comment {
            document.insert("comment", comment);
        }

        document
    }
}

/// Options for watch queries.
#[derive(Clone, Debug, Default)]
pub struct ChangeStreamOptions {
    pub resume_after: Option<Document>,
    pub full_document: Option<String>,
    pub batch_size: Option<i32>,
    pub max_await_time_ms: Option<i64>,
    pub collation: Option<Document>,
    pub read_preference: Option<ReadPreference>
}

impl ChangeStreamOptions {
    pub fn new() -> Self {
        let mut options = Self::default();

        options.full_document = Some("default".to_owned());

        options
    }
}

impl From<ChangeStreamOptions> for Document {
    fn from(options: ChangeStreamOptions) -> Self {
        let mut document = Document::new();

        if let Some(resume_after) = options.resume_after {
            document.insert("resumeAfter", resume_after);
        }

        if let Some(full_document) = options.full_document {
            document.insert("fullDocument", full_document);
        }

        if let Some(batch_size) = options.batch_size {
            document.insert("batchSize", batch_size);
        }

        if let Some(max_await_time_ms) = options.max_await_time_ms {
            document.insert("maxAwaitTimeMS", max_await_time_ms);
        }

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        document
    }
}

/// Options for count queries.
#[derive(Clone, Debug, Default)]
pub struct CountOptions {
    pub skip: Option<i64>,
    pub limit: Option<i64>,
    pub hint: Option<String>,
    pub hint_doc: Option<Document>,
    pub collation: Option<Document>,
    pub read_concern: Option<ReadConcern>,
    pub read_preference: Option<ReadPreference>,
}

impl CountOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<CountOptions> for Document {
    fn from(options: CountOptions) -> Self {
        let mut document = Document::new();

        if let Some(skip) = options.skip {
            document.insert("skip", skip);
        }

        if let Some(limit) = options.limit {
            document.insert("limit", limit);
        }

        if let Some(hint) = options.hint_doc {
            document.insert("hint", hint);
        } else if let Some(hint) = options.hint {
            document.insert("hint", hint);
        }

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        document
    }
}

/// Options for distinct queries.
#[derive(Clone, Debug, Default)]
pub struct DistinctOptions {
    pub collation: Option<Document>,
    pub read_concern: Option<ReadConcern>,
    pub read_preference: Option<ReadPreference>,
}

impl DistinctOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<DistinctOptions> for Document {
    fn from(options: DistinctOptions) -> Self {
        let mut document = Document::new();

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        document
    }
}

/// Options for collection queries.
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
    pub max_scan: Option<i64>,
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
    pub read_concern: Option<ReadConcern>,
    pub read_preference: Option<ReadPreference>
}

impl FindOptions {
    /// Creates a new FindOptions struct with default parameters.
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<FindOptions> for Document {
    fn from(options: FindOptions) -> Self {
        let mut document = Document::new();

        if let Some(sort) = options.sort {
            document.insert("sort", sort);
        }

        if let Some(projection) = options.projection {
            document.insert("projection", projection);
        }

        if let Some(hint) = options.hint_doc {
            document.insert("hint", hint);
        } else if let Some(hint) = options.hint {
            document.insert("hint", hint);
        }

        if let Some(skip) = options.skip {
            document.insert("skip", skip);
        }

        if let Some(limit) = options.limit {
            document.insert("limit", limit);
        }

        if let Some(batch_size) = options.batch_size {
            document.insert("batchSize", batch_size);
        }

        if let Some(single_batch) = options.single_batch {
            document.insert("singleBatch", single_batch);
        }

        if let Some(comment) = options.comment {
            document.insert("comment", comment);
        }

        if let Some(max_scan) = options.max_scan {
            document.insert("maxScan", max_scan);
        }

        if let Some(max_time_ms) = options.max_time_ms {
            document.insert("maxTimeMS", max_time_ms);
        }

        if let Some(max) = options.max {
            document.insert("max", max);
        }

        if let Some(min) = options.min {
            document.insert("min", min);
        }

        if let Some(return_key) = options.return_key {
            document.insert("returnKey", return_key);
        }

        if let Some(show_record_id) = options.show_record_id {
            document.insert("showRecordId", show_record_id);
        }

        if let Some(tailable) = options.tailable {
            document.insert("tailable", tailable);
        }

        if let Some(oplog_replay) = options.oplog_replay {
            document.insert("oplogReplay", oplog_replay);
        }

        if let Some(no_cursor_timeout) = options.no_cursor_timeout {
            document.insert("noCursorTimeout", no_cursor_timeout);
        }

        if let Some(await_data) = options.await_data {
            document.insert("awaitData", await_data);
        }

        if let Some(allow_partial_results) = options.allow_partial_results {
            document.insert("allowPartialResults", allow_partial_results);
        }

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        document
    }
}

/// Options for `findOneAndDelete` operations.
#[derive(Clone, Debug, Default)]
pub struct FindOneAndDeleteOptions {
    pub max_time_ms: Option<i64>,
    pub projection: Option<Document>,
    pub sort: Option<Document>,
    pub collation: Option<Document>,
    pub write_concern: Option<WriteConcern>
}

impl FindOneAndDeleteOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<FindOneAndDeleteOptions> for Document {
    fn from(options: FindOneAndDeleteOptions) -> Self {
        let mut document = Document::new();

        if let Some(max_time_ms) = options.max_time_ms {
            document.insert("maxTimeMS", max_time_ms);
        }

        if let Some(projection) = options.projection {
            document.insert("fields", projection);
        }

        if let Some(sort) = options.sort {
            document.insert("sort", sort);
        }

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        document
    }
}

/// Options for `findOneAndUpdate` operations.
#[derive(Clone, Debug, Default)]
pub struct FindOneAndUpdateOptions {
    pub return_document: Option<ReturnDocument>,
    pub max_time_ms: Option<i64>,
    pub projection: Option<Document>,
    pub sort: Option<Document>,
    pub upsert: Option<bool>,
    pub bypass_document_validation: Option<bool>,
    pub collation: Option<Document>,
    pub array_filters: Option<Vec<Document>>,
    pub write_concern: Option<WriteConcern>,
}

impl FindOneAndUpdateOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<FindOneAndUpdateOptions> for Document {
    fn from(options: FindOneAndUpdateOptions) -> Self {
        let mut document = Document::new();

        if let Some(return_document) = options.return_document {
            document.insert("new", return_document.as_bool());
        }

        if let Some(max_time_ms) = options.max_time_ms {
            document.insert("maxTimeMS", max_time_ms);
        }

        if let Some(projection) = options.projection {
            document.insert("fields", projection);
        }

        if let Some(sort) = options.sort {
            document.insert("sort", sort);
        }

        if let Some(upsert) = options.upsert {
            document.insert("upsert", upsert);
        }

        if let Some(bypass_document_validation) = options.bypass_document_validation {
            document.insert("bypassDocumentValidation", bypass_document_validation);
        }

        if let Some(array_filters) = options.array_filters {
            document.insert("arrayFilters", array_filters);
        }

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        document
    }
}

/// Options for insertMany operations.
#[derive(Clone, Debug, Default)]
pub struct InsertManyOptions {
    pub ordered: Option<bool>,
    pub bypass_document_validation: Option<bool>,
    pub write_concern: Option<WriteConcern>
}

impl InsertManyOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<InsertManyOptions> for Document {
    fn from(options: InsertManyOptions) -> Self {
        let mut document = Document::new();

        if let Some(ordered) = options.ordered {
            document.insert("ordered", ordered);
        }

        if let Some(bypass_document_validation) = options.bypass_document_validation {
            document.insert("bypassDocumentValidation", bypass_document_validation);
        }

        document
    }
}

/// Options for insertOne operations.
#[derive(Clone, Debug, Default)]
pub struct InsertOneOptions {
    pub bypass_document_validation: Option<bool>,
    pub write_concern: Option<WriteConcern>,
}

impl InsertOneOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<InsertOneOptions> for Document {
    fn from(options: InsertOneOptions) -> Self {
        let mut document = Document::new();

        if let Some(bypass_document_validation) = options.bypass_document_validation {
            document.insert("bypassDocumentValidation", bypass_document_validation);
        }

        if let Some(write_concern) = options.write_concern {
            document.insert("writeConcern", write_concern.to_document());
        }

        document
    }
}

/// Options for delete operations
#[derive(Clone, Debug, Default)]
pub struct DeleteOptions {
    pub ordered: Option<bool>,
    pub collation: Option<Document>,
    pub write_concern: Option<WriteConcern>
}

impl DeleteOptions {
    pub fn new() -> DeleteOptions {
        Default::default()
    }
}

impl From<DeleteOptions> for Document {
    fn from(options: DeleteOptions) -> Self {
        let mut document = Document::new();

        if let Some(ordered) = options.ordered {
            document.insert("ordered", ordered);
        }

        document
    }
}

/// Options for update operations.
#[derive(Clone, Debug, Default)]
pub struct UpdateOptions {
    pub ordered: Option<bool>,
    pub bypass_document_validation: Option<bool>,
    pub collation: Option<Document>,
    pub array_filters: Option<Vec<Document>>,
    pub upsert: Option<bool>,
    pub write_concern: Option<WriteConcern>
}

impl UpdateOptions {
    pub fn new() -> UpdateOptions {
        Default::default()
    }
}

impl From<UpdateOptions> for Document {
    fn from(options: UpdateOptions) -> Self {
        let mut document = Document::new();

        if let Some(ordered) = options.ordered {
            document.insert("ordered", ordered);
        }

        if let Some(bypass_document_validation) = options.bypass_document_validation {
            document.insert("bypassDocumentValidation", bypass_document_validation);
        }

        document
    }
}

/// Options for index operations.
#[derive(Clone, Debug, Default)]
pub struct IndexOptions {
    pub name: Option<String>,
    pub background: Option<bool>,
    pub unique: Option<bool>,
    pub partial_filter_expression: Option<Document>,
    pub sparse: Option<bool>,
    pub expire_after_seconds: Option<i32>,
    pub storange_engine: Option<Document>,
    pub weights: Option<Document>,
    pub default_language: Option<String>,
    pub language_override: Option<String>,
    pub text_index_version: Option<i32>,
    pub two_dsphere_index_version: Option<i32>,
    pub bits: Option<i32>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub bucket_size: Option<i32>,
    pub collation: Option<Document>,
}

impl IndexOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<IndexOptions> for Document {
    fn from(options: IndexOptions) -> Self {
        let mut document = Document::new();

        if let Some(name) = options.name {
            document.insert("name", name);
        }

        if let Some(background) = options.background {
            document.insert("background", background);
        }

        if let Some(unique) = options.unique {
            document.insert("unique", unique);
        }

        if let Some(partial_filter_expression) = options.partial_filter_expression {
            document.insert("partialFilterExpression", partial_filter_expression);
        }

        if let Some(sparse) = options.sparse {
            document.insert("sparse", sparse);
        }

        if let Some(expire_after_seconds) = options.expire_after_seconds {
            document.insert("expireAfterSeconds", expire_after_seconds);
        }

        if let Some(storange_engine) = options.storange_engine {
            document.insert("storageEngine", storange_engine);
        }

        if let Some(weights) = options.weights {
            document.insert("weights", weights);
        }

        if let Some(default_language) = options.default_language {
            document.insert("default_language", default_language);
        }

        if let Some(language_override) = options.language_override {
            document.insert("language_override", language_override);
        }

        if let Some(text_index_version) = options.text_index_version {
            document.insert("textIndexVersion", text_index_version);
        }

        if let Some(two_dsphere_index_version) = options.two_dsphere_index_version {
            document.insert("2dsphereIndexVersion", two_dsphere_index_version);
        }

        if let Some(bits) = options.bits {
            document.insert("bits", bits);
        }

        if let Some(min) = options.min {
            document.insert("min", min);
        }

        if let Some(max) = options.max {
            document.insert("max", max);
        }

        if let Some(bucket_size) = options.bucket_size {
            document.insert("bucketSize", bucket_size);
        }

        if let Some(collation) = options.collation {
            document.insert("collation", collation);
        }

        document
    }
}

/// A single index model
#[derive(Clone, Debug)]
pub struct IndexModel {
    pub keys: Document,
    pub options: IndexOptions,
}

impl IndexModel {
    pub fn new(keys: Document, options: Option<IndexOptions>) -> IndexModel {
        IndexModel {
            keys: keys,
            options: options.unwrap_or_default()
        }
    }

    /// Returns the name of the index as specified by the options, or
    /// as automatically generated using the keys.
    pub fn name(&self) -> Result<String> {
        Ok(match self.options.name {
            Some(ref name) => name.to_string(),
            None => self.generate_index_name()?,
        })
    }

    /// Generates the index name from keys.
    /// Auto-generated names have the form "key1_val1_key2_val2..."
    pub fn generate_index_name(&self) -> Result<String> {
        let mut name = String::new();
        for (key, bson) in self.keys.iter() {
            if !name.is_empty() {
                name.push_str("_");
            }

            name.push_str(key);
            name.push('_');
            match *bson {
                Bson::Int32(ref i) => name.push_str(&format!("{}", i)),
                Bson::String(ref s) if s == "text" || s == "hashed" || s == "2d" || s == "2dsphere" => name.push_str(s),
                _ => return Err(ArgumentError("Index model keys must map to i32.".to_string())),
            }
        }
        Ok(name)
    }

    pub fn to_document(&self) -> Result<Document> {

        let mut document = doc!{
            "key": self.keys.clone()
        };

        document.extend(self.options.clone());

        if !document.contains_key("name") {
            document.insert("name", self.generate_index_name()?);
        }

        Ok(document)
    }
}
