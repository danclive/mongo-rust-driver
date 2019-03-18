use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    if let Ok(target) = env::var("TARGET") {
        match target.as_str() {
            "x86_64-unknown-linux-musl" => {
                static_build();
                return;
            }
            _ => ()
        }
    }

    // println!("cargo:rustc-link-search=native={}", "/usr/local/lib");
    println!("cargo:rustc-link-lib=bson-1.0");
    println!("cargo:rustc-link-lib=mongoc-1.0");
}

fn static_build() {
    let mongoc_version = "1.14.0";

    let out_dir_var = env::var("OUT_DIR").expect("No out dir");
    let out_dir = format!("{}/{}", out_dir_var, mongoc_version);
    let driver_src_path = format!("mongo-c-driver-{}", mongoc_version);

    let libmongoc_path = Path::new(&out_dir).join("lib/libmongoc-static-1.0.a");

    if !libmongoc_path.exists() {
        // Download and extract driver archive
        let url = format!(
            "https://github.com/mongodb/mongo-c-driver/releases/download/{}/mongo-c-driver-{}.tar.gz",
            mongoc_version,
            mongoc_version
        );
        Command::new("curl").arg("-O") // Save to disk
            .arg("-L") // Follow redirects
            .arg(url)
            .status()
            .expect("Could not run curl");

        let archive_name = format!("mongo-c-driver-{}.tar.gz", mongoc_version);
        Command::new("tar")
            .arg("xzf")
            .arg(&archive_name)
            .status()
            .expect("Could not unarchive tar");

        // Configure
        let mut command = Command::new("cmake");
        command.arg("");
        command.arg("-DENABLE_AUTOMATIC_INIT_AND_CLEANUP=OFF");
        command.arg("-DENABLE_STATIC=ON");
        command.arg("-DENABLE_BSON=ON");
        command.arg("-DENABLE_SSL=OFF");
        command.arg("-DENABLE_SASL=OFF");
        command.arg("-DENABLE_SNAPPY=OFF");
        command.arg("-DENABLE_ZLIB=OFF");
        command.arg("-DENABLE_SRV=OFF");

        command.arg("-DENABLE_STATIC=ON");
        command.arg("-DENABLE_BSON=ON");

        // musl
        command.arg("-DCMAKE_C_COMPILER:PATH=/usr/bin/musl-gcc");
        command.arg("-DCMAKE_INSTALL_OLDINCLUDEDIR:PATH=/usr/include/x86_64-linux-musl");
        command.arg("-DCMAKE_C_FLAGS=-static -Os");
        command.arg(format!("-DCMAKE_ARCHIVE_OUTPUT_DIRECTORY:PATH={}/lib", &out_dir));

        command.arg(format!("-DCMAKE_INSTALL_PREFIX:PATH={}", &out_dir));
        command.current_dir(&driver_src_path);

        // Enable debug symbols if configured for this profile
        // if env::var("DEBUG") == Ok("true".to_string()) {
            // command.arg("--enable-debug-symbols=yes");
        // }

        // Use target that Cargo sets
        // if let Ok(target) = env::var("TARGET") {
            // command.arg(format!(" --build={}", target));
        // }

        // println!("cargo:verbose={:?}", command);

        assert!(command.status().expect("cmake failed").success());

        assert!(Command::new("make")
            .arg("bson_static")
            .current_dir(&driver_src_path)
            .status()
            .expect("Make failed").success());

        assert!(Command::new("make")
            .arg("mongoc_static")
            .current_dir(&driver_src_path)
            .status()
            .expect("Make failed").success());

        // assert!(Command::new("make")
        //     .arg("install")
        //     .current_dir(&driver_src_path)
        //     .status()
        //     .expect("make install failed").success());
    }

    // Output to Cargo
    println!("cargo:rustc-link-search=native={}/lib", &out_dir);
    println!("cargo:rustc-link-lib=static=bson-static-1.0");
    println!("cargo:rustc-link-lib=static=mongoc-static-1.0");
}
