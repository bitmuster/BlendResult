use anyhow::{self, Context};
use std::env;
use std::fs;
mod element;
mod rf_parser;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let filename = env::args()
        .nth(1)
        .context("Wrong amount of command line parameters")?;
    let csv_file = env::args()
        .nth(2)
        .context("Wrong amount of command line parameters")?;
    println!("Analyzing {}", filename);
    let xml = fs::read_to_string(filename).context("Reading failed")?;
    rf_parser::parse(&xml, &csv_file)?;
    Ok(())
}
