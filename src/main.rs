use std::env;
use std::fs;
mod element;
mod rf_parser;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).unwrap();
    println!("Analyzing {}", filename);

    let xml = fs::read_to_string(filename).unwrap();
    rf_parser::parse(&xml);

    Ok(())
}
