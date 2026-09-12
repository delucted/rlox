use std::fmt;
use std::rc::Rc;
use crate::language::callable::Callable;
use crate::language::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    // A runtime-only variant: the lexer never produces one of these, but the
    // interpreter needs somewhere to put a function value. `Rc` because the same
    // function can be referenced from many environments at once, and `dyn` so
    // native functions and (later) user-declared ones share one type.
    Callable(Rc<dyn Callable>)
}

// `Rc<dyn Callable>` can't be compared structurally, so `PartialEq` can no longer
// be derived. Lox compares functions by identity, which is what `ptr_eq` gives us.
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Nil, Self::Nil) => true,
            (Self::Callable(a), Self::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
            Self::Callable(c) => write!(f, "{c}")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32
}
