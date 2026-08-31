//! # errors.rs
//!
//! Simple error surfacing utilities.

/// Prints a basic error with line info to stderr.
pub fn lexer_error(line: u32, line_src: &str, message: &str) {
    eprintln!("Lexer Error: {message}\n\t{line} | {line_src}")
}

/// Prints a basic error with lexeme info to stderr.
pub fn parse_error(line: u32, lexeme: &str, message: &'static str) {
    eprintln!("Parser Error: {message}\n\t{line} | {lexeme}")
}

/// Prints a basic runtime error to stderr
pub fn runtime_error(message: &'static str) {
    eprintln!("Runtime Error: {message}")
}