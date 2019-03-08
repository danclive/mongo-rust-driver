use std::os::raw::c_char;

extern "C" {
    pub fn mongoc_init();
    pub fn bson_get_monotonic_time() -> i64;
    pub fn mongoc_get_version() -> *const c_char;
}
