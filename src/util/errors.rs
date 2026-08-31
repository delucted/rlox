use std::fmt;
use crate::language::token::Token;

#[derive(Debug)]
pub struct LexError {
    pub line: u32,
    pub preview: String,
    pub message: String
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token, // token is line and lexeme
    pub message: String,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String
}

#[derive(Debug)]
pub enum LoxError {
    Lex(Vec<LexError>),
    Parse(Vec<ParseError>),
    Runtime(RuntimeError)
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime Error: {}\n\t{} | {}", self.message, self.token.line, self.token.lexeme)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error: {}\n\t{} | {}", self.message, self.token.line, self.token.lexeme)
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lex Error: {}\n\t{} | {}", self.message, self.line, self.preview)
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex(errors) => {
                let mut o = String::new();
                for error in errors {
                    o.push_str(&error.to_string());
                }
                write!(f, "{o}")
            }
            Self::Parse(errors) => {
                let mut o = String::new();
                for error in errors {
                    o.push_str(&error.to_string());
                }
                write!(f, "{o}")
            }
            Self::Runtime(error) => {
                write!(f, "{error}")
            }
        }
    }
}

impl LoxError {
    pub(crate) fn exit_code(&self) -> u8 {
        match self {
            Self::Lex(_)     => 65,
            Self::Parse(_)   => 65,
            Self::Runtime(_) => 70
        }
    }
}

impl From<Vec<LexError>> for LoxError {
    fn from(value: Vec<LexError>) -> Self {
        LoxError::Lex(value)
    }
}

impl From<Vec<ParseError>> for LoxError {
    fn from(value: Vec<ParseError>) -> Self {
        LoxError::Parse(value)
    }
}

impl From<RuntimeError> for LoxError {
    fn from(value: RuntimeError) -> Self {
        LoxError::Runtime(value)
    }
}