use std::ffi::{CString, CStr};

use crate::sys::bsonc::*;
use crate::sys::mongoc::*;
use crate::core::collection::Collection;
use crate::core::bsonc::Bsonc;
use crate::core::error::MongocError;
use crate::error::Result;

pub struct Database(*mut mongoc_database_t);

impl Database {
    pub fn from_ptr(ptr: *mut mongoc_database_t) -> Database {
        assert!(!ptr.is_null());
        Database(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_database_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&self) -> *mut mongoc_database_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn get_name(&self) -> String {
        assert!(!self.0.is_null());
        let s = unsafe {
            let ptr = mongoc_database_get_name(self.0);
            CStr::from_ptr(ptr)
        };

        s.to_string_lossy().to_string()
    }

    pub fn get_collection(&self, name: impl Into<Vec<u8>>) -> Collection {
        assert!(!self.0.is_null());
        let cstring = CString::new(name).expect("CString::new failed");

        let collection = unsafe {
            mongoc_database_get_collection(self.0, cstring.as_ptr())
        };

        Collection::from_ptr(collection)
    }

    pub fn get_collection_names_with_opts(&self, opts: &Bsonc) -> Result<Vec<String>> {
        assert!(!self.0.is_null());
        let mut err = MongocError::empty();

        let ptr = unsafe {
            mongoc_database_get_collection_names_with_opts(self.0, opts.as_ptr(), err.mut_inner())
        };

        if ptr.is_null() {
            Err(err.into())
        } else {
            let mut vec_str = Vec::new();

            unsafe {
                let mut idx = 0;

                while !ptr.offset(idx).is_null() && !(*(ptr.offset(idx))).is_null() {
                    vec_str.push(
                        CString::from_raw(*(ptr.offset(idx))).into_string().unwrap()
                    );

                    idx += 1;
                }

                bson_strfreev(ptr);
            }

            Ok(vec_str)
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_database_destroy(self.0) }
    }
}

impl Clone for Database {
    fn clone(&self) -> Database {
        assert!(!self.0.is_null());
        unsafe { Database(mongoc_database_copy(self.0)) }
    }
}
