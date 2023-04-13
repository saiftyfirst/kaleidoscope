#[cfg(test)]
mod tests {
    use kaleidoscope::syntax::ast::GenericAst;
    use kaleidoscope::parse::parser::*;

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
}


