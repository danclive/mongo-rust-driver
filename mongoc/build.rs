// fn main() {
//     println!("cargo:rustc-link-search=native={}", "/usr/local/lib");
//     println!("cargo:rustc-link-lib=bson-1.0");
//     println!("cargo:rustc-link-lib=mongoc-1.0");
// }
fn main() {
    // println!("cargo:rustc-link-search=native={}", "/home/danc/code/rust/mongo-rust-driver/mongoc");
    println!("cargo:rustc-link-lib=bson-1.0");
    println!("cargo:rustc-link-lib=mongoc-1.0");
}
