use std::io::Result;

#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    TokEof,
    TokComment(String),

    TokDef,
    TokExtern,

    TokIdentifier(String),
    TokNumber(f64)
}

impl<'a> From<&'a str> for Token {
    fn from(token_str: &'a str) -> Token {
        match token_str {
            "def" => Token::TokDef,
            "extern" => Token::TokExtern,
            comment if comment.starts_with("#") => Token::TokComment(comment.to_string()),
            non_empty if !non_empty.is_empty() => Token::TokIdentifier(non_empty.to_string()),
            _ => Token::TokEof
        }
    }
}

impl From<f64> for Token {
    fn from(value: f64) -> Token {
        Token::TokNumber(value)
    }
}

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

pub struct Tokenizer<'a> {
    current_index: usize,
    current_data: &'a str
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &str) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            current_data: src
        }
    }

    pub fn parse_next_token(&mut self) -> Token {
        self.trim_start();

        if self.current_data.len() ==  0 {
            return Token::TokEof;
        }

        let first_char = self.current_data.chars().nth(0).unwrap();
        match first_char {
            'a'..='z' | 'A'..='Z' => {
                Token::from(self.read_token_str(Option::None))
            }
            '0'..='9' => {
                let value = self.read_token_str(Option::None).parse::<f64>().unwrap();
                Token::from(value)
            }
            '#' => {
                Token::from(self.read_token_str(Option::Some(true)))
            }
            _ => {
                Token::TokEof
            }
        }
    }

    fn trim_start(&mut self) {
        let (_, read_count) = read_while(self.current_data, |c| { c.is_whitespace() }).unwrap();
        self.slide_data_window(read_count);
    }

    fn read_token_str(&mut self, consume_space_char: Option<bool>) -> &str {
        let consume_space_char = consume_space_char.unwrap_or(false);

        let (token_str, read_count) = if !consume_space_char {
            read_while(self.current_data, |c| { !c.is_whitespace() })
        } else {
            read_while(self.current_data, |c| { !((c == '\r') || (c == '\n')) })
        }.unwrap();

        self.slide_data_window(read_count);

        token_str
    }

    fn slide_data_window(&mut self, read_count: usize) {
        self.current_index += read_count;
        self.current_data = &self.current_data[read_count..];
    }
}