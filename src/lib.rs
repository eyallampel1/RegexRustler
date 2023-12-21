// In lib.rs

use crate::processor::process_regex;
use anyhow::Result;
use fancy_regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub mod errors;
pub mod parser;
pub mod processor;
pub mod real_time;
use errors::AppError;

pub fn process_file(file_path: &str, regex_pattern: &str) -> Result<()> {
    let file = File::open(file_path).map_err(AppError::FileReadError)?;
    let reader = BufReader::new(file);
    let regex = Regex::new(regex_pattern).map_err(AppError::RegexError)?;

    let mut toggle = false;

    for line in reader.lines() {
        let line = line.map_err(AppError::FileReadError)?;
        let processed_line = process_regex(&line, &regex, &mut toggle)?;
        println!("{}", processed_line);
    }

    Ok(())
}
