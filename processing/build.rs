extern crate cbindgen;
use std::env;

use cbindgen::ParseConfig;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    println!("{}", crate_dir);

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_language(cbindgen::Language::C)
      .with_parse_include(&["libc"])
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("../ffi-test/bindings.h");
}