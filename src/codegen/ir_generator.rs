pub trait IRGenerator<C, T> {
    unsafe fn generate(&self, context: &mut C) -> T;
}