//! Interface for collection-level operations.
use std::collections::BTreeMap;

use object_id::ObjectId;
use bson::{Bson, Document};
use command_type::CommandType;
use common::{ReadPreference, WriteConcern, ReadConcern};
use cursor::{Cursor, DEFAULT_BATCH_SIZE};
use db::Database;
use error::Result;
use error::Error::{ArgumentError, ResponseError, OperationError, BulkWriteError};

use self::error::{BulkWriteException, WriteException};
use self::options::*;
use self::results::*;
use self::charge_stream::ChargeStream;

pub mod error;
pub mod options;
pub mod results;
pub mod charge_stream;

/// Interfaces with a MongoDB collection.
#[derive(Clone)]
pub struct Collection {
    /// A reference to the database that spawned this collection.
    pub db: Database,
    /// The namespace of this collection, formatted as db_name.coll_name.
    pub namespace: String,
    read_preference: ReadPreference,
    read_concern: ReadConcern,
    write_concern: WriteConcern,
}

impl Collection {
    /// Creates a collection representation with optional read and write controls.
    ///
    /// If `create` is specified, the collection will be explicitly created in the database.
    pub fn new(
        db: Database,
        name: &str,
        create: bool,
        read_preference: Option<ReadPreference>,
        read_concern: Option<ReadConcern>,
        write_concern: Option<WriteConcern>
    ) -> Collection {

        let rp = read_preference.unwrap_or_else(|| db.inner.read_preference.clone());
        let rc = read_concern.unwrap_or_else(|| db.inner.read_concern.clone());
        let wc = write_concern.unwrap_or_else(|| db.inner.write_concern.clone());

        if create {
            // Attempt to create the collection explicitly, or fail silently.
            let _ = db.create_collection(name, None);
        }

        Collection {
            db: db.clone(),
            namespace: format!("{}.{}", db.inner.name, name),
            read_preference: rp,
            read_concern: rc,
            write_concern: wc,
        }
    }

