use std::collections::HashMap;
use lazy_static::lazy_static;

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

lazy_static! {
    static ref KEYWORD_TO_TOKEN: HashMap<&'static str, Token> = {
        let mut keyword_to_token_map = HashMap::new();
        keyword_to_token_map.insert("def", Token::TokDef);
        keyword_to_token_map.insert("extern", Token::TokExtern);
        keyword_to_token_map
    };
}

fn parse_str_to_word(parsed_str: String) -> Token {
    if KEYWORD_TO_TOKEN.contains_key(parsed_str.as_str()) {
        return KEYWORD_TO_TOKEN.get(parsed_str.as_str()).unwrap().clone();
    }
    return Token::TokIdentifier(parsed_str);
}

fn parse_str_to_number(parsed_str: String) -> Token {
    return Token::TokNumber(parsed_str.parse::<f64>().unwrap());
}

fn parse_str_to_comment(parsed_str: String) -> Token {
    return Token::TokComment(parsed_str);
}

pub fn parse_next_token(snippet_ref: &str) -> (Token, &str) {
    let trimmed_snippet_ref = snippet_ref.trim_start();
    let mut parsed_token: Token = Token::TokEof;

    if trimmed_snippet_ref.len() < 1 {
        return (parsed_token, trimmed_snippet_ref)
    }

    let mut snippet_chars = trimmed_snippet_ref.chars();
    let first_char = snippet_chars.nth(0).unwrap();
    let mut parsed_str: String = String::from(first_char);

    let mut consume_until_end_char = |consume_space: bool| {
        while let Some(current_char) = snippet_chars.next() {
            match current_char {
                '\r' | '\n' => {
                    break;
                }
                ' ' => {
                    if !consume_space { break; }
                    parsed_str.push(current_char);
                }
                _ => {
                    parsed_str.push(current_char);
                }
            }
        }
    };

    match first_char {
        'a'..='z' | 'A'..='Z' => {
            consume_until_end_char(false);
            parsed_token = parse_str_to_word(parsed_str);
        }
        '0'..='9' => {
            consume_until_end_char(false);
            parsed_token = parse_str_to_number(parsed_str);
        }
        '#' => {
            consume_until_end_char(true);
            parsed_token = parse_str_to_comment(parsed_str);
        }
        _ => {}
    }

    (parsed_token, snippet_chars.as_str())
}