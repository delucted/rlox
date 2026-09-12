use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
// TODO: transfer to FxHasher for higher performance
use crate::language::token::{Literal, Token};
use crate::util::errors::RuntimeError;

#[derive(Debug, Default, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(parent),
        }
    }

    pub fn into_enclosing(self) -> Option<Rc<RefCell<Environment>>> {
        self.enclosing
    }

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        if let Some(literal) = self.values.get(&name.lexeme) {
            return Ok(literal.clone());
        }

        match &self.enclosing {
            Some(parent) => parent.borrow().get(name),
            None => Err(RuntimeError {
                token: name.clone(),
                message: format!("undefined variable \"{}\"", name.lexeme),
            }),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value); // can define already defined variables
    }

    pub fn assign(&mut self, name: Token, value: &Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.clone());
            return Ok(());
        }

        match &self.enclosing {
            Some(parent) => parent.borrow_mut().assign(name, value),
            None => Err(RuntimeError {
                message: format!("undefined variable \"{}\"", name.lexeme),
                token: name,
            }),
        }
    }
}
