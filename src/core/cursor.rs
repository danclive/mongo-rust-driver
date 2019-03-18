use crate::sys::mongoc::*;
use crate::sys::bsonc::*;
use crate::core::bsonc::Bsonc;
use crate::core::client::Client;
use crate::core::error::MongocError;
use crate::error::Error;

// see http://mongoc.org/libmongoc/current/mongoc_cursor_t.html
pub struct Cursor(*mut mongoc_cursor_t);

impl Cursor {
    pub fn new_from_command_reply_with_opts(client: &Client, reply: &mut Bsonc, opts: &Bsonc) -> Cursor {
        let ptr = unsafe {
            mongoc_cursor_new_from_command_reply_with_opts(client.as_mut_ptr(), reply.as_mut_ptr(), opts.as_ptr())
        };

        Cursor::from_ptr(ptr)
    }

    pub fn from_ptr(ptr: *mut mongoc_cursor_t) -> Cursor {
        assert!(!ptr.is_null());
        Cursor(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_cursor_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut mongoc_cursor_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn current(&self) -> Option<Bsonc> {
        let ptr = unsafe {
            mongoc_cursor_current(self.0)
        };

        if ptr.is_null() {
            None
        } else {
            Some(Bsonc::from_ptr(ptr as *mut _))
        }
    }

    pub fn error(&self) -> Option<Error> {
        let mut error = MongocError::empty();

        if unsafe { mongoc_cursor_error(self.0, error.as_mut_ptr()) } {
            return Some(error.into())
        }

        None
    }

    pub fn error_document(&self) -> Option<(Error, Bsonc)> {
        let mut error = MongocError::empty();
        let mut bson_ptr: *const bson_t = std::ptr::null();

        if unsafe { mongoc_cursor_error_document(self.0, error.as_mut_ptr(), &mut bson_ptr) } {
            let bsonc = Bsonc::from_ptr(bson_ptr as *mut _);
            return Some((error.into(), bsonc))
        }

        None
    }

    pub fn get_batch_size(&self) -> u32 {
        unsafe { mongoc_cursor_get_batch_size(self.0) }
    }

    pub fn get_hint(&self) -> u32 {
        unsafe { mongoc_cursor_get_hint(self.0) }
    }

    pub fn get_id(&self) -> i64 {
        unsafe { mongoc_cursor_get_id(self.0) }
    }

    pub fn get_limit(&self) -> i64 {
        unsafe { mongoc_cursor_get_limit(self.0) }
    }

    pub fn get_max_await_time_ms(&self) -> u32 {
        unsafe { mongoc_cursor_get_max_await_time_ms(self.0) }
    }

    pub fn more(&self) -> bool {
        unsafe { mongoc_cursor_more(self.0) }
    }

    pub fn next(&self) -> Option<Bsonc> {
        let mut bson_ptr: *const bson_t = std::ptr::null();

        if unsafe { mongoc_cursor_next(self.0, &mut bson_ptr) } {
            let bsonc = Bsonc::from_ptr(bson_ptr as *mut _);
            return Some(bsonc)
        }

        None
    }

    pub fn set_batch_size(&self, size: u32) {
        unsafe { mongoc_cursor_set_batch_size(self.0, size) }
    }

    pub fn set_hint(&self, server_id: u32) -> bool {
        unsafe { mongoc_cursor_set_hint(self.0, server_id) }
    }

    pub fn set_limit(&self, limit: i64) -> bool {
        unsafe { mongoc_cursor_set_limit(self.0, limit) }
    }

    pub fn set_max_await_time_ms(&self, max_await_time_ms: u32) {
        unsafe { mongoc_cursor_set_max_await_time_ms(self.0, max_await_time_ms) }
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { mongoc_cursor_destroy(self.0) }
    }
}

impl Clone for Cursor {
    fn clone(&self) -> Cursor {
        unsafe { Cursor(mongoc_cursor_clone(self.0)) }
    }
}
