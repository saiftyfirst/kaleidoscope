use core::fmt;
use std::fmt::Formatter;
use crate::ast::*;
use crate::ast::GenericAst::{BinaryExprAst, CallExprAst, VariableExprAst};
use crate::lexer::*;
use crate::token::*;

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
            return match self.lexer.peek() {
                Token::TokEof => Err(ParseError("EOF".to_string())),
                Token::TokDef => self.parse_def_ast(),
                Token::TokExtern => self.parse_extern_ast(),
                _ => self.parse_expression()
            }
    }

    fn parse_def_ast(&mut self) -> Result<GenericAst, ParseError> {
        self.lexer.pop(); // pop def
        Ok(GenericAst::FunctionAst{ proto: Box::from(self.parse_prototype()?), body: Box::from(self.parse_expression()?) })
    }

    fn parse_extern_ast(&mut self) -> Result<GenericAst, ParseError> {
        self.lexer.pop(); // pop extern
        self.parse_prototype()
    }

    fn parse_expression(&mut self) -> Result<GenericAst, ParseError> {
        let lhs = self.parse_primary_expression()?;
        self.parse_op_and_rhs(lhs, 0)
    }

    fn parse_prototype(&mut self) -> Result<GenericAst, ParseError> {
        if let Token::TokIdentifier(fn_ident) = self.lexer.pop() {
            let mut args = Vec::new();

            if self.lexer.pop() != Token::TokSymbol('(') {
                return Err(ParseError("Expected prototype AST to begin with '('.".to_string()));
            }

            while let Token::TokIdentifier(_) = self.lexer.peek() {
                if let Token::TokIdentifier(arg_ident) = self.lexer.pop() {
                    args.push(arg_ident);
                    if let Token::TokSymbol(',') = self.lexer.peek() {
                        self.lexer.pop(); // pop the comma
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
            while self.lexer.peek().is_tok_symbol() { // next operator
                let precedence = get_token_precedence(&self.lexer.peek());
                if precedence >= min_precedence {
                    if let Token::TokSymbol(op) = self.lexer.pop() {
                        let mut rhs = self.parse_primary_expression()?;
                        while self.lexer.peek().is_tok_symbol() {
                            let peek_precedence = get_token_precedence(&self.lexer.peek());
                            if peek_precedence > precedence {
                                rhs = self.parse_op_and_rhs(rhs, precedence+1)?;
                            } else {
                                break;
                            }
                            // equal condition ?
                            }
                        lhs = BinaryExprAst { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                    }
                } else {
                    break;
                }
        }
        Ok(lhs)
    }

    fn parse_primary_expression(&mut self) -> Result<GenericAst, ParseError> {
        match self.lexer.peek() {
            Token::TokNumber(_val) => self.parse_number_expression(),
            Token::TokIdentifier(_val) => self.parse_variable_or_call_expression(),
            Token::TokSymbol('(') => self.parse_parent_expression(),
            _ => Err(ParseError("Attempted to parse non-primary AST as primary.".to_string()))
        }
    }

    fn parse_number_expression(&mut self) -> Result<GenericAst, ParseError> {
        if let Token::TokNumber(val) = self.lexer.pop() {
            return Ok(GenericAst::NumberExprAst { number: val });
        }
        Err(ParseError("Attempted to parse non-number EXPR as number.".to_string()))
    }

    fn parse_variable_or_call_expression(&mut self) -> Result<GenericAst, ParseError> {
        if let Token::TokIdentifier(identifier) = self.lexer.pop() {
            if Token::TokSymbol('(') != *self.lexer.peek() {
                return Ok(VariableExprAst { name: identifier });
            }

            self.lexer.pop(); // pop '('

            let mut args = Vec::new();
            if Token::TokSymbol(')')  != *self.lexer.peek() {
                loop {
                    args.push(self.parse_expression()?);

                    if Token::TokSymbol(')') == *self.lexer.peek(){
                        self.lexer.pop(); // pop ')'
                        break;
                    }

                    if Token::TokSymbol(',') == *self.lexer.peek() {
                        self.lexer.pop(); // pop the comma
                    } else {
                        return Err(ParseError("Attempted to parse badly formatted function call (expected ',').".to_string()));
                    }
                }
            }
            Ok(CallExprAst {callee: identifier.to_string(), args })
        } else {
            return Err(ParseError("Attempted to incorrectly parse EXPR as variable or call expression.".to_string()));
        }
    }

    fn parse_parent_expression(&mut self) -> Result<GenericAst, ParseError> {
        self.lexer.pop(); // pop (
        let res = self.parse_expression();
        self.lexer.pop(); // pop )
        return res;
    }
}