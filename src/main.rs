use anyhow::Result;
use regex_rustler::parser::parse_args;
use regex_rustler::process_file;
use regex_rustler::real_time::real_time_regex_testing;

/// Entry point for the regex_rustler application.
/// It parses arguments and decides whether to run real-time regex testing or process a file based on the provided configuration.
fn main() -> Result<()> {
    let config = parse_args()?;

    if config.real_time {
        real_time_regex_testing(&config.file_path)?;
    } else {
        // Handle the case when regex_pattern is None
        if let Some(regex_pattern) = &config.regex_pattern {
            process_file(&config.file_path, regex_pattern)?;
        } else {
            return Err(anyhow::anyhow!("Regex pattern is required"));
        }
    }

    Ok(())
}
