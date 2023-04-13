use crate::syntax::ast;
use crate::syntax::ast::GenericAst;

pub trait Generator {
    /*
        Learning Note:
        - Single Static Assignment (SSA)
            SSA is a form of intermediate representation (IR) used in compilers.
            SSA specifies that each variable is assigned only once, and that each assignment is a definition of the variable.
            Existing variables are not modified, but new variables are created to hold the results of expressions.
            Versioning is used to keep track of the different values of a variable.
            In other words, there is no way to change an SSA value.
    */

//     NumberExprAst { number: f64 },
// VariableExprAst { name: String },
// BinaryExprAst { op: char, lhs: Box<GenericAst>, rhs: Box<GenericAst> },
// CallExprAst { callee: String, args: Vec<GenericAst> },
// PrototypeAst { name: String, args: Vec<String> },
// FunctionAst { proto: Box<GenericAst>, body: Box<GenericAst> }

    unsafe fn generate(&self, ast: &GenericAst) -> llvm_sys::prelude::LLVMTypeRef {
        match ast {
            GenericAst::FunctionAst { proto, body } => {
                let _context = llvm_sys::core::LLVMGetGlobalContext();
                unsafe { llvm_sys::core::LLVMIntType(32) }
            }
            _ => {
                let _context = llvm_sys::core::LLVMGetGlobalContext();
                unsafe { llvm_sys::core::LLVMIntType(32) }
            }
        }
    }






}

// impl GenericAst {
//     /*
//         Learning Note:
//         - Single Static Assignment (SSA)
//             SSA is a form of intermediate representation (IR) used in compilers.
//             SSA specifies that each variable is assigned only once, and that each assignment is a definition of the variable.
//             Existing variables are not modified, but new variables are created to hold the results of expressions.
//             Versioning is used to keep track of the different values of a variable.
//             In other words, there is no way to change an SSA value.
//     */
//     pub unsafe fn codegen(&self) -> llvm_sys::prelude::LLVMTypeRef {
//         match self{
//             _ => {
//                 let _context = llvm_sys::core::LLVMGetGlobalContext();
//                 unsafe { llvm_sys::core::LLVMIntType(32) }
//             }
//         }
//     }
// }