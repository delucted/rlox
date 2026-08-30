use crate::language::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Literal },
    Unary { operator: Token, right: Box<Expr> }
}

pub fn print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Binary { left, operator, right } => {
            format!("({} {} {})", print_expr(left), operator.lexeme, print_expr(right))
        }
        Expr::Grouping { expression } => print_expr(expr),
        Expr::Literal { value } => format!("{value:?}"),
        Expr::Unary {
            operator,
            right
        } => format!("({} {})", operator.lexeme, print_expr(right))
    }
}