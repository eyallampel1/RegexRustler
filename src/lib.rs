// In lib.rs
use crate::processor::process_regex;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod parser;
pub mod processor;

pub fn process_file(file_path: &str, regex_pattern: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let regex = Regex::new(regex_pattern).unwrap();

    let mut toggle = false;

    for line in reader.lines() {
        let line = line?;
        let processed_line = process_regex(&line, &regex, &mut toggle)?;
        println!("{}", processed_line);
    }

    Ok(())
}
