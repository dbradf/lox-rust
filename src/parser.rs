use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Token, Value},
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        if self.do_match(&[TokenType::Fun]) {
            self.function("function")
        } else if self.do_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn function(&mut self, kind: &str) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect function name");
        self.consume(TokenType::LeftParen, "Expect '(' after function name.");
        let mut parameters = vec![];
        loop {
            if parameters.len() >= 255 {
                eprintln!("Can't have more than 255 parameters.");
            }
            parameters.push(self.consume(TokenType::Identifier, "Expect parameter name."));
            if !self.do_match(&[TokenType::Comma]) {
                break;
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.");
        self.consume(TokenType::LeftBrace, "Expect '{' before function body.");
        let body = self.block();
        Stmt::Function {
            name,
            params: parameters,
            body,
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect variable name.");
        let mut initializer = None;
        if self.do_match(&[TokenType::Equal]) {
            initializer = Some(self.expression());
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        Stmt::Var { name, initializer }
    }

    fn statement(&mut self) -> Stmt {
        if self.do_match(&[TokenType::For]) {
            self.for_statement()
        } else if self.do_match(&[TokenType::If]) {
            self.if_statement()
        } else if self.do_match(&[TokenType::Print]) {
            self.print_statement()
        } else if self.do_match(&[TokenType::Return]) {
            self.return_statement()
        } else if self.do_match(&[TokenType::While]) {
            self.while_statement()
        } else if self.do_match(&[TokenType::LeftBrace]) {
            Stmt::Block {
                statements: self.block(),
            }
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous();
        let mut value = None;
        if !self.check(&TokenType::Semicolon) {
            value = Some(self.expression());
        }
        self.consume(TokenType::Semicolon, "Expect ';' after return value.");
        Stmt::Return { keyword, value }
    }

    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        let mut initializer = None;
        if self.do_match(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self.do_match(&[TokenType::Var]) {
            initializer = Some(self.var_declaration());
        } else {
            initializer = Some(self.expression_statement());
        }

        let mut condition = Expr::Literal {
            value: Value::from_bool(true),
        };
        if !self.do_match(&[TokenType::Semicolon]) {
            condition = self.expression();
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        let mut increment = None;
        if !self.do_match(&[TokenType::RightParen]) {
            increment = Some(self.expression());
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.");

        let mut body = self.statement();
        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expression: increment,
                    },
                ],
            };
        }

        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body],
            };
        }

        body
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");
        let body = self.statement();

        Stmt::While {
            condition,
            body: Box::new(body),
        }
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after 'if' condition.");

        let then_branch = self.statement();
        let mut else_branch = None;
        if self.do_match(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()));
        }

        Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        }
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        statements
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print { expression: value }
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Stmt::Expression { expression: expr }
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.do_match(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment();

            if let Expr::Variable { name } = expr {
                return Expr::Assign {
                    name,
                    value: Box::new(value),
                };
            }

            panic!("Invalid assignment target.");
        }

        expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.do_match(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.do_match(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality();
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.do_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.do_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.do_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.do_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.do_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            Expr::Unary {
                operator,
                right: Box::new(right),
            }
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            if self.do_match(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut arguments = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    eprintln!("Can't have more than 255 arguments");
                }
                arguments.push(self.expression());
                if !self.do_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.");

        Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    fn primary(&mut self) -> Expr {
        if self.do_match(&[TokenType::False]) {
            Expr::Literal {
                value: Value::False,
            }
        } else if self.do_match(&[TokenType::True]) {
            Expr::Literal { value: Value::True }
        } else if self.do_match(&[TokenType::Nil]) {
            Expr::Literal { value: Value::None }
        } else if self.do_match(&[TokenType::Number, TokenType::String]) {
            Expr::Literal {
                value: self.previous().literal,
            }
        } else if self.do_match(&[TokenType::Identifier]) {
            Expr::Variable {
                name: self.previous(),
            }
        } else if self.do_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            Expr::Grouping {
                expression: Box::new(expr),
            }
        } else {
            panic!("parsing failure");
        }
    }

    fn do_match(&mut self, types: &[TokenType]) -> bool {
        for tt in types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tt: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *tt
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, tt: TokenType, msg: &str) -> Token {
        if self.check(&tt) {
            self.advance()
        } else {
            panic!("{}", msg.to_string());
        }
    }
}
