extern crate url;

use url::percent_encoding::percent_decode;

fn main() {
    let input = "foo%20bar";
    let decoded = percent_decode(input.as_bytes()).decode_utf8();
    println!("{}", decoded.unwrap());
}
