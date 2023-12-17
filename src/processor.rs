// In processor.rs
use crate::errors::AppError;
use anyhow::Result;
use colored::*;
use fancy_regex::Regex;

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
