extern crate core;

pub mod parse {
    pub mod parser;
    pub mod lexer;
    pub mod token;
}

pub mod syntax {
    pub mod ast;
}

pub mod codegen {
    mod ir_generator;
    pub mod llvm_generator;
}

pub mod utils {
    pub mod display;
}
