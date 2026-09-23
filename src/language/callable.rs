use std::cell::RefCell;
use crate::language::interpreter::{ExecSignal, Interpreter};
use crate::language::token::Literal;
use std::fmt;
use std::rc::Rc;
use crate::language::environment::Environment;
use crate::language::stmt::Stmt;
use crate::util::errors::RuntimeError;

pub trait Callable: fmt::Debug + fmt::Display {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError>;
}

#[derive(Debug)]
pub struct LoxFunction {
    pub closure: Rc<RefCell<Environment>>,
    pub declaration: Stmt,
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        let Stmt::Function { params, .. } = &self.declaration else {
            unreachable!("LoxFunction must contain a function declaration");
        };

        params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError> {
        let Stmt::Function { params, body, .. } = &self.declaration else {
            unreachable!("LoxFunction must contain a function declaration");
        };

        let mut environment =
            Environment::from_parent(Rc::clone(&self.closure));

        for (param, argument) in params.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), argument);
        }

        match interpreter.execute_block(
            body.iter().map(|stmt| stmt.as_ref().clone()).collect(),
            Some(Rc::new(RefCell::new(environment))),
        ) {
            Ok(()) => Ok(Literal::Nil),
            // A Return signal from this body is the function's result, not an error.
            Err(ExecSignal::Return { value, .. }) => Ok(value),
            Err(ExecSignal::Runtime(error)) => Err(error),
        }
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Stmt::Function { name, .. } = &self.declaration else {
            unreachable!("LoxFunction must contain a function declaration");
        };

        write!(f, "<fn {}>", name.lexeme)
    }
}