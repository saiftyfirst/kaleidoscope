use crate::syntax::ast::GenericAst;

pub trait IRGenerator<T> {
    unsafe fn generate(&self, ast: &GenericAst) -> T;
}