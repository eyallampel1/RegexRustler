// main.rs
use regex_rustler::parser::parse_args;
use regex_rustler::process_file;
use std::io;

fn main() -> io::Result<()> {
    let config = parse_args();
    process_file(&config.file_path, &config.regex_pattern)
}
