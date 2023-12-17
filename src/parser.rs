// src/parser.rs
use anyhow::{anyhow, Result}; // Import Result from anyhow
use clap::{command, Arg};

pub struct Config {
    pub file_path: String,
    pub regex_pattern: String,
    pub real_time: bool,
}

pub fn parse_args() -> Result<Config> {
    let matches = command!()
        .author("Eyal Lampel")
        .about("Searches (and colors) for a regex pattern in a text file")
        .arg(
            Arg::new("file-path")
                .short('p')
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
        .arg(
            Arg::new("real-time-regex-testing")
                .short('t')
                .long("realTime")
                .aliases(&[
                    "rt",
                    "real-time",
                    "realtime",
                    "realTime",
                    "realTimeRegexTesting",
                ])
                .value_name("TEST")
                .required(false)
                .help("An argument to test your regex pattern in real time for example: [a-z]")
                //.value_hint(value_hint)
                .conflicts_with("regex-pattern"),
        )
        .get_matches();

    let file_path = matches
        .get_one::<String>("file-path")
        .ok_or_else(|| anyhow!("File path is required"))?
        .clone();
    let regex_pattern = matches
        .get_one::<String>("regex-pattern")
        .ok_or_else(|| anyhow!("Regex pattern is required"))?
        .clone();

    Ok(Config {
        file_path,
        regex_pattern,
        real_time: matches.args_present(),
    })
}
