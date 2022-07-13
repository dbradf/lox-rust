use crate::{interpreter::Interpreter, token::Value};

#[derive(Clone)]
pub enum LoxCallable {
    BuiltIn(BuiltInFunction),
}

#[derive(Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub arity: usize,
    pub func: fn(&Interpreter, &[Value]) -> Value,
}

impl LoxCallable {
    pub fn call(&self, interpreter: &Interpreter, arguments: &[Value]) -> Value {
        match self {
            LoxCallable::BuiltIn(callable) => (callable.func)(interpreter, arguments),
        }
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
