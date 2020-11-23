extern crate base64;

use core::str::from_utf8;
use crate::frequency::english::*;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use matasano::*;

fn main() {
    // read text
    let f = File::open("data/s1e6.txt").unwrap();
    let mut f = BufReader::new(f);
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    let text = text.replace("\n", "");
    let text = base64::decode(text).unwrap();

    // guess best 3 keysizes
    let mut keysizes: Vec<(usize, f64)> = Vec::new();
    for keysize in 2..=40 {
        let hd1 = hamming_distance(&text[0..keysize], &text[keysize..2*keysize]) as f64 / (keysize as f64);
        let hd2 = hamming_distance(&text[keysize..2*keysize], &text[2*keysize..3*keysize]) as f64 / (keysize as f64);
        let hd3 = hamming_distance(&text[2*keysize..3*keysize], &text[3*keysize..4*keysize]) as f64 / (keysize as f64);
        let hd4 = hamming_distance(&text[3*keysize..4*keysize], &text[4*keysize..5*keysize]) as f64 / (keysize as f64);

        let hd = (hd1 + hd2 + hd3 + hd4) / 4.0;

        keysizes.push((keysize, hd));
    }

    keysizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let keysizes = &keysizes[0..3];
    let keysizes = keysizes.iter().map(|x| x.0).collect::<Vec<usize>>();

    let english_char_frequency = get_english_char_frequency();

    let mut best_key: Option<Vec<u8>> = None;
    let mut min_english_proximity = std::f64::MAX;
    for keysize in keysizes {
        let blocks = transpose(&text, keysize);

        let keys: Vec<(u8, f64)> = blocks.iter().map(|block| get_best_xor_key(&block, &english_char_frequency)).collect();

        let key: Vec<u8> = keys.iter().map(|x| x.0).collect();
        let english_proximity = keys.iter().fold(0.0, |a, b| a + b.1) / keysize as f64;

        if english_proximity < min_english_proximity {
            min_english_proximity = english_proximity;
            best_key = Some(key);
        }
    }

    let decoded = repeated_xor(&text, &best_key.unwrap());

    println!("{}", from_utf8(&decoded).unwrap());
}

fn transpose(v: &[u8], block_size: usize) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();

    for _ in 0..block_size {
        result.push(Vec::new());
    }

    for i in 0..v.len() {
        result.get_mut(i % block_size).unwrap().push(v[i]);
    }

    result
}
