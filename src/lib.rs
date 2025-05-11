//! # C bindings to `Yescrypt` key derivation function


#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![allow(non_upper_case_globals)]


#[link(name = "yescrypt")]
extern "C" {
    pub fn yescrypt_hash(
        passwd: *const u8,
        buf: *mut u8,
    ) -> ::std::os::raw::c_int;
}


/// Derive fixed size key for given `salt` and `passphrase`
///
/// #Arguments:
/// passwd - password to be derived
/// output - resulting byte slice
///
pub fn yescrypt(passwd: &[u8], output: &mut [u8]) {
    unsafe {
        yescrypt_hash(
            passwd.as_ptr(),
            output.as_mut_ptr(),
        );
    }
}

pub fn get_yescrypt_hash<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    unsafe {
        let mut buffer = [0u8; 32];
        yescrypt_hash(input.as_ref().as_ptr(), buffer.as_mut_ptr());
        buffer
    }
}


#[cfg(test)]
mod tests {
    extern crate hex;
    use hex::{FromHex, ToHex};
    use super::*;
    use tests::hex::encode;
    
    #[test]
    fn test_yescrypt() {
        const PASSWD: &str = "240a5a20727df120f94999fd6e1df9a0dee583541e829597090ccaa5573b33b89f19121dbab36a503dfea48d17a160d100a78187ee80cf8ffd027bed3e82d03aa11e2d59da1cbc5ba79d011d00000a78";
        let passwd:[u8; 80] = <[u8; 80]>::from_hex(PASSWD).expect("Decoding failed");

        let mut buf = [0u8; 32];

        yescrypt(&passwd, &mut buf);

        assert_eq!("efae4a3fddcd185f4faddc8b41cf85f7cad91be3b525affd9ef17bfb00000000", encode(buf.as_ref()));
    }

        #[test]
    fn test_yescrypt_hash() {
        let passwd = "240a5a20727df120f94999fd6e1df9a0dee583541e829597090ccaa5573b33b89f19121dbab36a503dfea48d17a160d100a78187ee80cf8ffd027bed3e82d03aa11e2d59da1cbc5ba79d011d00000a78";
        let passwd_vec = Vec::from_hex(passwd).unwrap();
        let md = get_yescrypt_hash(passwd_vec);
        println!("input: {}", passwd);
        println!("output: {:?}", md.encode_hex::<String>());
        assert_eq!(md.to_vec(), Vec::from_hex("efae4a3fddcd185f4faddc8b41cf85f7cad91be3b525affd9ef17bfb00000000").unwrap())
    }
}
