#![feature(test)]
extern crate hex;
extern crate rust_yescrypt;
extern crate test;

use hex::FromHex;

use rust_yescrypt::{yescrypt};
use test::Bencher;



#[bench]
fn bench_encrypt_scrypt(b: &mut Bencher) {

    const PASSWD: &str = "240a5a20727df120f94999fd6e1df9a0dee583541e829597090ccaa5573b33b89f19121dbab36a503dfea48d17a160d100a78187ee80cf8ffd027bed3e82d03aa11e2d59da1cbc5ba79d011d00000a78";
    let passwd:[u8; 80] = <[u8; 80]>::from_hex(PASSWD).expect("Decoding failed");

    let mut buf = [0u8; 32];

    b.iter(|| yescrypt(&passwd, &mut buf));
}