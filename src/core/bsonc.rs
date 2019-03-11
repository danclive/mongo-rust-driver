use bsonrs::Document;
use bsonrs::encode::EncodeResult;
use bsonrs::decode::DecodeResult;

use crate::sys::bsonc;

#[derive(Debug)]
pub struct Bsonc(*mut bsonc::bson_t);

impl Bsonc {
    pub fn new() -> Bsonc {
        Bsonc::from_ptr(unsafe { bsonc::bson_new() })
    }

    pub fn from_ptr(ptr: *mut bsonc::bson_t) -> Bsonc {
        assert!(!ptr.is_null());
        Bsonc(ptr)
    }

    pub fn from_doc(doc: &Document) -> EncodeResult<Bsonc> {
        let vec = doc.to_vec()?;

        let inner = unsafe {
            bsonc::bson_new_from_data(vec.as_ptr(), vec.len())
        };

        assert!(!inner.is_null());
        Ok(Bsonc(inner))
    }

    pub fn as_doc(&self) -> DecodeResult<Document> {
        assert!(!self.0.is_null());

        let ptr = unsafe { bsonc::bson_get_data(self.0) };
        assert!(!ptr.is_null());

        let len = unsafe {
            let bson = *self.0;
            bson.len
        };

        let slice = unsafe {
            std::slice::from_raw_parts(ptr, len as usize)
        };

        Ok(Document::from_slice(slice)?)
    }

    pub fn as_ptr(&self) -> *const bsonc::bson_t {
        assert!(!self.0.is_null());
        self.0
    }

    pub fn as_mut_ptr(&self) -> *mut bsonc::bson_t {
        assert!(!self.0.is_null());
        self.0
    }
}

impl Drop for Bsonc {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { bsonc::bson_destroy(self.0) }
    }
}

impl Clone for Bsonc {
    fn clone(&self) -> Bsonc {
        assert!(!self.0.is_null());
        unsafe { Bsonc(bsonc::bson_copy(self.0)) }
    }
}
