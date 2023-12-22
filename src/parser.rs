// src/parser.rs
/// This module handles parsing of command-line arguments using the clap library.
/// It is used by the binary crate (src/main.rs).
/// It is not used by the integration tests (tests/integration_test.rs).
use anyhow::{anyhow, Result};
use clap::{command, Arg, ArgAction};

pub struct Config {
    pub file_path: String,
    pub regex_pattern: Option<String>,
    pub real_time: bool,
}

/// Parses arguments from the command line to create a Config object.
/// It sets up the necessary options and flags for the application's CLI.
/// # Returns
/// * `Config` - A Config object that holds the parsed arguments.
/// # Errors
/// * `anyhow::Error` - An error indicating that the file path is required.
/// * `anyhow::Error` - An error indicating that the regex pattern is required.
/// # Examples
/// ```
/// use regex_rustler::parser::parse_args;
/// use regex_rustler::parser::Config;
/// 
/// let config = parse_args().unwrap();
/// assert_eq!(config.file_path, "test.txt");
/// assert_eq!(config.regex_pattern, Some("[a-z]".to_string()));
/// assert_eq!(config.real_time, false);
/// ```
pub fn parse_args() -> Result<Config> {
    let matches = command!()
        .author("Eyal Lampel")
        .about("Searches (and colors) for a regex pattern in a text file")
        .arg(
            Arg::new("file-path")
                .short('p')
                .long("path")
                .aliases(["fpath", "path", "text-file", "file"])
                .value_name("FILE")
                .required(true)
                .help("Path to the Text file to search for example: /home/user/text.txt"),
        )
        .arg(
            Arg::new("regex-pattern")
                .short('r')
                .long("regex")
                .value_name("REGEX")
                .required_unless_present("real-time-regex-testing")
                .help("Regex pattern to match for example: [a-z]"),
        )
        .arg(
            Arg::new("real-time-regex-testing")
                .short('t')
                .long("realTime")
                .aliases([
                    "rt",
                    "real-time",
                    "realtime",
                    "realTime",
                    "realTimeRegexTesting",
                ])
                .action(ArgAction::SetTrue)
                .help("Enter interactive mode for real-time regex testing"),
        )
        .get_matches();

    let file_path = matches
        .get_one::<String>("file-path")
        .ok_or_else(|| anyhow!("File path is required"))?
        .clone();
    let regex_pattern = matches
        .get_one::<String>("regex-pattern")
        .map(|s| s.to_string());
    let real_time = matches.get_flag("real-time-regex-testing");

    Ok(Config {
        file_path,
        regex_pattern,
        real_time,
    })
}
