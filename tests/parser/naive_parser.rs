#[cfg(test)]
mod tests {
    use kaleidoscope::syntax::ast::GenericAst;
    use kaleidoscope::parse::parser::*;

    /*
        Learning Notes on Rust Macros Placeholder Syntax:
            $x:expr: Matches and captures any Rust expression.
            $stmt:stmt: Matches and captures any statement (e.g., variable binding, function call, etc.).
            $pat:pat: Matches and captures a pattern (e.g., used in match expressions).
            $ident:ident: Matches and captures an identifier (e.g., variable or function name).
            $block:block: Matches and captures a block of code enclosed in curly braces.
            $item:item: Matches and captures an item (e.g., function, struct, trait, etc.).
            $ty:ty: Matches and captures a type.
            $path:path: Matches and captures a path (e.g., module path, function path).
            $vis:vis: Matches and captures a visibility qualifier (e.g., pub, crate).
    */
    macro_rules! base_passing_parser_test {
        ($name:ident, $src:expr, $count:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let mut parser = Parser::new($src);

                for i in 0..$count {
                    let got = parser.build_next_ast().unwrap();
                    assert_eq!(got, $should_be[i]);
                }
            }
        }
    }

    base_passing_parser_test!(
        can_parse_no_arg_extern,
        r###"
            extern atan2()
        "###, 1 =>
        vec![GenericAst::PrototypeAst { name: "atan2".to_string(), args: vec![] }]
    );

    base_passing_parser_test!(
        can_parse_multi_arg_extern,
        r###"
            extern atan2(arg, arg2)
        "###, 1 =>
        vec![GenericAst::PrototypeAst { name: "atan2".to_string(), args: vec!["arg".to_string(), "arg2".to_string()] }]
    );

    base_passing_parser_test!(
        can_parse_basic_arithmetic_expression,
        r###"
            x + 1
        "###, 1 =>
        vec![GenericAst::BinaryExprAst { op: '+', lhs: Box::new(GenericAst::VariableExprAst { name: "x".to_string() }), rhs: Box::new(GenericAst::NumberExprAst { number: 1.0 }) }]
    );

    base_passing_parser_test!(
        can_parse_chained_arithmetic_expression,
        r###"
            x + 2 -4 * q / y
        "###, 1 =>
        vec![
            GenericAst::BinaryExprAst {
                op: '-',
                lhs: Box::new(Parser::new("x + 2").build_next_ast().unwrap()),
                rhs: Box::new(Parser::new("4 * q / y").build_next_ast().unwrap())
            }
        ]
    );

    base_passing_parser_test!(
        can_parse_precedence_changing_arithmetic_expression,
        r###"
            x + 2 -4 * q / y + 2
        "###, 1 =>
        vec![
            GenericAst::BinaryExprAst {
                op: '+',
                lhs: Box::new(GenericAst::BinaryExprAst {
                    op: '-',
                    lhs: Box::new(Parser::new("x + 2").build_next_ast().unwrap()),
                    rhs: Box::new(Parser::new("4 * q / y").build_next_ast().unwrap())
                }),
                rhs: Box::new(GenericAst::NumberExprAst { number: 2.0 })
          }
        ]
    );

    base_passing_parser_test!(
        can_parse_parethesis_containing_arithmetic_expression,
        r###"
            x / (2 - 4 + q) / y
        "###, 1 =>
        vec![
            GenericAst::BinaryExprAst {
                op: '/',
                lhs: Box::new(GenericAst::BinaryExprAst {
                    op: '/',
                    lhs:Box::new(GenericAst::VariableExprAst { name: "x".to_string() }),
                    rhs: Box::new(Parser::new("2 - 4 + q").build_next_ast().unwrap())
                }),
                rhs: Box::new(GenericAst::VariableExprAst { name: "y".to_string() })
            }
        ]
    );

    base_passing_parser_test!(
        can_parse_parse_compound_b2b_statements,
        r###"
            x * (z * q) / y
            extern atan2(arg, arg2)
        "###, 2 =>
        vec![
            GenericAst::BinaryExprAst {
                op: '/',
                lhs: Box::new(GenericAst::BinaryExprAst {
                    op: '*',
                    lhs:Box::new(GenericAst::VariableExprAst { name: "x".to_string() }),
                    rhs: Box::new(Parser::new("(z * q)").build_next_ast().unwrap())
                }),
                rhs: Box::new(GenericAst::VariableExprAst { name: "y".to_string() })
            },
            GenericAst::PrototypeAst { name: "atan2".to_string(), args: vec!["arg".to_string(), "arg2".to_string()] }
        ]
    );

    base_passing_parser_test!(
        can_parse_single_argument_function,
        r###"
            def my_tan(arg1)
                arg1
        "###, 1 =>
        vec![
            GenericAst::FunctionAst {
                proto: Box::new(GenericAst::PrototypeAst { name: "my_tan".to_string(), args: vec!["arg1".to_string()] }),
                body: Box::new(GenericAst::VariableExprAst { name: "arg1".to_string() })
            }
        ]
    );

    base_passing_parser_test!(
        can_parse_multi_argument_function,
        r###"
            def my_tan(arg1, arg2)
                arg1 + arg2
        "###, 1 =>
        vec![
            GenericAst::FunctionAst {
                proto: Box::new(GenericAst::PrototypeAst { name: "my_tan".to_string(), args: vec!["arg1".to_string(), "arg2".to_string()] }),
                body: Box::new(GenericAst::BinaryExprAst {
                    op: '+',
                    lhs:Box::new(GenericAst::VariableExprAst { name: "arg1".to_string() }),
                    rhs:Box::new(GenericAst::VariableExprAst { name: "arg2".to_string() }),
                }),
            }
        ]
    );
}