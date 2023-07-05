# kaleidoscope

A run through https://llvm.org/docs/tutorial/MyFirstLanguageFrontend in Rust.

### Simple Expression Parsing without branching (19-02-2023)
- Supports '(', ')', '+', '-', '*', '/', '<', '>' as primary binary ops
- Structure of an Expression: LHS (BinOp) RHS
- LHS - Number / VariableReference / FunctionCall(Args: <Expression / Number / VariableReference / FunctionCall>)
- BinOp - One of the primary binary operators that are supported
- RHS - Expression / Number / VariableReference / FunctionCall(Args: <Expression / Number / VariableReference / FunctionCall>)

### Limitations
- Can only use floats as arguments and return types
- All functions return a mandatory float

### Ideas
1. Pipeline Infrastructure (Stage 1 -> Stage 2 -> ... -> Stage N) 
   - Templated Input -> Templated Output in each stage 
   - Design such that arbitrary stages can be added constrained on whether their input and output times are compatible
2. Move to InkWell for IR Generation