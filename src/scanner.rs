use std::{iter::Peekable, str::Chars};

use crate::token::{Pos, Token};

pub(crate) struct Scanner<'a> {
    source: &'a str,
    iter: Peekable<Chars<'a>>,
    start: usize,
    current: usize,
    pos: Pos,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            iter: source.chars().peekable(),
            start: 0,
            current: 0,
            pos: Pos { line: 1, col: 0 },
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            let token = self.next_token();
            tokens.push(token);
        }
        return tokens;
    }

    fn next_token(&mut self) -> Token {
        loop {
            let o = self.advance();
            let c = match o {
                None => return Token::Eof,
                Some(c) => c,
            };
            return match c {
                '(' => Token::LeftParen(self.pos),
                ')' => Token::RightParen(self.pos),
                '{' => Token::LeftBrace(self.pos),
                '}' => Token::RightBrace(self.pos),
                ',' => Token::Comma(self.pos),
                '.' => Token::Dot(self.pos),
                '-' => Token::Minus(self.pos),
                '+' => Token::Plus(self.pos),
                ';' => Token::Semicolon(self.pos),
                '*' => Token::Star(self.pos),
                '!' => {
                    if self.matches('=') {
                        Token::BangEqual(self.pos)
                    } else {
                        Token::Bang(self.pos)
                    }
                }
                '=' => {
                    if self.matches('=') {
                        Token::EqualEqual(self.pos)
                    } else {
                        Token::Equal(self.pos)
                    }
                }
                '<' => {
                    if self.matches('=') {
                        Token::LessEqual(self.pos)
                    } else {
                        Token::Less(self.pos)
                    }
                }
                '>' => {
                    if self.matches('=') {
                        Token::GreaterEqual(self.pos)
                    } else {
                        Token::Greater(self.pos)
                    }
                }
                '/' => {
                    if self.matches('/') {
                        while self.peek() != Some(&'\n') && !self.is_at_end() {
                            self.advance();
                        }
                        continue;
                    } else {
                        Token::Slash(self.pos)
                    }
                }
                ' ' | '\r' | '\t' => {
                    self.advance();
                    self.start += 1;
                    continue;
                },
                '\n' => {
                    self.pos.line += 1;
                    self.pos.col = 0;
                    continue;
                }
                '"' => self.string(),
                '0'..'9' => self.number(),
                // reserved words
                // identifiers
                _ => panic!("Unknown character"),
            };
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some(&'"') && !self.is_at_end() {
            if self.peek() == Some(&'\n') {
                self.pos.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            panic!("{}: Unterminated string.", self.pos.line);
        }

        self.advance();
        let lexeme = &self.source[(self.start + 1)..(self.current - 1)];

        Token::String {
            lexeme: lexeme.to_string(),
            pos: self.pos,
        }
    }

    fn number(&mut self) -> Token {
        self.digits();

        if self.matches('.') {
            self.advance();
            self.digits();
        }

        let lexeme = self.source[self.start..self.current].to_string();
        let val = lexeme.parse::<f64>().unwrap();

        Token::Number {
            lexeme,
            val,
            pos: self.pos,
        }
    }

    fn digits(&mut self) {
        // consume digits
        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn matches(&mut self, ch: char) -> bool {
        match self.iter.peek() {
            None => false,
            Some(c) => {
                if ch == *c {
                    self.advance();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        match self.iter.next() {
            Some(c) => {
                self.pos.col += 1;
                self.current += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}
