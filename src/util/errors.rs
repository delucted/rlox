//! # errors.rs
//!
//! Simple error surfacing utilities.

/// Prints a basic error to stderr.
pub fn error(line: u32, line_src: &str, message: &str) {
    eprintln!("Error: {message}\n\t{line} | {line_src}")
}