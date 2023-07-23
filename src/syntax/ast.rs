use crate::utils::display;

/*
    Learning Note:
        Default implementations are derived when we use the above derive macro.
        PartialEq, for example, will loosely implement a field by field comparision.
*/
#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum ExprAst {
    NumberExpr { number: f64 },
    VariableExpr { name: String },
    BinaryExpr { op: char, lhs: Box<ExprAst>, rhs: Box<ExprAst> },
    CallExpr { callee: String, args: Vec<ExprAst> }
}

#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum FuncAst {
    Prototype { name: String, args: Vec<String> },
    Function { proto: Box<FuncAst>, body: Box<ExprAst> }
}

#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum GenericAst {
    ExprAst(ExprAst),
    FuncAst(FuncAst)
}

impl std::fmt::Display for ExprAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprAst::NumberExpr { number } => write!(f, "{}", number),
            ExprAst::VariableExpr { name } => write!(f, "{}", name),
            ExprAst::BinaryExpr { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            ExprAst::CallExpr { callee, args } => {
                write!(f, "{}(", callee)?;
                display::structured_slice_print(args, f)
            }
        }
    }
}

impl std::fmt::Display for FuncAst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncAst::Prototype { name, args } => {
                write!(f, "def {}(", name)?;
                display::structured_slice_print(args, f)
            },
            FuncAst::Function { proto, body } => {
                write!(f, "{}\n", proto)?;
                write!(f, "\t{}", body)
            }
        }
    }
}