    /// Returns a unique operational request id.
    pub fn get_req_id(&self) -> i32 {
        self.db.inner.client.get_req_id()
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

    /// Permanently deletes the collection from the database.
    pub fn drop(&self) -> Result<()> {
        self.db.drop_collection(&self.name())
    }

    /// Runs an aggregation framework pipeline.
    pub fn aggregate(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>
    ) -> Result<Cursor> {
        //let pipeline = pipeline.into_iter().map(Bson::Document).collect::<Vec<Bson>>();

        let mut aggregate_command = doc!{
            "aggregate": self.name(),
            "pipeline": pipeline
        };

        let mut read_preference = self.read_preference.clone();
        let mut read_concern = self.read_concern.clone();
        let mut batch_size = DEFAULT_BATCH_SIZE;
        let mut max_time_ms = 0;

        match options {
            Some(aggregate_options) => {
                if let Some(ref read_preference_option) = aggregate_options.read_preference {
                    read_preference = read_preference_option.clone();
                }

                if let Some(ref read_concern_option) = aggregate_options.read_concern {
                    read_concern = read_concern_option.clone();
                }

                if let Some(size) = aggregate_options.batch_size {
                    batch_size = size
                }

                if let Some(time_ms) = aggregate_options.max_time_ms {
                    max_time_ms = time_ms;
                }

                aggregate_command.extend(aggregate_options);
            }
            None => {
                aggregate_command.insert("cursor", doc!{});
            }
        }

        aggregate_command.insert("readConcern", read_concern.to_document());

        Cursor::command(self.db.clone(), aggregate_command, CommandType::Aggregate, Some(batch_size), Some(max_time_ms), Some(read_preference))
    }

    /// Watch changes on this collection
    pub fn watch(
        &self,
        pipeline: Vec<Document>,
        options: Option<ChangeStreamOptions>
    ) -> Result<ChargeStream> {
        ChargeStream::new(self.clone(), pipeline, options)
    }

    /// Gets the number of documents matching the filter.
    pub fn count(
        &self,
        filter: Document,
        options: Option<CountOptions>
    ) -> Result<i64> {

        let mut count_command = doc!{
            "count": self.name(),
            "query": filter
        };

        let mut read_preference = self.read_preference.clone();
        let mut read_concern = self.read_concern.clone();

        if let Some(count_options) = options {
            if let Some(ref read_preference_option) = count_options.read_preference {
                read_preference = read_preference_option.clone();
            }

            if let Some(ref read_concern_option) = count_options.read_concern {
                read_concern = read_concern_option.clone();
            }

            count_command.extend(count_options);
        }

        count_command.insert("readConcern", read_concern.to_document());

        let result = self.db.command(count_command, CommandType::Count, Some(read_preference))?;
        match result.get("n") {
            Some(&Bson::Int32(n)) => Ok(n as i64),
            Some(&Bson::Int64(n)) => Ok(n),
            _ => Err(ResponseError("No count received from server.".to_string())),
        }
    }

    /// Finds the distinct values for a specified field across a single collection.
    pub fn distinct(
        &self,
        field_name: &str,
        filter: Document,
        options: Option<DistinctOptions>
    ) -> Result<Vec<Bson>> {

        let mut distinct_command = doc!{
            "distinct": self.name(),
            "key": field_name.to_owned(),
            "query": filter
        };

        let mut read_preference = self.read_preference.clone();
        let mut read_concern = self.read_concern.clone();

        if let Some(distinct_options) = options {
            if let Some(ref read_preference_option) = distinct_options.read_preference {
                read_preference = read_preference_option.clone();
            }

            if let Some(ref read_concern_option) = distinct_options.read_concern {
                read_concern = read_concern_option.clone();
            }

            distinct_command.extend(distinct_options);
        }

        distinct_command.insert("readConcern", read_concern.to_document());

        let result = self.db.command(distinct_command, CommandType::Distinct, Some(read_preference))?;

        match result.get("values") {
            Some(&Bson::Array(ref vals)) => Ok(vals.to_vec()),
            _ => Err(ResponseError("No values received from server.".to_string())),
        }
    }

    /// Returns a list of documents within the collection that match the filter.
    pub fn find(
        &self,
        filter: Document,
        options: Option<FindOptions>
    ) -> Result<Cursor> {

        let mut find_command = doc!{
            "find": self.name(),
            "filter": filter
        };

        let mut read_preference = self.read_preference.clone();
        let mut read_concern = self.read_concern.clone();
        let mut batch_size = DEFAULT_BATCH_SIZE;
        let mut max_time_ms = 0;

        if let Some(find_options) = options {
            if let Some(ref read_preference_option) = find_options.read_preference {
                read_preference = read_preference_option.clone();
            }

            if let Some(ref read_concern_option) = find_options.read_concern {
                read_concern = read_concern_option.clone();
            }

            if let Some(size) = find_options.batch_size {
                batch_size = size
            }

            if let Some(time_ms) = find_options.max_time_ms {
                max_time_ms = time_ms;
            }

            find_command.extend(find_options);
        }

        find_command.insert("readConcern", read_concern.to_document());

        Cursor::command(self.db.clone(), find_command, CommandType::Find, Some(batch_size), Some(max_time_ms), Some(read_preference))
    }

    /// Returns the first document within the collection that matches the filter, or None.
    pub fn find_one(
        &self,
        filter: Document,
        options: Option<FindOptions>
    ) -> Result<Option<Document>> {

        let mut find_one_options = options.unwrap_or_default();
        find_one_options.limit = Some(1);
        find_one_options.batch_size = Some(1);

        let mut cursor = self.find(filter, Some(find_one_options))?;

        match cursor.next() {
            Some(Ok(doc)) => Ok(Some(doc)),
            Some(Err(err)) => Err(err),
            None => Ok(None)
        }
    }

    /// Finds a single document and deletes it, returning the original.
    pub fn find_one_and_delete(
        &self,
        filter: Document,
        options: Option<FindOneAndDeleteOptions>
    ) -> Result<Option<Document>> {
        let mut find_and_modify_command = doc!{
            "findAndModify": self.name(),
            "query": filter,
            "remove": true
        };

        let mut write_concern = self.write_concern.clone();

        if let Some(find_and_delete_options) = options {
            if let Some(ref write_concern_option) = find_and_delete_options.write_concern {
                write_concern = write_concern_option.clone();
            }

            find_and_modify_command.extend(find_and_delete_options);
        }

        find_and_modify_command.insert("writeConcern", write_concern.to_document());

        let mut result = self.db.command(find_and_modify_command, CommandType::FindAndModify, None)?;

        WriteException::validate_write_result(&result, write_concern)?;

        let doc = match result.remove("value") {
            Some(Bson::Document(nested_doc)) => Some(nested_doc),
            _ => None
        };

        Ok(doc)
    }

    /// Finds a single document and replaces it, returning either the original
    /// or replaced document.
    pub fn find_one_and_replace(
        &self,
        filter: Document,
        replacement: Document,
        options: Option<FindOneAndUpdateOptions>
    ) -> Result<Option<Document>> {
        Collection::validate_replace(&replacement)?;

        let mut find_and_modify_command = doc!{
            "findAndModify": self.name(),
            "query": filter,
            "update": replacement
        };

        let mut write_concern = self.write_concern.clone();

        if let Some(find_and_replace_options) = options {
            if let Some(ref write_concern_option) = find_and_replace_options.write_concern {
                write_concern = write_concern_option.clone();
            }

            find_and_modify_command.extend(find_and_replace_options);
        }

        find_and_modify_command.insert("writeConcern", write_concern.to_document());

        let mut result = self.db.command(find_and_modify_command, CommandType::FindAndModify, None)?;

        WriteException::validate_write_result(&result, write_concern)?;

        let doc = match result.remove("value") {
            Some(Bson::Document(nested_doc)) => Some(nested_doc),
            _ => None
        };

        Ok(doc)
    }

    /// Finds a single document and updates it, returning either the original
    /// or updated document.
    pub fn find_one_and_update(
        &self,
        filter: Document,
        update: Document,
        options: Option<FindOneAndUpdateOptions>
    ) -> Result<Option<Document>> {
        Collection::validate_update(&update)?;

        let mut find_and_modify_command = doc!{
            "findAndModify": self.name(),
            "query": filter,
            "update": update
        };

        let mut write_concern = self.write_concern.clone();

        if let Some(find_and_update_options) = options {
            if let Some(ref write_concern_option) = find_and_update_options.write_concern {
                write_concern = write_concern_option.clone();
            }

            find_and_modify_command.extend(find_and_update_options);
        }

        find_and_modify_command.insert("writeConcern", write_concern.to_document());

        let mut result = self.db.command(find_and_modify_command, CommandType::FindAndModify, None)?;

        WriteException::validate_write_result(&result, write_concern)?;

        let doc = match result.remove("value") {
            Some(Bson::Document(nested_doc)) => Some(nested_doc),
            _ => None
        };

        Ok(doc)
    }

    // Internal insertion helper function. Returns a vec of collected ids and a possible exception.
    fn insert(
        &self,
        docs: Vec<Document>,
        options: Option<InsertManyOptions>
    ) -> Result<(Vec<Bson>, Option<BulkWriteException>)> {

        let mut converted_docs = Vec::new();
        let mut ids = Vec::new();

        for doc in &docs {
            let mut cdoc = doc.clone();
            match doc.get("_id") {
                Some(id) => ids.push(id.clone()),
                None => {
                    let id = Bson::ObjectId(ObjectId::new()?);
                    cdoc.insert("_id", id.clone());
                    ids.push(id);
                }
            }
            converted_docs.push(Bson::Document(cdoc));
        }

        let mut insert_command = doc! {
            "insert": self.name(),
            "documents": converted_docs
        };

        let mut write_concern = self.write_concern.clone();

        if let Some(insert_options) = options {
            if let Some(ref write_concern_option) = insert_options.write_concern {
                write_concern = write_concern_option.clone();
            }

            insert_command.extend(insert_options);
        }

        insert_command.insert("writeConcern", write_concern.to_document());

        let result = self.db.command(insert_command, CommandType::Insert, None)?;

        // Intercept bulk write exceptions and insert into the result
        let exception = match BulkWriteException::validate_bulk_write_result(&result, write_concern) {
            Ok(()) => None,
            Err(BulkWriteError(err)) => Some(err),
            Err(e) => return Err(e),
        };

        Ok((ids, exception))
    }

    /// Inserts the provided document. If the document is missing an identifier,
    /// the driver should generate one.
    pub fn insert_one(
        &self,
        doc: Document,
        options: Option<InsertOneOptions>
    ) -> Result<InsertOneResult> {
        let mut insert_many_options = InsertManyOptions::new();

        if let Some(insert_one_options) = options {
            insert_many_options.bypass_document_validation = insert_one_options.bypass_document_validation;
            insert_many_options.write_concern = insert_one_options.write_concern;
        }

        let (ids, bulk_exception) = self.insert(vec![doc], Some(insert_many_options))?;

        if ids.is_empty() {
            return Err(OperationError("No ids returned for insert_one.".to_string()));
        }

        // Downgrade bulk exception, if it exists.
        let exception = match bulk_exception {
            Some(e) => Some(WriteException::with_bulk_exception(e)),
            None => None,
        };

        let id = match exception {
            Some(ref exc) => {
                match exc.write_error {
                    Some(_) => None,
                    None => Some(ids[0].to_owned()),
                }
            }
            None => Some(ids[0].to_owned()),
        };

        Ok(InsertOneResult::new(id, exception))
    }

    /// Inserts the provided documents. If any documents are missing an identifier,
    /// the driver should generate them.
    pub fn insert_many(
        &self,
        docs: Vec<Document>,
        options: Option<InsertManyOptions>
    ) -> Result<InsertManyResult> {

        let (ids, exception) = self.insert(docs, options)?;

        let mut map = BTreeMap::new();
        for (i, item) in ids.iter().enumerate() {
            map.insert(i as i64, item.to_owned());
        }

        if let Some(ref exc) = exception {
            for error in &exc.write_errors {
                map.remove(&(error.index as i64));
            }
        }

        Ok(InsertManyResult::new(Some(map), exception))
    }

    fn delete(
        &self,
        deletes: Vec<Document>,
        options: Option<DeleteOptions>
    ) -> Result<BulkDeleteResult> {

        let mut delete_command = doc!{
            "delete": self.name(),
        };

        let mut write_concern = self.write_concern.clone();
        let mut deletes = deletes;

        if let Some(delete_options) = options {
            if let Some(ref write_concern_option) = delete_options.write_concern {
                write_concern = write_concern_option.clone();
            }

            if let Some(ref collation) = delete_options.collation {
                for delete in deletes.iter_mut() {
                    delete.insert("collation", collation.clone());
                }
            }

            delete_command .extend(delete_options);
        }

        delete_command.insert("writeConcern", write_concern.to_document());
        delete_command.insert("deletes", deletes);

        let result = self.db.command(delete_command, CommandType::Delete, None)?;

        // Intercept write exceptions and insert into the result
        let exception = match BulkWriteException::validate_bulk_write_result(&result, write_concern) {
            Ok(()) => None,
            Err(BulkWriteError(err)) => Some(err),
            Err(e) => return Err(e),
        };

        Ok(BulkDeleteResult::new(result, exception))
    }

    pub fn delete_one(
        &self,
        filter: Document,
        options: Option<DeleteOptions>
    ) -> Result<DeleteResult> {
        let delete = doc!{
            "q": filter,
            "limit": 1
        };

        let result = self.delete(vec![delete], options)?;

        Ok(DeleteResult::with_bulk_result(result))
    }

    pub fn delete_many(
        &self,
        filter: Document,
        options: Option<DeleteOptions>
    ) -> Result<DeleteResult> {
        let delete = doc!{
            "q": filter,
            "limit": 0
        };

        let result = self.delete(vec![delete], options)?;

        Ok(DeleteResult::with_bulk_result(result))
    }

    fn update(
        &self,
        updates: Vec<Document>,
        options: Option<UpdateOptions>
    ) -> Result<BulkUpdateResult> {
        let mut update_command = doc!{
            "update": self.name()
        };

        let mut write_concern = self.write_concern.clone();
        let mut updates = updates;

        if let Some(update_options) = options {
            if let Some(ref write_concern_option) = update_options.write_concern {
                write_concern = write_concern_option.clone();
            }

            for update in updates.iter_mut() {
                if let Some(upsert) = update_options.upsert {
                    update.insert("upsert", upsert);
                }

                if let Some(ref collation) = update_options.collation {
                    update.insert("collation", collation.clone());
                }

                if let Some(ref array_filters) = update_options.array_filters {
                    update.insert("arrayFilters", array_filters.clone());
                }
            }

            update_command.extend(update_options);
        }

        update_command.insert("writeConcern", write_concern.to_document());
        update_command.insert("updates", updates);

        let result = self.db.command(update_command, CommandType::Update, None)?;

        // Intercept write exceptions and insert into the result
        let exception = match BulkWriteException::validate_bulk_write_result(&result, write_concern) {
            Ok(()) => None,
            Err(BulkWriteError(err)) => Some(err),
            Err(e) => return Err(e),
        };

        Ok(BulkUpdateResult::new(result, exception))
    }

    pub fn update_one(
        &self,
        filter: Document,
        update: Document,
        options: Option<UpdateOptions>
    ) -> Result<UpdateResult> {
        Collection::validate_update(&update)?;

        let update = doc!{
            "q": filter,
            "u": update,
            "multi": true
        };

        let result = self.update(vec![update], options)?;

        Ok(UpdateResult::with_bulk_result(result))
    }

    pub fn update_many(
        &self,
        filter: Document,
        update: Document,
        options: Option<UpdateOptions>
    ) -> Result<UpdateResult> {
        Collection::validate_update(&update)?;

        let update = doc!{
            "q": filter,
            "u": update,
            "multi": true
        };

        let result = self.update(vec![update], options)?;

        Ok(UpdateResult::with_bulk_result(result))
    }

    pub fn replace_one(
        &self,
        filter: Document,
        replacement: Document,
        options: Option<UpdateOptions>
    ) -> Result<UpdateResult> {
         Collection::validate_replace(&replacement)?;

        let update = doc!{
            "q": filter,
            "u": replacement,
            "multi": false
        };

        let result = self.update(vec![update], options)?;

        Ok(UpdateResult::with_bulk_result(result))
    }

    fn validate_replace(replacement: &Document) -> Result<()> {
        for key in replacement.keys() {
            if key.starts_with('$') {
                return Err(ArgumentError("Replacement cannot include $ operators.".to_string()));
            }
        }

        Ok(())
    }

    fn validate_update(update: &Document) -> Result<()> {
        for key in update.keys() {
            if !key.starts_with('$') {
                return Err(ArgumentError("Update only works with $ operators.".to_string()));
            }
        }

        Ok(())
    }

    /// List all indexes in the collection.
    pub fn list_indexes(&self) -> Result<Cursor> {
        let list_indexs_command = doc!{
            "listIndexes": self.name()
        };

        Cursor::command(
            self.db.clone(),
            list_indexs_command,
            CommandType::ListIndexes,
            None,
            None,
            None
        )
    }

    /// Create a single index.
    pub fn create_index(
        &self,
        keys: Document,
        options: Option<IndexOptions>,
        write_concern: Option<WriteConcern>
    ) -> Result<String> {
        let model = IndexModel::new(keys, options);
        self.create_index_model(model, write_concern)
    }

    /// Create a single index with an IndexModel.
    pub fn create_index_model(
        &self,
        model: IndexModel,
        write_concern: Option<WriteConcern>
    ) -> Result<String> {
        let result = self.create_indexes(vec![model], write_concern)?;

        Ok(result[0].to_string())
    }

    /// Create multiple indexes.
    pub fn create_indexes(
        &self,
        models: Vec<IndexModel>,
        write_concern: Option<WriteConcern>
    ) -> Result<Vec<String>> {
        let mut names = Vec::with_capacity(models.len());
        let mut indexes = Vec::with_capacity(models.len());

        for model in models {
            names.push(model.name()?);
            indexes.push(model.to_document()?);
        }

        let write_concern = write_concern.unwrap_or(self.write_concern.clone());

        let create_indexes_command = doc!{
            "createIndexes": self.name(),
            "indexes": indexes,
            "writeConcern": write_concern.to_document()
        };

        let result = self.db.command(create_indexes_command, CommandType::CreateIndexes, None)?;

        match result.get("errmsg") {
            Some(&Bson::String(ref msg)) => Err(OperationError(msg.to_string())),
            _ => Ok(names)
        }
    }

    /// Drop an index.
    pub fn drop_index(&self, keys: Document, options: Option<IndexOptions>) -> Result<()> {
        let model = IndexModel::new(keys, options);
        self.drop_index_model(model)
    }

    /// Drop an index by name.
    pub fn drop_index_string(&self, name: String) -> Result<()> {
        let mut options = IndexOptions::new();
        options.name = Some(name.to_string());

        let model = IndexModel::new(Document::new(), Some(options));
        self.drop_index_model(model)
    }

    /// Drop an index by IndexModel.
    pub fn drop_index_model(
        &self,
        model: IndexModel
    ) -> Result<()> {
        let drop_index_command = doc!{
            "dropIndexes": self.name(),
            "index": model.name()?
        };

        let result = self.db.command(drop_index_command, CommandType::DropIndexes, None)?;
        match result.get("errmsg") {
            Some(&Bson::String(ref msg)) => Err(OperationError(msg.to_string())),
            _ => Ok(()),
        }
    }

    /// Drop all indexes in the collection.
    pub fn drop_indexes(&self) -> Result<()> {
        let mut options = IndexOptions::new();
        options.name = Some("*".to_owned());

        let model = IndexModel::new(doc!{}, Some(options));
        self.drop_index_model(model)
    }
}
