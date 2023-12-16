use assert_cmd::prelude::*;
use std::env;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_regex_coloring() -> Result<(), Box<dyn std::error::Error>> {
    // Set the environment variable to force colored output
    env::set_var("CLICOLOR_FORCE", "1");

    let mut file = NamedTempFile::new()?;
    writeln!(file, "Hello World Rust Programming")?;

    // Additional test content
    let complex_text = "Rust: safe, fast, productive. Pick three.";
    writeln!(file, "{}", complex_text)?;

    let regex_pattern_chars = "\\w";
    let regex_pattern_words = "\\w+";
    let lookahead_regex = "Rust(?=:)"; // Matches 'Rust' only if followed by ':'
    let complex_word_pattern = "\\b\\w{4,}\\b"; // Matches words with 4 or more characters

    let run_and_capture_output = |pattern| -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("regex_rustler")?;
        cmd.arg("--path")
            .arg(file.path())
            .arg("--regex")
            .arg(pattern);

        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    };

    let output_chars = run_and_capture_output(regex_pattern_chars)?;
    let output_words = run_and_capture_output(regex_pattern_words)?;
    let output_lookahead = run_and_capture_output(lookahead_regex)?;
    let output_complex_words = run_and_capture_output(complex_word_pattern)?;

    // Print the outputs for debugging along with the regex patterns and expected colors
    println!(
        "Regex pattern '{}' (expected blue 'H', red 'e'):\n{}",
        regex_pattern_chars, output_chars
    );
    println!(
        "Regex pattern '{}' (expected blue 'Hello', red 'World'):\n{}",
        regex_pattern_words, output_words
    );
    println!(
        "Regex pattern '{}' (expected blue 'Rust'):\n{}",
        lookahead_regex, output_lookahead
    );
    println!(
        "Regex pattern '{}' (expected red 'safe', blue 'fast', red 'productive'):\n{}",
        complex_word_pattern, output_complex_words
    );

    // Assertions
    assert!(output_chars.contains("\x1b[34mH\x1b[0m")); // Blue 'H'
    assert!(output_chars.contains("\x1b[31me\x1b[0m")); // Red 'e'
    assert!(output_words.contains("\x1b[34mHello\x1b[0m")); // Blue 'Hello'
    assert!(output_words.contains("\x1b[31mWorld\x1b[0m")); // Red 'World'
    assert!(output_lookahead.contains("\x1b[34mRust\x1b[0m")); // Blue 'Rust' (if followed by ':')
    assert!(output_complex_words.contains("\x1b[31msafe\x1b[0m")); // Red 'safe'
    assert!(output_complex_words.contains("\x1b[34mfast\x1b[0m")); // Blue 'fast'
    assert!(output_complex_words.contains("\x1b[31mproductive\x1b[0m")); // Red 'productive'

    Ok(())
}
