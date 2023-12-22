// errors.rs
/// This module defines custom error types for the regex_rustler application.
/// It is used by the binary crate (src/main.rs) and the integration tests (tests/integration_test.rs).
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error reading file: {0}")]
    FileReadError(#[from] io::Error),

    #[error("Error compiling regex: {0}")]
    RegexError(#[from] fancy_regex::Error),
}
