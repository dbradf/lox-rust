use crate::{
    environment::Environment, expr::Expr, stmt::Stmt, token::Value, token_type::TokenType,
};

pub trait Interpreter {
    fn evaluate(&self, environment: &mut Environment) -> Value;
}

pub fn interpret(statements: &[Stmt]) {
    let mut environment = Environment::new(None);
    for statement in statements {
        statement.evaluate(&mut environment);
    }
}

fn execute_block(statements: &Vec<Stmt>, environment: &mut Environment) {
    for statement in statements {
        statement.evaluate(environment);
    }
}

impl Interpreter for Stmt {
    fn evaluate(&self, environment: &mut Environment) -> Value {
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
                    value = init.evaluate(environment);
                }
                environment.define(name.lexeme.clone(), value);
                Value::None
            }
            Stmt::Block { statements } => {
                let mut new_environment = Environment::new(Some(Box::new(environment.clone())));
                execute_block(statements, &mut new_environment);
                Value::None
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(environment).is_truthy() {
                    then_branch.evaluate(environment);
                } else if let Some(else_branch) = else_branch {
                    else_branch.evaluate(environment);
                }
                Value::None
            }
        }
    }
}

impl Interpreter for Expr {
    fn evaluate(&self, environment: &mut Environment) -> Value {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment);
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
            Expr::Variable { name } => environment.get(name),
            Expr::Assign { name, value } => {
                let value = value.evaluate(environment);
                environment.assign(name, value.clone());
                value
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(environment);

                if operator.token_type == TokenType::Or {
                    if left.is_truthy() {
                        return left;
                    }
                } else if !left.is_truthy() {
                    return left;
                }

                right.evaluate(environment)
            }
        }
    }
}
