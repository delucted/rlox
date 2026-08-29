//! # errors.rs
//!
//! Simple error surfacing utilities.

use std::process;

/// Prints a basic error to stderr.
pub fn error(line: u32, line_src: &str, message: &str) {
    eprintln!("Error: {message}\n\t{line} | {line_src}")
}

/// Errors and then exits.
pub fn kill(line: u32, line_src: &str, message: &str) {
    error(line, line_src, message);
    process::exit(1);
}