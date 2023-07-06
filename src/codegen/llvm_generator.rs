use std::collections::HashMap;
use std::ffi::{CStr};
use std::os::raw::{c_char};
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};

// llvm-sys
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::LLVMRealPredicate::{LLVMRealOGT, LLVMRealOLT};

use crate::codegen::ir_generator::IRGenerator;
use crate::syntax::ast::*;
use crate::syntax::vocabulary::SYMBOL_OP_CHARS;

pub struct LLVMGeneratorContext {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    named_values: HashMap<String, LLVMValueRef>
}

impl LLVMGeneratorContext
{
    pub fn new() -> LLVMGeneratorContext
    {
        unsafe
            {
            /* Learning Note:
                The context is used to hold and manage various LLVM **objects and data structures**.
                The builder is used to construct **LLVM instructions** within a basic block.
                A module is a container for **LLVM functions and global variables**.
                LLVM values or variables.
            */
            let context = LLVMContextCreate();
            let builder = LLVMCreateBuilderInContext(context);
            let module = LLVMModuleCreateWithNameInContext(
                "default_module\0".as_ptr() as *const i8, context);
            let named_values = HashMap::new();

            LLVMGeneratorContext
            {
                context,
                module,
                builder,
                named_values
            }
        }
    }

    pub fn get_module_as_string(&self) -> String {
        unsafe {
            return CStr::from_ptr(LLVMPrintModuleToString(self.module)).to_str().unwrap().to_string();
        }
    }
}

impl IRGenerator<LLVMGeneratorContext, LLVMValueRef> for GenericAst
{
    /*
        Learning Notes:
        - Single Static Assignment (SSA)
            SSA is a form of intermediate representation (IR) used in compilers.
            SSA specifies that each variable is assigned only once, and that each assignment is a definition of the variable.
            Existing variables are not modified, but new variables are created to hold the results of expressions.
            Versioning is used to keep track of the different values of a variable.
            In other words, there is no way to change an SSA value.
    */
    unsafe fn generate(&self, context: &mut LLVMGeneratorContext) -> LLVMValueRef {
        match self {
            GenericAst::NumberExprAst {number} => {
                LLVMConstReal(LLVMBFloatTypeInContext(context.context), *number)
            },
            GenericAst::VariableExprAst {name} => {
                if let Some(value) = context.named_values.get(name) {
                    *value
                } else {
                    panic!("Unknown variable name: {}", name);
                }
            },
            GenericAst::BinaryExprAst {op, lhs, rhs} => {
                let lhs_ir = lhs.generate(context);
                let rhs_ir = rhs.generate(context);

                if SYMBOL_OP_CHARS.contains(op) {
                    match op {
                        '+' => {
                            /*
                                Learning Note: Why is the builder passed in ?
                                The builder is used to construct LLVM instructions within a basic block.
                                The builder keeps track of the current insertion point in the basic block and
                                is responsible for generating and appending the LLVM instruction to the block.
                            */
                            LLVMBuildFAdd(context.builder, lhs_ir, rhs_ir, "addtmp\0".as_ptr() as *const i8)
                        },
                        '-' => {
                            LLVMBuildFSub(context.builder, lhs_ir, rhs_ir, "subtmp\0".as_ptr() as *const i8)
                        },
                        '*' => {
                            LLVMBuildFMul(context.builder, lhs_ir, rhs_ir, "multmp\0".as_ptr() as *const i8)
                        },
                        '/' => {
                            LLVMBuildFDiv(context.builder, lhs_ir, rhs_ir, "divtmp\0".as_ptr() as *const i8)
                        },
                        '>' => {
                            LLVMBuildFCmp(context.builder, LLVMRealOGT, lhs_ir, rhs_ir, "cmpgt\0".as_ptr() as *const i8)
                        },
                        '<' => {
                            LLVMBuildFCmp(context.builder, LLVMRealOLT, lhs_ir, rhs_ir, "cmplt\0".as_ptr() as *const i8)
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
                    panic!("Function {} called with unexpected number of arguments", callee);
                }

                let mut generated_args = Vec::new();
                for arg in args.iter() {
                    generated_args.push(arg.generate(context));
                }

                LLVMBuildCall2(context.builder,
                               LLVMTypeOf(func),
                               func,
                               generated_args.as_mut_ptr(),
                               call_arg_count,
                               "calltmp\0".as_ptr() as *const i8)
            },
            GenericAst::FunctionAst {proto, body} => {
                let proto_unboxed = &**proto;

                if let GenericAst::PrototypeAst { name, args : _} = proto_unboxed {
                    let mut func_proto = LLVMGetNamedFunction(
                        context.module,
                        name.as_ptr() as *const i8);
                    if func_proto.is_null() {
                        func_proto = proto.generate(context);
                    }

                    // TODO (saif) check if null again ?!
                    // TODO (saif) check if empty ?!
                    // let first_block = LLVMGetFirstBasicBlock(func_proto);
                    // TODO (saif) check for redefinition ?

                    let basic_block = LLVMAppendBasicBlockInContext(
                        context.context,
                        func_proto,
                        "entry\0".as_ptr() as *const i8);
                    LLVMPositionBuilderAtEnd(context.builder, basic_block);

                    // TODO (saif) consider clearing the named_values map ?
                    //context.named_values.clear();
                    for idx in 0..LLVMCountParams(func_proto)  {
                        let param = LLVMGetParam(func_proto, idx);
                        let mut length: usize = 0;
                        let name_buffer: *const c_char = unsafe { LLVMGetValueName2(param, &mut length) };
                        LLVMGetValueName2(param, &mut length);
                        println!("-- {}", CStr::from_ptr(name_buffer).to_str().unwrap());
                        context.named_values.insert(CStr::from_ptr(name_buffer).to_str().unwrap().to_string(), param);
                    }

                    let body_ir = body.generate(context);
                    // TODO (saif) optionals instead of nulls/panics?
                    if !body_ir.is_null() {
                        LLVMBuildRet(context.builder, body_ir);
                        LLVMVerifyFunction(func_proto, LLVMVerifierFailureAction::LLVMPrintMessageAction);
                    } else {
                        //erase?
                    }
                    return func_proto;
                } else {
                    panic!("Expected Prototype Ast!");
                }
            },
            GenericAst::PrototypeAst {name, args} => {
                // TODO (saif) remove assumption that our functions always return a float
                let return_type = LLVMBFloatTypeInContext(context.context);
                let mut arg_types = std::vec![LLVMBFloatTypeInContext(context.context); args.len()];

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