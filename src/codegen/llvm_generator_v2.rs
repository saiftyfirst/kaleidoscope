use std::collections::HashMap;
use std::rc::Rc;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValue, AnyValueEnum, FloatMathValue, AnyValue, BasicValueEnum};
use inkwell::{FloatPredicate};

use crate::codegen::IRGenerator;
use crate::syntax::ast::*;
use crate::syntax::vocabulary::SYMBOL_OP_CHARS;
use crate::error::{Error, PrefixedError};
use crate::syntax::ast::*;

struct LLVMGeneratorLocal<'ctx>
{
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    builder: &'ctx Builder<'ctx>,

    /*
        Learning Note on lifetimes:
        In the line below, 'ctx is essentially imposing that both &str and &AnyValueEnum
        elements stored in the HashMap live at least as long as LLVMGeneratorLocal. AnyValueEnum
        stores a reference as well and hence must be passed the same lifetime in order to trickle down
        the lifetime imposition to the underlying reference
    */
    variable_references: HashMap<String, BasicValueEnum<'ctx>>,
}

impl<'ctx> LLVMGeneratorLocal<'ctx>
{
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>, builder: &'ctx Builder<'ctx>) -> LLVMGeneratorLocal<'ctx>
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
                    context,
                    module,
                    builder,
                    variable_references: HashMap::new()
                }
            }
    }

    pub fn get_module_as_string(&self) -> String
    {
        self.module.to_string()
    }
}

