use anyhow::{bail, Result};
use thiserror::Error;

use crate::{
    built_in::register_builtins, environment::Environment, expr::Expr, lox_callable::LoxCallable,
    stmt::Stmt, token::Value, token_type::TokenType,
};

#[derive(Error, Debug)]
pub enum ReturnError {
    #[error("return value")]
    ReturnValue { value: Value },
}

pub struct Interpreter {
    globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut environment = Environment::new(None);
        register_builtins(&mut environment);

        Self {
            globals: environment,
        }
    }

    pub fn get_globals(&self) -> Environment {
        self.globals.clone()
    }

    pub fn interpret(&self, statements: &[Stmt]) -> Result<()> {
        let environment = self.get_globals();
        for statement in statements {
            self.visit_statement(statement, environment.clone())?;
        }
        Ok(())
    }

    pub fn execute_block(&self, statements: &Vec<Stmt>, environment: Environment) -> Result<Value> {
        for statement in statements {
            let result = self.visit_statement(statement, environment.clone());
            if let Err(err) = result {
                match err.downcast_ref::<ReturnError>() {
                    Some(ReturnError::ReturnValue { value }) => return Ok(value.clone()),
                    None => (),
                }
                return Err(err);
            }
        }
        Ok(Value::None)
    }

    fn visit_statement(&self, statement: &Stmt, environment: Environment) -> Result<()> {
        match statement {
            Stmt::Expression { expression } => {
                self.visit_expression(expression, environment)?;
                Ok(())
            }
            Stmt::Print { expression } => {
                let value = self.visit_expression(expression, environment)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let mut value = Value::None;
                if let Some(init) = initializer {
                    value = self.visit_expression(init, environment.clone())?;
                }
                environment.define(name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Block { statements } => {
                let new_environment = Environment::new(Some(environment));
                self.execute_block(statements, new_environment)?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self
                    .visit_expression(condition, environment.clone())?
                    .is_truthy()
                {
                    self.visit_statement(then_branch, environment)?;
                } else if let Some(else_branch) = else_branch {
                    self.visit_statement(else_branch, environment)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self
                    .visit_expression(condition, environment.clone())?
                    .is_truthy()
                {
                    self.visit_statement(body, environment.clone())?;
                }
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let function = LoxCallable::LoxFunction(
                    Box::new(Stmt::Function {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                    }),
                    environment.clone(),
                );
                environment.define(name.lexeme.clone(), Value::Callable(function));
                Ok(())
            }
            Stmt::Return { keyword, value } => {
                let mut return_value = Value::None;
                if let Some(value) = value {
                    return_value = self.visit_expression(value, environment)?;
                }
                bail!(ReturnError::ReturnValue {
                    value: return_value
                })
            }
        }
    }

    fn visit_expression(&self, expression: &Expr, environment: Environment) -> Result<Value> {
        match expression {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expression(left, environment.clone())?;
                let right = self.visit_expression(right, environment)?;

                match operator.token_type {
                    TokenType::Plus => {
                        if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                            return Ok(Value::Number(l + r));
                        }

                        if let (Value::String(l), Value::String(r)) = (left, right) {
                            return Ok(Value::String(format!("{}{}", l, r)));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Minus => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Ok(Value::Number(l - r));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Slash => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Ok(Value::Number(l / r));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Star => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Ok(Value::Number(l * r));
                        }
                        panic!("Invalid syntax");
                    }

                    TokenType::Greater => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Ok(Value::from_bool(l > r));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::GreaterEqual => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Ok(Value::from_bool(l >= r));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Less => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Ok(Value::from_bool(l < r));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::LessEqual => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Ok(Value::from_bool(l <= r));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::BangEqual => Ok(Value::from_bool(!left.is_equal(&right))),
                    TokenType::EqualEqual => Ok(Value::from_bool(left.is_equal(&right))),
                    _ => panic!("Invalid syntax"),
                }
            }
            Expr::Grouping { expression } => self.visit_expression(expression, environment),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Unary { operator, right } => {
                let right = self.visit_expression(right, environment)?;

                match operator.token_type {
                    TokenType::Bang => Ok(Value::from_bool(!right.is_truthy())),
                    TokenType::Minus => {
                        if let Value::Number(value) = right {
                            return Ok(Value::Number(-value));
                        }
                        panic!("Invalid syntax");
                    }
                    _ => panic!("Invalid syntax"),
                }
            }
            Expr::Variable { name } => Ok(environment.get(name)),
            Expr::Assign { name, value } => {
                let value = self.visit_expression(value, environment.clone())?;
                environment.assign(name, value.clone());
                Ok(value)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expression(left, environment.clone())?;

                if operator.token_type == TokenType::Or {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                } else if !left.is_truthy() {
                    return Ok(left);
                }

                self.visit_expression(right, environment)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.visit_expression(callee, environment.clone())?;

                let argument_list: Result<Vec<Value>> = arguments
                    .iter()
                    .map(|a| self.visit_expression(a, environment.clone()))
                    .collect();

                if let Value::Callable(callable) = callee {
                    return callable.call(self, &argument_list?);
                }

                panic!("Syntax error");
            }
        }
    }
}
