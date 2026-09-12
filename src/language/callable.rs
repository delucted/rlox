use crate::language::interpreter::Interpreter;
use crate::language::token::Literal;
use std::fmt;
use crate::util::errors::RuntimeError;

pub trait Callable: fmt::Debug + fmt::Display {
    fn arity(&self) -> usize;
    
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError>;
}