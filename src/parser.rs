use crate::lexer::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(src: &str) -> Parser {
        println!("input src: \n{}", src);
        Parser {
            lexer: Lexer::new(src),
            curr_token: Token::TokEof
        }
    }

    pub fn build_ast(&mut self) {
        self.curr_token = self.lexer.parse_next_token();
        loop {
            match self.curr_token {
                Token::TokEof => { return; },
                _ => {
                    println!("{}", self.curr_token);
                    self.curr_token = self.lexer.parse_next_token();
                }
            }
        }
    }
}