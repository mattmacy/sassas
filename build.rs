extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()

        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .no_unstable_rust()
        .clang_arg("-Ibindings")
        //.clang_arg(inc)
        // The input header we would like to generate
        // bindings for.
        .header("bindings/wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
