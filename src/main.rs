#![allow(unused)]
use std::io;
use std::env::args;
use clap::Parser;
use std::path::PathBuf;
use std::env;
use std::fs;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
struct Args{
    // input_file: String,
    #[arg(long,short)]
    search: String,
    #[arg(long,short)]
    path : String,
    #[arg(long,short)]                               
    count : bool
    
}

fn main() -> io::Result<()> {
    // let mut input = String::new();

    // io::stdin().read_line(&mut input)?;

    // println!("You typed: {}", input.trim());

    let args = Args::parse();

    // println!("name: {:?}", args.search);

    let path = PathBuf::from(&args.path);
    // println!("path: {:?}", path.display());

    let count = args.count;
    // println!("count: {:?}", count);

    

    let contents = fs::read_to_string(path).with_context(|| format!("Failed to read instrs from {}", path))?;

    // println!("With text:\n{contents}");
    let mut count_match = 0;
    for line in contents.lines() {
        if(line.contains(&args.search)) {
            count_match += 1;
        }
    }

    if count{
        println!("With count: {}", count_match);
    }
    else{
        println!("No match");
    }

    Ok(())
}