use crate::sys::mongoc::*;
use crate::core::bsonc::Bsonc;
use crate::core::error::MongocError;
use crate::error::Result;

pub struct Cursor(*mut mongoc_cursor_t);

impl Cursor {
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

    pub fn error(&self) -> Result<()> {
        let mut error = MongocError::empty();

        if unsafe { mongoc_cursor_error(self.0, error.mut_inner()) } {
            return Err(error.into())
        }

        Ok(())
    }

    pub fn error_document(&self) {
        let mut error = MongocError::empty();
        let reply = Bsonc::new();

        if unsafe { mongoc_cursor_error_document(self.0, error.mut_inner(), &mut reply.as_ptr()) } {

        }
    }

    pub fn get_batch_size(&self) -> u32 {
        unsafe { mongoc_cursor_get_batch_size(self.0) }
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
