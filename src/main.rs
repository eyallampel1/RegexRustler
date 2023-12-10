// main.rs
use std::{env, io};
use RegexRustler::process_file;
use clap::{command, Arg};
fn main() -> io::Result<()> {
    let matches= command!().about("Searches(and colors) for a regex pattern in a text file")
    .arg(
        Arg::new("file-path")
        .short('f')
        .long("path")
        .aliases(["fpath", "path", "text-file","file"])
        .value_name("FILE")
        .required(true)
        .help("Path to the Text file to search for example: /home/user/text.txt")
    )   
    .arg(
        Arg::new("regex-pattern")
        .short('r')
        .long("regex")
        .value_name("REGEX")
        .required(true)
        .help("Regex pattern to match for example: [a-z]")
    )
        .get_matches();

    process_file(matches.get_one::<String>("file-path").unwrap(), matches.get_one::<String>("regex-pattern").unwrap())
}
