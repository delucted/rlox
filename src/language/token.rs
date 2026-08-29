use crate::language::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32
}