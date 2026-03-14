use core::panic;

use crate::token::Token;

enum Stmt {}

#[derive(Debug)]
pub enum Expr {
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Expr {

}

trait Visitor<T> {
    fn visitUnaryExpr(expr: Expr) -> T;
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while matches!(self.peek(), Token::BangEqual(_) | Token::EqualEqual(_)) {
            let operator = self.advance();
            let right = self.comparison();
            expr = Expr::Binary {
                operator,
                left: Box::new(expr), 
                right: Box::new(right),
            }
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while matches!(self.peek(), Token::Greater(_) | Token::GreaterEqual(_) | Token::Less(_) | Token::LessEqual(_)) {
            let operator = self.advance();
            let right = self.term();
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right)
            };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while matches!(self.peek(), Token::Minus(_) | Token::Plus(_)) {
            let operator = self.advance();
            let right = self.factor();
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right)
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while matches!(self.peek(), Token::Slash(_) | Token::Star(_)) {
            let operator = self.advance();
            let right = self.unary();
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right)
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if matches!(self.peek(), Token::Bang(_) | Token::Minus(_)) {
            let operator = self.advance();
            let right = self.unary();
            Expr::Unary {
                operator,
                right: Box::new(right)
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        match self.advance() {
            Token::False(_) => Expr::Bool(false),
            Token::True(_) => Expr::Bool(true),
            Token::Nil(_) => Expr::Nil,
            Token::Number { val, .. } => Expr::Num(val),
            Token::String { lexeme, .. } => Expr::Str(lexeme),
            Token::LeftParen(_) => self.grouping(),
            _ => panic!("invalid token"),
        }
    }

    fn grouping(&mut self) -> Expr {
        let expr = self.expression();
        self.consume_if(|t| matches!(t, Token::RightParen(_)), "Expect ')' after expression.");
        Expr::Grouping(Box::new(expr))
    }

    fn consume_if<F>(&mut self, cond: F, message: &str) 
    where F: FnOnce(&Token) -> bool
    {
        if cond(&self.peek()) {
            self.advance();
            return;
        }
        panic!("{message}");
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == Token::Eof
    }
}
