use crate::syntax::ast;
use crate::syntax::ast::GenericAst;

pub trait Generator<T> {

//     NumberExprAst { number: f64 },
// VariableExprAst { name: String },
// BinaryExprAst { op: char, lhs: Box<GenericAst>, rhs: Box<GenericAst> },
// CallExprAst { callee: String, args: Vec<GenericAst> },
// PrototypeAst { name: String, args: Vec<String> },
// FunctionAst { proto: Box<GenericAst>, body: Box<GenericAst> }

    unsafe fn generate(&self, ast: &GenericAst) -> T {
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