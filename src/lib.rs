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
}

pub mod pipelining {
    mod staging {
        pub mod stage;
    }
    mod pipeline;
}

pub mod utils {
    pub mod display;
}
