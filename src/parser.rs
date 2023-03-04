use core::fmt;
use std::fmt::Formatter;
use crate::ast::*;
use crate::ast::GenericAst::{BinaryExprAst, CallExprAst, VariableExprAst};
use crate::lexer::*;
use crate::token::*;

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
    lexer: Lexer<'a>, // TODO: Fix #2 get lexer as input
    curr_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(src: &str) -> Parser {
        Parser {
            lexer: Lexer::new(src),
            curr_token: Token::TokEof
        }
    }

    pub fn build_next_ast(&mut self) -> Result<GenericAst, ParseError> {
        loop {
            return match self.peek_token() {
                Token::TokEof => Err(ParseError("EOF".to_string())),
                Token::TokDef => self.parse_def_ast(),
                Token::TokExtern => self.parse_extern_ast(),
                _ => self.parse_expression()
            }
        }
    }

    // TODO : curr_token overriden by both peek and get -> this will get confusing
    fn peek_token(&self) -> &Token {
        self.lexer.peek()
    }

    fn pop_token(&mut self) {
        self.curr_token = self.lexer.pop()
    }

    fn parse_def_ast(&mut self) -> Result<GenericAst, ParseError> {
        self.pop_token();
        if let Token::TokDef = self.curr_token {
            let prototype_ast = self.parse_prototype()?;
            let expression_ast = self.parse_expression()?;
            Ok(GenericAst::FunctionAst{ proto: Box::from(prototype_ast), body: Box::from(expression_ast) })
        } else {
            Err(ParseError("Attempted to parse non-def AST as def.".to_string()))
        }
    }

    fn parse_extern_ast(&mut self) -> Result<GenericAst, ParseError> {
        self.pop_token(); // pop extern
        if let Token::TokExtern = self.curr_token {
            self.parse_prototype()
        } else {
            Err(ParseError("Attempted to parse non-extern AST as extern.".to_string()))
        }
    }

    fn parse_prototype(&mut self) -> Result<GenericAst, ParseError> {
        self.pop_token();
        if let Token::TokIdentifier(fn_ident) = self.curr_token.clone() { // TODO: String causes issues with moving/cloning/<Not sure yet>
            let mut args = Vec::new();

            self.pop_token();
            if self.curr_token != Token::TokSymbol('(') {
                return Err(ParseError("Expected prototype AST to begin with '('.".to_string()));
            }

            while let Token::TokIdentifier(arg_ident) = self.peek_token().clone() {
                self.pop_token();
                args.push(arg_ident.to_string()); // TODO: clone due to String?

                if let Token::TokSymbol(',') = self.peek_token().clone() {
                    self.pop_token(); // pop the comma
                } else {
                    break;
                }
            }

            self.pop_token();
            if self.curr_token != Token::TokSymbol(')') {
                return Err(ParseError("Expected prototype AST to end with ')'.".to_string()));
            }
            Ok(GenericAst::PrototypeAst { name: fn_ident.to_string(), args })
        } else {
            return Err(ParseError("Attempted to parse non-prototype AST as prototype.".to_string()));
        }
    }

    fn parse_expression(&mut self) -> Result<GenericAst, ParseError> {
        let lhs = self.parse_primary_expression()?;
        self.parse_op_and_rhs(lhs, 0)
    }

    fn parse_op_and_rhs(&mut self, mut lhs: GenericAst, min_precedence: i8) -> Result<GenericAst, ParseError> {
        loop {
            if let Token::TokSymbol(op) = self.peek_token().clone() { // next operator
                let precedence = get_token_precedence(&self.peek_token());
                if precedence >= min_precedence {
                    self.pop_token(); // pop operator
                    let mut rhs = self.parse_primary_expression()?;
                    loop {
                        if let Token::TokSymbol(_peek_op) = self.peek_token().clone() {
                            let peek_precedence = get_token_precedence(&self.peek_token());
                            if peek_precedence > precedence {
                                rhs = self.parse_op_and_rhs(rhs, precedence+1)?;
                            } else {
                                break;
                            }
                            // equal condition ?
                        } else {
                            break;
                        }
                    }

                    lhs = BinaryExprAst { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn parse_primary_expression(&mut self) -> Result<GenericAst, ParseError> {
        match self.peek_token() {
            Token::TokNumber(_val) => self.parse_number_expression(),
            Token::TokIdentifier(_val) => self.parse_variable_or_call_expression(),
            Token::TokSymbol('(') => self.parse_parent_expression(),
            _ => Err(ParseError("Attempted to parse non-primary AST as primary.".to_string()))
        }
    }

    fn parse_number_expression(&mut self) -> Result<GenericAst, ParseError> {
        self.pop_token();
        if let Token::TokNumber(val) = self.curr_token { // duplication
            return Ok(GenericAst::NumberExprAst { number: val });
        }
        Err(ParseError("Attempted to parse non-number EXPR as number.".to_string()))
    }

    fn parse_variable_or_call_expression(&mut self) -> Result<GenericAst, ParseError> {
        self.pop_token();
        let identifier; // costly ?
        if let Token::TokIdentifier(val) = self.curr_token.clone() { // duplication
            identifier = val;
        } else {
            return Err(ParseError("Attempted to incorrectly parse EXPR as variable or call expression.".to_string()));
        }

        if Token::TokSymbol('(') != *self.peek_token() {
            return Ok(VariableExprAst { name: identifier.to_string() });
        }
        self.pop_token(); // pop '('

        let mut args = Vec::new();
        if  Token::TokSymbol(')')  != *self.peek_token() {
            loop {
                args.push(self.parse_expression()?);

                if Token::TokSymbol(')') == *self.peek_token(){
                    self.pop_token();
                    break;
                }

                if Token::TokSymbol(',') == *self.peek_token() {
                    self.pop_token(); // pop the comma
                } else {
                    return Err(ParseError("Attempted to parse badly formatted function call (expected ',').".to_string()));
                }
            }
        }
        Ok(CallExprAst {callee: identifier.to_string(), args })
    }

    fn parse_parent_expression(&mut self) -> Result<GenericAst, ParseError> {
        self.pop_token(); // pop (
        // TODO CHECK pop?
        let res = self.parse_expression();
        self.pop_token(); // pop )
        // TODO CHECK pop?
        return res;
    }
}