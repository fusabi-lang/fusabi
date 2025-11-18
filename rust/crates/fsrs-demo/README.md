# FSRS Demo Host

Demo host application for the FSRS (F# Script Runtime System). This binary demonstrates the complete pipeline from Mini-F# source code to execution.

## Features

- **Complete Pipeline Integration**: Source → Lexer → Parser → Compiler → VM → Execution
- **CLI Interface**: Execute scripts from files or evaluate expressions directly
- **Bytecode Disassembly**: Optional disassembly output for debugging
- **Comprehensive Error Reporting**: Clear error messages at each pipeline stage

## Usage

```bash
# Run a script file
fsrs-demo examples/hello.fsrs

# Evaluate an expression directly
fsrs-demo -e "let x = 42 in x + 1"

# Show bytecode disassembly
fsrs-demo --disasm examples/arithmetic.fsrs

# Show help
fsrs-demo --help

# Show version
fsrs-demo --version
```

## Supported Features (Phase 1 MVP)

### Data Types
- **Integers**: `42`, `-5`, `0`
- **Booleans**: `true`, `false`
- **Strings**: `"hello world"`
- **Unit**: `()`

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparison**: `<`, `<=`, `>`, `>=`, `=`, `<>`
- **Logical**: `&&`, `||`

### Language Constructs
- **Let Bindings**: `let x = 42 in x + 1`
- **Conditionals**: `if x > 5 then 1 else 0`
- **Nested Expressions**: Full support for nested let and if

## Examples

### Hello World
```fsharp
"Hello, FSRS!"
```

### Arithmetic
```fsharp
let a = 10 in
let b = 5 in
a * 2 + b - 3
```

### Conditionals
```fsharp
let x = 42 in
let y = 17 in
if x > y then x else y
```

### Fibonacci
```fsharp
let f0 = 0 in
let f1 = 1 in
let f2 = f0 + f1 in
let f3 = f1 + f2 in
f3
```

## Error Handling

The demo provides clear error messages for:
- **Lexer errors**: Invalid tokens or characters
- **Parser errors**: Syntax errors
- **Compiler errors**: Undefined variables, too many constants/locals
- **Runtime errors**: Division by zero, type mismatches, stack underflow

Example:
```bash
$ fsrs-demo -e "x + 1"
Error: Compiler Error: Undefined variable: x
```

## Architecture

The demo uses a clean pipeline architecture:

1. **Lexical Analysis** (`fsrs_frontend::Lexer`): Tokenizes source code
2. **Parsing** (`fsrs_frontend::Parser`): Builds AST from tokens
3. **Compilation** (`fsrs_frontend::Compiler`): Generates bytecode from AST
4. **Execution** (`fsrs_vm::Vm`): Executes bytecode and returns result

## Development

### Building
```bash
cargo build --package fsrs-demo
```

### Testing
```bash
cargo test --package fsrs-demo
```

### Running Examples
```bash
cargo run --package fsrs-demo -- examples/hello.fsrs
```

## Implementation Notes

### Phase 1 Limitations
- **No Lambda Functions**: Function definitions and applications not yet supported
- **No Recursion**: Recursive functions not supported in Phase 1
- **No Type Annotations**: Type inference only
- **No Pattern Matching**: Simple expressions only

These features will be added in Phase 2.

### Compiler Fixes

This demo integration uncovered and fixed two compiler bugs:
1. **Let binding scope management**: Result values were being popped incorrectly
2. **If-then-else POPs**: Unnecessary POP instructions after JumpIfFalse

## Testing

The test suite includes:
- **38 integration tests** covering the full pipeline
- **Pipeline tests**: Literals, arithmetic, types
- **Let binding tests**: Simple, nested, shadowing
- **Conditional tests**: If-then-else with comparisons
- **Comparison tests**: All comparison operators
- **Logical tests**: Boolean operations
- **Error tests**: Lexer, parser, compiler, runtime errors
- **Complex tests**: Fibonacci, max, absolute value

All tests pass successfully.

## Contributing

See the main project README and CONTRIBUTING.md for development guidelines.

## License

See the main project LICENSE file.
