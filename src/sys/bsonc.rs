#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_char;

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
    pub value_type: bson_type_t,
    pub padding: i32,
    pub value: _bson_value_t__bindgen_ty_1,
}

#[test]
fn name() {
    use std::mem;
    let a = mem::size_of::<_bson_value_t__bindgen_ty_1>();
    assert_eq!(a, 24);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _bson_value_t__bindgen_ty_1 {
    pub v_oid: bson_oid_t,
    pub v_int64: i64,
    pub v_int32: i32,
    pub v_int8: i8,
    pub v_double: f64,
    pub v_bool: bool,
    pub v_datetime: i64,
    pub v_timestamp: _bson_value_t__bindgen_ty_1__bindgen_ty_1,
    pub v_utf8: _bson_value_t__bindgen_ty_1__bindgen_ty_2,
    pub v_doc: _bson_value_t__bindgen_ty_1__bindgen_ty_3,
    pub v_binary: _bson_value_t__bindgen_ty_1__bindgen_ty_4,
    pub v_regex: _bson_value_t__bindgen_ty_1__bindgen_ty_5,
    pub v_dbpointer: _bson_value_t__bindgen_ty_1__bindgen_ty_6,
    pub v_code: _bson_value_t__bindgen_ty_1__bindgen_ty_7,
    pub v_codewscope: _bson_value_t__bindgen_ty_1__bindgen_ty_8,
    pub v_symbol: _bson_value_t__bindgen_ty_1__bindgen_ty_9,
    pub v_decimal128: bson_decimal128_t,
    _bindgen_union_align: [u64; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_1 {
    pub timestamp: u32,
    pub increment: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_2 {
    pub str: *mut ::std::os::raw::c_char,
    pub len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_3 {
    pub data: *mut u8,
    pub data_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_4 {
    pub data: *mut u8,
    pub data_len: u32,
    pub subtype: bson_subtype_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_5 {
    pub regex: *mut ::std::os::raw::c_char,
    pub options: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_6 {
    pub collection: *mut ::std::os::raw::c_char,
    pub collection_len: u32,
    pub oid: bson_oid_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_7 {
    pub code: *mut ::std::os::raw::c_char,
    pub code_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_8 {
    pub code: *mut ::std::os::raw::c_char,
    pub scope_data: *mut u8,
    pub code_len: u32,
    pub scope_len: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bson_value_t__bindgen_ty_1__bindgen_ty_9 {
    pub symbol: *mut ::std::os::raw::c_char,
    pub len: u32,
}

pub const bson_type_t_BSON_TYPE_EOD: bson_type_t = 0;
pub const bson_type_t_BSON_TYPE_DOUBLE: bson_type_t = 1;
pub const bson_type_t_BSON_TYPE_UTF8: bson_type_t = 2;
pub const bson_type_t_BSON_TYPE_DOCUMENT: bson_type_t = 3;
pub const bson_type_t_BSON_TYPE_ARRAY: bson_type_t = 4;
pub const bson_type_t_BSON_TYPE_BINARY: bson_type_t = 5;
pub const bson_type_t_BSON_TYPE_UNDEFINED: bson_type_t = 6;
pub const bson_type_t_BSON_TYPE_OID: bson_type_t = 7;
pub const bson_type_t_BSON_TYPE_BOOL: bson_type_t = 8;
pub const bson_type_t_BSON_TYPE_DATE_TIME: bson_type_t = 9;
pub const bson_type_t_BSON_TYPE_NULL: bson_type_t = 10;
pub const bson_type_t_BSON_TYPE_REGEX: bson_type_t = 11;
pub const bson_type_t_BSON_TYPE_DBPOINTER: bson_type_t = 12;
pub const bson_type_t_BSON_TYPE_CODE: bson_type_t = 13;
pub const bson_type_t_BSON_TYPE_SYMBOL: bson_type_t = 14;
pub const bson_type_t_BSON_TYPE_CODEWSCOPE: bson_type_t = 15;
pub const bson_type_t_BSON_TYPE_INT32: bson_type_t = 16;
pub const bson_type_t_BSON_TYPE_TIMESTAMP: bson_type_t = 17;
pub const bson_type_t_BSON_TYPE_INT64: bson_type_t = 18;
pub const bson_type_t_BSON_TYPE_DECIMAL128: bson_type_t = 19;
pub const bson_type_t_BSON_TYPE_MAXKEY: bson_type_t = 127;
pub const bson_type_t_BSON_TYPE_MINKEY: bson_type_t = 255;
pub type bson_type_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bson_decimal128_t {
    pub low: u64,
    pub high: u64,
}

pub const bson_subtype_t_BSON_SUBTYPE_BINARY: bson_subtype_t = 0;
pub const bson_subtype_t_BSON_SUBTYPE_FUNCTION: bson_subtype_t = 1;
pub const bson_subtype_t_BSON_SUBTYPE_BINARY_DEPRECATED: bson_subtype_t = 2;
pub const bson_subtype_t_BSON_SUBTYPE_UUID_DEPRECATED: bson_subtype_t = 3;
pub const bson_subtype_t_BSON_SUBTYPE_UUID: bson_subtype_t = 4;
pub const bson_subtype_t_BSON_SUBTYPE_MD5: bson_subtype_t = 5;
pub const bson_subtype_t_BSON_SUBTYPE_USER: bson_subtype_t = 128;
pub type bson_subtype_t = u32;
