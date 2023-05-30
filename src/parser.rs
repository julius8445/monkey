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
        let p = Program {
            statements: vec![
                Statement::Let(Let {
                    name: Ident { value: "x".into() },
                    value: Expression::Ident(Ident { value: "x".into() }),
                }),
                Statement::Let(Let {
                    name: Ident { value: "y".into() },
                    value: Expression::Ident(Ident { value: "x".into() }),
                }),
                /*Statement::Return(Return {
                    value: Expression::Ident(Ident { value: "x".into() }),
                }),*/
                Statement::Let(Let {
                    name: Ident {
                        value: "foobar".into(),
                    },
                    value: Expression::Ident(Ident { value: "x".into() }),
                }),
            ],
        };
        None
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
