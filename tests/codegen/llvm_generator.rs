#[cfg(test)]
mod tests {
    use llvm_sys::core::*;
    use std::ffi::{CStr};

    use kaleidoscope::codegen::ir_generator::IRGenerator;
    use kaleidoscope::parse::parser::*;
    use kaleidoscope::codegen::llvm_generator::*;
    use kaleidoscope::syntax::ast::GenericAst;

    fn parse_source_to_ast(src: &str) -> GenericAst {
        Parser::new(src).build_next_ast().unwrap()
    }

    fn create_code_generator() -> LLVMGeneratorContext {
        LLVMGeneratorContext::new()
    }

    #[test]
    fn generate_addition_expression() {
        let mut llvm_context = LLVMGeneratorContext::new();


        let ast = parse_source_to_ast(
            r###"
                1 + 2
            "###);

        unsafe {
            let llvm_value_ref = ast.generate(&mut create_code_generator());
            println!("{}", CStr::from_ptr(LLVMPrintValueToString(llvm_value_ref)).to_str().unwrap());
        }

        let ast = parse_source_to_ast(
            r###"
                2
            "###);

        unsafe {
            let llvm_value_ref = ast.generate(&mut create_code_generator());
            println!("{}", CStr::from_ptr(LLVMPrintValueToString(llvm_value_ref)).to_str().unwrap());
        }

        let ast = parse_source_to_ast(
            r###"
                3
            "###);

        unsafe {

            let llvm_value_ref = ast.generate(&mut llvm_context);

            llvm_context.print_module();
            println!("{}", CStr::from_ptr(LLVMPrintValueToString(LLVMConstReal(LLVMBFloatType(), 1.0))).to_str().unwrap());
        }
    }

    #[test]
    fn generate_arithmetic_function() {
        let mut llvm_context = LLVMGeneratorContext::new();

        let ast = parse_source_to_ast(
            r###"
                def foo(a, b) a*a + 2*a*b + b*b
            "###);

        unsafe {
            ast.generate(&mut llvm_context);
            llvm_context.print_module();
        }
    }

}