use bsonrs::Document;
use bsonrs::encode::EncodeResult;
use bsonrs::decode::DecodeResult;

pub mod bsonc;
pub mod mongoc;

pub struct Bsonc {
    inner: *mut bsonc::bson_t
}

impl Bsonc {
    pub fn new() -> Bsonc {
        Bsonc::from_ptr(unsafe { bsonc::bson_new() })
    }

    pub fn from_ptr(ptr: *const bsonc::bson_t) -> Bsonc {
        assert!(!ptr.is_null());
        Bsonc { inner: ptr as *mut bsonc::bson_t }
    }

    pub fn from_doc(doc: &Document) -> EncodeResult<Bsonc> {
        let vec = doc.to_vec()?;

        let inner = unsafe {
            bsonc::bson_new_from_data(vec.as_ptr(), vec.len())
        };

        assert!(!inner.is_null());

        Ok(Bsonc { inner })
    }

    pub fn as_doc(&self) -> DecodeResult<Document> {
        assert!(!self.inner.is_null());

        let ptr = unsafe { bsonc::bson_get_data(self.inner) };
        assert!(!ptr.is_null());

        let len = unsafe {
            let bson = *self.inner;
            bson.len
        };

        let slice = unsafe {
            std::slice::from_raw_parts(ptr, len as usize)
        };

        Ok(Document::from_slice(slice)?)
    }

    pub fn inner(&self) -> *const bsonc::bson_t {
        assert!(!self.inner.is_null());
        self.inner
    }

    pub fn mut_inner(&mut self) -> *mut bsonc::bson_t {
        assert!(!self.inner.is_null());
        self.inner
    }
}

impl Drop for Bsonc {
    fn drop(&mut self) {
        unsafe { bsonc::bson_destroy(self.inner) }
    }
}
