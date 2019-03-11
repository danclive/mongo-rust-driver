use std::ffi::CStr;
use crate::sys::mongoc::*;

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
