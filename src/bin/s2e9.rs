use matasano::*;

fn main() {
    let s = b"YELLOW SUBMARINE!";
    let block_size = 20;

    let block = pad_block(s, block_size);

    println!("{:?}", block);
}
