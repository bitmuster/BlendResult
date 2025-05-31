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
        output: Option<String>,
    },
    Blend {
        output: String,
        input: Vec<String>,
    },
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { filename, output } => {
            println!("Parsing {}", filename.as_ref().unwrap());
            let xml = fs::read_to_string(filename.as_ref().unwrap()).context("Reading failed")?;
            rf_parser::parse(&xml, &output.as_ref().unwrap())?;
        }
        Commands::Blend { input, output } => {
            println!("Blending {:?} {}", input, output);
            rf_parser::blend(input, output)?;
        }
    }
    Ok(())
}
