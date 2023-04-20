use crate::syntax::ast::GenericAst;

pub trait IRGenerator<C, T> {
    unsafe fn generate(&self, context: &mut C, ast: &GenericAst) -> T;
}