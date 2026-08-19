use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Include interrupt linker script
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::File::create(out_dir.join("hal_defaults.x"))
        .unwrap()
        .write_all(include_bytes!("hal_defaults.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hal_defaults.x");
}
