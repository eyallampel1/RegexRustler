// In processor.rs
/// This module is responsible for the actual processing of text using regex patterns.
/// It provides functionality to apply regex patterns to a line of text and color the matches.
/// It is used by the binary crate (src/main.rs) and the integration tests (tests/integration_test.rs).
/// # Examples
/// ```
/// use regex_rustler::processor::process_regex;
/// use fancy_regex::Regex;
/// 
/// let regex = Regex::new("[a-z]").unwrap();
/// let mut toggle = false;
/// let line = "This is a test";
/// let processed_line = process_regex(line, &regex, &mut toggle).unwrap();
/// assert_eq!(processed_line, "This is a test");
/// ```
use crate::errors::AppError;
use anyhow::Result;
use colored::*;
use fancy_regex::Regex;

/// Processes a single line of text using the provided regex pattern.
/// It toggles colors for matched patterns and returns the processed line.
/// # Arguments
/// * `line` - A string slice that holds the line of text to process.
/// * `regex` - A fancy_regex::Regex object that holds the regex pattern to apply to the line.
/// * `toggle` - A mutable boolean that toggles the color of the matched patterns.
/// # Returns
/// * `Result<String>` - A Result object that holds the processed line.
/// # Errors
/// * `AppError::RegexError` - An error indicating that the regex pattern is invalid.
/// # Examples
/// ```
/// use regex_rustler::processor::process_regex;
/// use fancy_regex::Regex;
/// 
/// let regex = Regex::new("[a-z]").unwrap();
/// let mut toggle = false;
/// let line = "This is a test";
/// let processed_line = process_regex(line, &regex, &mut toggle).unwrap();
/// assert_eq!(processed_line, "This is a test");
/// ```
/// # Notes
/// * This function is not used in the real_time.rs file.
/// * This function is not used in the integration_test.rs file.
/// * This function is not used in the parser.rs file.
/// * This function is not used in the errors.rs file.
/// * This function is not used in the lib.rs file.
/// * This function is not used in the main.rs file.
pub fn process_regex(line: &str, regex: &Regex, toggle: &mut bool) -> Result<String> {
    let mut new_line = String::new();
    let mut last = 0;

    // Iterate over the matches and handle each Result item
    let matches = regex.find_iter(line);
    for match_result in matches {
        let mat = match_result.map_err(AppError::RegexError)?;

        // Process each match
        new_line.push_str(&line[last..mat.start()]);
        let matched = if *toggle {
            mat.as_str().red().to_string()
        } else {
            mat.as_str().blue().to_string()
        };
        *toggle = !*toggle;
        new_line.push_str(&matched);
        last = mat.end();
    }

    new_line.push_str(&line[last..]);
    Ok(new_line)
}
