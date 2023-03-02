use std::fs::read;
use std::io::Result;

use crate::token::*;

fn read_while<F>(data: &str, pred: F) -> Result<(&str, usize)>
    where F: Fn(char) -> bool {
    let mut read_count = 0;
    for elem in data.chars() {
        if !pred(elem) {
            break;
        }
        read_count += 1;
    }
    Ok((&data[..read_count], read_count))
}

pub struct Lexer<'a> {
    current_index: usize,
    current_data: &'a str,
    current_token: Token
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        let mut lexer = Lexer {
            current_index: 0,
            current_data: src,
            current_token: Token::TokEof
        };
        lexer.init();
        lexer
    }

    fn init(&mut self) {
        self.current_token = self.parse_token();
    }

    pub fn peek(&mut self) -> &Token {
        &self.current_token
    }

    pub fn pop(&mut self) -> Token {
        let popped = self.current_token.clone();
        self.current_token = self.parse_token();
        popped
    }

    fn parse_token(&mut self) -> Token {
        self.trim_start();

        if self.current_data.len() ==  0 {
            return Token::TokEof;
        }

        let first_char = self.current_data.chars().nth(0).unwrap();
        match first_char {
            'a'..='z' | 'A'..='Z' => {
                Token::from(self.read_token_str(false))
            }
            '0'..='9' => {
                let value = self.read_token_str(false).parse::<f64>().unwrap();
                Token::from(value)
            }
            '#' => {
                Token::from(self.read_token_str(true))
            }
            _ => {
                Token::from(self.read_primary_token())
            }
        }
    }

    fn trim_start(&mut self) {
        let (_, read_count) = read_while(self.current_data, |c| { c.is_whitespace() }).unwrap();
        self.slide_data_window(read_count);
    }

    fn read_token_str(&mut self, consume_space_char: bool) -> &str {
        let (token_str, read_count) = if !consume_space_char {
            read_while(self.current_data, |c| { !(c.is_whitespace() || is_symbol_char(c)) })
        } else {
            read_while(self.current_data, |c| { !((c == '\r') || (c == '\n') || is_symbol_char(c)) })
        }.unwrap();

        self.slide_data_window(read_count);

        token_str
    }

    fn read_primary_token(&mut self) -> char {
        let primary_tok_char = self.current_data.chars().nth(0).unwrap();
        self.slide_data_window(1);
        primary_tok_char
    }

    fn slide_data_window(&mut self, read_count: usize) {
        self.current_index += read_count;
        self.current_data = &self.current_data[read_count..];
    }
}