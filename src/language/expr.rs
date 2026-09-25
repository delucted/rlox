use crate::language::token::{Literal, Token};
use crate::language::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Literal },
    Array { elements: Vec<Box<Expr>> },
    Unary { operator: Token, right: Box<Expr> },
    Variable { id: usize, name: Token },
    Assign { id: usize, name: Token, value: Box<Expr> },
    Logical { left: Box<Expr>, operator: TokenType, right: Box<Expr> },
    Call { callee: Box<Expr>, paren: Token, arguments: Vec<Box<Expr>> },
    Index { object: Box<Expr>, bracket: Token, index: Box<Expr> },
    Get { object: Box<Expr>, name: Token },
    Set { object: Box<Expr>, name: Token, value: Box<Expr> },
    SetIndex { object: Box<Expr>, index: Box<Expr>, eq: Token, value: Box<Expr> },
    This { id: usize, keyword: Token },
    Super { id: usize, keyword: Token, method: Token }
}
