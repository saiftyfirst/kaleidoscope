use std::fmt::Error;

// legacy
pub trait IRGenerator<C, T> {
    unsafe fn generate(&self, context: &mut C) -> T;
}

pub trait CodeGenerator {
    type Item;
    type AstContainer: IntoIterator<Item=Self::Item>;

    fn generate(&self, container: Self::AstContainer) -> Result<(), Error>;
}

pub struct Compiler<Generator: CodeGenerator>{
    generator: Generator
}

impl<Generator: CodeGenerator> Compiler<Generator> {
    pub fn new(generator: Generator) -> Self {
        Compiler { generator }
    }

    pub fn compile(&self, asts: Generator::AstContainer) -> Result<(), Error> {
        self.generator.generate(asts)
    }
}
