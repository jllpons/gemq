use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
use clap::Parser;
use colored_json::to_colored_json_auto;
// use jaq_core;
// use jaq_json::Val as JaqValue;
// use serde_json::{json, Value};

mod cli;
mod gbparser;
mod genbank;

fn open_file(file_path: &str) -> Result<Box<dyn BufRead>> {
    // If the file name is "-", read from stdin.
    // Otherwise, open the file.
    // Return a BufRead trait object.
    match file_path {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_path)?))),
    }
}

fn run(args: cli::Args) -> Result<()> {
    println!("{:?}", args);

    let file = open_file(&args.input)?;

    let gb_record = gbparser::parse_gb(file)?;

    let json_str = serde_json::to_value(&gb_record).unwrap();
    println!("{}", to_colored_json_auto(&json_str).unwrap());

    Ok(())
}

fn main() {
    let args = cli::Args::parse();

    if let Err(err) = run(args) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    std::process::exit(0);
}
