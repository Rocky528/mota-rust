extern crate hex;
use matasano::aes::*;
use matasano::{random_key, random_vec};

use rand::Rng;

fn encrypt(plain: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key = random_key();

    let pad_size = rng.gen_range(5,11);


    let mut padded: Vec<u8> = Vec::new();

    let pad = random_vec(pad_size);
    padded.extend(&pad);
    padded.extend(plain);
    padded.extend(&pad);

    if rng.gen_bool(0.5) {
        aes_encrypt_ecb(&padded, &key)
    } else {
        aes_encrypt_cbc(&padded, &key, None)
    }
}

fn main() {
    let result = encrypt(&vec![0u8; 48]);

    let chunks: Vec<Vec<u8>> = result.chunks(16).map(|x| x.to_vec()).collect();
    for chunk in chunks.iter() {
        println!("{}", hex::encode(chunk));
    }

    if hex::encode(&chunks[1]) == hex::encode(&chunks[2]) {
        println!("ecb");
    } else {
        println!("cbc");
    }
}
