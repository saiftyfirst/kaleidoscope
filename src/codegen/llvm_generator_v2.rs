use std::collections::HashMap;
use std::ffi::{CStr};
use std::os::raw::{c_char};
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};

// llvm-sys
use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::LLVMRealPredicate::{LLVMRealOGT, LLVMRealOLT};

use crate::codegen::IRGenerator;
use crate::syntax::ast::*;
use crate::syntax::vocabulary::SYMBOL_OP_CHARS;

// Generation 2
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::FloatValue;
use inkwell::{FloatPredicate};

macro_rules! panic_with_reason {
    ($reason:expr, $($arg:tt)*) => ({
        panic!(concat!("IR Generation Failed: ", $reason), $($arg)*);
    });
}

enum InkwellType {
    FloatValue,
}

struct LLVMGeneratorLocal<'ctx> {
    context: Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    named_variables: HashMap<&'ctx str, InkwellType>,
    function_types: HashMap<&'ctx str, LLVMTypeRef>
}

impl<'ctx> LLVMGeneratorLocal<'ctx>
{
    pub fn new(module_identifier: &str) -> LLVMGeneratorLocal
    {
        unsafe
            {
                /* Learning Note:
                    The context is used to hold and manage various LLVM **objects and data structures**.
                    The builder is used to construct **LLVM instructions** within a basic block.
                    A module is a container for **LLVM functions and global variables**.
                    LLVM values or variables.
                */
                LLVMGeneratorLocal
                {
                    context: Context::create(),
                    module: Self::context.create_module(module_identifier),
                    builder: Self::context.create_builder(),
                    named_variables: HashMap::new(),
                    function_types: HashMap::new()
                }
            }
    }

    pub fn get_module_as_string(&self) -> String {
        self.module.to_string()
    }
}

impl<'ctx> IRGenerator<LLVMGeneratorLocal<'ctx>, InkwellType> for GenericAst
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
    unsafe fn generate(&self, gen_local: &mut LLVMGeneratorLocal) -> InkwellType {
        let f32_type = gen_local.context.f32_type();


        match self {
            GenericAst::NumberExprAst {number} => {
                f32_type.const_float(*number) // Limitation 1
            },
            GenericAst::VariableExprAst {name} => {
                if let Some(value) = gen_local.named_variables.get(name) {
                    *value
                } else {
                    panic_with_reason!("Cannot refer to unknown variable: {}", name);
                }
            },
            GenericAst::BinaryExprAst {op, lhs, rhs} => {
                let lhs_ir = lhs.generate(gen_local);
                let rhs_ir = rhs.generate(gen_local);

                // TODO (safe) assert the types ?
                if SYMBOL_OP_CHARS.contains(op) {
                    match op {
                        '+' => {
                            /*
                                Learning Note: Why is the builder passed in ?
                                The builder is used to construct LLVM instructions within a basic block.
                                The builder keeps track of the current insertion point in the basic block and
                                is responsible for generating and appending the LLVM instruction to the block.
                            */
                            gen_local.builder.build_float_add(lhs_ir, rhs_ir, "add")
                        },
                        '-' => {
                            gen_local.builder.build_float_sub(lhs_ir, rhs_ir, "sub")
                        },
                        '*' => {
                            gen_local.builder.build_float_mul(lhs_ir, rhs_ir, "mul")
                        },
                        '/' => {
                            gen_local.builder.build_float_div(lhs_ir, rhs_ir, "div")
                        },
                        '>' => {
                            gen_local.builder.build_float_compare(FloatPredicate::OGT, lhs_ir, rhs_ir, "gt")
                        },
                        '<' => {
                            gen_local.builder.build_float_compare(FloatPredicate::OLT, lhs_ir, rhs_ir, "lt")
                        },
                        _ => {
                            panic_with_reason!("Cannot do computation with a non-arithmetic operator {}", op)
                        }
                    }
                }
                else {
                    panic_with_reason!("Cannot do computation with an unknown operator {}", op)
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

                // let mut type_arr: [LLVMTypeRef; 2] = [LLVMBFloatTypeInContext(context.context), LLVMBFloatTypeInContext(context.context)];
                // let _params = LLVMGetParamTypes(LLVMTypeOf(func), type_arr.as_mut_ptr());

                for arg in args.iter() {
                    println!("Generated arg: {:?}", arg);
                    generated_args.push(arg.generate(context));
                }

                let function_type = *context.function_types.get(callee).unwrap();

                LLVMBuildCall2(context.builder,
                               function_type,
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
                        context.named_values.insert(CStr::from_ptr(name_buffer).to_str().unwrap().to_string(), param);
                    }

                    let body_ir = body.generate(context);
                    // TODO (saif) optionals instead of nulls/panics?
                    if !body_ir.is_null() {
                        LLVMBuildRet(context.builder, body_ir);

                        context.named_values.insert("cache\0".to_string(), body_ir);
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
                let function_type = LLVMFunctionType(return_type,
                                                     arg_types.as_mut_ptr(),
                                                     args.len() as u32,
                                                     0);
                context.function_types.insert(name.clone(), function_type);
                let func_proto = LLVMAddFunction(context.module,
                                                 name.as_ptr() as *const i8,
                                                 function_type);

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