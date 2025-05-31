use anyhow::{self, Context};
use std::env;
use std::fs;
mod element;
mod multi_result_list;
mod rf_parser;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        filename: Option<String>,
        csv_file: Option<String>,
    },
    Blend {
        name: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { filename, csv_file } => {
            println!("Parsing {}", filename.as_ref().unwrap());
            let xml = fs::read_to_string(filename.as_ref().unwrap()).context("Reading failed")?;
            rf_parser::parse(&xml, &csv_file.as_ref().unwrap())?;
        }
        Commands::Blend { name } => {
            println!("'myapp add' was used, name is: {name:?}");
        }
    }
    Ok(())
}
