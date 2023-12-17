// main.rs
use anyhow::Result;
use regex_rustler::parser::parse_args;
use regex_rustler::process_file;

fn main() -> Result<()> {
    let config = parse_args()?;
    process_file(&config.file_path, &config.regex_pattern)
}
