use crate::sys::mongoc::*;
use crate::sys::bsonc::*;
use crate::core::bsonc::Bsonc;
use crate::core::error::MongocError;
use crate::error::Error;

// see http://mongoc.org/libmongoc/current/mongoc_change_stream_t.html
pub struct ChangeStream(*mut mongoc_change_stream_t);

impl ChangeStream {
    pub fn from_ptr(ptr: *mut mongoc_change_stream_t) -> ChangeStream {
        assert!(!ptr.is_null());
        ChangeStream(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_change_stream_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut mongoc_change_stream_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn error_document(&self) -> Option<(Error, Bsonc)> {
        let mut error = MongocError::empty();
        let mut bson_ptr: *const bson_t = std::ptr::null();

        if unsafe { mongoc_change_stream_error_document(self.0, error.as_mut_ptr(), &mut bson_ptr) } {
            let bsonc = Bsonc::from_ptr(bson_ptr as *mut _);
            return Some((error.into(), bsonc))
        }

        None
    }

    pub fn next(&self) -> Option<Bsonc> {
        let mut bson_ptr: *const bson_t = std::ptr::null();

        if unsafe { mongoc_change_stream_next(self.0, &mut bson_ptr) } {
            let bsonc = Bsonc::from_ptr(bson_ptr as *mut _);
            return Some(bsonc)
        }

        None
    }
}

impl Drop for ChangeStream {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_change_stream_destroy(self.0) }
    }
}

