use std::io::{self, Write};
use std::ffi::{CStr, CString};

use llvm_sys::core::LLVMPrintValueToString;

use kaleidoscope::parse::parser::*;
use kaleidoscope::codegen::llvm_generator::*;
use kaleidoscope::codegen::ir_generator::IRGenerator;

const QUIT_CMD : &str = "quit";

pub struct Driver {}

impl Driver {
    pub fn run() {
        let mut llvm_generator_context = LLVMGeneratorContext::new();
        loop {
            print!("ready>> ");
            io::stdout().flush().unwrap(); // flushes the buffer

            let mut prompt = String::new();
            io::stdin().read_line(&mut prompt).unwrap();

            if prompt.trim() == QUIT_CMD {
                break;
            }

            let mut parser = Parser::new(&prompt);
            let ast = parser.build_next_ast().unwrap();
            println!("{}", ast);

            unsafe {
                let llvm_value_ref = ast.generate(&mut llvm_generator_context, &ast);
                println!("{}", CStr::from_ptr(LLVMPrintValueToString(llvm_value_ref)).to_str().unwrap());
            }
        }
    }
}

fn main() {
    Driver::run();
}