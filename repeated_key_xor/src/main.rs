use std::env;
use std::process::exit;
use std::str::Bytes;

const KEY: &'static str = "secretkey";

fn repeated_key_xor(cleartext: Bytes, key: Bytes) -> Vec<u8> {
    let mut result = Vec::new();

    for (a, b) in cleartext.zip(key.cycle()) {
        result.push(a ^ b);
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
    let cipher = repeated_key_xor(input.bytes(), KEY.bytes());

    for x in cipher.iter() {
        print!("{:x?}", x);
    }
    print!("\n");
}
