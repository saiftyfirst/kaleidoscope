use llvm_sys::prelude::LLVMTypeRef;

use crate::codegen::generator::Generator;
use crate::syntax::ast::GenericAst;

impl Generator<LLVMTypeRef> for GenericAst {
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
            _ => {
                let _context = llvm_sys::core::LLVMGetGlobalContext();
                unsafe { llvm_sys::core::LLVMIntType(32) }
            }
        }
    }
}