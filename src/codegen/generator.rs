use crate::syntax::ast::GenericAst;

pub trait Generator<T> {
    unsafe fn generate(&self, ast: &GenericAst) -> T;
}