use bsonrs::{Document, Value};

use crate::core::bsonc::Bsonc;
use crate::core::collection::Collection as CoreCollection;
use crate::db::DB;
use crate::client::Client;
use crate::read_preference::ReadPreference;
use crate::cursor::Cursor;
use crate::error::Result;

use options::FindOptions;
use options::FindAndModifyOpts;

pub mod options;

pub struct Collection {
    pub db: DB,
    pub name: String
}

impl Collection {
    pub fn new(db: DB, name: &str) -> Collection {
        Collection {
            db,
            name: name.to_owned()
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn acquire_client(&self) -> Client {
        self.db.acquire_client()
    }

    #[inline]
    pub fn get_core_collection(&self, client: &Client) -> CoreCollection {
        let collection = client.get_collection(self.db.name(), self.name());
        collection
    }

    pub fn command_simple(&self, command: Document, read_prefs: Option<ReadPreference>) ->  Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let command = Bsonc::from_doc(&command)?;

        let reply = collection.command_simple(&command, read_prefs)?;

        Ok(reply.as_doc()?)
    }

    pub fn count_documents(
        &self,
        filter: Document,
        opts: Option<Document>,
        read_prefs: Option<ReadPreference>
    ) -> Result<usize> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let filter = Bsonc::from_doc(&filter)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let (_, count) = collection.count_documents(&filter, &opts, read_prefs)?;

        Ok(count as usize)
    }

    pub fn estimated_document_count(
        &self,
        opts: Option<Document>,
        read_prefs: Option<ReadPreference>
    ) -> Result<usize> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let (_, count) = collection.estimated_document_count(&opts, read_prefs)?;

        Ok(count as usize)
    }

    pub fn find(
        &self,
        filter: Document,
        opts: Option<FindOptions>,
        read_prefs: Option<ReadPreference>
    ) -> Result<Cursor> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let filter = Bsonc::from_doc(&filter)?;
        let opts = Bsonc::from_doc(&opts.map(|o| o.into()).unwrap_or_default())?;

        let core_cursor = collection.find_with_opts(&filter, &opts, read_prefs);

        Ok(Cursor::new(core_cursor, client))
    }

    pub fn find_one(
        &self,
        _filter: Document,
        _opts: Option<FindOptions>,
        _read_prefs: Option<ReadPreference>
    ) -> Result<Document> {
        unimplemented!()
    }

    pub fn find_by_id(
        _id: impl Into<Value>,
        _opts: Option<FindOptions>,
        _read_prefs: Option<ReadPreference>
    ) -> Result<Document> {
        unimplemented!()
    }

    pub fn find_and_modify(
        &self,
        query: Document,
        opts: FindAndModifyOpts
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let query = Bsonc::from_doc(&query)?;

        let reply = collection.find_and_modify_with_opts(&query, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn insert_one(
        &self,
        doc: Document,
        opts: Option<Document>
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let doc = Bsonc::from_doc(&doc)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let reply = collection.insert_one(&doc, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn insert_many(
        &self,
        docs: Vec<Document>,
        opts: Option<Document>
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);

        let mut bsoncs = Vec::new();
        for doc in docs {
            bsoncs.push(Bsonc::from_doc(&doc)?);
        }

        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let reply = collection.insert_many(&bsoncs, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn update_one(
        &self,
        selector: Document,
        update: Document,
        opts: Option<Document>
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let selector = Bsonc::from_doc(&selector)?;
        let update = Bsonc::from_doc(&update)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let reply = collection.update_one(&selector, &update, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn update_many(
        &self,
        selector: Document,
        update: Document,
        opts: Option<Document>
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let selector = Bsonc::from_doc(&selector)?;
        let update = Bsonc::from_doc(&update)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let reply = collection.update_many(&selector, &update, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn delete_one(
        &self,
        selector: Document,
        opts: Option<Document>
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let selector = Bsonc::from_doc(&selector)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let reply = collection.delete_one(&selector, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn delete_many(
        &self,
        selector: Document,
        opts: Option<Document>
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let selector = Bsonc::from_doc(&selector)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let reply = collection.delete_many(&selector, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn replace_one(
        &self,
        selector: Document,
        replace: Document,
        opts: Option<Document>
    ) -> Result<Document> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let selector = Bsonc::from_doc(&selector)?;
        let replace = Bsonc::from_doc(&replace)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let reply = collection.replace_one(&selector, &replace, &opts)?;

        Ok(reply.as_doc()?)
    }

    pub fn aggregate(
        &self,
        pipeline: Document,
        opts: Option<Document>,
        read_prefs: Option<ReadPreference>
    ) -> Result<Cursor> {
        let client = self.acquire_client();
        let collection = self.get_core_collection(&client);
        let pipeline = Bsonc::from_doc(&pipeline)?;
        let opts = Bsonc::from_doc(&opts.unwrap_or_default())?;

        let core_cursor = collection.aggregate(0, &pipeline, &opts, read_prefs);

        Ok(Cursor::new(core_cursor, client))
    }
}
