extern crate cc;
use std::env;
use std::path::PathBuf;

fn main(){
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut cc = cc::Build::new();

    cc.flag("-march=native")
    .include("ext/yescrypt")
    .file("ext/yescrypt/yescrypt.c")
    .compile("libyescrypt.a");

    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=static=yescrypt");
}
