extern crate base64;
extern crate hex;
extern crate openssl;

use hex::FromHex;
use openssl::symm::{decrypt, Cipher};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // read text
    let f = File::open("data/s1e7.txt").unwrap();
    let mut f = BufReader::new(f);
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    let text = text.replace("\n", "");
    let text = base64::decode(text).unwrap();

    let iv =
        Vec::from_hex("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F").unwrap();

    // decode
    let key = b"YELLOW SUBMARINE".as_ref();

    let cipher = Cipher::aes_128_ecb();

    let decoded = decrypt(cipher, key, Some(&iv), &text).unwrap();

    let decoded = std::str::from_utf8(&decoded).unwrap();

    println!("{}", decoded);
}
