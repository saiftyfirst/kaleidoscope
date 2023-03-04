use std::fmt::Formatter;

const SYMBOL_NON_OP_CHARS: &'static [char; 3] = &['(', ')', ','];
const SYMBOL_OP_CHARS: &'static [char; 6] = &['+', '-', '*', '/', '>', '<'];
pub fn is_symbol_char(c: char) -> bool {
    SYMBOL_NON_OP_CHARS.contains(&c) || SYMBOL_OP_CHARS.contains(&c)
}
fn get_op_precedence(op: &char) -> i8 {
    match op {
        '<' | '>' => 10,
        '+' | '-' => 20,
        '*' | '/' => 30,
        _ => -1
    }
}
pub fn get_token_precedence(tok: &Token) -> i8 {
    if let Token::TokSymbol(ch) = tok {
        return get_op_precedence(ch);
    }
    -1
}

#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    TokEof,
    TokComment(String),

    TokDef,
    TokExtern,

    TokSymbol(char),
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
            Token::TokSymbol(val) => write!(f, "<primary> {}", val),
            Token::TokIdentifier(val) => write!(f, "<identifier> {}", val),
            Token::TokNumber(val) => write!(f, "<number> {}", val)
        }
    }
}

impl From<&str> for Token {
    fn from(token_str: &str) -> Token {
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
        Token::TokSymbol(value)
    }
}