#[cfg(test)]
mod tests {
    use kaleidoscope::parser::*;

    #[test]
    fn parser_driver() {
        let src = r###"
            def fib(x)
                if x < 3 then
                    1
                else
                    fib(x-1) + fib(x-2)

            fib(40)
        "###;
        let mut parser = Parser::new(src);
        parser.build_ast();
    }
}