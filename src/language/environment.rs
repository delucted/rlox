use std::collections::HashMap; // TODO: transfer to FxHasher for higher performance
use crate::language::token::{Literal, Token};
use crate::util::errors::RuntimeError;

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Literal>
}

impl Environment {
    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => {
                Err(RuntimeError {
                    token: name.clone(),
                    message: "undefined literal".to_string()
                })
            }
        }
    }
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value); // can define already defined variables
    }
}