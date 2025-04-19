// combined from https://github.com/tafia/quick-xml

use std::env;
use std::fs;
mod blend_result;
mod element;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).unwrap();
    println!("Analyzing {}", filename);

    let xml = fs::read_to_string(filename).unwrap();
    blend_result::parse(&xml);

    Ok(())
}
