use crate::lexer::{Lexer, Token};

use std::mem;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    peek_token: Token,
    current_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            lexer: l,
            peek_token: Token::Eof,
            current_token: Token::Eof,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.peek_token, Token::Eof);
        self.peek_token = self.lexer.next().unwrap();
    }
}
