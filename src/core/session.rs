use crate::sys::mongoc::*;

pub struct Session(*mut mongoc_session_opt_t);

impl Session {
    pub fn new() -> Session {
        let ptr = unsafe { mongoc_session_opts_new() };
        assert!(!ptr.is_null());
        Session::from_ptr(ptr)
    }

    pub fn from_ptr(ptr: *mut mongoc_session_opt_t) -> Session {
        assert!(!ptr.is_null());
        Session(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_session_opt_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut mongoc_session_opt_t {
        assert!(!self.0.is_null());
        self.0
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_session_opts_destroy(self.0); }
    }
}

impl Clone for Session {
    fn clone(&self) -> Session {
        assert!(!self.0.is_null());
        unsafe { Session(mongoc_session_opts_clone(self.0)) }
    }
}
