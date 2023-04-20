use std::collections::HashMap;

// llvm-sys
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::LLVMValue;

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
            let module = LLVMModuleCreateWithNameInContext("default_module".as_ptr() as *const i8, context);
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

impl IRGenerator<LLVMGeneratorContext, *const LLVMValue> for GenericAst {
    /*
        Learning Notes:
        - Single Static Assignment (SSA)
            SSA is a form of intermediate representation (IR) used in compilers.
            SSA specifies that each variable is assigned only once, and that each assignment is a definition of the variable.
            Existing variables are not modified, but new variables are created to hold the results of expressions.
            Versioning is used to keep track of the different values of a variable.
            In other words, there is no way to change an SSA value.
    */
    unsafe fn generate(&self, context: &mut LLVMGeneratorContext, ast: &GenericAst) -> *const LLVMValue {
        match ast {
            GenericAst::NumberExprAst {number} => {
                LLVMConstReal(LLVMBFloatType(), *number)
            },
            GenericAst::VariableExprAst {name} => {
                LLVMConstReal(LLVMBFloatType(), 2.2)
            },
            GenericAst::BinaryExprAst {op, lhs, rhs} => {
                let lhs = LLVMConstReal(LLVMBFloatType(), 2.2);
                let rhs = LLVMConstReal(LLVMBFloatType(), 2.2);

                LLVMBuildFAdd(context.builder, lhs, rhs, "addtmp".as_ptr() as *const i8)

            },
            GenericAst::CallExprAst {callee, args} => {
                LLVMConstReal(LLVMBFloatType(), 2.2)
            },
            GenericAst::FunctionAst {proto, body} => {
                LLVMConstReal(LLVMBFloatType(), 2.2)
            },
            GenericAst::PrototypeAst {name, args} => {
                LLVMConstReal(LLVMBFloatType(), 2.2)
            }
        }
    }
}