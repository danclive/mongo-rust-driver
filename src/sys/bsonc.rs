#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_char;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bson_t {
    pub flags: u32,
    pub len: u32,
    pub padding: [u8; 120usize]
}

extern "C" {
    pub fn bson_new() -> *mut bson_t;
    pub fn bson_init(b: *mut bson_t);
    pub fn bson_new_from_data(data: *const u8, length: usize) -> *mut bson_t;
    pub fn bson_copy(bson: *const bson_t) -> *mut bson_t;
    pub fn bson_copy_to(src: *const bson_t, dst: *mut bson_t);
    pub fn bson_get_data(bson: *const bson_t) -> *const u8;
    pub fn bson_destroy(bson: *mut bson_t);
    pub fn bson_strfreev(strv: *mut *mut c_char);
    pub fn bson_value_copy(src: *const bson_value_t, dst: *mut bson_value_t);
    pub fn bson_value_destroy(value: *mut bson_value_t);
    pub fn bson_count_keys(bson: *const bson_t) -> u32;
    pub fn bson_has_field(bson: *const bson_t, key: *const c_char) -> bool;
    pub fn bson_free(mem: *mut c_void);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bson_error_t {
    pub domain: u32,
    pub code: u32,
    pub message: [c_char; 504usize]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bson_oid_t {
    pub bytes: [u8; 12usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bson_value_t {
    pub value_type: u32,
    pub padding: i32,
    pub value: bson_v,
}

#[test]
fn name() {
    use std::mem;
    let a = mem::size_of::<bson_v>();
    assert_eq!(a, 24);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bson_v {
    pub v_oid: bson_oid_t,
    pub v_int64: i64,
    pub v_int32: i32,
    pub v_int8: i8,
    pub v_double: f64,
    pub v_bool: bool,
    pub v_datetime: i64,
    pub v_timestamp: v_timestamp,
    pub v_utf8: v_utf8,
    pub v_doc: v_doc,
    pub v_binary: v_binary,
    pub v_regex: v_regex,
    pub v_dbpointer: v_dbpointer,
    pub v_code: v_code,
    pub v_codewscope: v_codewscope,
    pub v_symbol: v_symbol,
    pub v_decimal128: v_decimal128,
    _bindgen_union_align: [u64; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_timestamp {
    pub timestamp: u32,
    pub increment: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_utf8 {
    pub str: *mut ::std::os::raw::c_char,
    pub len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_doc {
    pub data: *mut u8,
    pub data_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_binary {
    pub data: *mut u8,
    pub data_len: u32,
    pub subtype: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_regex {
    pub regex: *mut ::std::os::raw::c_char,
    pub options: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_dbpointer {
    pub collection: *mut ::std::os::raw::c_char,
    pub collection_len: u32,
    pub oid: bson_oid_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_code {
    pub code: *mut ::std::os::raw::c_char,
    pub code_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_codewscope {
    pub code: *mut ::std::os::raw::c_char,
    pub scope_data: *mut u8,
    pub code_len: u32,
    pub scope_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_symbol {
    pub symbol: *mut ::std::os::raw::c_char,
    pub len: u32,
}

pub const BSON_TYPE_EOD: u32 = 0;
pub const BSON_TYPE_DOUBLE: u32 = 1;
pub const BSON_TYPE_UTF8: u32 = 2;
pub const BSON_TYPE_DOCUMENT: u32 = 3;
pub const BSON_TYPE_ARRAY: u32 = 4;
pub const BSON_TYPE_BINARY: u32 = 5;
pub const BSON_TYPE_UNDEFINED: u32 = 6;
pub const BSON_TYPE_OID: u32 = 7;
pub const BSON_TYPE_BOOL: u32 = 8;
pub const BSON_TYPE_DATE_TIME: u32 = 9;
pub const BSON_TYPE_NULL: u32 = 10;
pub const BSON_TYPE_REGEX: u32 = 11;
pub const BSON_TYPE_DBPOINTER: u32 = 12;
pub const BSON_TYPE_CODE: u32 = 13;
pub const BSON_TYPE_SYMBOL: u32 = 14;
pub const BSON_TYPE_CODEWSCOPE: u32 = 15;
pub const BSON_TYPE_INT32: u32 = 16;
pub const BSON_TYPE_TIMESTAMP: u32 = 17;
pub const BSON_TYPE_INT64: u32 = 18;
pub const BSON_TYPE_DECIMAL128: u32 = 19;
pub const BSON_TYPE_MAXKEY: u32 = 127;
pub const BSON_TYPE_MINKEY: u32 = 255;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct v_decimal128 {
    pub low: u64,
    pub high: u64,
}

pub const BSON_SUBTYPE_BINARY: u32 = 0;
pub const BSON_SUBTYPE_FUNCTION: u32 = 1;
pub const BSON_SUBTYPE_BINARY_DEPRECATED: u32 = 2;
pub const BSON_SUBTYPE_UUID_DEPRECATED: u32 = 3;
pub const BSON_SUBTYPE_UUID: u32 = 4;
pub const BSON_SUBTYPE_MD5: u32 = 5;
pub const BSON_SUBTYPE_USER: u32 = 128;
