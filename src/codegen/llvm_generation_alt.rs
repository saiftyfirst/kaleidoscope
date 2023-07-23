use crate::codegen::ir_generator::CodeGenerator;
use crate::syntax::ast::GenericAst;

pub struct CodeGeneratorLLVM;

impl CodeGeneratorLLVM {}

impl CodeGenerator for CodeGeneratorLLVM {
    type Item = GenericAst;
    type AstContainer = Vec<GenericAst>;

    fn generate(&self, asts: Self::AstContainer) -> Result<(), String> {
        Err("".to_string())
    }
}
