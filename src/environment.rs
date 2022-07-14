use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::token::{Token, Value};

#[derive(Clone, Debug)]
pub struct Environment {
    enclosing: Option<Arc<Mutex<RefCell<Environment>>>>,
    values: Arc<Mutex<RefCell<HashMap<String, Value>>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            enclosing: enclosing.map(|e| Arc::new(Mutex::new(RefCell::new(e)))),
            values: Arc::new(Mutex::new(RefCell::new(HashMap::new()))),
        }
    }

    pub fn define(&self, name: String, value: Value) {
        let map_lock = self.values.lock().unwrap();
        let mut map = map_lock.borrow_mut();
        map.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Value {
        if let Some(value) = self.values.lock().unwrap().borrow().get(&name.lexeme) {
            value.clone()
        } else if let Some(enclosing) = &self.enclosing {
            let enclosing = enclosing.lock().unwrap();
            let wrapper = enclosing.borrow();
            wrapper.get(name)
        } else {
            panic!("Undefined variable: {}", name.lexeme);
        }
    }

    pub fn assign(&self, name: &Token, value: Value) {
        let map_lock = self.values.lock().unwrap();
        let mut map = map_lock.borrow_mut();
        if map.contains_key(&name.lexeme) {
            if let Some(old_value) = map.get_mut(&name.lexeme) {
                *old_value = value;
                return;
            }
        }

        if let Some(enclosing) = &self.enclosing {
            let enclosing = enclosing.lock().unwrap();
            enclosing.borrow_mut().assign(name, value);
        } else {
            panic!("Undefined variable {}", name.lexeme);
        }
    }
}
