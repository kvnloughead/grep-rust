#![allow(unused)]

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use clap::Parser;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/// Search for a pattern in a file and display the lines contained in it
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    /// The pattern to search for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    println!("{:#?}", &args);

    let f = File::open(&args.path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) { println!("{:?}", line) }
            }
            Err(ref e) => println!("error reading line: {:?}", line),
        }
    }

    Ok(())
}
