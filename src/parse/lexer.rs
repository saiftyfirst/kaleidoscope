use crate::parse::token::*;
use crate::syntax::vocabulary::*;

pub struct Lexer<'a> {
    data: &'a str,
    token_cache: Token,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        let mut lexer = Lexer {
            data: src,
            token_cache: Token::TokEof
        };
        lexer.init();
        lexer
    }

    pub fn peek(&self) -> &Token {
        &self.token_cache
    }

    pub fn pop(&mut self) -> Token {
        let popped = std::mem::take(&mut self.token_cache);
        (self.token_cache, self.data) = Self::parse_and_slide(self.data);
        popped
    }

    fn init(&mut self) {
        (self.token_cache, self.data) = Self::parse_and_slide(self.data)
    }

    fn parse_and_slide(data: &str) -> (Token, &str) {
        let (token, read_count) = Self::parse_token(data);
        (token, &data[read_count..])
    }

    fn parse_token(data: &str) -> (Token, usize) {
        let trim_count = Self::trim_start(data);

        let trimmed_data = &data[trim_count..];

        if trimmed_data.len() ==  0 {
            return (Token::TokEof, trim_count);
        }

        let first_char = trimmed_data.chars().nth(0).unwrap();
        let (token, token_count) = match first_char {
            'a'..='z' | 'A'..='Z' => {
                let (token_str, token_count) = Self::read_token_str(trimmed_data, false);
                (Token::from(token_str), token_count)
            }
            '0'..='9' => {
                let (token_value, token_count) = Self::read_token_str(trimmed_data, false);
                (Token::from(token_value.parse::<f64>().unwrap()), token_count)
            }
            '#' => {
                let (comment_str, token_count) = Self::read_token_str(trimmed_data, true);
                (Token::from(comment_str), token_count)
            }
            _ => {
                let (token_char, token_count) = Self::read_primary_token(trimmed_data);
                (Token::from(token_char), token_count)
            }
        };
        (token, (trim_count + token_count))
    }

    fn trim_start(data: &str) -> usize {
        Self::read_while(data, |c| { c.is_whitespace() })
    }

    fn read_token_str(data: &str, consume_space_char: bool) -> (&str, usize) {
        let read_count = if !consume_space_char {
            Self::read_while(data, |c| { !(c.is_whitespace() || is_symbol_char(c)) })
        } else {
            Self::read_while(data, |c| { !((c == '\r') || (c == '\n') || is_symbol_char(c)) })
        };
        (&data[..read_count], read_count)
    }

    fn read_primary_token(data: &str) -> (char, usize) {
        let primary_tok_char = data.chars().nth(0).unwrap();
        (primary_tok_char, 1)
    }

    fn read_while<F>(data: &str, pred: F) -> usize
        where F: Fn(char) -> bool {
        let mut read_count = 0;
        for elem in data.chars() {
            if !pred(elem) {
                break;
            }
            read_count += 1;
        }
        read_count
    }
}