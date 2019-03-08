use std::ffi::CStr;

use mongors;

fn main() {
    println!("{:?}", "hello");
    unsafe {
        mongors::mongoc_init();
        let a = mongors::bson_get_monotonic_time();
        println!("{:?}", a);

        let b = mongors::mongoc_get_version();

        let c = CStr::from_ptr(b);

        println!("{:?}", c);
    }
    println!("{:?}", "world");
}
