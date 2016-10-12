use std::env;
use std::path::Path;

#[cfg(feature = "serde_codegen")]
fn main() {
    extern crate serde_codegen;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("tests/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");

    serde_codegen::expand(&src, &dst).unwrap();
}

#[cfg(not(feature = "serde_codegen"))]
fn main() {}
