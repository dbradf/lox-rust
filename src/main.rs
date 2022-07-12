use std::{fs, io};

use ast_printer::AstPrinter;
use parser::Parser;
use scanner::Scanner;

use crate::interpreter::{interpret, Interpreter};

mod ast_printer;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;

fn main() {
    let mut args = std::env::args();
    if args.len() > 1 {
        eprintln!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args.nth(1).unwrap());
    } else {
        run_prompt();
    }
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
    let mut parser = Parser::new(tokens);

    let statements = parser.parse();
    interpret(&statements);
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}
