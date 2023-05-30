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

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut p = Program {
            statements: Vec::new(),
        };

        while !self.is_token(Token::Eof) {
            let stmt = self.parse_statement();
            if let Some(x) = stmt {
                p.statements.push(x);
            }
            self.next_token();
        }

        Some(p)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement().map(|x| Statement::Let(x)),
            Token::Return => self.parse_return_statement().map(|x| Statement::Return(x)),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Let> {
        if !self.expect_peek(Token::Ident("".into())) {
            None
        } else {
            None
        }
    }

    fn parse_return_statement(&mut self) -> Option<Return> {
        None
    }

    fn is_token(&self, tok: Token) -> bool {
        self.peek_token.variant_eq(&tok)
    }

    fn is_peek_token(&self, tok: Token) -> bool {
        self.peek_token.variant_eq(&tok)
    }

    fn expect_peek(&self, tok: Token) -> bool {
        self.peek_token.variant_eq(&tok)
    }

    fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.peek_token, Token::Eof);
        self.peek_token = self.lexer.next().unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Let, Statement},
        lexer::Lexer,
    };

    use super::Parser;

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
        
            let foobar = 838383;
        ";

        let identifiers = vec!["x", "y", "foobar"];

        let mut p = Parser::new(Lexer::new(input.into()));

        let program = p.parse_program().expect("parser should produce valid ast");

        assert_eq!(program.statements.len(), 3);

        for (i, stmt) in program.statements.iter().enumerate() {
            let Statement::Let(s) = stmt else {
                panic!("expected let statement. Found {stmt:?}");
            };
            assert_eq!(s.name.value, identifiers[i]);
        }
    }
}
