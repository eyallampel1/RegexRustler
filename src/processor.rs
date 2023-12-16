// In processor.rs
use colored::*;
use fancy_regex::Regex;
use std::io;

pub fn process_regex(line: &str, regex: &Regex, toggle: &mut bool) -> io::Result<String> {
    let mut new_line = String::new();
    let mut last = 0;

    // Iterate over the matches and handle each Result item
    let matches = regex.find_iter(line);
    for match_result in matches {
        match match_result {
            Ok(mat) => {
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
            Err(e) => {
                // Handle any errors that occurred during matching
                return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
            }
        }
    }

    new_line.push_str(&line[last..]);
    Ok(new_line)
}
