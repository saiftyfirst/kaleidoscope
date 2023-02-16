use crate::lexer::*;
use crate::token::*;

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
                Token::TokEof => break,
                Token::TokDef => {
                    self.handle_def();
                    break;
                },
                Token::TokExtern => {
                    self.handle_extern();
                    break;
                },
                _ => {
                    self.handle_expression();
                    break;
                }
            }
        }
    }

    fn handle_def(&mut self) {}

    fn handle_extern(&mut self) {}

    fn handle_expression(&mut self) {}
}