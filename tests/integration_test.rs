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

#[test]
fn test_simple_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Set the environment variable to force colored output
    env::set_var("CLICOLOR_FORCE", "1");

    // Create a temporary file and write test content to it
    let mut file = NamedTempFile::new()?;
    writeln!(file, "abc ABC 123")?;

    // Define a simple regex pattern
    let simple_regex_pattern = "[a-z]";

    // Function to run the application and capture output
    let run_and_capture_output = |pattern| -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("regex_rustler")?;
        cmd.arg("--path")
            .arg(file.path())
            .arg("--regex")
            .arg(pattern);

        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    };

    // Run the application with the simple regex pattern and capture output
    let output = run_and_capture_output(simple_regex_pattern)?;

    // Print the output for debugging
    println!(
        "Output with simple pattern '{}':\n{}",
        simple_regex_pattern, output
    );

    // Assertions
    // Check if the output contains colored 'a', 'b', and 'c'
    assert!(output.contains("\x1b[34ma\x1b[0m")); // Blue 'a'
    assert!(output.contains("\x1b[31mb\x1b[0m")); // Red 'b'
    assert!(output.contains("\x1b[34mc\x1b[0m")); // Blue 'c'

    Ok(())
}

#[test]
fn test_complex_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Set the environment variable to force colored output
    env::set_var("CLICOLOR_FORCE", "1");

    // Create a temporary file and write test content to it
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Rust: safe, fast, productive. Pick three.")?;

    // Define complex regex patterns
    let patterns = vec![
        ("\\b\\w{4,}\\b", "Words with 4 or more characters"),
        ("Rust(?=:)", "Rust followed by a colon"),
        ("\\b[a-zA-Z]{5}\\b", "Exactly 5 letter words"),
        ("\\d+", "One or more digits"),
    ];

    // Function to run the application and capture output
    let run_and_capture_output = |pattern| -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("regex_rustler")?;
        cmd.arg("--path")
            .arg(file.path())
            .arg("--regex")
            .arg(pattern);

        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    };

    for (pattern, description) in patterns {
        // Run the application with the complex regex pattern and capture output
        let output = run_and_capture_output(pattern)?;

        // Print the output for debugging
        println!(
            "Output for pattern '{}' ({}):\n{}",
            pattern, description, output
        );

        // Assertions
        // Here you can add specific assertions for each pattern
        // For example, check if the output contains expected matches
        // assert!(output.contains(...));
    }

    Ok(())
}

#[test]
fn test_no_match_pattern() -> Result<(), Box<dyn std::error::Error>> {
    // Set the environment variable to force colored output
    env::set_var("CLICOLOR_FORCE", "1");

    // Create a temporary file and write test content to it
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Hello World Rust Programming")?;

    // Define a regex pattern that should not match anything in the test content
    let no_match_regex_pattern = "XYZ123"; // This pattern should not match anything in the file

    // Function to run the application and capture output
    let run_and_capture_output = |pattern| -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("regex_rustler")?;
        cmd.arg("--path")
            .arg(file.path())
            .arg("--regex")
            .arg(pattern);

        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    };

    // Run the application with the no-match regex pattern and capture output
    let output = run_and_capture_output(no_match_regex_pattern)?;

    // Print the output for debugging
    println!(
        "Output with no-match pattern '{}':\n{}",
        no_match_regex_pattern, output
    );

    // Assertions
    // Check if the output is the same as the input (i.e., no coloring applied)
    assert_eq!(output.trim(), "Hello World Rust Programming");

    Ok(())
}

#[test]
fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Set the environment variable to force colored output
    env::set_var("CLICOLOR_FORCE", "1");

    // Create a temporary file and write some content to it
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Some test content")?;

    // Define an invalid regex pattern (missing closing bracket)
    let invalid_regex_pattern = "[a-z";

    // Function to run the application and capture output or error
    let run_and_capture_error = |pattern| -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("regex_rustler")?;
        cmd.arg("--path")
            .arg(file.path())
            .arg("--regex")
            .arg(pattern);

        let output = cmd.output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(String::from_utf8_lossy(&output.stderr).to_string())
        }
    };

    // Run the application with the invalid regex pattern and capture error output
    let error_output = run_and_capture_error(invalid_regex_pattern)?;

    // Print the error output for debugging
    println!(
        "Error output with invalid regex pattern '{}':\n{}",
        invalid_regex_pattern, error_output
    );

    // Assertions
    // Check if the error output contains the actual error message
    let expected_error_message = "Error compiling regex: Invalid character class";
    assert!(
        error_output.contains(expected_error_message),
        "Expected error message '{}', got: {}",
        expected_error_message,
        error_output
    );

    Ok(())
}
