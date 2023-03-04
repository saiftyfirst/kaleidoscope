# kaleidoscope

A run through https://llvm.org/docs/tutorial/MyFirstLanguageFrontend in Rust.

### Simple Expression Parsing without branching (19-02-2023)
- Supports '(', ')', '+', '-', '*', '/', '<', '>' as primary binary ops
- Structure of an Expression: LHS (BinOp) RHS
- LHS - Number / VariableReference / FunctionCall(Args: <Expression / Number / VariableReference / FunctionCall>)
- BinOp - One of the primary binary operators that are supported
- RHS - Expression / Number / VariableReference / FunctionCall(Args: <Expression / Number / VariableReference / FunctionCall>)

### Refactoring:
- Reorganise parse
- Remove unnecessary clones
- Check usage of unwraps
- Use str instead of String
- Do Token specific checks as impl for Token (e.g. isParenthesis)