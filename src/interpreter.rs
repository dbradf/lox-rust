use std::{cell::RefCell, rc::Rc};

use crate::{
    built_in::register_builtins,
    environment::{self, Environment},
    expr::Expr,
    lox_callable::LoxCallable,
    stmt::Stmt,
    token::Value,
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

    pub fn get_globals(&self) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(self.globals.clone()))
    }

    pub fn interpret(&self, statements: &[Stmt]) {
        // let environment = Rc::new(RefCell::new(self.globals.clone()));
        let environment = self.get_globals();
        for statement in statements {
            self.visit_statement(statement, environment.clone());
        }
    }

    pub fn execute_block(&self, statements: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) {
        for statement in statements {
            self.visit_statement(statement, environment.clone());
        }
    }

    fn visit_statement(&self, statement: &Stmt, environment: Rc<RefCell<Environment>>) {
        match statement {
            Stmt::Expression { expression } => {
                self.visit_expression(expression, environment);
            }
            Stmt::Print { expression } => {
                let value = self.visit_expression(expression, environment);
                println!("{}", value);
            }
            Stmt::Var { name, initializer } => {
                let mut value = Value::None;
                if let Some(init) = initializer {
                    value = self.visit_expression(init, environment.clone());
                }
                environment.borrow_mut().define(name.lexeme.clone(), value);
            }
            Stmt::Block { statements } => {
                let new_environment = Environment::new(Some(environment));
                self.execute_block(statements, Rc::new(RefCell::new(new_environment)));
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self
                    .visit_expression(condition, environment.clone())
                    .is_truthy()
                {
                    self.visit_statement(then_branch, environment);
                } else if let Some(else_branch) = else_branch {
                    self.visit_statement(else_branch, environment);
                }
            }
            Stmt::While { condition, body } => {
                while self
                    .visit_expression(condition, environment.clone())
                    .is_truthy()
                {
                    self.visit_statement(body, environment.clone());
                }
            }
            Stmt::Function { name, params, body } => {
                let function = LoxCallable::LoxFunction(Box::new(Stmt::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                }));
                environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Value::Callable(function));
            }
        }
    }

    fn visit_expression(&self, expression: &Expr, environment: Rc<RefCell<Environment>>) -> Value {
        match expression {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expression(left, environment.clone());
                let right = self.visit_expression(right, environment);

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
            Expr::Grouping { expression } => self.visit_expression(expression, environment),
            Expr::Literal { value } => value.clone(),
            Expr::Unary { operator, right } => {
                let right = self.visit_expression(right, environment);

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
                let value = self.visit_expression(value, environment.clone());
                environment.borrow_mut().assign(name, value.clone());
                value
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expression(left, environment.clone());

                if operator.token_type == TokenType::Or {
                    if left.is_truthy() {
                        return left;
                    }
                } else if !left.is_truthy() {
                    return left;
                }

                self.visit_expression(right, environment)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.visit_expression(callee, environment.clone());

                let argument_list: Vec<Value> = arguments
                    .iter()
                    .map(|a| self.visit_expression(a, environment.clone()))
                    .collect();

                if let Value::Callable(callable) = callee {
                    return callable.call(self, &argument_list);
                }

                panic!("Syntax error");
            }
        }
    }
}
