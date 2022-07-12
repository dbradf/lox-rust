use crate::{expr::Expr, token::Value, token_type::TokenType};

pub trait Interpreter {
    fn evaluate(&self) -> Value;
}

impl Interpreter for Expr {
    fn evaluate(&self) -> Value {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate();
                let right = right.evaluate();

                match operator.token_type {
                    TokenType::Plus => {
                        if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                            return Value::Number(l - r);
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
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Literal { value } => value.clone(),
            Expr::Unary { operator, right } => {
                let right = right.evaluate();

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
        }
    }
}
