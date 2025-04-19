use anyhow::{self, Context};
use std::env;
use std::fs;
mod element;
mod rf_parser;

fn main() -> anyhow::Result<()> {
    let filename = env::args()
        .nth(1)
        .context("Wrong amount of command line parameters")?;
    println!("Analyzing {}", filename);
    let xml = fs::read_to_string(filename).context("Reading failed")?;
    rf_parser::parse(&xml)?;
    Ok(())
}
