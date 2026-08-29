use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;
use crate::util::errors::{error, kill};
use std::collections::HashMap;
use std::sync::OnceLock;

static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

fn keywords() -> &'static HashMap<&'static str, TokenType> {
    KEYWORDS.get_or_init(|| {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    })
}

fn get_by_keyword(keyword: &str) -> Option<TokenType> {
    keywords().get(keyword).copied()
}

#[derive(Default)]
pub struct Lexer {
    pub source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    line_src: String
}

impl Lexer {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter()
    }

    pub(crate) fn new(source: String) -> Self {
        Lexer {
            line: 1,
            source,
            ..Lexer::default()
        }
    }

    fn end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars()
            .nth(self.current)
            .unwrap();
        self.current += 1;
        self.line_src.push(c);
        c
    }

    fn add_token(&mut self, _type: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token {_type, literal, line: self.line, lexeme})
    }

    fn token(&mut self, _type: TokenType) {
        self.add_token(_type, None)
    }

    fn _match(&mut self, expected: char) -> bool {
        if self.end() {
            return false
        }
        if !(self.source.chars().nth(self.current).unwrap() == expected) {
            return false
        }

        self.advance();
        true
    }

    fn peek(&self, offset: Option<usize>) -> char {
        let index = self.current + offset.unwrap_or_else(|| 0);
        if index >= self.source.len() {
            return '\0'
        }
        self.source.chars().nth(index).unwrap()
    }

    fn line_preview(&self) -> String {
        let remaining_line = self.source[self.current..]
            .split('\n')
            .next()
            .unwrap_or("");

        let complete_line = format!("{}{}", self.line_src, remaining_line);

        let mut characters = complete_line.chars();
        let preview: String = characters.by_ref().take(40).collect();

        if characters.next().is_some() {
            format!("{preview}... (truncated)")
        } else {
            preview
        }
    }

    fn string(&mut self) {
        while self.peek(None) != '"' && !self.end() {
            let c = self.advance();

            if c == '\n' {
                self.line += 1;
                self.line_src.clear();
            }
        }

        if self.end() {
            let line = self.line_preview();
            kill(self.line, &line, "unterminated string")
        }

        // Consume the closing quote.
        self.advance();

        // Remove the opening and closing quotes.
        let value = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::String, Some(Literal::String(value)));
    }

    fn number(&mut self) {
        while self.peek(None).is_digit(10) {
            self.advance();
        }

        if self.peek(None) == '.' && self.peek(Some(1)).is_digit(10) {
            self.advance();

            while self.peek(None).is_digit(10) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number, Some(Literal::Number(
                self.source[self.start..self.current]
                    .parse()
                    .unwrap()
            ))
        )
    }

    fn identifier(&mut self) {
        while self.peek(None).is_alphanumeric() || self.peek(None) == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        self.token(get_by_keyword(text).unwrap_or_else(|| TokenType::Identifier));
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.token(TokenType::LeftParen),
            ')' => self.token(TokenType::RightParen),
            '{' => self.token(TokenType::LeftBrace),
            '}' => self.token(TokenType::RightBrace),
            ',' => self.token(TokenType::Comma),
            '.' => self.token(TokenType::Dot),
            '-' => self.token(TokenType::Minus),
            '+' => self.token(TokenType::Plus),
            ';' => self.token(TokenType::Semicolon),
            '*' => self.token(TokenType::Star),
            '!' => {
                let is_match = self._match('=');
                self.token(
                    if is_match {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                )
            }
            '=' => {
                let is_match = self._match('=');
                self.token(
                    if is_match {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                )
            }
            '<' => {
                let is_match = self._match('=');
                self.token(
                    if is_match {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                )
            }
            '>' => {
                let is_match = self._match('=');
                self.token(
                    if is_match {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                )
            }
            '/' => {
                if self._match('/') {
                    while !(self.peek(None) == '\n') && !self.end() {
                        self.advance();
                    }
                } else {
                    self.token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => { self.line += 1; self.line_src.clear(); }
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                    return;
                }
                if c.is_alphabetic() || c == '_' {
                    self.identifier();
                    return;
                }
                let line = self.line_preview();
                kill(self.line, &line, "unexpected character \"{c}\"")
            }
        }
    }
    pub(crate) fn scan_tokens(&mut self) {
        while !self.end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            _type: TokenType::EOF,
            lexeme: "".to_owned(),
            literal: None,
            line: self.line
        });
    }
}