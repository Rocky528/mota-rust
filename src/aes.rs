use openssl::symm::{Cipher, Crypter, Mode};

use crate::*;

pub fn aes_encrypt_ecb(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    while i < plain.len() {
        let block_start = i;
        let block_end = (i + 16).min(plain.len());
        let plain_block = pad_block(&plain[block_start..block_end], 16);

        let cipher_block = aes_encrypt(&plain_block, key);

        result.extend(cipher_block);

        i += 16;
    }

    result
}

pub fn aes_decrypt_ecb(cipher: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    assert!(cipher.len() % 16 == 0);

    while i < cipher.len() {
        let cipher_block = &cipher[i..i + 16];
        let plain_block = aes_decrypt(cipher_block, key);

        result.extend(plain_block);

        i += 16;
    }

    result
}

pub fn aes_encrypt_cbc(plain: &[u8], key: &[u8], iv: Option<&[u8]>) -> Vec<u8> {
    let default_iv = vec![0; 16];
    let iv = iv.unwrap_or(&default_iv);
    let mut result: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    while i < plain.len() {
        let block_start = i;
        let block_end = (i + 16).min(plain.len());
        let mut plain_block = pad_block(&plain[block_start..block_end], 16);

        if i == 0 {
            plain_block = xor(&plain_block, &iv);
        } else {
            plain_block = xor(&plain_block, &result[i - 16..i]);
        }

        let cipher_block = aes_encrypt(&plain_block, key);

        result.extend(cipher_block);

        i += 16;
    }

    result
}

pub fn aes_decrypt_cbc(cipher: &[u8], key: &[u8], iv: Option<&[u8]>) -> Vec<u8> {
    let default_iv = vec![0; 16];
    let iv = iv.unwrap_or(&default_iv);
    let mut result: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    assert!(cipher.len() % 16 == 0);

    while i < cipher.len() {
        let cipher_block = &cipher[i..i + 16];
        let mut plain_block = aes_decrypt(cipher_block, key);

        if i == 0 {
            plain_block = xor(&plain_block, &iv);
        } else {
            plain_block = xor(&plain_block, &cipher[i - 16..i]);
        }

        result.extend(plain_block);

        i += 16;
    }

    result
}

fn aes_decrypt(cipher: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(cipher.len() == 16);
    assert!(key.len() == 16);
    let mut plain = vec![0; cipher.len() + 16];

    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
    decrypter.update(cipher, &mut plain).unwrap();

    plain[0..16].to_vec()
}

fn aes_encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(plain.len() == 16);
    assert!(key.len() == 16);
    let mut cipher = vec![0; plain.len() + 16];

    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None).unwrap();
    encrypter.update(plain, &mut cipher).unwrap();

    cipher[0..16].to_vec()
}
