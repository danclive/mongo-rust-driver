use crate::sys::mongoc::*;

pub struct FindAndModifyOpts(*mut mongoc_find_and_modify_opts_t);

impl FindAndModifyOpts {
    pub fn new() -> FindAndModifyOpts {
        let ptr = unsafe { mongoc_find_and_modify_opts_new() };
        FindAndModifyOpts::from_ptr(ptr)
    }

    pub fn from_ptr(ptr: *mut mongoc_find_and_modify_opts_t) -> FindAndModifyOpts {
        assert!(!ptr.is_null());
        FindAndModifyOpts(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_find_and_modify_opts_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut mongoc_find_and_modify_opts_t {
        assert!(!self.0.is_null());
        self.0
    }
}

impl Drop for FindAndModifyOpts {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_find_and_modify_opts_destroy(self.0) }
    }
}

