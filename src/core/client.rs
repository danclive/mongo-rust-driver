use std::ffi::CString;

use crate::sys::bsonc::*;
use crate::sys::mongoc::*;
use crate::core::uri::Uri;
use crate::core::database::Database;
use crate::core::collection::Collection;
use crate::core::bsonc::Bsonc;
use crate::core::error::MongocError;
use crate::error::Result;

pub struct Client(*mut mongoc_client_t);

impl Client {
    pub fn new(url: impl Into<Vec<u8>>) -> Option<Client> {
        let cstring = CString::new(url).expect("CString::new failed");

        let client = unsafe {
            mongoc_client_new(cstring.as_ptr())
        };

        if client.is_null() {
            None
        } else {
            Some(Client(client))
        }
    }

    pub fn new_from_uri(uri: &Uri) -> Client {
        let client = unsafe {
            mongoc_client_new_from_uri(uri.as_ptr())
        };

        assert!(!client.is_null());
        Client(client)
    }

    pub fn from_ptr(ptr: *mut mongoc_client_t) -> Client {
        assert!(!ptr.is_null());
        Client(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_client_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&self) -> *mut mongoc_client_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn get_database(&self, name: impl Into<Vec<u8>>) -> Database {
        assert!(!self.0.is_null());
        let cstring = CString::new(name).expect("CString::new failed");
        let database = unsafe {
            mongoc_client_get_database(self.0, cstring.as_ptr())
        };

        assert!(!database.is_null());
        Database::from_ptr(database)
    }

    pub fn get_database_names_with_opts(&self, opts: &Bsonc) -> Result<Vec<String>> {
        assert!(!self.0.is_null());
        let mut err = MongocError::empty();

        let ptr = unsafe {
            mongoc_client_get_database_names_with_opts(self.0, opts.as_ptr(), err.mut_inner())
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

    pub fn get_collection(&self, db: &str, collection: &str) -> Collection {
        assert!(!self.0.is_null());
        let cstring1 = CString::new(db).expect("CString::new failed");
        let cstring2 = CString::new(collection).expect("CString::new failed");

        let collection = unsafe {
            mongoc_client_get_collection(self.0, cstring1.as_ptr(), cstring2.as_ptr())
        };

        assert!(!collection.is_null());

        Collection::from_ptr(collection)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_client_destroy(self.0) }
    }
}
