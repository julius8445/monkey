use std::mem;

use crate::ast::*;
use crate::lexer::*;

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

#[cfg(test)]
mod test {

    #[test]
    fn test_let_statements() {
        let _input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let _identifiers = vec!["x", "y", "foobar"];
    }
}
