use std::env;
use std::process::exit;
use std::str;
use std::str::Bytes;

const KEY: &'static str = "secretkey";

fn one_time_pad(cleartext: Bytes, key: Bytes) -> Vec<u8> {
    let mut result = Vec::new();

    for (a, b) in cleartext.zip(key) {
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
    println!("INPUT:\n{}\n{}\n", input, hex::encode(&input));
    let cipher = one_time_pad(input.bytes(), KEY.bytes());

    println!(
        "RESULT:\n{}\n{}",
        String::from_utf8_lossy(&cipher),
        hex::encode(&cipher)
    );
}
