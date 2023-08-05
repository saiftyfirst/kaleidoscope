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

#[derive(PartialEq, Debug, Clone)]
pub struct Prototype {
    name: String,
    args: Vec<String>
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    proto: Prototype,
    body: Box<ExprAst>
}

#[repr(i8)]
#[derive(PartialEq, Debug, Clone)]
pub enum GenericAst {
    ExprAst(ExprAst),
    PrototypeAst(Prototype),
    FuncAst(Function)
}

impl Prototype
{
    pub fn new(name: String, args: Vec<String>) -> Prototype
    {
        Prototype { name, args }
    }

    pub fn get_name(&self) -> &str
    {
        self.name.as_str()
    }

    pub fn get_args(&self) -> &[String]
    {
        self.args.as_slice()
    }
}

impl Function
{
    pub fn new(proto: Prototype, body: Box<ExprAst>) -> Function
    {
        Function { proto, body }
    }

    pub fn get_proto(&self) -> &Prototype
    {
        &self.proto
    }

    pub fn get_body(&self) -> &ExprAst
    {
        &self.body
    }
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

impl std::fmt::Display for Prototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "def {}(", self.name)?;
        display::structured_slice_print(self.args.as_slice(), f)
    }
}

impl<'ctx> std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.proto)?;
        write!(f, "\t{}", self.body)
    }
}