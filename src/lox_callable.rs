use crate::{interpreter::Interpreter, stmt::Stmt, token::Value};

#[derive(Clone)]
pub enum LoxCallable {
    BuiltIn(BuiltInFunction),
    LoxFunction(Box<Stmt>),
}

#[derive(Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub arity: usize,
    pub func: fn(&Interpreter, &[Value]) -> Value,
}

impl LoxCallable {
    pub fn call(self, interpreter: &Interpreter, arguments: &[Value]) -> Value {
        match self {
            LoxCallable::BuiltIn(callable) => (callable.func)(interpreter, arguments),
            LoxCallable::LoxFunction(declaration) => match *declaration {
                Stmt::Function { name, params, body } => {
                    let environment = interpreter.get_globals();
                    for (i, argument) in params.iter().enumerate() {
                        environment
                            .borrow_mut()
                            .define(argument.lexeme.clone(), arguments[i].clone());
                    }

                    interpreter.execute_block(&body, environment);
                    Value::None
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
