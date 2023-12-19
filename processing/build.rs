extern crate cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("{}", crate_dir);

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_language(cbindgen::Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("../ffi-test/bindings.h");

    let profile = env::var("PROFILE").unwrap();
    
    if profile == "release" {
      std::fs::copy("../target/release/libprocessing.a", "../ffi-test/libprocessing.a").expect("Could not copy static lib");
    }
}