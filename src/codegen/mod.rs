pub trait IRGenerator<C, T> {
    unsafe fn generate(&self, context: &mut C) -> T;
}

pub mod llvm_generator;
pub mod llvm_generator_v2;