use crate::sys::mongoc::*;

pub struct WriteConcern(*mut mongoc_write_concern_t);

impl WriteConcern {
    pub fn new() -> WriteConcern {
        let ptr = unsafe { mongoc_write_concern_new() };
        WriteConcern::from_ptr(ptr)
    }

    pub fn from_ptr(ptr: *mut mongoc_write_concern_t) -> WriteConcern {
        assert!(!ptr.is_null());
        WriteConcern(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_write_concern_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut mongoc_write_concern_t {
        assert!(!self.0.is_null());
        self.0
    }
}

impl Drop for WriteConcern {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_write_concern_destroy(self.0) }
    }
}

impl Clone for WriteConcern {
    fn clone(&self) -> Self {
        unsafe { WriteConcern(mongoc_write_concern_copy(self.0)) }
    }
}

