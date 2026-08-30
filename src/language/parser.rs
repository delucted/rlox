use crate::language::expr::Expr;
use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;
use crate::util::errors::serror;

#[derive(Debug, Default)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: u16,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            ..Parser::default()
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current as usize]
    }

    fn previous(&self) -> &Token {
        &self.tokens[(self.current-1) as usize]
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

    fn report(token: &Token, message: &'static str) {
        serror(token.line, &token.lexeme, message)
    }

    fn consume(&mut self, kind: TokenType, message: &'static str) -> Result<&Token, &'static str> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Parser::report(self.peek(), message);
        Err(message)
    }

    fn primary(&mut self) -> Result<Expr, &'static str> {
        if self.match_next(&[TokenType::False]) {
            return Ok(Expr::Literal { value: Literal::Boolean(false) })
        }
        if self.match_next(&[TokenType::True]) {
            return Ok(Expr::Literal { value: Literal::Boolean(true) })
        }
        if self.match_next(&[TokenType::Nil]) {
            return Ok(Expr::Literal { value: Literal::Nil })
        }
        if self.match_next(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {value: self.previous().literal.as_ref().cloned().unwrap()})
        }
        if self.match_next(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "unterminated parenthesis")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) })
        }
        Parser::report(self.peek(), "expression expected");
        Err("expression expected")
    }

    fn unary(&mut self) -> Result<Expr, &'static str> {
        if self.match_next(
            &[TokenType::Bang, TokenType::Minus]
        ) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {operator, right: Box::new(right)})
        }

        self.primary()
    }

    fn factor(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.unary()?;

        while self.match_next(
            &[TokenType::Slash, TokenType::Star]
        ) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {left: Box::new(expr), operator, right: Box::new(right)};
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.factor()?;

        while self.match_next(
            &[TokenType::Minus, TokenType::Plus]
        ) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {left: Box::new(expr), operator, right: Box::new(right)};
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.term()?;

        while self.match_next(
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual
            ]
        ) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.comparison()?;

        while self.match_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::from(expr), operator, right: Box::from(right) };
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, &'static str> {
        self.equality()
    }

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

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        self.expression()
    }
}