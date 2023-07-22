# kaleidoscope

A run through https://llvm.org/docs/tutorial/MyFirstLanguageFrontend in Rust.

### Simple Expression Parsing without branching (19-02-2023)
- Supports '(', ')', '+', '-', '*', '/', '<', '>' as primary binary ops
- Structure of an Expression: LHS (BinOp) RHS
- LHS - Number / VariableReference / FunctionCall(Args: <Expression / Number / VariableReference / FunctionCall>)
- BinOp - One of the primary binary operators that are supported
- RHS - Expression / Number / VariableReference / FunctionCall(Args: <Expression / Number / VariableReference / FunctionCall>)

### Limitations
1. All integral types are float types
2. Usage of unwraps in the Lexer/Parser (TODO: remove this pattern - Handle None cases explicitly)
3. Can only use floats as arguments and return types
4. All functions return a mandatory float
5. Decouple Parser and Lexer functionality -> Let Lexer run through and drain all Tokens into the parser 

### Ideas
1. Pipeline Infrastructure (Stage 1 -> Stage 2 -> ... -> Stage N) 
   - Templated Input -> Templated Output in each stage 
   - Design such that arbitrary stages can be added constrained on whether their input and output times are compatible
2. Move to InkWell for IR Generation