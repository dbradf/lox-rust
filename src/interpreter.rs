use std::{cell::RefCell, rc::Rc};

use crate::{
    built_in::register_builtins, environment::Environment, expr::Expr, stmt::Stmt, token::Value,
    token_type::TokenType,
};

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

    pub fn interpret(&self, statements: &[Stmt]) {
        let environment = Rc::new(RefCell::new(self.globals.clone()));
        for statement in statements {
            statement.evaluate(environment.clone());
        }
    }
}

trait Interpretable {
    fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Value;
}

// pub fn interpret(statements: &[Stmt]) {
//     let environment = Rc::new(RefCell::new(Environment::new(None)));
//     for statement in statements {
//         statement.evaluate(environment.clone());
//     }
// }

fn execute_block(statements: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) {
    for statement in statements {
        statement.evaluate(environment.clone());
    }
}

impl Interpretable for Stmt {
    fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Value {
        match self {
            Stmt::Expression { expression } => {
                expression.evaluate(environment);
                Value::None
            }
            Stmt::Print { expression } => {
                let value = expression.evaluate(environment);
                println!("{}", value);
                Value::None
            }
            Stmt::Var { name, initializer } => {
                let mut value = Value::None;
                if let Some(init) = initializer {
                    value = init.evaluate(environment.clone());
                }
                environment.borrow_mut().define(name.lexeme.clone(), value);
                Value::None
            }
            Stmt::Block { statements } => {
                let new_environment = Environment::new(Some(environment));
                execute_block(statements, Rc::new(RefCell::new(new_environment)));
                Value::None
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(environment.clone()).is_truthy() {
                    then_branch.evaluate(environment);
                } else if let Some(else_branch) = else_branch {
                    else_branch.evaluate(environment);
                }
                Value::None
            }
            Stmt::While { condition, body } => {
                while condition.evaluate(environment.clone()).is_truthy() {
                    body.evaluate(environment.clone());
                }
                Value::None
            }
        }
    }
}

impl Interpretable for Expr {
    fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Value {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment.clone());
                let right = right.evaluate(environment);

                match operator.token_type {
                    TokenType::Plus => {
                        if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                            return Value::Number(l + r);
                        }

                        if let (Value::String(l), Value::String(r)) = (left, right) {
                            return Value::String(format!("{}{}", l, r));
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Minus => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Value::Number(l - r);
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Slash => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Value::Number(l / r);
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Star => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Value::Number(l * r);
                        }
                        panic!("Invalid syntax");
                    }

                    TokenType::Greater => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Value::from_bool(l > r);
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::GreaterEqual => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Value::from_bool(l >= r);
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::Less => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Value::from_bool(l < r);
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::LessEqual => {
                        if let (Value::Number(l), Value::Number(r)) = (left, right) {
                            return Value::from_bool(l <= r);
                        }
                        panic!("Invalid syntax");
                    }
                    TokenType::BangEqual => Value::from_bool(!left.is_equal(&right)),
                    TokenType::EqualEqual => Value::from_bool(left.is_equal(&right)),
                    _ => panic!("Invalid syntax"),
                }
            }
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Literal { value } => value.clone(),
            Expr::Unary { operator, right } => {
                let right = right.evaluate(environment);

                match operator.token_type {
                    TokenType::Bang => Value::from_bool(!right.is_truthy()),
                    TokenType::Minus => {
                        if let Value::Number(value) = right {
                            return Value::Number(-value);
                        }
                        panic!("Invalid syntax");
                    }
                    _ => panic!("Invalid syntax"),
                }
            }
            Expr::Variable { name } => environment.borrow().get(name),
            Expr::Assign { name, value } => {
                let value = value.evaluate(environment.clone());
                environment.borrow_mut().assign(name, value.clone());
                value
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment.clone());

                if operator.token_type == TokenType::Or {
                    if left.is_truthy() {
                        return left;
                    }
                } else if !left.is_truthy() {
                    return left;
                }

                right.evaluate(environment)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = callee.evaluate(environment.clone());

                let argument_list: Vec<Value> = arguments
                    .iter()
                    .map(|a| a.evaluate(environment.clone()))
                    .collect();

                if let Value::Callable(callable) = callee {
                    // return callable.call(interpreter, &argument_list)
                    return callable.call();
                }

                panic!("Syntax error");
            }
        }
    }
}
