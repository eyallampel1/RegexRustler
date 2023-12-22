// In lib.rs

use crate::processor::process_regex;
use anyhow::Result;
use fancy_regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
/// This is the core library file for regex_rustler.
/// It exposes the primary functionality of processing files with regex patterns.
/// It also exposes the error types used in the application.
/// It is used by the binary crate (src/main.rs) and the integration tests (tests/integration_test.rs).
pub mod errors;
pub mod parser;
pub mod processor;
pub mod real_time;
use errors::AppError;

/// Processes a file at the given path using the specified regex pattern.
/// It reads the file line by line and applies the regex pattern, printing the processed text.
/// # Arguments
/// * `file_path` - A string slice that holds the path to the file to process.
/// * `regex_pattern` - A string slice that holds the regex pattern to apply to the file.
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
