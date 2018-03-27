use std::{io, fs};
use std::sync::Arc;

use object_id::ObjectId;
use bson::Document;

use db::Database;
use collection::Collection;
use collection::options::FindOptions;
use cursor::Cursor;
use error::Error::{self, ArgumentError};
use error::Result;

use self::file::{File, Mode};

pub mod file;

/// A default cursor wrapper that maps bson documents into GridFS file representations.
pub struct FileCursor {
    store: Store,
    cursor: Cursor,
    err: Option<Error>,
}

impl Iterator for FileCursor {
    type Item = File;

    fn next(&mut self) -> Option<File> {
        match self.cursor.next() {
            Some(Ok(bdoc)) => Some(File::with_doc(self.store.clone(), bdoc)),
            Some(Err(err)) => {
                self.err = Some(err);
                None
            }
            None => None,
        }
    }
}

impl FileCursor {
    /// Returns the next n files.
    pub fn next_n(&mut self, n: i32) -> Result<Vec<File>> {
        let docs = self.cursor.next_n(n)?;
        Ok(docs.into_iter()
            .map(|doc| File::with_doc(self.store.clone(), doc.clone()))
            .collect())
    }

    /// Returns the next batch of files.
    pub fn drain_current_batch(&mut self) -> Result<Vec<File>> {
        let docs = self.cursor.drain_current_batch()?;
        Ok(docs.into_iter()
            .map(|doc| File::with_doc(self.store.clone(), doc))
            .collect())
    }
}

#[derive(Clone)]
pub struct Store {
    pub inner: Arc<StoreInner>,
}

pub struct StoreInner {
    files: Collection,
    chunks: Collection,
}

fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

impl Store {
    pub fn with_db(db: Database) -> Store {
        Store::with_prefix(db, "fs".to_string())
    }

    pub fn with_prefix(db: Database, prefix: String) -> Store {
        is_send::<Store>();
        is_sync::<Store>();

        Store {
            inner: Arc::new(StoreInner {
                files: db.collection(&format!("{}.files", prefix)),
                chunks: db.collection(&format!("{}.chunks", prefix)),
            })
        }
    }

    pub fn create(&self, name: String) -> Result<File> {
        Ok(File::with_name(self.clone(), name, ObjectId::new()?, Mode::Write))
    }

    pub fn open(&self, name: String) -> Result<File> {
        let mut options = FindOptions::new();
        options.sort = Some(doc!{ "uploadDate": 1 });

        match self.inner.files.find_one(doc!{ "filename": name }, Some(options))? {
            Some(bdoc) => Ok(File::with_doc(self.clone(), bdoc)),
            None => Err(ArgumentError("File does not exist.".to_string())),
        }
    }

    pub fn open_id(&self, id: ObjectId) -> Result<File> {
        match self.inner.files.find_one(doc!{ "_id": id }, None)? {
            Some(bdoc) => Ok(File::with_doc(self.clone(), bdoc)),
            None => Err(ArgumentError("File does not exist.".to_string())),
        }
    }

    pub fn find(&self, filter: Document, options: Option<FindOptions>) -> Result<FileCursor> {
        Ok(FileCursor {
            store: self.clone(),
            cursor: self.inner.files.find(filter, options)?,
            err: None,
        })
    }

    pub fn remove(&self, name: String) -> Result<()> {
        let mut options = FindOptions::new();
        options.projection = Some(doc!{ "_id": 1 });

        let cursor = self.find(doc!{ "filename": name }, Some(options))?;
        for doc in cursor {
            self.remove_id(doc.id.clone())?;
        }

        Ok(())
    }

    pub fn remove_id(&self, id: ObjectId) -> Result<()> {
        self.inner.files.delete_many(doc!{ "_id": id.clone() }, None)?;
        self.inner.chunks.delete_many(doc!{ "files_id": id.clone() }, None)?;
        Ok(())
    }

    pub fn put(&self, name: String) -> Result<()> {
        let mut file = self.create(name.clone())?;
        let mut f = fs::File::open(name)?;
        io::copy(&mut f, &mut file)?;
        file.close()?;
        Ok(())
    }

    pub fn get(&self, name: String) -> Result<()> {
        let mut f = fs::File::create(name.clone())?;
        let mut file = self.open(name)?;
        io::copy(&mut file, &mut f)?;
        file.close()?;
        Ok(())
    }
}
