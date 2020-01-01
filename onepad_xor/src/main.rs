use std::env;
use std::process::exit;
use std::str::Bytes;

const KEY: &'static str = "secretkey";

fn hexify(input: &str) -> Result<Bytes, String> {
    unimplemented!();
}

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
    let hex = hexify(&input).unwrap();
    let cipher = one_time_pad(hex, KEY.bytes());

    for x in cipher.iter() {
        print!("{:x?}", x);
    }
    print!("\n");
}
