use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;
use crate::util::errors::LexError;

fn get_by_keyword(keyword: &str) -> Option<TokenType> {
    match keyword {
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

#[derive(Default)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    errors: Vec<LexError>
}

impl Lexer {
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
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();
        c
    }

    fn add_token(&mut self, kind: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token {kind, literal, line: self.line, lexeme})
    }

    fn token(&mut self, kind: TokenType) {
        self.add_token(kind, None)
    }

    fn _match(&mut self, expected: char) -> bool {
        if self.end() {
            return false
        }
        if !(self.source[self.current..].chars().next().unwrap() == expected) {
            return false
        }

        self.advance();
        true
    }

    fn peek(&self) -> char {
        self.source[self.current..].chars().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source[self.current+1..].chars().next().unwrap_or('\0')
    }

    fn line_of(&self, pos: usize) -> &str {
        let start = self.source[..pos].rfind('\n').map_or(0, |i| i + 1);
        let end = self.source[pos..].find('\n').map_or(self.source.len(), |i| pos + i);
        &self.source[start..end]
    }

    fn line_preview(&self, pos: usize) -> String {
        let mut chars = self.line_of(pos).chars();
        let preview: String = chars.by_ref().take(40).collect();
        if chars.next().is_some() {
            format!("{preview}... (truncated)")
        } else {
            preview
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.end() {
            let c = self.advance();

            if c == '\n' {
                self.line += 1;
            }
        }

        if self.end() {
            self.errors.push(LexError {
                line: self.line,
                preview: self.line_preview(self.current - 1),
                message: String::from("unterminated string")
            });
            return;
        }

        // Consume the closing quote.
        self.advance();

        // Remove the opening and closing quotes.
        let value = self.source[self.start + 1..self.current - 1].to_owned();
        self.add_token(TokenType::String, Some(Literal::String(value)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
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
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
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
                    while !(self.peek() == '\n') && !self.end() {
                        self.advance();
                    }
                } else {
                    self.token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                    return;
                }
                if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                    return;
                }
                self.errors.push(LexError {
                    line: self.line,
                    preview: self.line_preview(self.current),
                    message: String::from("unexpected character")
                });
            }
        }
    }
    pub(crate) fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<LexError>> {
        while !self.end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenType::EOF,
            lexeme: "".to_owned(),
            literal: None,
            line: self.line
        });

        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }
}