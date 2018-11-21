fn main() {
    println!("cargo:rustc-link-search=native={}", "/usr/local/lib");
    println!("cargo:rustc-link-lib=bson-1.0");
    println!("cargo:rustc-link-lib=mongoc-1.0");
}
