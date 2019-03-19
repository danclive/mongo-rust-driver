use crate::sys::mongoc::*;

pub struct ClientSession(*mut mongoc_client_session_t);

impl ClientSession {
    pub fn from_ptr(ptr: *mut mongoc_client_session_t) -> ClientSession {
        assert!(!ptr.is_null());
        ClientSession(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_client_session_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut mongoc_client_session_t {
        assert!(!self.0.is_null());
        self.0
    }
}

impl Drop for ClientSession {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_client_session_destroy(self.0); }
    }
}
