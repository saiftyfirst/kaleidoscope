use crate::syntax::ast::*;
use crate::parse::lexer::*;
use crate::parse::token::*;
use crate::error::{PrefixedError, Error};
extern crate llvm_sys;

pub struct Parser<'a> {
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(src: &str) -> Parser {
        Parser {
            lexer: Lexer::new(src)
        }
    }

    pub fn build_next_ast(&mut self) -> Result<GenericAst, Error> {
            return match self.peek_lexer() {
                Token::TokEof => Err(Error::new("EOF".to_string())),
                Token::TokDef => Ok(GenericAst::FuncAst(self.parse_function_definition()?)),
                Token::TokExtern => Ok(GenericAst::FuncAst(self.parse_extern_call_expression()?)),
                _default => Ok(GenericAst::ExprAst(self.parse_abstract_expression()?))
            }
    }

    fn parse_function_definition(&mut self) -> Result<FuncAst, Error> {
        self.lexer.pop(); // pop def
        // TODO (saif) when parse_prototype works but not parse_abstract_expression, the error is obscure!
        Ok(FuncAst::Function{ proto: Box::from(self.parse_prototype()?), body: Box::from(self.parse_abstract_expression()?) })
    }

    fn parse_extern_call_expression(&mut self) -> Result<FuncAst, Error> {
        self.lexer.pop(); // pop extern
        self.parse_prototype()
    }

    fn parse_abstract_expression(&mut self) -> Result<ExprAst, Error> {
        let lhs = self.parse_single_expression_unit()?;
        self.parse_op_and_rhs(lhs, 0)
    }

    fn parse_prototype(&mut self) -> Result<FuncAst, Error> {
        if let Token::TokIdentifier(fn_ident) = self.lexer.pop() {
            let mut args = Vec::new();

            if self.lexer.pop() != Token::TokSymbol('(') {
                return Err(self.error("Expected prototype AST to begin with '('."));
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
                return Err(self.error("Expected prototype AST to end with ')'."));
            }
            Ok(FuncAst::Prototype { name: fn_ident.to_string(), args })
        } else {
            return Err(self.error("Attempted to parse non-prototype AST as prototype."));
        }
    }

    fn parse_op_and_rhs(&mut self, mut lhs: ExprAst, min_precedence: i8) -> Result<ExprAst, Error> {
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
                        lhs = ExprAst::BinaryExpr { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                    }
                } else {
                    break;
                }
        }
        Ok(lhs)
    }

    fn parse_single_expression_unit(&mut self) -> Result<ExprAst, Error> {
        match self.peek_lexer() {
            Token::TokNumber(_val) => self.parse_number_expression(),
            Token::TokIdentifier(_val) => self.parse_variable_or_call_expression(),
            Token::TokSymbol('(') => self.parse_enclosed_expression(),
            _ => Err(self.error("Attempted to parse non-primary AST as primary."))
        }
    }

    fn parse_number_expression(&mut self) -> Result<ExprAst, Error> {
        if let Token::TokNumber(val) = self.pop_lexer() {
            return Ok(ExprAst::NumberExpr { number: val });
        }
        Err(self.error("Attempted to parse non-number EXPR as number."))
    }

    fn parse_variable_or_call_expression(&mut self) -> Result<ExprAst, Error> {
        if let Token::TokIdentifier(identifier) = self.pop_lexer() {
            if Token::TokSymbol('(') != *self.peek_lexer() {
                return Ok(ExprAst::VariableExpr { name: identifier });
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
                        return Err(self.error("Attempted to parse badly formatted function call (expected ',')."));
                    }
                }
            }
            Ok(ExprAst::CallExpr {callee: identifier.to_string(), args })
        } else {
            Err(self.error("Attempted to incorrectly parse EXPR as variable or call expression."))
        }
    }

    fn parse_enclosed_expression(&mut self) -> Result<ExprAst, Error> {
        self.pop_lexer(); // pop (
        let res = self.parse_abstract_expression();
        self.pop_lexer(); // pop )
        res
    }

    fn pop_lexer(&mut self) -> Token {
        self.lexer.pop()
    }

    fn peek_lexer(&mut self) -> &Token {
        self.lexer.peek()
    }
}

impl PrefixedError for Parser<'_> {
    fn get_prefix(&self) -> &str {
        "Parse Failed: "
    }
}
