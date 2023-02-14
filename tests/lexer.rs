#[cfg(test)]
mod tests {
    use kaleidoscope::lexer::*;

    macro_rules! single_tokenize_test {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let mut tokenizer = Tokenizer::new($src);

                let got = tokenizer.parse_next_token();

                assert_eq!(got, $should_be);
            }
        }
    }

    single_tokenize_test!(can_tokenize_eof, " " => Token::TokEof);
    single_tokenize_test!(can_tokenize_float, "   1.6   " => Token::TokNumber(1.6));
    single_tokenize_test!(can_tokenize_def, "def " => Token::TokDef);
    single_tokenize_test!(can_tokenize_extern, "extern " => Token::TokExtern);
    single_tokenize_test!(can_tokenize_strings, "saiftyfirst " => Token::TokIdentifier(String::from("saiftyfirst")));
    single_tokenize_test!(can_tokenize_comments, "# defo herlmeer weg  \n" => Token::TokComment(String::from("# defo herlmeer weg  ")));
}