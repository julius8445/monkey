use crate::{
    ast::Program,
    lexer::{Lexer, Token},
};

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

    pub fn parse_program(&mut self) -> Option<Program> {
        None
    }

    fn next_token(&mut self) {
        self.current_token = mem::replace(&mut self.peek_token, Token::Eof);
        self.peek_token = self.lexer.next().unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Statement;

    use super::{Lexer, Parser};

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let identifiers = vec!["x", "y", "foobar"];

        let l = Lexer::new(input.into());
        let mut p = Parser::new(l);

        let program = p.parse_program().unwrap();

        assert_eq!(program.statements.len(), 3);

        for (index, stmt) in program.statements.iter().enumerate() {
            if let Statement::Let(x) = stmt {}
        }
    }

    fn test_let_statement() {}
}
