use std::fmt::Formatter;

const PRIMARY_CHARS: &'static [char; 8] = &['(', ')', '+', '-', '*', '/', '>', '<'];
pub fn is_primary_char(c: char) -> bool {
    PRIMARY_CHARS.contains(&c)
}

#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    TokEof,
    TokComment(String),

    TokDef,
    TokExtern,

    TokPrimary(char),
    TokIdentifier(String),
    TokNumber(f64)
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::TokEof => write!(f, "<eof>"),
            Token::TokComment(val) => write!(f, "<comment> {}", val),
            Token::TokDef => write!(f, "<def>"),
            Token::TokExtern => write!(f, "<extern>"),
            Token::TokPrimary(val) => write!(f, "<primary> {}", val),
            Token::TokIdentifier(val) => write!(f, "<identifier> {}", val),
            Token::TokNumber(val) => write!(f, "<number> {}", val)
        }
    }
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

impl From<char> for Token {
    fn from(value: char) -> Token {
        Token::TokPrimary(value)
    }
}