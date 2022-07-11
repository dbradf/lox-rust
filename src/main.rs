use std::{fs, io};

use ast_printer::AstPrinter;
use expr::Expr;
use scanner::Scanner;
use token::{Token, Value};
use token_type::TokenType;

mod ast_printer;
mod expr;
mod scanner;
mod token;
mod token_type;

// fn main() {
//     let mut args = std::env::args();
//     if args.len() > 1 {
//         eprintln!("Usage: lox [script]");
//         std::process::exit(64);
//     } else if args.len() == 2 {
//         run_file(&args.nth(1).unwrap());
//     } else {
//         run_prompt();
//     }
// }

fn main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: Value::None,
                line: 1,
            },
            right: Box::new(Expr::Literal {
                value: Value::Number(123.0),
            }),
        }),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: Value::None,
            line: 1,
        },
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Value::Number(45.67),
            }),
        }),
    };

    println!("{}", expression.accept());
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    run(&contents);
}

fn run_prompt() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        run(&buffer);
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}
