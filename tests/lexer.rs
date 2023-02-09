#[cfg(test)]
mod tests {
    use kaleidoscope::lexer::*;

    #[test]
    fn can_parse_eof() {
        let snippet = String::from("    ");
        let _snip_tok = parse_next_token(snippet.as_str()).0;
        matches!(Token::TokEof, _snip_tok);
    }

    #[test]
    fn can_parse_keyword_def() {
        let snippet = String::from("  def ");
        let _snip_tok = parse_next_token(snippet.as_str()).0;
        assert_eq!(Token::TokDef, _snip_tok);
    }

    #[test]
    fn can_parse_keyword_extern() {
        let snippet = String::from("  extern ");
        let _snip_tok = parse_next_token(snippet.as_str()).0;
        assert_eq!(Token::TokExtern, _snip_tok);
    }

    #[test]
    fn can_parse_strings() {
        let snippet = String::from("  defo ");
        let _snip_tok = parse_next_token(snippet.as_str()).0;
        assert_eq!(Token::TokIdentifier("defo".to_string()), _snip_tok);
    }

    #[test]
    fn can_parse_comment() {
        let snippet = String::from("  # defo herlmeer weg ");
        let _snip_tok = parse_next_token(snippet.as_str()).0;
        assert_eq!(Token::TokComment("# defo herlmeer weg ".to_string()), _snip_tok);
    }
}