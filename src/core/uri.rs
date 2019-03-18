use std::ffi::CString;

use crate::sys::mongoc::*;
use crate::core::error::MongocError;
use crate::error::Result;

#[derive(Debug)]
pub struct Uri(*mut mongoc_uri_t);

impl Uri {
    pub fn new(url: impl Into<Vec<u8>>) -> Option<Uri> {
        let cstring = CString::new(url).expect("CString::new failed");

        let uri = unsafe {
            mongoc_uri_new(cstring.as_ptr())
        };

        if uri.is_null() {
            None
        } else {
            Some(Uri(uri))
        }
    }

    pub fn new_with_error(url: impl Into<Vec<u8>>) -> Result<Uri> {
        let cstring = CString::new(url).expect("CString::new failed");
        let mut err = MongocError::empty();

        let uri = unsafe {
            mongoc_uri_new_with_error(cstring.as_ptr(), err.as_mut_ptr())
        };

        if uri.is_null() {
            Err(err.into())
        } else {
            Ok(Uri(uri))
        }
    }

    pub fn new_for_host_port(host: impl Into<Vec<u8>>, port: u16) -> Uri {
        let cstring = CString::new(host).expect("CString::new failed");
        let uri = unsafe {
            mongoc_uri_new_for_host_port(cstring.as_ptr(), port)
        };

        Uri(uri)
    }

    pub fn from_ptr(ptr: *mut mongoc_uri_t) -> Uri {
        assert!(!ptr.is_null());
        Uri(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_uri_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&self) -> *mut mongoc_uri_t {
        assert!(!self.0.is_null());
        self.0
    }
}

impl Drop for Uri {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { mongoc_uri_destroy(self.0) }
    }
}

impl Clone for Uri {
    fn clone(&self) -> Uri {
        assert!(!self.0.is_null());
        unsafe { Uri(mongoc_uri_copy(self.0)) }
    }
}

#[test]
fn new_uri() {
    let uri = Uri::new("aaa");
    assert!(uri.is_none());

    let uri = Uri::new("mongodb://localhost/?replicaSet=myreplset");
    assert!(uri.is_some());
}

#[test]
fn new_uri_with_error() {
    let uri = Uri::new_with_error("aaa");
    assert!(uri.is_err());

    let uri = Uri::new_with_error("mongodb://localhost/?replicaSet=myreplset");
    assert!(uri.is_ok());
}

#[test]
fn new_for_host_port() {
    let uri = Uri::new_for_host_port("localhost", 27017);
    assert!(!uri.as_ptr().is_null());
}
