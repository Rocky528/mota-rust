extern crate base64;
extern crate hex;
extern crate openssl;

use std::collections::BTreeMap;
use matasano::hamming_distance;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // read text
    let f = File::open("data/s1e8.txt").unwrap();
    let mut f = BufReader::new(f);
    let mut text = String::new();
    let lines: Vec<Vec<u8>> = f.lines().map(|x| x.unwrap())
        .map(hex::decode)
        .map(|x| x.unwrap())
        .collect();

    let lines_with_repeated_blocks: Vec<(usize, String)> = lines
        .iter()
        .enumerate()
        .filter(|(i, line)| has_repeated_blocks(line, 16))
        .map(|(i, line)| (i, hex::encode(line)))
        .collect();

    println!("{:?}", lines_with_repeated_blocks);
}

fn has_repeated_blocks(v: &[u8], size: usize) -> bool {
    let mut map: BTreeMap<String, bool> = BTreeMap::new();

    let mut found_one = false;

    let mut i = 0usize;

    while i + size < v.len() {
        let s = hex::encode(&v[i..i+size]);

        if map.contains_key(&s) {
            found_one = true;
        }

        map.insert(s, true);
        i+=size;
    }

    found_one
}
