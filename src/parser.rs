use std::collections::HashMap;
use std::mem;

use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("expected next token to be {0:?}, got {1:?} instead")]
    UnexpectedToken(TokenKind, Token),
}

type InfixParser = fn() -> Expression;
type PrefixParser = fn(Expression) -> Expression;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    peek_token: Token,
    current_token: Token,
    infix_parsers: HashMap<TokenKind, InfixParser>,
    prefix_parsers: HashMap<TokenKind, PrefixParser>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            lexer: l,
            peek_token: Token::Eof,
            current_token: Token::Eof,
            infix_parsers: HashMap::new(),
            prefix_parsers: HashMap::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut p = Program {
            statements: Vec::new(),
        };

        while !self.is_token(TokenKind::Eof) {
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
        if !self.expect(TokenKind::Ident) {
            return None;
        }

        let name = if let Token::Ident(s) = &self.current_token {
            Ident { value: s.clone() }
        } else {
            return None;
        };

        if !self.expect(TokenKind::Assign) {
            return None;
        }

        let value = Expression::Ident(Ident { value: " ".into() });

        while !self.is_token(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(Let { name, value })
    }

    fn parse_return_statement(&mut self) -> Option<Return> {
        self.next_token();

        let value = Expression::Ident(Ident { value: " ".into() });

        while !self.is_token(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(Return { value })
    }

    fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.peek_token, Token::Eof);
        self.peek_token = self.lexer.next().unwrap();
    }

    fn is_token(&self, tok: TokenKind) -> bool {
        self.current_token.kind() == tok
    }

    fn is_peek_token(&self, tok: TokenKind) -> bool {
        self.peek_token.kind() == tok
    }

    fn expect(&mut self, tok: TokenKind) -> bool {
        if self.is_peek_token(tok) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn register_infix(&mut self, tok: TokenKind, p: InfixParser) {
        self.infix_parsers.insert(tok, p);
    }

    fn register_prefix(&mut self, tok: TokenKind, p: PrefixParser) {
        self.prefix_parsers.insert(tok, p);
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::ast::Statement;
    use crate::lexer::Lexer;

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
            println!("{stmt:?}");
            let Statement::Let(s) = stmt else {
                panic!("expected let statement. Found {stmt:?}");
            };
            assert_eq!(s.name.value, identifiers[i]);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";

        let mut p = Parser::new(Lexer::new(input.into()));
        let program = p.parse_program().expect("parser should produce valid ast");

        assert_eq!(program.statements.len(), 3);

        for stmt in program.statements.iter() {
            println!("{stmt:?}");
            let Statement::Return(_) = stmt else {
                panic!("expected return statement. Found {stmt:?}");
            };
        }
    }
}
