use colored::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <regex_pattern>", args[0]);
        return Ok(());
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let regex = Regex::new(&args[2]).unwrap();

    let mut toggle = false;

    for line in reader.lines() {
        let line = line?;
        let mut new_line = String::new();
        let mut last = 0;

        for mat in regex.find_iter(&line) {
            new_line.push_str(&line[last..mat.start()]);
            let matched = if toggle {
                mat.as_str().red().to_string()
            } else {
                mat.as_str().blue().to_string()
            };
            toggle = !toggle;
            new_line.push_str(&matched);
            last = mat.end();
        }

        new_line.push_str(&line[last..]);
        println!("{}", new_line);
    }

    Ok(())
}
