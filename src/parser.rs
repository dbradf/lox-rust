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
        if self.do_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
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
        if self.do_match(&[TokenType::Print]) {
            self.print_statement()
        } else if self.do_match(&[TokenType::LeftBrace]) {
            Stmt::Block {
                statements: self.block(),
            }
        } else {
            self.expression_statement()
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
        let expr = self.equality();

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
            self.primary()
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
