#[cfg(test)]
mod tests {
    use kaleidoscope::parse::lexer::*;
    use kaleidoscope::parse::token::*;

    macro_rules! single_tokenization_test {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let mut tokenizer = Lexer::new($src);

                let got = tokenizer.pop();

                assert_eq!(got, $should_be);
            }
        }
    }

    single_tokenization_test!(can_tokenize_empty, "" => Token::TokEof);
    single_tokenization_test!(can_tokenize_open_parenthesis, " ( " => Token::TokSymbol('('));
    single_tokenization_test!(can_tokenize_close_parenthesis, " ) " => Token::TokSymbol(')'));
    single_tokenization_test!(can_tokenize_op_add, " + " => Token::TokSymbol('+'));
    single_tokenization_test!(can_tokenize_op_sub, " - " => Token::TokSymbol('-'));
    single_tokenization_test!(can_tokenize_op_mul, " * " => Token::TokSymbol('*'));
    single_tokenization_test!(can_tokenize_op_div, " / " => Token::TokSymbol('/'));
    single_tokenization_test!(can_tokenize_op_le, " < " => Token::TokSymbol('<'));
    single_tokenization_test!(can_tokenize_op_ge, " > " => Token::TokSymbol('>'));
    single_tokenization_test!(can_tokenize_op_comma, " , " => Token::TokSymbol(','));
    single_tokenization_test!(can_tokenize_eof, "  " => Token::TokEof);
    single_tokenization_test!(can_tokenize_float, "   1.6   " => Token::TokNumber(1.6));
    single_tokenization_test!(can_tokenize_def, " def " => Token::TokDef);
    single_tokenization_test!(can_tokenize_extern, " extern " => Token::TokExtern);
    single_tokenization_test!(can_tokenize_strings, " saiftyfirst " => Token::TokIdentifier("saiftyfirst".to_string()));
    single_tokenization_test!(can_tokenize_atan2, " atan2 " => Token::TokIdentifier("atan2".to_string()));
    single_tokenization_test!(can_tokenize_comments, " # defo herlmeer weg\n" => Token::TokComment("# defo herlmeer weg".to_string()));
}