impl<'ctx> IRGenerator<LLVMGeneratorLocal<'ctx>, Result<BasicValueEnum<'ctx>, Error>> for ExprAst
{
    unsafe fn generate(&self, generator_local: &mut LLVMGeneratorLocal<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>
    {
        let f32_type = generator_local.context.f32_type();
        match self
        {
            ExprAst::NumberExpr { number } =>
            {
                Ok(f32_type.const_float(*number).as_basic_value_enum()) // Limitation 1
            },
            ExprAst::VariableExpr { name } =>
            {
                if let Some(value) = generator_local.variable_references.get(name)
                {
                    Ok(*value) // Learning Note: since AnyValueEnum derives copy, this is a copy by default and not a move
                }
                else
                {
                    Err(Error::new(format!("Cannot refer to unknown variable '{}'", name)))
                    // Err(self.error(format!("Cannot refer to unknown variable '{}'", name)))
                }
            },
            ExprAst::BinaryExpr { op, lhs, rhs } =>
            {
                // TODO (safe) this into may panic
                let lhs_ir = lhs.generate(generator_local)?.into_float_value();
                let rhs_ir = rhs.generate(generator_local)?.into_float_value();

                // TODO (safe) assert the types ?
                if SYMBOL_OP_CHARS.contains(op)
                {
                    match op
                    {
                        '+' =>
                        {
                            /*
                                Learning Note: Why is the builder passed in ?
                                The builder is used to construct LLVM instructions within a basic block.
                                The builder keeps track of the current insertion point in the basic block and
                                is responsible for generating and appending the LLVM instruction to the block.
                            */
                            let float_add = generator_local.builder.build_float_add(lhs_ir, rhs_ir, "_fadd");
                            Ok(float_add.as_basic_value_enum())
                        },
                        '-' =>
                        {
                            let float_sub = generator_local.builder.build_float_sub(lhs_ir, rhs_ir, "_fsub");
                            Ok(float_sub.as_basic_value_enum())
                        },
                        '*' =>
                        {
                            let float_mul = generator_local.builder.build_float_mul(lhs_ir, rhs_ir, "_fmul");
                            Ok(float_mul.as_basic_value_enum())
                        },
                        '/' =>
                        {
                            let float_div = generator_local.builder.build_float_div(lhs_ir, rhs_ir, "_fdiv");
                            Ok(float_div.as_basic_value_enum())
                        },
                        '>' =>
                        {
                            let float_gt = generator_local.builder.build_float_compare(FloatPredicate::OGT, lhs_ir, rhs_ir, "_fgt");
                            Ok(float_gt.as_basic_value_enum())
                        },
                        '<' =>
                        {
                            let float_lt = generator_local.builder.build_float_compare(FloatPredicate::OLT, lhs_ir, rhs_ir, "_flt");
                            Ok(float_lt.as_basic_value_enum())
                        },
                        _ =>
                        {
                            Err(Error::new(format!("Cannot do computation with a non-arithmetic operator {}", op)))
                            // Err(self.error(format!("Cannot do computation with a non-arithmetic operator {}", op)))
                        }
                    }
                } else
                {
                    Err(Error::new(format!("Cannot do computation with an unknown operator {}", op)))
                    // TODO TODO TODO TODO (!!!CRITICAL)
                    // Err(self.error(format!("Cannot do computation with an unknown operator {}", op)))
                }
            },
            ExprAst::CallExpr { callee, args } =>
            {
                let function_option = generator_local.module.get_function(callee);

                if let Some(function) = function_option
                {
                    if function.count_params() != args.len() as u32
                    {
                        return Err(Error::new(format!("Cannot call function {} with {} parameters (requires {}).", args.len(), callee, function.count_params())));
                        // Err(self.error(format!("Cannot call function {} with {} parameters (requires {}).", args.len(), callee, function.count_params())))
                    }
                    let generated_args: Vec<_> = args.iter().map(|arg| arg.generate(generator_local)).collect();
                    // TODO TODO TODO TODO (!!!CRITICAL)
                    // generator_local.builder.build_call(function, generated_args, format!("{}_call", callee))
                    Err(Error::new(format!("Cannot refer to undefined function {}", callee)))
                    // Err(self.error(format!("Cannot refer to undefined function {}", callee)))
                } else
                {
                    Err(Error::new(format!("Cannot refer to undefined function {}", callee)))
                    // Err(self.error(format!("Cannot refer to undefined function {}", callee)))
                }
            }
        }
    }
}

impl<'ctx> IRGenerator<LLVMGeneratorLocal<'ctx>, Result<AnyValueEnum<'ctx>, Error>> for Function {
    unsafe fn generate(&self, generator_local: &mut LLVMGeneratorLocal<'ctx>) -> Result<AnyValueEnum<'ctx>, Error> {
        let prototype_ast = self.get_proto();;

        let mut function_option = generator_local.module.get_function(prototype_ast.get_name());
        let function = match function_option
        {
            Some(function) => function,
            None => prototype_ast.generate(generator_local)?.into_function_value()
        };

        let basic_block = generator_local.context.append_basic_block(function, "_entry");
        generator_local.builder.position_at_end(basic_block);
        let body_ir = self.get_body().generate(generator_local)?;
        // TODO (saif) remove assumption that functions cannot return nulls
        Ok(generator_local.builder.build_return(Some(&body_ir)).as_any_value_enum())
    }
}

impl<'ctx> IRGenerator<LLVMGeneratorLocal<'ctx>, Result<AnyValueEnum<'ctx>, Error>> for Prototype
{
    unsafe fn generate(&self, generator_local: &mut LLVMGeneratorLocal<'ctx>) -> Result<AnyValueEnum<'ctx>, Error> {
        let f32_type = generator_local.context.f32_type();
        // TODO (saif) remove assumption that our functions always take floats and returns a float
        let function_args = std::vec![f32_type.into(); self.get_args().len()];
        let function_signature = f32_type.fn_type(function_args.as_slice(), false);

        /* Learning Note:
           the prototype with name is not registered in the module's symbol table
            until the function is defined.
        */
        let function_name = self.get_name();
        // generator_local.context.function_types.insert(function_name, function_type);
        let function = generator_local.module.add_function(function_name, function_signature, None);
        // set the names of the variables and keep copies of the parameter references
        for (param, identifier) in function.get_params().iter().zip(self.get_args().iter())
        {
            param.set_name(identifier.as_str());
            generator_local.variable_references.insert(format!("{}_{}", function, identifier), *param);
        }
        Ok(function.as_any_value_enum())
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


    unsafe fn generate(&self, generator_local: &mut LLVMGeneratorLocal<'ctx>) -> Result<AnyValueEnum<'ctx>, Error> {
        self.generate(generator_local)
    }
}