use core::fmt;
use std::fmt::Formatter;
use crate::ast::PrototypeAst;
use crate::lexer::*;
use crate::token::*;

#[derive(Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError(s) => write!(f, "Custom error: {}", s)
        }
    }
}

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
        self.get_next_token();
        loop {
            match self.curr_token {
                Token::TokEof => break,
                Token::TokDef => {
                    // self.hand
                    break;
                },
                Token::TokExtern => {
                    self.parse_extern();
                    break;
                },
                _ => {
                    // self.handle_expression();
                    break;
                }
            }
        }
    }

    fn get_next_token(&mut self) {
        self.curr_token = self.lexer.parse_next_token()
    }

    fn parse_extern(&mut self) -> Result<PrototypeAst, ParseError> {
        if let Token::TokExtern = self.curr_token {
            self.get_next_token();
            self.parse_prototype()
        } else {
            Err(ParseError("Attempted to parse non-extern AST as extern.".to_string()))
        }
    }

    fn parse_prototype(&mut self) -> Result<PrototypeAst, ParseError> {
        if let Token::TokIdentifier(fn_ident) = &self.curr_token {
            let mut ast = PrototypeAst::from(fn_ident);

            self.get_next_token();
            if self.curr_token != Token::TokPrimary('(') {
                return Err(ParseError("Expected prototype AST to begin with '('.".to_string()));
            }
            self.get_next_token();

            while let Token::TokIdentifier(arg_ident) = &self.curr_token {
                ast.add_arg(arg_ident);
                self.get_next_token();
            }

            if self.curr_token != Token::TokPrimary(')') {
                return Err(ParseError("Expected prototype AST to end with ')'.".to_string()));
            }
            Ok(ast)
        } else {
            return Err(ParseError("Attempted to parse non-prototype AST as prototype.".to_string()));
        }
    }
}