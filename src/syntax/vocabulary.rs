pub const SYMBOL_NON_OP_CHARS: &'static [char; 3] = &['(', ')', ','];
pub const SYMBOL_OP_CHARS: &'static [char; 6] = &['+', '-', '*', '/', '>', '<'];
pub fn is_symbol_char(c: char) -> bool {
    SYMBOL_NON_OP_CHARS.contains(&c) || SYMBOL_OP_CHARS.contains(&c)
}
pub fn get_op_precedence(op: &char) -> i8 {
    match op {
        '<' | '>' => 10,
        '+' | '-' => 20,
        '*' | '/' => 30,
        _ => -1
    }
}