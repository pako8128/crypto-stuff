use std::env;
use std::process::exit;
use std::str::Bytes;

const KEY: &'static str = "secretkey";

fn hexify(input: &str) -> String {
    let mut result = String::new();
    for character in input.bytes() {
        result += &format!("{:02x}", character);
    }
    result
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
    println!("INPUT:  {}", hexify(&input));
    let hex = hexify(&input);
    let cipher = one_time_pad(hex.bytes(), KEY.bytes());

    print!("RESULT: ");
    for x in cipher.iter() {
        print!("{:x?}", x);
    }
    print!("\n");
}
