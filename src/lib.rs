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

pub mod frequency;
