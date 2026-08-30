//! # errors.rs
//!
//! Simple error surfacing utilities.

/// Prints a basic error with line info to stderr.
pub fn error(line: u32, line_src: &str, message: &str) {
    eprintln!("Error: {message}\n\t{line} | {line_src}")
}

/// Prints a basic error with no line info to stderr.
pub fn serror(line: u32, lexeme: &str, message: &'static str) {
    eprintln!("Error: {message}\n\t{line} | {lexeme}")
}