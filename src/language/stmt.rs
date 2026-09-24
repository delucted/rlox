use crate::language::expr::Expr;
use crate::language::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>
    },
    Block(Vec<Stmt>),
    Class { name: Token, superclass: Option<Expr>, methods: Vec<Stmt> },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>
    },
    While { condition: Option<Expr>, body: Box<Stmt> },
    Return { keyword: Token, value: Option<Expr> }
}