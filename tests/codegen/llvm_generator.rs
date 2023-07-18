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

    macro_rules! llvm_ir_generation_module_test {
        ($name:ident, $src:expr) => {
            #[test]
            fn $name() {
                let mut llvm_context = create_code_generator();
                let ast = parse_source_to_ast($src);

                unsafe {
                    let _ = ast.generate(&mut llvm_context);
                    println!("Generated LLVM IR Module: {}", llvm_context.get_module_as_string());
                }
            }
        }
    }

    macro_rules! llvm_ir_generation_instruction_test {
        ($name:ident, $src:expr) => {
            #[test]
            fn $name() {
                let mut llvm_context = create_code_generator();
                let ast = parse_source_to_ast($src);

                unsafe {
                    let llvm_value_ref = ast.generate(&mut llvm_context);
                     println!("{}", CStr::from_ptr(LLVMPrintValueToString(llvm_value_ref)).to_str().unwrap());
                }
            }
        }
    }

    llvm_ir_generation_instruction_test!(
        generate_simple_addition_expression,
        r###"
            4 + 5
        "###
    );

    llvm_ir_generation_module_test!(
        generate_funtion_definition_with_arguments,
        r###"
            def foo(a, b) a*a + 2*a*b + b*b
        "###
    );

    llvm_ir_generation_module_test!(
        generate_extern_cosine_with_param,
        r###"
            extern cos(x)
        "###
    );

    #[test]
    fn generate_multi_function_ir() {
        let mut llvm_context = create_code_generator();
        let ast_foo = parse_source_to_ast(
            r###"
                def foo(a) a*2
            "###
        );

        let ast_bar = parse_source_to_ast(
            r###"
                def bar(a) foo(a)
            "###
        );

        unsafe {
            let _ = ast_foo.generate(&mut llvm_context);
            let _ = ast_bar.generate(&mut llvm_context);
            println!("Generated LLVM IR Module: {}", llvm_context.get_module_as_string());
        }
    }
}