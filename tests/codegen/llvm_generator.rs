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

    macro_rules! llvm_ir_generation_single_snippet_test {
        ($name:ident, $src:expr) => {
            #[test]
            fn $name() {
                let mut llvm_context = create_code_generator();
                let ast = parse_source_to_ast($src);

                unsafe {
                    let llvm_value_ref = ast.generate(&mut llvm_context);

                    // visual check
                    // if $module_only {
                        println!("Generated LLVM IR Module: {}", llvm_context.get_module_as_string());
                    // } else {
                        // println!("Generated LLVM IR Value:");
                        // println!("{}", CStr::from_ptr(LLVMPrintValueToString(llvm_value_ref)).to_str().unwrap());
                    // }
                }
            }
        }
    }

    // llvm_ir_generation_single_snippet_test!(
    //     generate_simple_addition_expression,
    //     r###"
    //         4 + 5
    //     "###,
    //     false
    // );

    llvm_ir_generation_single_snippet_test!(
        generate_funtion_definition_with_arguments,
        r###"
            def foo(a, b) a*a + 2*a*b + b*b
        "###
    );

    // llvm_ir_generation_single_snippet_test!(
    //     generate_funtion_definition_with_arguments_and_literals,
    //     r###"
    //         def bar(a) foo(a, 4.0) + bar(31337)
    //     "###,
    //     true
    // );

    llvm_ir_generation_single_snippet_test!(
        generate_extern_cosine_with_param,
        r###"
            extern cos(x)
        "###
    );
}