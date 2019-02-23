extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if let Ok(ld_library_path) = env::var("LD_LIBRARY_PATH") {
        println!("cargo:rustc-link-search=native={}", ld_library_path);
        // println!("cargo:rustc-flags=-L {}", ld_library_path);
    }

    // println!("cargo:rustc-link-lib=dylib=blaze");
    println!("cargo:rustc-flags=-l blaze");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .trust_clang_mangling(false)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
