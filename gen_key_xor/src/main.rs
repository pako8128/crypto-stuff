use std::env;
use std::process::exit;
use std::str::Bytes;

const SEED: u64 = 0xef4c9245aa551fe4;
const NUMBER: u64 = 194963;

struct Key {
    seed: u64,
}

impl Key {
    fn new(seed: u64) -> Key {
        Key { seed }
    }
}

impl Iterator for Key {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.seed = (self.seed.wrapping_mul(NUMBER)) % u64::max_value();
        Some(self.seed as u8)
    }
}

fn gen_key_xor(cleartext: Bytes, key: Key) -> Vec<u8> {
    let mut result = Vec::new();

    for (a, k) in cleartext.zip(key) {
        result.push(a ^ k);
    }

    result
}

fn main() {
    let input = match env::args().skip(1).next() {
        Some(arg) => arg,
        None => {
            println!("expect exactly one argument");
            exit(1);
        }
    };
    let cipher = gen_key_xor(input.bytes(), Key::new(SEED));
    for c in cipher.iter() {
        print!("{:x?}", c);
    }
    print!("\n");
}
