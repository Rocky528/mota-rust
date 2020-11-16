extern crate base64;
extern crate hex;

pub fn hex_to_base64(s: &str) -> String {
    hex::decode(s).map(base64::encode).unwrap()
}

pub fn xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn xor_with(v1: &[u8], key: u8) -> Vec<u8> {
    v1.iter().map(|x| x ^ key).collect()
}

pub fn repeated_xor(v1: &[u8], key: &[u8]) -> Vec<u8> {
    v1.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

pub fn hamming_distance(v1: &[u8], v2: &[u8]) -> u32 {
    assert_eq!(v1.len(), v2.len());
    v1.iter()
        .zip(v2)
        .map(|(a, b)| a ^ b)
        .map(count_bits)
        .fold(0, |a, b| a + b)
}

fn count_bits(mut x: u8) -> u32 {
    let mut count = 0;
    while x > 0 {
        if (x & 1) == 1 {
            count += 1;
        }
        x = x >> 1;
    }

    count
}

pub mod frequency;
