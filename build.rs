extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if let Ok(lib_path) = env::var("BLAZE_PATH") {
        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("Using BLAZE_PATH = {}", lib_path);
    }
    println!("cargo:rustc-link-lib=dylib=blaze");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .trust_clang_mangling(false)
        .rustfmt_bindings(true)
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
