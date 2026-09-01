use crate::language::expr::Expr;
use crate::language::token::Token;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>
    }
}