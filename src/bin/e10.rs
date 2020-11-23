extern crate base64;
extern crate openssl;

use matasano::aes::aes_decrypt_cbc;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("data/s2e10.txt").unwrap();
    let mut f = BufReader::new(f);
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    let text = text.replace("\n", "");
    let text = base64::decode(text).unwrap();

    let key = b"YELLOW SUBMARINE";

    let d = aes_decrypt_cbc(&text, &key[..], None);

    println!("{}", std::str::from_utf8(&d).unwrap_or("<error>"));
}

