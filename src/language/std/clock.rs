use std::fmt;
use crate::language::interpreter::Interpreter;
use crate::language::token::Literal;
use crate::util::errors::RuntimeError;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::language::callable::Callable;

#[derive(Debug)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        self: Rc<Self>,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError> {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before unix epoch")
            .as_secs_f64();

        Ok(Literal::Number(seconds))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}