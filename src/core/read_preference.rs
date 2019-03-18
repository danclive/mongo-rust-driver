use crate::sys::mongoc::*;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ReadMode {
    Primary = MONGOC_READ_PRIMARY,
    Secondary = MONGOC_READ_SECONDARY,
    PrimaryPreferred = MONGOC_READ_PRIMARY_PREFERRED,
    SecondaryPreferred = MONGOC_READ_SECONDARY_PREFERRED,
    Nearest = MONGOC_READ_NEAREST
}

pub struct ReadPreference(*mut mongoc_read_prefs_t);

impl ReadPreference {
    pub fn new(read_mode: ReadMode) -> ReadPreference {
        let ptr = unsafe { mongoc_read_prefs_new(read_mode as u32) };
        ReadPreference::from_ptr(ptr)
    }

    pub fn from_ptr(ptr: *mut mongoc_read_prefs_t) -> ReadPreference {
        assert!(!ptr.is_null());
        ReadPreference(ptr)
    }

    pub fn as_ptr(&self) -> *const mongoc_read_prefs_t {
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut mongoc_read_prefs_t {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        unsafe { mongoc_read_prefs_is_valid(self.0) }
    }
}

impl Drop for ReadPreference {
    fn drop(&mut self) {
        unsafe { mongoc_read_prefs_destroy(self.0) }
    }
}

impl Clone for ReadPreference {
    fn clone(&self) -> ReadPreference {
        unsafe { ReadPreference::from_ptr(mongoc_read_prefs_copy(self.0)) }
    }
}

impl Default for ReadPreference {
    fn default() -> ReadPreference {
        ReadPreference::new(ReadMode::Primary)
    }
}
