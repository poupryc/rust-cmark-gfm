extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    let dst = Config::new("cmark-gfm")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=cmark-gfm");
    println!("cargo:rustc-link-lib=static=cmark-gfm-extensions");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-I", "cmark-gfm/src", "-I"])
        .clang_arg(format!("{}", dst.join("include").display()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
