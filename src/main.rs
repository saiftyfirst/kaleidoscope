use std::io::{self, Write};
use kaleidoscope::parse::parser::*;

const QUIT_CMD : &str = "quit";

pub struct Driver {}

impl Driver {
    pub fn run() {
        loop {
            print!("ready>> ");
            io::stdout().flush().unwrap(); // flushes the buffer

            let mut prompt = String::new();
            io::stdin().read_line(&mut prompt).unwrap();

            if prompt.trim() == QUIT_CMD {
                break;
            }

            let mut parser = Parser::new(&prompt);
            println!("{}", parser.build_next_ast().unwrap());
        }
    }
}

fn main() {
    Driver::run();
}