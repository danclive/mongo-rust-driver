use std::ffi::CStr;
use crate::sys::mongoc::*;
use crate::sys::bsonc::*;
use crate::core::bsonc::Bsonc;
use crate::core::read_preference::ReadPreference;
use crate::core::find_and_modify::FindAndModifyOpts;
use crate::core::error::MongocError;
use crate::core::cursor::Cursor;
use crate::core::change_stream::ChangeStream;
use crate::error::Result;

// see http://mongoc.org/libmongoc/current/mongoc_collection_t.html
pub struct Collection(*mut mongoc_collection_t);

impl Collection {
    pub fn from_ptr(ptr: *mut mongoc_collection_t) -> Collection {
        assert!(!ptr.is_null());
        Collection(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_collection_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&self) -> *mut mongoc_collection_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn get_name(&self) -> String {
        assert!(!self.0.is_null());
        let s = unsafe {
            let ptr = mongoc_collection_get_name(self.0);
            CStr::from_ptr(ptr)
        };

        s.to_string_lossy().to_string()
    }

    pub fn command_simple(&self, command: &Bsonc, read_prefs: Option<ReadPreference>) -> Result<Bsonc> {
        let read_prefs = read_prefs.unwrap_or_default();

        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_command_simple(
            self.0,
            command.as_ptr(),
            read_prefs.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        return Err(error.into())
    }

    pub fn command_with_opts(
        &self,
        command: &Bsonc,
        read_prefs: Option<ReadPreference>,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let read_prefs = read_prefs.unwrap_or_default();

        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_command_with_opts(
            self.0,
            command.as_ptr(),
            read_prefs.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        return Err(error.into())
    }

    pub fn read_command_with_opts(
        &self,
        command: &Bsonc,
        read_prefs: Option<ReadPreference>,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let read_prefs = read_prefs.unwrap_or_default();

        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_read_command_with_opts(
            self.0,
            command.as_ptr(),
            read_prefs.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        return Err(error.into())
    }

    pub fn write_command_with_opts(
        &self,
        command: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_write_command_with_opts(
            self.0,
            command.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        return Err(error.into())
    }

    pub fn read_write_command_with_opts(
        &self,
        command: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_read_write_command_with_opts(
            self.0,
            command.as_ptr(),
            std::ptr::null(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        return Err(error.into())
    }

    pub fn count_documents(
        &self,
        filter: &Bsonc,
        opts: &Bsonc,
        read_prefs: Option<ReadPreference>
    ) -> Result<(Bsonc, i64)> {
        let read_prefs = read_prefs.unwrap_or_default();

        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        let count = unsafe {
            mongoc_collection_count_documents(
                self.0,
                filter.as_ptr(),
                opts.as_ptr(),
                read_prefs.as_ptr(),
                reply.as_mut_ptr(),
                error.as_mut_ptr()
            )
        };

        if count < 0 {
            return Err(error.into())
        }

        return Ok((reply, count))
    }

    pub fn estimated_document_count(
        &self,
        opts: &Bsonc,
        read_prefs: Option<ReadPreference>
    ) -> Result<(Bsonc, i64)> {
        let read_prefs = read_prefs.unwrap_or_default();

        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        let count = unsafe {
            mongoc_collection_estimated_document_count(
                self.0,
                opts.as_ptr(),
                read_prefs.as_ptr(),
                reply.as_mut_ptr(),
                error.as_mut_ptr()
            )
        };

        if count < 0 {
            return Err(error.into())
        }

        return Ok((reply, count))
    }

    // see http://mongoc.org/libmongoc/current/mongoc_collection_find_with_opts.html
    pub fn find_with_opts(
        &self,
        filter: &Bsonc,
        opts: &Bsonc,
        read_prefs: Option<ReadPreference>
    ) -> Cursor {
        let read_prefs = read_prefs.unwrap_or_default();

        let cursor = unsafe {
            mongoc_collection_find_with_opts(
                self.0,
                filter.as_ptr(),
                opts.as_ptr(),
                read_prefs.as_ptr()
            )
        };

        Cursor::from_ptr(cursor)
    }

    pub fn find_and_modify_with_opts(
        &self,
        query: &Bsonc,
        opts: &FindAndModifyOpts
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_find_and_modify_with_opts(
            self.0,
            query.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn insert_one(
        &self,
        doc: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_insert_one(
            self.0,
            doc.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn insert_many(
        &self,
        docs: &Vec<Bsonc>,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();
        let mut docs: Vec<*const _> = docs.iter().map(|b| b.as_ptr()).collect();

        if unsafe { mongoc_collection_insert_many(
            self.0,
            docs.as_mut_ptr(),
            docs.len(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn update_one(
        &self,
        selector: &Bsonc,
        update: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_update_one(
            self.0,
            selector.as_ptr(),
            update.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn update_many(
        &self,
        selector: &Bsonc,
        update: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_update_many(
            self.0,
            selector.as_ptr(),
            update.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn delete_one(
        &self,
        selector: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_delete_one(
            self.0,
            selector.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn delete_many(
        &self,
        selector: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_delete_many(
            self.0,
            selector.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn replace_one(
        &self,
        selector: &Bsonc,
        replace: &Bsonc,
        opts: &Bsonc
    ) -> Result<Bsonc> {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_collection_replace_one(
            self.0,
            selector.as_ptr(),
            replace.as_ptr(),
            opts.as_ptr(),
            reply.as_mut_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(reply)
        }

        Err(error.into())
    }

    pub fn aggregate(
        &self,
        flags: u32,
        pipeline: &Bsonc,
        opts: &Bsonc,
        read_prefs: Option<ReadPreference>
    ) -> Cursor {
        let read_prefs = read_prefs.unwrap_or_default();

        let cursor = unsafe {
            mongoc_collection_aggregate(
                self.0,
                flags,
                pipeline.as_ptr(),
                opts.as_ptr(),
                read_prefs.as_ptr()
            )
        };

        Cursor::from_ptr(cursor)
    }

    pub fn watch(
        &self,
        pipeline: &Bsonc,
        opts: &Bsonc
    ) -> ChangeStream {
        let change_stream = unsafe {
            mongoc_collection_watch(
                self.0,
                pipeline.as_ptr(),
                opts.as_ptr()
            )
        };

        ChangeStream::from_ptr(change_stream)
    }

    pub fn drop_with_opts(
        &self,
        opts: &Bsonc
    ) -> Result<()> {
        let mut error = MongocError::empty();

        if unsafe { mongoc_collection_drop_with_opts(
            self.0,
            opts.as_ptr(),
            error.as_mut_ptr()
        ) } {
            return Ok(())
        }

        Err(error.into())
    }

    pub fn keys_to_index_string(&self, keys: &Bsonc) -> String {
        let ptr = unsafe {
            mongoc_collection_keys_to_index_string(keys.as_ptr())
        };

        assert!(!ptr.is_null());

        let s = unsafe { CStr::from_ptr(ptr as *const _) };
        let s = s.to_string_lossy().to_string();

        unsafe { bson_free(ptr as _); }

        s
    }
}

impl Drop for Collection {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_collection_destroy(self.0) }
    }
}

impl Clone for Collection {
    fn clone(&self) -> Collection {
        assert!(!self.0.is_null());
        unsafe { Collection(mongoc_collection_copy(self.0)) }
    }
}
