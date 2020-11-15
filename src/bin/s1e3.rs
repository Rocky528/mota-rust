extern crate hex;
use matasano::*;
use matasano::frequency::*;
use std::str::from_utf8;

fn main() {
    let text = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();


    let english_char_frequency = english::get_english_char_frequency();

    let mut best_key: u8 = 0;
    let mut best_freq_comparison: f64 = std::f64::MAX;
    for key in 0..=255 {
        let decoded = xor_with(&text, key);
        let char_frequency = get_char_frequency(&decoded);
        let freq_comparison = compare_frequencies(&char_frequency, &english_char_frequency);

        if freq_comparison < best_freq_comparison {
            best_key = key;
            best_freq_comparison = freq_comparison;
        }
    }
    let decoded = xor_with(&text, best_key);

    println!("{}", from_utf8(&decoded).unwrap());
}
