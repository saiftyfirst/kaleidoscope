use std::collections::HashMap;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};

// llvm-sys
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::LLVMRealPredicate::{LLVMRealOGT, LLVMRealOLT};

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
            /* Learning Note:
                The context is used to hold and manage various LLVM **objects and data structures**.
                The builder is used to construct **LLVM instructions** within a basic block.
                A module is a container for **LLVM functions and global variables**.
                LLVM values or variables.
            */
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
    unsafe fn generate(&self, context: &mut LLVMGeneratorContext, _ast: &GenericAst) -> LLVMValueRef {
        match self {
            GenericAst::NumberExprAst {number} => {
                LLVMConstReal(LLVMBFloatType(), *number)
            },
            GenericAst::VariableExprAst {name} => {
                if let Some(value) = context.named_values.get(name) {
                    *value
                } else {
                    panic!("Unknown variable name: {}", name);
                }
            },
            GenericAst::BinaryExprAst {op, lhs, rhs} => {
                let lhs_ir = lhs.generate(context, lhs);
                let rhs_ir = rhs.generate(context, rhs);

                if SYMBOL_OP_CHARS.contains(op) {
                    match op {
                        '+' => {
                            /*
                                Learning Note: Why is the builder passed in ?
                                The builder is used to construct LLVM instructions within a basic block.
                                The builder keeps track of the current insertion point in the basic block and
                                is responsible for generating and appending the LLVM instruction to the block.
                            */
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
                        _ => {
                            panic!("Implementation missing for operator {}", op)
                        }
                    }
                }
                else {
                    panic!("Unknown operator {}", op)
                }
            },
            GenericAst::CallExprAst {callee, args} => {
                let func = LLVMGetNamedFunction(context.module, callee.as_ptr() as *const i8);
                if func.is_null() {
                    panic!("Unknown function referenced {}", callee);
                }

                let call_arg_count = LLVMCountParams(func);
                if (call_arg_count as usize) != args.len() {
                    panic!("Funtion {} called with unexpected number of arguments", callee);
                }

                let mut generated_args = Vec::new();
                for arg in args.iter() {
                    generated_args.push(arg.generate(context, arg));
                }

                LLVMBuildCall2(context.builder,
                               LLVMTypeOf(func),
                               func,
                               generated_args.as_mut_ptr(),
                               call_arg_count,
                               "calltmp".as_ptr() as *const i8)
            },
            GenericAst::FunctionAst {proto, body} => {
                let proto_unboxed = &**proto;

                if let GenericAst::PrototypeAst { name, args} = proto_unboxed {

                    let mut func_proto = LLVMGetNamedFunction(context.module,
                                                              name.as_ptr() as *const i8);
                    if func_proto.is_null() {
                        // TODO (saif) weird that proto has to call a member and pass itself
                        // This is due to bad interface design. Fix It!
                        // Moreover, look at the function so far, the 3rd parameter is never used
                        func_proto = proto.generate(context, proto_unboxed);
                    }

                    let first_block = LLVMGetFirstBasicBlock(func_proto);
                    if first_block.is_null() {
                        let basic_block = LLVMAppendBasicBlockInContext(context.context,
                                                                        func_proto,
                                                                        "entry".as_ptr() as *const i8);
                        LLVMPositionBuilderAtEnd(context.builder, basic_block);

                        context.named_values.clear();
                        // Why are we adding the args to the named_values map?
                        for (i, arg_name) in args.iter().enumerate() {
                            let arg = LLVMGetParam(func_proto, i as u32);
                            context.named_values.insert(arg_name.clone(), arg);
                        }

                        let body_ir = body.generate(context, body);
                        // TODO (saif) optionals instead of nulls/panics?
                        if !body_ir.is_null() {
                            LLVMBuildRet(context.builder, body_ir);
                            LLVMVerifyFunction(func_proto, LLVMVerifierFailureAction::LLVMPrintMessageAction);
                        } else {
                            //erase?
                        }
                        return func_proto;
                    } else {
                        panic!("Function {} redefinition is not allowed", name);
                    }
                } else {
                    panic!("Expected Prototype Ast!");
                }
            },
            GenericAst::PrototypeAst {name, args} => {
                // TODO (saif) remove assumption that our functions always return a float
                let return_type = LLVMBFloatType();
                let mut arg_types = std::vec![LLVMBFloatType(); args.len()];

                /* Learning Note:
                    the prototype with name is not registered in the module's symbol table
                    until the function is defined.
                */
                let func_proto = LLVMAddFunction(context.module,
                                                 name.as_ptr() as *const i8,
                                                 LLVMFunctionType(return_type,
                                                                  arg_types.as_mut_ptr(),
                                                                  args.len() as u32,
                                                                  0));

                // set the names of the variables
                for (idx, arg) in args.iter().enumerate() {
                    LLVMSetValueName2(LLVMGetParam(func_proto, idx as u32),
                                      arg.as_ptr() as *const i8,
                                      arg.len() as usize)
                }
                func_proto
            }
        }
    }
}