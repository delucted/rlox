use std::cell::RefCell;
use crate::language::class::LoxInstance;
use crate::language::interpreter::{ExecSignal, Interpreter};
use crate::language::token::{Literal, Token};
use std::fmt;
use std::rc::Rc;
use crate::language::environment::Environment;
use crate::language::stmt::Stmt;
use crate::util::errors::RuntimeError;

pub trait Callable: fmt::Debug + fmt::Display {
    fn arity(&self) -> usize;

    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
        paren: &Token,
    ) -> Result<Literal, RuntimeError>;
}

#[derive(Debug)]
pub struct LoxFunction {
    pub closure: Rc<RefCell<Environment>>,
    pub declaration: Stmt,
    pub is_initializer: bool,
}

impl LoxFunction {
    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
        let mut environment = Environment::from_parent(Rc::clone(&self.closure));
        environment.define(String::from("this"), Literal::Instance(instance));

        LoxFunction {
            closure: Rc::new(RefCell::new(environment)),
            declaration: self.declaration.clone(),
            is_initializer: self.is_initializer,
        }
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        let Stmt::Function { params, .. } = &self.declaration else {
            unreachable!("LoxFunction must contain a function declaration");
        };

        params.len()
    }

    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
        _paren: &Token,
    ) -> Result<Literal, RuntimeError> {
        let Stmt::Function { params, body, .. } = &self.declaration else {
            unreachable!("LoxFunction must contain a function declaration");
        };

        let mut environment =
            Environment::from_parent(Rc::clone(&self.closure));

        for (param, argument) in params.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), argument);
        }

        let result = interpreter.execute_block(
            body.iter().map(|stmt| stmt.as_ref().clone()).collect(),
            Some(Rc::new(RefCell::new(environment))),
        );

        match result {
            Ok(()) => {
                if self.is_initializer {
                    return Ok(Environment::get_at(&self.closure, 0, "this"))
                }
                Ok(Literal::Nil)
            }
            Err(ExecSignal::Return { value, .. }) => {
                if self.is_initializer {
                    return Ok(Environment::get_at(&self.closure, 0, "this"))
                }
                Ok(value)
            }
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
