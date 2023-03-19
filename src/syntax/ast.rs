use crate::utils::display;

#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum GenericAst {
    NumberExprAst { number: f64 },
    VariableExprAst { name: String },
    BinaryExprAst { op: char, lhs: Box<GenericAst>, rhs: Box<GenericAst> },
    CallExprAst { callee: String, args: Vec<GenericAst> },
    PrototypeAst { name: String, args: Vec<String> },
    FunctionAst { proto: Box<GenericAst>, body: Box<GenericAst> }
}

impl GenericAst {
    pub unsafe fn codegen(&self) -> llvm_sys::prelude::LLVMTypeRef {
        match self{
            _ => {
                let _context = llvm_sys::core::LLVMGetGlobalContext();
                unsafe { llvm_sys::core::LLVMIntType(32) }
            }
        }
    }
}

impl std::fmt::Display for GenericAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let x = vec![1, 2, 3];
        for (i, n) in x.iter().enumerate() {
            println!("{} - {}", i, n);
        }

        match self {
            GenericAst::NumberExprAst { number } => write!(f, "{}", number),
            GenericAst::VariableExprAst { name } => write!(f, "{}", name),
            GenericAst::BinaryExprAst { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            GenericAst::CallExprAst { callee, args } => {
                write!(f, "{}(", callee)?;
                display::structured_slice_print(args, f)
            },
            GenericAst::PrototypeAst { name, args } => {
                write!(f, "{}(", name)?;
                display::structured_slice_print(args, f)
            },
            GenericAst::FunctionAst { proto, body } => {
                write!(f, "{}\n", proto)?;
                write!(f, "{}\n", body)
            }
        }
    }
}