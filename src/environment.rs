use std::collections::HashMap;

use crate::token::{Token, Value};

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Value {
        if let Some(value) = self.values.get(&name.lexeme) {
            value.clone()
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            panic!("Undefined variable: {}", name.lexeme);
        }
    }

    pub fn assign(&mut self, name: &Token, value: Value) {
        if let Some(old_value) = self.values.get_mut(&name.lexeme) {
            *old_value = value;
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value);
        } else {
            panic!("Undefined variable {}", name.lexeme);
        }
    }
}
