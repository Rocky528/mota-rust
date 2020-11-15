extern crate hex;
use core::str::from_utf8;
use matasano::frequency::english::*;
use matasano::*;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let stdin = io::stdin();

    let reader = BufReader::new(stdin);

    let mut best_freq = std::f64::MAX;
    let mut best_line = "";
    let mut decoded: Vec<u8>;

    let english_char_frequency = get_english_char_frequency();

    let mut i = 1;
    for line in reader.lines() {
        println!("{}", i);
        i+=1;
        let line_bytes = hex::decode(line.unwrap()).unwrap();
        let (key, freq) = get_best_xor_key(&line_bytes, &english_char_frequency);
        if freq < best_freq {
            decoded = xor_with(&line_bytes, key);
            best_line = from_utf8(&decoded).unwrap_or("<error>");
            best_freq = freq;
        }
    }

    println!("{}", best_line);
}
