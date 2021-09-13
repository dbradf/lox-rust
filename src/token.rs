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
