use std::collections::HashMap;

// llvm-sys
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::LLVMRealPredicate::{LLVMRealOGT, LLVMRealOLT};
use llvm_sys::LLVMValue;

use crate::codegen::ir_generator::IRGenerator;
use crate::syntax::ast::*;
use crate::syntax::vocabulary::SYMBOL_OP_CHARS;

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

impl IRGenerator<LLVMGeneratorContext, LLVMValueRef> for GenericAst {
    /*
        Learning Notes:
        - Single Static Assignment (SSA)
            SSA is a form of intermediate representation (IR) used in compilers.
            SSA specifies that each variable is assigned only once, and that each assignment is a definition of the variable.
            Existing variables are not modified, but new variables are created to hold the results of expressions.
            Versioning is used to keep track of the different values of a variable.
            In other words, there is no way to change an SSA value.
    */
    unsafe fn generate(&self, context: &mut LLVMGeneratorContext, ast: &GenericAst) -> LLVMValueRef {
        match ast {
            GenericAst::NumberExprAst {number} => {
                LLVMConstReal(LLVMBFloatType(), *number)
            },
            GenericAst::VariableExprAst {name} => {
                LLVMConstReal(LLVMBFloatType(), 2.2)
                // TODO (saif) complete implementation for VariableExprAst
            },
            GenericAst::BinaryExprAst {op, lhs, rhs} => {
                let lhs_ir = lhs.generate(context, lhs);
                let rhs_ir = rhs.generate(context, rhs);

                if SYMBOL_OP_CHARS.contains(op) {
                    match op {
                        '+' => {
                            LLVMBuildFAdd(context.builder, lhs_ir, rhs_ir, "addtmp".as_ptr() as *const i8)
                        },
                        '-' => {
                            LLVMBuildFSub(context.builder, lhs_ir, rhs_ir, "subtmp".as_ptr() as *const i8)
                        },
                        '*' => {
                            LLVMBuildFMul(context.builder, lhs_ir, rhs_ir, "multmp".as_ptr() as *const i8)
                        },
                        '/' => {
                            LLVMBuildFDiv(context.builder, lhs_ir, rhs_ir, "divtmp".as_ptr() as *const i8)
                        },
                        '>' => {
                            LLVMBuildFCmp(context.builder, LLVMRealOGT, lhs_ir, rhs_ir, "cmpgt".as_ptr() as *const i8)
                        },
                        '<' => {
                            LLVMBuildFCmp(context.builder, LLVMRealOLT, lhs_ir, rhs_ir, "cmplt".as_ptr() as *const i8)
                        },
                        _ => !panic!("Implementation missing for operator {}", op)
                    }
                }
                else {
                    panic!("Unknown operator {}", op)
                }
            },
            GenericAst::CallExprAst {callee, args} => {
                // TODO (saif) complete implementation for CallExprAst
                LLVMConstReal(LLVMBFloatType(), 2.2)
            },
            GenericAst::FunctionAst {proto, body} => {
                // TODO (saif) complete implementation for FunctionAst
                LLVMConstReal(LLVMBFloatType(), 2.2)
            },
            GenericAst::PrototypeAst {name, args} => {
                // TODO (saif) complete implementation for PrototypeAst
                LLVMConstReal(LLVMBFloatType(), 2.2)
            }
        }
    }
}