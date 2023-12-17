// errors.rs
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error reading file: {0}")]
    FileReadError(#[from] io::Error),

    #[error("Error compiling regex: {0}")]
    RegexError(#[from] fancy_regex::Error),
}
