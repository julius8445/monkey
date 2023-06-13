use monkey::lexer::{Lexer, Token};

use std::io::{self, Write};

const PROMPT_STRING: &str = ">>> ";

const WELCOME_TEXT: &str = "\
Welcome user! This is the monkey programming language!!\n\
Feel free to type in commands\
";

fn prompt() -> String {
    let mut input = String::new();
    
    print!("{}", PROMPT_STRING);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn main() -> io::Result<()> {
    println!("{}", WELCOME_TEXT);
    loop {
        let input = prompt();
        
        match input.as_str() {
            ":exit" => break,
            _ => {
                let l = Lexer::new(input);
                for token in l {
                    if token == Token::Eof {
                        break;
                    }                    
                    println!("Token: {token:?}");
                }
            }
        }
    }

    Ok(())
}
