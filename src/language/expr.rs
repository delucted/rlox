use crate::language::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Literal },
    Unary { operator: Token, right: Box<Expr> },
    Variable { name: Token }
    //Stmt { expression: Box<Expr> },
    //Print { expr: Box<Expr> } // TODO: convert to standard lib function
}