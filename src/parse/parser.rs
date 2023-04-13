use core::fmt;
use std::fmt::Formatter;

use crate::syntax::ast::*;
use crate::parse::lexer::*;
use crate::parse::token::*;

extern crate llvm_sys;

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError(s) => write!(f, "Custom error: {}", s)
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(src: &str) -> Parser {
        Parser {
            lexer: Lexer::new(src)
        }
    }

    pub fn build_next_ast(&mut self) -> Result<GenericAst, ParseError> {
            return match self.peek_lexer() {
                Token::TokEof => Err(ParseError("EOF".to_string())),
                Token::TokDef => self.parse_function_definition(),
                Token::TokExtern => self.parse_extern_call_expression(),
                _default => self.parse_abstract_expression()
            }
    }

    fn parse_function_definition(&mut self) -> Result<GenericAst, ParseError> {
        self.lexer.pop(); // pop def
        Ok(GenericAst::FunctionAst{ proto: Box::from(self.parse_prototype()?), body: Box::from(self.parse_abstract_expression()?) })
    }

    fn parse_extern_call_expression(&mut self) -> Result<GenericAst, ParseError> {
        self.lexer.pop(); // pop extern
        self.parse_prototype()
    }

    fn parse_abstract_expression(&mut self) -> Result<GenericAst, ParseError> {
        let lhs = self.parse_single_expression_unit()?;
        self.parse_op_and_rhs(lhs, 0)
    }

    fn parse_prototype(&mut self) -> Result<GenericAst, ParseError> {
        if let Token::TokIdentifier(fn_ident) = self.lexer.pop() {
            let mut args = Vec::new();

            if self.lexer.pop() != Token::TokSymbol('(') {
                return Err(ParseError("Expected prototype AST to begin with '('.".to_string()));
            }

            while let Token::TokIdentifier(_) = self.peek_lexer() {
                if let Token::TokIdentifier(arg_ident) = self.lexer.pop() {
                    args.push(arg_ident);
                    if let Token::TokSymbol(',') = self.peek_lexer() {
                        self.pop_lexer(); // pop the comma
                    } else {
                        break;
                    }
                }
            }

            if self.lexer.pop() != Token::TokSymbol(')') {
                return Err(ParseError("Expected prototype AST to end with ')'.".to_string()));
            }
            Ok(GenericAst::PrototypeAst { name: fn_ident.to_string(), args })
        } else {
            return Err(ParseError("Attempted to parse non-prototype AST as prototype.".to_string()));
        }
    }

    fn parse_op_and_rhs(&mut self, mut lhs: GenericAst, min_precedence: i8) -> Result<GenericAst, ParseError> {
            while self.peek_lexer().is_tok_symbol() { // next operator
                let precedence = get_token_precedence(&self.peek_lexer());
                if precedence >= min_precedence {
                    if let Token::TokSymbol(op) = self.lexer.pop() {
                        let mut rhs = self.parse_single_expression_unit()?;
                        while self.peek_lexer().is_tok_symbol() {
                            let peek_precedence = get_token_precedence(&self.peek_lexer());
                            if peek_precedence > precedence {
                                rhs = self.parse_op_and_rhs(rhs, precedence+1)?;
                            } else {
                                break;
                            }
                            // equal condition ?
                            }
                        lhs = GenericAst::BinaryExprAst { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                    }
                } else {
                    break;
                }
        }
        Ok(lhs)
    }

    fn parse_single_expression_unit(&mut self) -> Result<GenericAst, ParseError> {
        match self.peek_lexer() {
            Token::TokNumber(_val) => self.parse_number_expression(),
            Token::TokIdentifier(_val) => self.parse_variable_or_call_expression(),
            Token::TokSymbol('(') => self.parse_enclosed_expression(),
            _ => Err(ParseError("Attempted to parse non-primary AST as primary.".to_string()))
        }
    }

    fn parse_number_expression(&mut self) -> Result<GenericAst, ParseError> {
        if let Token::TokNumber(val) = self.pop_lexer() {
            return Ok(GenericAst::NumberExprAst { number: val });
        }
        Err(ParseError("Attempted to parse non-number EXPR as number.".to_string()))
    }

    fn parse_variable_or_call_expression(&mut self) -> Result<GenericAst, ParseError> {
        if let Token::TokIdentifier(identifier) = self.pop_lexer() {
            if Token::TokSymbol('(') != *self.peek_lexer() {
                return Ok(GenericAst::VariableExprAst { name: identifier });
            }

            self.pop_lexer(); // pop '('

            let mut args = Vec::new();
            if Token::TokSymbol(')')  != *self.peek_lexer() {
                loop {
                    args.push(self.parse_abstract_expression()?);

                    if Token::TokSymbol(')') == *self.peek_lexer(){
                        self.pop_lexer(); // pop ')'
                        break;
                    }

                    if Token::TokSymbol(',') == *self.peek_lexer() {
                        self.pop_lexer(); // pop the comma
                    } else {
                        return Err(ParseError("Attempted to parse badly formatted function call (expected ',').".to_string()));
                    }
                }
            }
            Ok(GenericAst::CallExprAst {callee: identifier.to_string(), args })
        } else {
            return Err(ParseError("Attempted to incorrectly parse EXPR as variable or call expression.".to_string()));
        }
    }

    fn parse_enclosed_expression(&mut self) -> Result<GenericAst, ParseError> {
        self.pop_lexer(); // pop (
        let res = self.parse_abstract_expression();
        self.pop_lexer(); // pop )
        return res;
    }

    fn pop_lexer(&mut self) -> Token {
        self.lexer.pop()
    }

    fn peek_lexer(&mut self) -> &Token {
        self.lexer.peek()
    }
}
