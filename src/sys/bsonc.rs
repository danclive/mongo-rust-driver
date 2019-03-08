#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bson_t {
    pub flags: u32,
    pub len: u32,
    pub padding: [u8; 120usize],
}

pub type bson_t = _bson_t;

extern "C" {
    pub fn bson_new() -> *mut bson_t;
    // pub fn bson_init(b: *mut bson_t);
    pub fn bson_new_from_data(data: *const u8, length: usize) -> *mut bson_t;
    // pub fn bson_copy(bson: *const bson_t) -> *mut bson_t;
    // pub fn bson_copy_to(src: *const bson_t, dst: *mut bson_t);
    pub fn bson_get_data(bson: *const bson_t) -> *const u8;
    pub fn bson_destroy(bson: *mut bson_t);
}

