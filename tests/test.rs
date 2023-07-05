extern crate core;

pub mod parser {
    pub mod naive_parser;
    pub mod lexer;
}

pub mod codegen {
    pub mod llvm_generator;
}