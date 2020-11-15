extern crate hex;
use matasano::xor;

fn main() {
    let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();

    let b = hex::decode("686974207468652062756c6c277320657965").unwrap();

    let result = hex::encode(xor(&a, &b));

    println!("{}", result);
}
