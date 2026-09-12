use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Literal },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token },
    Assign { name: Token, value: Box<Expr> },
    Logical { left: Box<Expr>, operator: TokenType, right: Box<Expr> },
    Call { callee: Box<Expr>, paren: Token, arguments: Vec<Box<Expr>> }
}
