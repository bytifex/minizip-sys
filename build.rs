use cmake::{self, Config};
use std::path::PathBuf;

fn main() {
    // Build minizip with cmake
    let dst = Config::new("minizip")
        .define("MZ_COMPAT", "ON")
        .define("MZ_BZIP2", "OFF")
        .define("MZ_ZLIB", "ON")
        .define("MZ_OPENSSL", "OFF")
        .define("MZ_ZSTD", "OFF")
        .define("MZ_LZMA", "OFF")
        .target("minizip zlibstatic")
        .build();

    // Tell cargo to tell rustc to link the minizip library.
    println!(
        "cargo:rustc-link-search={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static:+bundle=minizip");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=static=z");
        println!("cargo:rustc-link-lib=iconv");
    }
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=zlibstatic");
    }

    // Generate Rust FFI bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(true)
        .use_core()
        .ctypes_prefix("libc")
        .allowlist_function("mz_.*") // it adds recursively all used types so the next line in this case changes nothing for this particular case
        .allowlist_type("mz_.*")
        .allowlist_function("zip.*")
        .allowlist_function("unz.*")
        .prepend_enum_name(false)
        .constified_enum_module("mz_op")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to src folder to make rls autocomplete work.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
