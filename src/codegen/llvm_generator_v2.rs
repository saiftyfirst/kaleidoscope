use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValue, AnyValueEnum, FloatMathValue, AnyValue};
use inkwell::{FloatPredicate};

use crate::codegen::IRGenerator;
use crate::syntax::ast::*;
use crate::syntax::vocabulary::SYMBOL_OP_CHARS;
use crate::error::{Error, PrefixedError};

struct LLVMGeneratorLocal<'ctx> {
    context: Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    /*
        Learning Note on lifetimes:
        In the line below, 'ctx is essentially imposing that both &str and &AnyValueEnum
        elements stored in the HashMap live at least as long as LLVMGeneratorLocal. AnyValueEnum
        stores a reference as well and hence must be passed the same lifetime in order to trickle down
        the lifetime imposition to the underlying reference
    */
    variable_references: HashMap<String, AnyValueEnum<'ctx>>,
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
                let context = Context::create();
                let module = context.create_module(module_identifier);
                let builder = context.create_builder();
                let variable_references = HashMap::new();

                LLVMGeneratorLocal { context, module, builder, variable_references }
            }
    }

    pub fn get_module_as_string(&self) -> String {
        self.module.to_string()
    }
}

impl<'ctx> IRGenerator<LLVMGeneratorLocal<'ctx>, Result<AnyValueEnum<'ctx>, Error>> for GenericAst
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
    // unsafe fn generate(&self, context: &mut C) -> T;

    unsafe fn generate(&self, generator_local: &mut LLVMGeneratorLocal) -> Result<AnyValueEnum<'ctx>, Error> {
        let f32_type = generator_local.context.f32_type();
        match self {
            GenericAst::NumberExprAst {number} => {
                Ok(f32_type.const_float(*number).as_any_value_enum()) // Limitation 1
            },
            GenericAst::VariableExprAst {name} => {
                if let Some(value) = generator_local.variable_references.get(name) {
                    Ok(*value) // Learning Note: since AnyValueEnum derives copy, this is a copy by default and not a move
                } else {
                    Err(Error::new(format!("Cannot refer to unknown variable '{}'", name)))
                    // Err(self.error(format!("Cannot refer to unknown variable '{}'", name)))
                }
            },
            GenericAst::BinaryExprAst { op, lhs, rhs} => {
                // TODO (safe) this into may panic
                let lhs_ir = lhs.generate(generator_local)?.into_float_value();
                let rhs_ir = rhs.generate(generator_local)?.into_float_value();

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
                            let float_add = generator_local.builder.build_float_add(lhs_ir, rhs_ir, "_fadd");
                            Ok(float_add.as_any_value_enum())
                        },
                        '-' => {
                            let float_sub = generator_local.builder.build_float_sub(lhs_ir, rhs_ir, "_fsub");
                            Ok(float_sub.as_any_value_enum())
                        },
                        '*' => {
                            let float_mul = generator_local.builder.build_float_mul(lhs_ir, rhs_ir, "_fmul");
                            Ok(float_mul.as_any_value_enum())
                        },
                        '/' => {
                            let float_div = generator_local.builder.build_float_div(lhs_ir, rhs_ir, "_fdiv");
                            Ok(float_div.as_any_value_enum())
                        },
                        '>' => {
                            let float_gt = generator_local.builder.build_float_compare(FloatPredicate::OGT, lhs_ir, rhs_ir, "_fgt");
                            Ok(float_gt.as_any_value_enum())
                        },
                        '<' => {
                            let float_lt = generator_local.builder.build_float_compare(FloatPredicate::OLT, lhs_ir, rhs_ir, "_flt");
                            Ok(float_lt.as_any_value_enum())
                        },
                        _ => {
                            Err(Error::new(format!("Cannot do computation with a non-arithmetic operator {}", op)))
                            // Err(self.error(format!("Cannot do computation with a non-arithmetic operator {}", op)))
                        }
                    }
                }
                else {
                    Err(Error::new(format!("Cannot do computation with an unknown operator {}", op)))
                    // TODO TODO TODO TODO (!!!CRITICAL)
                    // Err(self.error(format!("Cannot do computation with an unknown operator {}", op)))
                }
            },
            GenericAst::CallExprAst {callee, args} => {
                let function_option = generator_local.module.get_function(callee);

                if let Some(function) = function_option {
                    if function.count_params() != args.len() as u32 {
                        return Err(Error::new(format!("Cannot call function {} with {} parameters (requires {}).", args.len(), callee, function.count_params())));
                        // Err(self.error(format!("Cannot call function {} with {} parameters (requires {}).", args.len(), callee, function.count_params())))
                    }
                    let generated_args = args.iter().map(|arg| arg.generate(generator_local)).collect();
                    // TODO TODO TODO TODO (!!!CRITICAL)
                    // generator_local.builder.build_call(function, generated_args, format!("{}_call", callee))
                    Err(Error::new(format!("Cannot refer to undefined function {}", callee)))
                    // Err(self.error(format!("Cannot refer to undefined function {}", callee)))

                } else {
                    Err(Error::new(format!("Cannot refer to undefined function {}", callee)))
                    // Err(self.error(format!("Cannot refer to undefined function {}", callee)))
                }
            },
            GenericAst::FunctionAst {proto, body} => {
                let prototype_ast = &**proto;

                if let GenericAst::PrototypeAst { name, args : _} = prototype_ast {
                    let mut function_option = generator_local.module.get_function(name);
                    let function = match function_option {
                        Some(function) => function,
                        None => prototype_ast.generate(generator_local)?.into_function_value()
                    };

                    let basic_block = generator_local.context.append_basic_block(function, "_entry");
                    generator_local.builder.position_at_end(basic_block);
                    let body_ir = body.generate(generator_local)?;
                    // TODO (saif) remove assumption that functions cannot return nulls
                    generator_local.builder.build_return(Some(&body_ir))
                } else {
                    Err(self.error(format!("Cannot generate IR for faulty function prototype -- {}", prototype_ast)))
                }
            },
            GenericAst::PrototypeAst {name, args} => {
                // TODO (saif) remove assumption that our functions always take floats and returns a float
                let function_args = std::vec![f32_type.into(), args.len()];
                let function_signature = f32_type.fn_type(function_args.into(), false);

                /* Learning Note:
                    the prototype with name is not registered in the module's symbol table
                    until the function is defined.
                */
                // context.function_types.insert(name.clone(), function_type);
                let function = generator_local.module.add_function(name, function_signature, None);
                // set the names of the variables and keep copies of the parameter references
                for (param, identifier) in  function.get_params().iter().zip(args.iter()) {
                    param.set_name(identifier.as_str());
                    generator_local.variable_references.insert(format!("{}_{}", name, identifier), param)
                }
                function
            }
        }
    }
}

// impl<'ctx> PrefixedError for dyn IRGenerator<LLVMGeneratorLocal<'ctx>, Result<AnyValueEnum<'ctx>, Error>> {
//     fn get_prefix(&self) -> &str {
//         "LLVM IR Generation Failed: "
//     }
// }