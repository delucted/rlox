use crate::language::expr::Expr;
use crate::language::stmt::Stmt;
use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;
use crate::util::errors::ParseError;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_EXPR_ID: AtomicUsize = AtomicUsize::new(0);

fn next_expr_id() -> usize {
    NEXT_EXPR_ID.fetch_add(1, Ordering::Relaxed)
}

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

    fn consume(&mut self, kind: TokenType, message: &str) -> Option<Token> {
        if self.check(kind) {
            return Some(self.advance().clone())
        }
        self.errors.push(ParseError {
            token: self.peek().clone(),
            message: String::from(message)
        });
        None
    }

    fn primary(&mut self) -> Result<Option<Expr>, ParseError> {
        if self.match_next(&[TokenType::False]) {
            return Ok(Some(Expr::Literal { value: Literal::Boolean(false) }))
        }
        if self.match_next(&[TokenType::True]) {
            return Ok(Some(Expr::Literal { value: Literal::Boolean(true) }))
        }
        if self.match_next(&[TokenType::Nil]) {
            return Ok(Some(Expr::Literal { value: Literal::Nil }))
        }
        if self.match_next(&[TokenType::Number, TokenType::String]) {
            return Ok(Some(
                Expr::Literal {value: self.previous().literal.as_ref().cloned().unwrap()}
            ))
        }
        if self.match_next(&[TokenType::LeftBracket]) {
            let mut elements: Vec<Box<Expr>> = Vec::new();
            loop {
                match self.peek().kind {
                    TokenType::RightBracket => {
                        self.advance();
                        break
                    },
                    TokenType::Semicolon => {
                        return Err(ParseError {
                            token: self.peek().clone(),
                            message: String::from("unterminated bracket")
                        })
                    }
                    _ => {}
                }
                elements.push(Box::from(self.expression()?));
                match self.peek().kind {
                    TokenType::Comma => { self.advance(); }
                    TokenType::RightBracket => {}
                    _ => return Err(ParseError {
                        token: self.peek().clone(),
                        message: String::from("array elements not separated by commas")
                    })
                }
            }
            return Ok(Some(Expr::Array { elements }))
        }
        if self.match_next(&[TokenType::This]) {
            return Ok(Some(Expr::This { id: next_expr_id(), keyword: self.previous().clone() }))
        }
        if self.match_next(&[TokenType::Super]) {
            let keyword = self.previous().clone();
            self.consume(TokenType::Dot, "expect '.' after 'super'");

            let Some(method) = self.consume(
                TokenType::Identifier,
                "expect superclass method name"
            ) else { return Ok(None) };

            return Ok(Some(Expr::Super { id: next_expr_id(), keyword, method }))
        }
        if self.match_next(&[TokenType::Identifier]) {
            return Ok(Some(Expr::Variable { id: next_expr_id(), name: self.previous().clone() }))
        }
        if self.match_next(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "unterminated parenthesis");
            return Ok(Some(Expr::Grouping { expression: Box::new(expr) }))
        }
        self.errors.push(ParseError {
            token: self.peek().clone(),
            message: String::from("expression expected")
        });
        self.advance();
        Ok(None)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments: Vec<Box<Expr>> = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParseError {
                        token: self.peek().clone(),
                        message: String::from("argument size exceeds 255")
                    })
                }
                arguments.push(Box::from(self.expression()?));
                
                if !self.match_next(&[TokenType::Comma]) {
                    break
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "expect ')' after arguments"
        );
        if let None = paren {
            return Err(ParseError {
                token: self.peek().clone(),
                message: String::from("expect ')' after arguments")
            })
        }
        Ok(Expr::Call {
            callee: Box::from(callee), paren: paren.unwrap(), arguments
        })
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        if let None = expr {
            return Err(ParseError {
                token: self.peek().clone(),
                message: String::from("unknown error")
            })
        }

        loop {
            if self.match_next(&[TokenType::LeftParen]) {
                expr = Some(self.finish_call(expr.unwrap())?);
            } else if self.match_next(&[TokenType::Dot]) {
                let Some(name) = self.consume(
                    TokenType::Identifier,
                    "expect property name after '.'"
                ) else { break };

                expr = Some(Expr::Get {
                    object: Box::from(expr.take().unwrap()), name
                });
            } else if self.match_next(&[TokenType::LeftBracket]) {
                let bracket = self.previous().clone();
                let index = self.expression()?;
                self.consume(TokenType::RightBracket, "expect ']' after index");
                expr = Some(Expr::Index {
                    object: Box::from(expr.take().unwrap()), bracket, index: Box::from(index)
                });
            } else {
                break;
            }
        }

        match expr {
            Some(e) => Ok(e),
            None => Err(ParseError {
                token: self.peek().clone(),
                message: String::from("unable to call nil")
            })
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_next(
            &[TokenType::Bang, TokenType::Minus]
        ) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {operator, right: Box::new(right)})
        }

        self.call()
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
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

    fn term(&mut self) -> Result<Expr, ParseError> {
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

    fn comparison(&mut self) -> Result<Expr, ParseError> {
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

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::from(expr), operator, right: Box::from(right) };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_next(&[TokenType::And]) {
            let right = self.equality()?;
            expr = Expr::Logical { left: Box::from(expr), operator: TokenType::And, right: Box::from(right) };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_next(&[TokenType::Or]) {
            let right = self.and()?;
            expr = Expr::Logical { left: Box::from(expr), operator: TokenType::Or, right: Box::from(right) };
        }

        Ok(expr)
    }
    
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        
        if self.match_next(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            
            match expr {
                Expr::Variable { name, .. } =>
                    return Ok(Expr::Assign { id: next_expr_id(), name, value: Box::from(value) }),
                Expr::Get { object, name } =>
                    return Ok(Expr::Set { object, name, value: Box::from(value) }),
                Expr::Index { object, index, .. } =>
                    return Ok(Expr::SetIndex { object, index, eq: equals, value: Box::from(value) }),
                _=>{}
            };

            self.errors.push(ParseError {
                token: equals.clone(),
                message: "invalid assignment target".to_string()
            })
        }
        
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.end() {
            if self.previous().kind == TokenType::Semicolon {
                return
            }

            match self.peek().kind {
                TokenType::Class  => { return }
                TokenType::Fun    => { return }
                TokenType::Var    => { return }
                TokenType::For    => { return }
                TokenType::If     => { return }
                TokenType::While  =>  { return }
                TokenType::Print  =>  { return }
                TokenType::Return => { return }
                _=>{}
            }

            self.advance();
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "semicolon expected after value");
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "semicolon expected after value");
        Ok(Stmt::Expression(expr))
    }
    
    fn block(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        
        while !self.check(TokenType::RightBrace) && !self.end() {
            statements.push(self.declaration().unwrap());
        }
        
        self.consume(TokenType::RightBrace, "expected closing right brace for block");
        statements
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "expect '(' after 'if'");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "expect ')' after if condition");

        let then_branch = self.statement()?;
        let mut else_branch: Option<Stmt> = None;
        if self.match_next(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::If {
            condition,
            then_branch: Box::from(then_branch),
            else_branch: Box::from(else_branch)
        })
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        let mut value: Option<Expr> = None;

        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "expect ';' after return value");
        Ok(Stmt::Return {
            keyword: keyword.clone(), value
        })
    }
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_next(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_next(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_next(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_next(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()))
        }
        if self.match_next(&[TokenType::While]) {
            self.consume(TokenType::LeftParen, "expect '(' after 'while'");
            let condition = self.expression()?;
            self.consume(TokenType::RightParen, "expect ')' after condition");
            let body = self.statement()?;

            return Ok(Stmt::While { condition: Some(condition), body: Box::from(body) })
        }
        if self.match_next(&[TokenType::For]) {
            self.consume(TokenType::LeftParen, "expect '(' after 'for'");

            let initializer = if self.match_next(&[TokenType::Semicolon]) {
                None
            } else if self.match_next(&[TokenType::Var]) {
                Some(self.var_declaration()?)
            } else {
                Some(self.expression_statement()?)
            };

            let condition = if !self.check(TokenType::Semicolon) {
                Some(self.expression()?)
            } else { None };

            self.consume(TokenType::Semicolon, "expect ';' after loop condition");

            let increment = if !self.check(TokenType::RightParen) {
                Some(self.expression()?)
            } else { None };

            self.consume(TokenType::RightParen, "expect ')' after for clauses");

            let mut body = self.statement()?;

            if let Some(inc) = increment {
                body = Stmt::Block(vec![body, Stmt::Expression(inc)]);
            }
            
            body = Stmt::While {condition, body: Box::new(body)};
            
            if let Some(init) = initializer {
                body = Stmt::Block(vec![init, body]);
            }
            
            return Ok(body);
        }

        Ok(self.expression_statement()?)
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(
            TokenType::Identifier,
            "expected identifier"
        );

        let initializer: Option<Expr> = if self.match_next(&[TokenType::Equal]) {
             Some(self.expression()?)
        } else { None };

        self.consume(
            TokenType::Semicolon,
            "semicolon expected after variable declaration"
        );
        
        Ok(Stmt::Var { name: name.unwrap(), initializer })
    }

    fn function(&mut self, kind: &'static str) -> Result<Stmt, ParseError> {
        let name = self.consume(
            TokenType::Identifier,
            &format!("expected {} name", kind)
        );
        if let None = name {
            return Err(ParseError {
                token: self.peek().clone(),
                message: format!("expected {} name", kind)
            })
        }
        self.consume(
            TokenType::LeftParen,
            &format!("expected '(' after {} identifier", kind)
        );
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    self.errors.push(ParseError {
                        token: self.peek().clone(),
                        message: String::from("255 parameter limit exceeded")
                    })
                }
                if let Some(t) = self.consume(
                    TokenType::Identifier,
                    "parameter name expected"
                ) {
                    parameters.push(t);
                }
                if !self.match_next(&[TokenType::Comma]) {
                    break
                }
            }
        }
        self.consume(TokenType::RightParen, "expected ')' after parameters");

        self.consume(
            TokenType::LeftBrace,
            &format!("expect '{{' before {} body", kind)
        );
        let body: Vec<Stmt> = self.block();
        let mut body_stmts: Vec<Box<Stmt>> = Vec::new();
        for stmt in body {
            body_stmts.push(Box::from(stmt));
        }
        Ok(Stmt::Function {
            name: name.unwrap(), params: parameters, body: body_stmts
        })
    }

    fn class_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(
            TokenType::Identifier,
            "expect class name"
        ).unwrap();
        let mut superclass: Option<Expr> = None;
        if self.match_next(&[TokenType::Less]) {
            if let Some(superclass_name) = self.consume(
                TokenType::Identifier,
                "expect superclass name"
            ) {
                superclass = Some(Expr::Variable {
                    id: next_expr_id(), name: superclass_name
                });
            }
        }

        self.consume(TokenType::LeftBrace, "expect '{' before class body");

        let mut methods: Vec<Stmt> = Vec::new();
        while (!self.check(TokenType::RightBrace) && !self.end()) {
            methods.push(self.function("method")?);
        }

        self.consume(TokenType::RightBrace, "expect '}' after class body");

        Ok(Stmt::Class {
            name, superclass, methods
        })
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_next(&[TokenType::Var]) {
            return self.var_declaration()
        }
        if self.match_next(&[TokenType::Class]) {
            return self.class_declaration()
        }
        if self.match_next(&[TokenType::Fun]) {
            return self.function("function")
        }
        self.statement()
    }

    pub fn parse(mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(E) =>
                    {
                        self.errors.push(E);
                        return Err(self.errors)
                    }
            }
        }

        if !self.errors.is_empty() {
            return Err(self.errors)
        }
        Ok(statements)
    }
}
