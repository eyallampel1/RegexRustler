// src/parser.rs
use clap::{command, Arg};

pub struct Config {
    pub file_path: String,
    pub regex_pattern: String,
}

pub fn parse_args() -> Config {
    let matches = command!()
        .author("Eyal Lampel")
        .about("Searches (and colors) for a regex pattern in a text file")
        .arg(
            Arg::new("file-path")
                .short('f')
                .long("path")
                .aliases(&["fpath", "path", "text-file", "file"])
                .value_name("FILE")
                .required(true)
                .help("Path to the Text file to search for example: /home/user/text.txt"),
        )
        .arg(
            Arg::new("regex-pattern")
                .short('r')
                .long("regex")
                .value_name("REGEX")
                .required(true)
                .help("Regex pattern to match for example: [a-z]"),
        )
        .get_matches();

    Config {
        file_path: matches.get_one::<String>("file-path").unwrap().clone(),
        regex_pattern: matches.get_one::<String>("regex-pattern").unwrap().clone(),
    }
}
