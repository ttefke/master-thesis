use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Include assembly
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();
    fs::copy(
        format!("bin/assembly.a"),
        out_dir.join(format!("lib{name}.a")),
    )
    .unwrap();

    println!("cargo:rustc-link-lib=static={name}");
    println!("cargo:rustc-link-search={}", out_dir.display());
}
