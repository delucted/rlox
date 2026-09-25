use std::fmt;
use crate::language::interpreter::Interpreter;
use crate::language::token::{Literal, Token};
use crate::util::errors::RuntimeError;
use std::rc::Rc;
use crate::language::callable::Callable;

#[derive(Debug)]
pub struct Push;

impl Callable for Push {
    fn arity(&self) -> usize {
        2
    }

    fn call(
        self: Rc<Self>,
        _interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
        paren: &Token,
    ) -> Result<Literal, RuntimeError> {
        match arguments[0].clone() {
            Literal::Array(elements) => {
                elements.borrow_mut().push(arguments[1].clone());
                Ok(Literal::Nil)
            },
            Literal::String(mut s) => {
                s.push_str(&arguments[1].to_string());

                Ok(Literal::String(s))
            },
            _ => Err(RuntimeError {
                token: paren.clone(),
                message: String::from("can only append to arrays and strings"),
            })
        }
    }
}

impl fmt::Display for Push {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}
