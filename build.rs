// https://rust-lang.github.io/rust-bindgen/tutorial-3.html

use bindgen::Builder as BindgenBuilder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=yaml");

    let bindings = BindgenBuilder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
