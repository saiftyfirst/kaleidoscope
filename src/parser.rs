use crate::ast::PrototypeAst;
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
        self.get_next_token();
        loop {
            match self.curr_token {
                Token::TokEof => break,
                Token::TokDef => {
                    // self.handle_def();
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

    fn parse_extern(&mut self) -> Option<PrototypeAst> {
        if let Token::TokExtern = self.curr_token {
            self.get_next_token();
            self.parse_prototype()
        } else {
          // Throw ?
            None
        }
    }

    fn parse_prototype(&mut self) -> Option<PrototypeAst> {

        if let Token::TokIdentifier(fn_ident) = &self.curr_token {
            let mut ast = PrototypeAst::from(fn_ident);

            self.get_next_token();
            if self.curr_token != Token::TokPrimary('(') {
                // throw ?
            }
            self.get_next_token();

            while let Token::TokIdentifier(arg_ident) = &self.curr_token {
                ast.add_arg(arg_ident);
                self.get_next_token();
            }

            if self.curr_token != Token::TokPrimary(')') {
                // throw ?
            }
            return Some(ast);
        } else {
            // throw ?
            None
        }
    }
}