// In lib.rs
use crate::processor::process_regex;
use fancy_regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod parser;
pub mod processor;

pub fn process_file(file_path: &str, regex_pattern: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Handle the potential error from Regex::new()
    let regex = match Regex::new(regex_pattern) {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("Error compiling regex: {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
        }
    };

    let mut toggle = false;

    for line in reader.lines() {
        let line = line?;
        let processed_line = process_regex(&line, &regex, &mut toggle)?;
        println!("{}", processed_line);
    }

    Ok(())
}
