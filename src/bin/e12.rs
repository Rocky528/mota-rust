#[macro_use]
extern crate lazy_static;
extern crate base64;
extern crate hex;

use matasano::aes::*;
use matasano::random_key;
use std::{collections::BTreeMap, str::from_utf8};

lazy_static! {
    static ref KEY: Vec<u8> = random_key();
}

fn encrypt(plain: &[u8]) -> Vec<u8> {
    let cipher = base64::decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    let mut plain_extended = plain.to_vec();
    plain_extended.extend(cipher);

    aes_encrypt_ecb(&plain_extended, &*KEY)
}

fn main() {
    let suffix_length = encrypt("".as_bytes()).len();

    let mut block_size: Option<usize> = None;
    let mut previous_len = encrypt(b"").len();
    for i in 2..=50 {
        let b = "A".repeat(i).into_bytes();
        let len = encrypt(&b).len();
        if len != previous_len {
            block_size = Some(len - previous_len);
        }
        previous_len = len;
    }

    let block_size = block_size.unwrap();

    // println!("Block size is {}", block_size);

    let encrypted = encrypt(&"A".repeat(2 * block_size).into_bytes());

    let is_ecb = hex::encode(&encrypted[0..block_size])
        == hex::encode(&encrypted[block_size..2 * block_size]);

    assert!(is_ecb);

    // guess first character
    let mut map: BTreeMap<String, u8> = BTreeMap::new();
    let mut chars: Vec<u8> = Vec::new();

    for _ in 0..suffix_length {
        let byte = guess_char(&mut map, block_size, &chars);
        chars.push(byte);
    }

    let chars: Vec<_> = chars.into_iter().filter(|&x| x != 0).collect();

    println!("{}", from_utf8(&chars).unwrap());
}

fn guess_char(map: &mut BTreeMap<String, u8>, block_size: usize, chars: &[u8]) -> u8 {
    let prefix_length = block_size - 1 - (chars.len() % block_size);
    let prefix = "A".repeat(prefix_length).into_bytes();

    let block_number = chars.len() / block_size;
    let block_start = (block_number) * block_size;
    let block_end = (block_number + 1) * block_size;

    for i in 0..=255u8 {
        let mut byte = prefix.clone();
        byte.extend(chars);
        byte.push(i);
        let encrypted = encrypt(&byte);
        let first_block = hex::encode(&encrypted[block_start..block_end]);
        map.insert(first_block, i);
    }

    let encrypted = encrypt(&prefix);
    let first_block = hex::encode(&encrypted[block_start..block_end]);

    let result = map.get(&first_block);

    *result.unwrap_or(&0)
}
