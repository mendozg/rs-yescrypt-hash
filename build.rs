extern crate cc;

fn main(){

    let mut cc = cc::Build::new();
    cc.file("src/yescrypt_hash.c")
      .include("src")
      .flag("-march=native")
      .compile("src");
}
