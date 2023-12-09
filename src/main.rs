// main.rs
use std::{env, io};
use my_regex_project::process_file;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <regex_pattern>", args[0]);
        return Ok(());
    }

    process_file(&args[1], &args[2])
}
