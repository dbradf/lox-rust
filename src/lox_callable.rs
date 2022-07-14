use crate::{environment::Environment, interpreter::Interpreter, stmt::Stmt, token::Value};
use anyhow::Result;
use std::fmt::Debug;

#[derive(Clone)]
pub enum LoxCallable {
    BuiltIn(BuiltInFunction),
    LoxFunction(Box<Stmt>, Environment),
}

#[derive(Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub arity: usize,
    pub func: fn(&Interpreter, &[Value]) -> Value,
}

impl LoxCallable {
    pub fn call(self, interpreter: &Interpreter, arguments: &[Value]) -> Result<Value> {
        match self {
            LoxCallable::BuiltIn(callable) => Ok((callable.func)(interpreter, arguments)),
            LoxCallable::LoxFunction(declaration, closure) => match *declaration {
                Stmt::Function { name, params, body } => {
                    let environment = closure;
                    for (i, argument) in params.iter().enumerate() {
                        environment.define(argument.lexeme.clone(), arguments[i].clone());
                    }

                    let result = interpreter.execute_block(&body, environment)?;
                    Ok(result)
                }
                _ => panic!("Syntax error"),
            },
        }
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Debug for LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuiltIn(arg0) => f.debug_tuple("BuiltIn").finish(),
            Self::LoxFunction(arg0, arg1) => f.debug_tuple("LoxFunction").finish(),
        }
    }
}
