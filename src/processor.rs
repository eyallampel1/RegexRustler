// In processor.rs
use colored::*;
use regex::Regex;
use std::io;

pub fn process_regex(line: &str, regex: &Regex, toggle: &mut bool) -> io::Result<String> {
    let mut new_line = String::new();
    let mut last = 0;

    for mat in regex.find_iter(line) {
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
