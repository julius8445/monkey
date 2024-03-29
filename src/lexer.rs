#[derive(Hash, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal(String),
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn variant_eq(&self, tok: &Token) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(tok)
    }

    pub fn kind(&self) -> TokenKind {
        match self {
            Token::Illegal(_) => TokenKind::Illegal,
            Token::Eof => TokenKind::Eof,
            Token::Ident(_) => TokenKind::Ident,
            Token::Int(_) => TokenKind::Int,
            Token::Assign => TokenKind::Assign,
            Token::Plus => TokenKind::Plus,
            Token::Minus => TokenKind::Minus,
            Token::Bang => TokenKind::Bang,
            Token::Asterisk => TokenKind::Asterisk,
            Token::Slash => TokenKind::Slash,
            Token::Lt => TokenKind::Lt,
            Token::Gt => TokenKind::Gt,
            Token::Eq => TokenKind::Eq,
            Token::NotEq => TokenKind::NotEq,
            Token::Comma => TokenKind::Comma,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Lparen => TokenKind::Lparen,
            Token::Rparen => TokenKind::Rparen,
            Token::Lbrace => TokenKind::Lbrace,
            Token::Rbrace => TokenKind::Rbrace,
            Token::Function => TokenKind::Function,
            Token::Let => TokenKind::Let,
            Token::True => TokenKind::True,
            Token::False => TokenKind::False,
            Token::If => TokenKind::If,
            Token::Else => TokenKind::Else,
            Token::Return => TokenKind::Return,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let mut result = String::new();
        match self {
            Token::Illegal(s) => result.push_str(s),
            Token::Eof => result.push_str("\0"),
            Token::Ident(s) => result.push_str(s),
            Token::Int(s) => result.push_str(s),
            Token::Assign => result.push_str("="),
            Token::Plus => result.push_str("+"),
            Token::Minus => result.push_str("-"),
            Token::Bang => result.push_str("!"),
            Token::Asterisk => result.push_str("*"),
            Token::Slash => result.push_str("/"),
            Token::Lt => result.push_str("<"),
            Token::Gt => result.push_str(">"),
            Token::Eq => result.push_str("=="),
            Token::NotEq => result.push_str("!="),
            Token::Comma => result.push_str(","),
            Token::Semicolon => result.push_str(";"),
            Token::Lparen => result.push_str("("),
            Token::Rparen => result.push_str(")"),
            Token::Lbrace => result.push_str("{"),
            Token::Rbrace => result.push_str("}"),
            Token::Function => result.push_str("fn"),
            Token::Let => result.push_str("let"),
            Token::True => result.push_str("true"),
            Token::False => result.push_str("false"),
            Token::If => result.push_str("if"),
            Token::Else => result.push_str("else"),
            Token::Return => result.push_str("return"),
        }
        result
    }
}

#[derive(Debug)]
pub struct Lexer {
    ch: u8,
    input: Vec<u8>,
    position: usize,
    read_position: usize,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return Some(match ident.as_str() {
                    "if" => Token::If,
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "else" => Token::Else,
                    "true" => Token::True,
                    "false" => Token::False,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                });
            }
            b'0'..=b'9' => return Some(Token::Int(self.read_integer())),
            0 => Token::Eof,
            _ => {
                let s = String::from_utf8_lossy(&[self.ch]).to_string();
                Token::Illegal(s)
            }
        };
        self.read_char();
        Some(tok)
    }
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            ch: 0,
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
        };
        lex.read_char();
        lex
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let current_position = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[current_position..self.position]).to_string()
    }

    fn read_integer(&mut self) -> String {
        let current_position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[current_position..self.position]).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn get_next_token() {
        let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            }; 

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ";

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::Gt,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Int("10".into()),
            Token::Eq,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("10".into()),
            Token::NotEq,
            Token::Int("9".into()),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = Lexer::new(input.into());

        for token in tokens {
            let lex_token = l.next().unwrap();
            println!("expected: {token:?} received: {lex_token:?}");
            assert_eq!(token, lex_token);
        }
    }
}
