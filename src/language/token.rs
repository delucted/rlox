use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use crate::language::callable::Callable;
use crate::language::class::{LoxClass, LoxInstance};
use crate::language::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Rc<RefCell<Vec<Literal>>>), // arrays are shared references.
    Nil,
    Callable(Rc<dyn Callable>),
    Class(Rc<LoxClass>),
    Instance(Rc<RefCell<LoxInstance>>)
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => a == b,
            (Self::Nil, Self::Nil) => true,
            (Self::Callable(a), Self::Callable(b)) => Rc::ptr_eq(a, b),
            (Self::Class(a), Self::Class(b)) => Rc::ptr_eq(a, b),
            (Self::Instance(a), Self::Instance(b)) => Rc::ptr_eq(a, b),
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
            Self::Array(a) => {
                write!(f, "[")?;
                for (i, element) in a.borrow().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{element}")?;
                }
                write!(f, "]")
            },
            Self::Nil => write!(f, "nil"),
            Self::Callable(c) => write!(f, "{c}"),
            Self::Class(c) => write!(f, "{c}"),
            Self::Instance(i) => write!(f, "{}", i.borrow())
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
