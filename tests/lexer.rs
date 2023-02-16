#[cfg(test)]
mod tests {
    use kaleidoscope::lexer::*;

    macro_rules! single_tokenization_test {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let mut tokenizer = Tokenizer::new($src);

                let got = tokenizer.parse_next_token();

                assert_eq!(got, $should_be);
            }
        }
    }

    single_tokenization_test!(can_tokenize_empty, "" => Token::TokEof);
    single_tokenization_test!(can_tokenize_open_parenthesis, " ( " => Token::TokPrimary('('));
    single_tokenization_test!(can_tokenize_close_parenthesis, " ) " => Token::TokPrimary(')'));
    single_tokenization_test!(can_tokenize_op_add, " + " => Token::TokPrimary('+'));
    single_tokenization_test!(can_tokenize_op_sub, " - " => Token::TokPrimary('-'));
    single_tokenization_test!(can_tokenize_op_mul, " * " => Token::TokPrimary('*'));
    single_tokenization_test!(can_tokenize_op_div, " / " => Token::TokPrimary('/'));
    single_tokenization_test!(can_tokenize_op_le, " < " => Token::TokPrimary('<'));
    single_tokenization_test!(can_tokenize_op_ge, " > " => Token::TokPrimary('>'));
    single_tokenization_test!(can_tokenize_eof, " " => Token::TokEof);
    single_tokenization_test!(can_tokenize_float, "   1.6   " => Token::TokNumber(1.6));
    single_tokenization_test!(can_tokenize_def, " def " => Token::TokDef);
    single_tokenization_test!(can_tokenize_extern, " extern " => Token::TokExtern);
    single_tokenization_test!(can_tokenize_strings, " saiftyfirst " => Token::TokIdentifier(String::from("saiftyfirst")));
    single_tokenization_test!(can_tokenize_comments, " # defo herlmeer weg\n" => Token::TokComment(String::from("# defo herlmeer weg")));
}