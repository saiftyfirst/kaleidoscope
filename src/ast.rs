pub enum GenericAst {
    NumberExprAst { number: f64 },
    VariableExprAst { name: String },
    BinaryExprAst { op: char, lhs: Box<GenericAst>, rhs: Box<GenericAst> },
    CallExprAst { callee: String, args: Vec<String> },
}

pub struct PrototypeAst {
    name: String,
    args: Vec<String>
}

pub struct FunctionAst {
    proto: PrototypeAst,
    body: GenericAst
}

impl From<&String> for PrototypeAst {
    fn from(value: &String) -> Self {
        PrototypeAst {
            name: value.to_string(),
            args: vec![]
        }
    }
}

impl PrototypeAst {
    pub fn add_arg(&mut self, arg: &String) {
        self.args.push(arg.to_string());
    }
}
