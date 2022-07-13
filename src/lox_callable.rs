use crate::{interpreter::Interpreter, token::Value};

#[derive(Clone)]
pub enum LoxCallable {
    BuiltIn(BuiltInFunction),
}

#[derive(Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub arity: usize,
    // func: fn(&Interpreter, &[Value]) -> Value,
    pub func: fn() -> Value,
}

impl LoxCallable {
    // pub fn call(&self, interpreter: &Interpreter, arguments: &[Value]) -> Value {
    pub fn call(&self) -> Value {
        match self {
            LoxCallable::BuiltIn(callable) => (callable.func)(),
        }
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
