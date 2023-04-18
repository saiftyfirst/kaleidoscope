use std::collections::HashMap;

// llvm-sys
use llvm_sys::prelude::*;
use llvm_sys::core::*;

use crate::codegen::ir_generator::IRGenerator;
use crate::syntax::ast::*;

struct LLVMGeneratorContext {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    named_values: HashMap<String, LLVMValueRef>
}

impl LLVMGeneratorContext {
    pub fn new() -> LLVMGeneratorContext {
        unsafe {
            let context = LLVMContextCreate();
            let builder = LLVMCreateBuilderInContext(context);
            let module = LLVMModuleCreateWithNameInContext("my_module".as_ptr() as *const i8, context);
            let named_values = HashMap::new();

            LLVMGeneratorContext {
                context,
                module,
                builder,
                named_values
            }
        }
    }
}

impl IRGenerator<LLVMTypeRef> for GenericAst {
    /*
        Learning Notes:
        - Single Static Assignment (SSA)
            SSA is a form of intermediate representation (IR) used in compilers.
            SSA specifies that each variable is assigned only once, and that each assignment is a definition of the variable.
            Existing variables are not modified, but new variables are created to hold the results of expressions.
            Versioning is used to keep track of the different values of a variable.
            In other words, there is no way to change an SSA value.
    */
    unsafe fn generate(&self, ast: &GenericAst) -> LLVMTypeRef {
        match ast {
            GenericAst::NumberExprAst {number} => {
                LLVMBFloatType()
            },
            GenericAst::VariableExprAst {name} => {
                LLVMBFloatType()
            },
            GenericAst::BinaryExprAst {op, lhs, rhs} => {
                LLVMBFloatType()
            },
            GenericAst::CallExprAst {callee, args} => {
                LLVMBFloatType()
            },
            GenericAst::FunctionAst {proto, body} => {
                LLVMBFloatType()
            },
            GenericAst::PrototypeAst {name, args} => {
                LLVMBFloatType()
            }
        }
    }
}