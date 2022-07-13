use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    environment::Environment,
    interpreter::Interpreter,
    lox_callable::{BuiltInFunction, LoxCallable},
    token::Value,
};

pub fn register_builtins(environment: &mut Environment) {
    environment.define(
        "clock".to_string(),
        Value::Callable(LoxCallable::BuiltIn(BuiltInFunction {
            name: "clock".to_string(),
            arity: 0,
            func: clock,
        })),
    );
}

// fn clock(_: &Interpreter, _: &[Value]) -> Value {
fn clock() -> Value {
    Value::Number(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64,
    )
}
