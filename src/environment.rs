use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::token::{Token, Value};

#[derive(Clone, Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
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
            enclosing.borrow().get(name)
        } else {
            panic!("Undefined variable: {}", name.lexeme);
        }
    }

    pub fn assign(&mut self, name: &Token, value: Value) {
        if self.values.contains_key(&name.lexeme) {
            if let Some(old_value) = self.values.get_mut(&name.lexeme) {
                *old_value = value;
                return;
            }
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value);
        } else {
            panic!("Undefined variable {}", name.lexeme);
        }
    }
}
