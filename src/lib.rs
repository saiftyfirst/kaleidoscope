extern crate core;

pub mod parse {
    pub mod parser;
    pub mod lexer;
    pub mod token;
}

pub mod syntax {
    pub mod ast;
    pub mod vocabulary;
}

pub mod codegen {
    pub mod ir_generator;
    pub mod llvm_generator;
    pub mod llvm_generation_alt;
}

pub mod utils {
    pub mod display;
}
