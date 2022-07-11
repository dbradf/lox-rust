use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    None,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Value,
    pub line: usize,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => f.write_str(s),
            Value::Number(n) => f.write_fmt(format_args!("{}", n)),
            Value::None => f.write_str("nil"),
        }
    }
}
