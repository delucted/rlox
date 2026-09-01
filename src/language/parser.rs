use crate::language::expr::Expr;
use crate::language::stmt::Stmt;
use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;
use crate::util::errors::ParseError;

#[derive(Debug, Default)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            ..Parser::default()
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current-1]
    }

    fn end(&self) -> bool {
        match self.peek().kind {
            TokenType::EOF => true,
            _ => false
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, kind: TokenType) -> bool {
        !self.end() && self.peek().kind == kind
    }

    fn match_next(&mut self, types: &[TokenType]) -> bool {
        for &kind in types {
            if self.check(kind) {
                self.advance();
                return true
            }
        }

        false
    }

    fn consume(&mut self, kind: TokenType, message: &'static str) -> Option<Token> {
        if self.check(kind) {
            return Some(self.advance().clone())
        }
        self.errors.push(ParseError {
            token: self.peek().clone(),
            message: String::from(message)
        });
        None
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_next(&[TokenType::False]) {
            return Some(Expr::Literal { value: Literal::Boolean(false) })
        }
        if self.match_next(&[TokenType::True]) {
            return Some(Expr::Literal { value: Literal::Boolean(true) })
        }
        if self.match_next(&[TokenType::Nil]) {
            return Some(Expr::Literal { value: Literal::Nil })
        }
        if self.match_next(&[TokenType::Number, TokenType::String]) {
            return Some(Expr::Literal {value: self.previous().literal.as_ref().cloned().unwrap()})
        }
        if self.match_next(&[TokenType::Identifier]) {
            return Some(Expr::Variable { name: self.previous().clone() })
        }
        if self.match_next(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "unterminated parenthesis");
            return Some(Expr::Grouping { expression: Box::new(expr) })
        }
        self.errors.push(ParseError {
            token: self.peek().clone(),
            message: String::from("expression expected")
        });
        None
    }

    fn unary(&mut self) -> Expr {
        if self.match_next(
            &[TokenType::Bang, TokenType::Minus]
        ) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary {operator, right: Box::new(right)}
        }

        self.primary().unwrap_or(Expr::Literal {value: Literal::Nil })
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_next(
            &[TokenType::Slash, TokenType::Star]
        ) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {left: Box::new(expr), operator, right: Box::new(right)};
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_next(
            &[TokenType::Minus, TokenType::Plus]
        ) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {left: Box::new(expr), operator, right: Box::new(right)};
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_next(
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual
            ]
        ) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary { left: Box::from(expr), operator, right: Box::from(right) };
        }

        expr
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.end() {
            if self.previous().kind == TokenType::Semicolon {
                return
            }

            match self.peek().kind {
                TokenType::Class => { return }
                TokenType::Fun => { return }
                TokenType::Var => { return }
                TokenType::For => { return }
                TokenType::If => { return }
                TokenType::While => { return }
                TokenType::Print => { return }
                TokenType::Return => { return }
                _=>{}
            }

            self.advance();
        }
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "semicolon expected after value");
        Stmt::Print(value)
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "semicolon expected after value");
        Stmt::Expression(expr)
    }

    fn statement(&mut self) -> Stmt {
        if self.match_next(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self.consume(
            TokenType::Identifier,
            "expected identifier"
        )?;

        let initializer: Option<Expr> = if self.match_next(&[TokenType::Equal]) {
             Some(self.expression())
        } else { None };

        self.consume(
            TokenType::Semicolon,
            "semicolon expected after variable declaration"
        )?;
        
        Some(Stmt::Var { name, initializer })
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_next(&[TokenType::Var]) {
            return self.var_declaration()
        }
        Some(self.statement())
    }

    pub fn parse(mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.end() {
            match self.declaration() {
                Some(stmt) => statements.push(stmt),
                None => return Err(self.errors)
            }
        }

        if !self.errors.is_empty() {
            return Err(self.errors)
        }
        Ok(statements)
    }
}