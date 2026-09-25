use std::fmt;
use crate::language::interpreter::Interpreter;
use crate::language::token::{Literal, Token};
use crate::util::errors::RuntimeError;
use std::rc::Rc;
use crate::language::callable::Callable;

#[derive(Debug)]
pub struct Len;

impl Callable for Len {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        self: Rc<Self>,
        _interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
        paren: &Token,
    ) -> Result<Literal, RuntimeError> {
        match &arguments[0] {
            Literal::Array(elements) => Ok(Literal::Number(elements.borrow().len() as f64)),
            Literal::String(s) => Ok(Literal::Number(s.chars().count() as f64)),
            _ => Err(RuntimeError {
                token: paren.clone(),
                message: String::from("unable to get len of this type"),
            })
        }
    }
}

impl fmt::Display for Len {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}
