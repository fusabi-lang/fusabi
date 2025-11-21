# Contributing to Fusabi

Welcome to the Fusabi project! This guide will help you understand our architecture, development processes, and how to contribute effectively.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Project Structure](#project-structure)
3. [Development Setup](#development-setup)
4. [Adding New Features](#adding-new-features)
5. [Testing Guidelines](#testing-guidelines)
6. [Code Style](#code-style)
7. [PR Process](#pr-process)
8. [Community Guidelines](#community-guidelines)

## Architecture Overview

Fusabi implements a 3-layer architecture designed for clarity, modularity, and extensibility:

### Layer 1: Frontend (Parsing & Compilation)
**Crate:** `fusabi-frontend`

The frontend layer transforms F# source code into bytecode:

```
Source Code → Lexer → Tokens → Parser → AST → Type Checker → Compiler → Bytecode
```

#### Components:
- **Lexer** (`lexer.rs`): Tokenizes source text into a stream of tokens
  - Handles F# syntax including indentation-sensitive parsing
  - Supports string interpolation, operators, and keywords
  - Location tracking for error reporting

- **Parser** (`parser.rs`): Builds Abstract Syntax Tree (AST)
  - Recursive descent parser with precedence climbing
  - Handles F# constructs: let bindings, functions, pattern matching
  - Supports discriminated unions, records, and modules

- **Type Inference** (`inference.rs`): Hindley-Milner type system
  - Type variable generation and unification
  - Constraint solving for parametric polymorphism
  - Row polymorphism for records

- **Compiler** (`compiler.rs`): AST to bytecode translation
  - Stack-based bytecode generation
  - Constant folding and basic optimizations
  - Pattern matching compilation

### Layer 2: Virtual Machine
**Crate:** `fusabi-vm`

The VM executes bytecode with a stack-based architecture:

```
Bytecode → Instruction Fetch → Execute → Stack Manipulation → Value Management
```

#### Components:
- **Value System** (`value.rs`): Runtime value representation
  - Tagged union for different value types
  - Reference-counted heap objects
  - Immutable data structures

- **Instruction Set** (`instruction.rs`): Bytecode operations
  - Stack manipulation: Push, Pop, Dup
  - Arithmetic: Add, Sub, Mul, Div
  - Control flow: Jump, Call, Return
  - Pattern matching: CheckInt, CheckTuple, etc.

- **VM Core** (`vm.rs`): Execution engine
  - Stack-based evaluation
  - Call frame management
  - Upvalue capture for closures

- **Standard Library** (`stdlib/`): Built-in functions
  - List operations: map, filter, fold
  - String manipulation
  - Option type utilities

- **Host Interop** (`host.rs`): FFI capabilities
  - Register Rust functions as F# callable
  - Value conversion between Rust and Fusabi
  - Async function support

### Layer 3: Integration & CLI
**Crate:** `fusabi`

The top layer provides the user interface and orchestration:

- **CLI** (`main.rs`): Command-line interface
  - REPL for interactive development
  - Script execution
  - Bytecode compilation and caching

- **Error Reporting**: User-friendly diagnostics
  - Source location tracking
  - Contextual error messages
  - Suggestion system

- **Host API** (`host_api.rs`): High-level embedding API
  - Simple VM instantiation
  - Script loading and execution
  - Result handling

## Project Structure

```
fusabi/
├── rust/
│   └── crates/
│       ├── fusabi-frontend/    # Layer 1: Parsing & Compilation
│       │   ├── src/
│       │   │   ├── lexer.rs       # Tokenization
│       │   │   ├── parser.rs      # AST construction
│       │   │   ├── ast.rs         # AST definitions
│       │   │   ├── compiler.rs    # Bytecode generation
│       │   │   ├── inference.rs   # Type inference
│       │   │   ├── types.rs       # Type system
│       │   │   └── modules.rs     # Module system
│       │   └── tests/
│       ├── fusabi-vm/          # Layer 2: Virtual Machine
│       │   ├── src/
│       │   │   ├── vm.rs          # VM execution loop
│       │   │   ├── value.rs       # Value representation
│       │   │   ├── instruction.rs # Instruction set
│       │   │   ├── chunk.rs       # Bytecode chunks
│       │   │   ├── closure.rs     # Closure support
│       │   │   ├── host.rs        # Host interop
│       │   │   └── stdlib/        # Standard library
│       │   └── tests/
│       └── fusabi/             # Layer 3: CLI & Integration
│           ├── src/
│           │   ├── main.rs        # CLI entry point
│           │   ├── lib.rs         # Public API
│           │   └── host_api.rs    # Embedding API
│           └── tests/
├── examples/                   # Usage examples
├── docs/                      # Documentation
└── scripts/                   # F# test scripts
```

## Development Setup

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- A text editor with Rust support (VS Code recommended)

### Initial Setup
```bash
# Clone the repository
git clone https://github.com/fusabi-lang/fusabi.git
cd fusabi

# Build the project
cargo build

# Run tests
cargo test

# Run with examples
cargo run -- run examples/hello.fsx
```

### Development Commands
```bash
# Build in release mode
cargo build --release

# Run all tests
cargo test

# Run specific test suite
cargo test -p fusabi-frontend

# Run benchmarks
cargo bench

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Adding New Features

### Adding a New Bytecode Instruction

1. **Define the instruction** in `fusabi-vm/src/instruction.rs`:
```rust
pub enum Instruction {
    // ... existing instructions

    /// Your new instruction description
    YourInstruction(parameters),
}
```

2. **Update the compiler** in `fusabi-frontend/src/compiler.rs`:
```rust
// Add emission logic in the appropriate compilation method
fn compile_expression(&mut self, expr: &Expr) -> Result<(), CompileError> {
    match expr {
        // ... existing cases
        Expr::YourCase => {
            // Compile subexpressions if needed
            // Emit your instruction
            self.emit(Instruction::YourInstruction(params));
        }
    }
}
```

3. **Implement execution** in `fusabi-vm/src/vm.rs`:
```rust
fn execute_instruction(&mut self, instr: &Instruction) -> Result<(), VmError> {
    match instr {
        // ... existing cases
        Instruction::YourInstruction(params) => {
            // Pop operands from stack
            // Perform operation
            // Push result to stack
        }
    }
}
```

4. **Add tests**:
```rust
#[test]
fn test_your_instruction() {
    let mut vm = Vm::new();
    // Test your instruction behavior
}
```

### Adding a Standard Library Function

1. **Choose the appropriate module** in `fusabi-vm/src/stdlib/`:
   - `list.rs` for list operations
   - `string.rs` for string operations
   - `option.rs` for Option utilities
   - Create a new module for new categories

2. **Implement the function**:
```rust
pub fn your_function(args: Vec<Value>) -> Result<Value, String> {
    // Validate arguments
    if args.len() != expected_count {
        return Err(format!("Expected {} arguments", expected_count));
    }

    // Extract and validate types
    let arg1 = match &args[0] {
        Value::YourType(val) => val,
        _ => return Err("Type error".to_string()),
    };

    // Perform operation
    let result = /* your logic */;

    // Return wrapped in Value
    Ok(Value::from(result))
}
```

3. **Register in host registry** (`fusabi-vm/src/host.rs`):
```rust
impl HostRegistry {
    pub fn with_stdlib() -> Self {
        let mut registry = Self::new();

        // ... existing registrations

        registry.register_sync(
            "YourModule.yourFunction",
            Arc::new(your_function)
        );

        registry
    }
}
```

4. **Document the function**:
```fsharp
// In docs/stdlib.md
## YourModule.yourFunction

**Signature:** `'a -> 'b -> 'c`

**Description:** What the function does

**Examples:**
```fsharp
let result = YourModule.yourFunction arg1 arg2
// result = expected_value
```

### Adding Language Features

For new language constructs:

1. **Update lexer** if new tokens needed
2. **Extend AST** in `ast.rs`
3. **Update parser** to recognize syntax
4. **Extend type system** if needed
5. **Update compiler** to generate bytecode
6. **Add comprehensive tests**
7. **Update language documentation**

## Testing Guidelines

### Test Organization

- **Unit tests**: In the same file as the code (`#[cfg(test)]` module)
- **Integration tests**: In `tests/` directory of each crate
- **End-to-end tests**: In `fusabi/tests/`
- **Example scripts**: In `scripts/` for manual testing

### Writing Tests

```rust
#[test]
fn test_feature_behavior() {
    // Arrange
    let input = "test input";
    let expected = Value::Int(42);

    // Act
    let result = function_under_test(input);

    // Assert
    assert_eq!(result, expected);
}
```

### Test Coverage Goals

- Unit tests for all public functions
- Integration tests for feature interactions
- Property-based tests for complex algorithms
- Regression tests for fixed bugs

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name

# Integration tests only
cargo test --test '*'

# With coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## Code Style

### Rust Guidelines

- Follow standard Rust conventions
- Use `cargo fmt` before committing
- Address `cargo clippy` warnings
- Document public APIs with doc comments
- Use meaningful variable names
- Keep functions under 50 lines
- Prefer composition over inheritance

### Documentation

```rust
/// Brief description of what this does.
///
/// # Arguments
/// * `param1` - Description of parameter
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// let result = function(arg);
/// assert_eq!(result, expected);
/// ```
pub fn function(param1: Type) -> ReturnType {
    // Implementation
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Create specific error types for each module
- Provide context in error messages
- Never use `.unwrap()` in library code
- Use `.expect()` only with clear messages

## PR Process

### Before Creating a PR

1. **Create a feature branch**:
```bash
git checkout -b feat/your-feature
# or
git checkout -b fix/bug-description
```

2. **Write tests first** (TDD approach)
3. **Implement the feature**
4. **Run full test suite**:
```bash
cargo test
cargo fmt
cargo clippy
```

5. **Update documentation** if needed

### PR Guidelines

1. **Title format**: `type: Brief description`
   - Types: `feat`, `fix`, `docs`, `test`, `refactor`, `perf`, `chore`
   - Example: `feat: Add async/await support`

2. **Description template**:
```markdown
## Description
Brief description of changes

## Motivation
Why these changes are needed

## Changes Made
- Bullet points of specific changes
- Include file paths when relevant

## Testing
How the changes were tested

## Screenshots (if UI changes)
Before/after if applicable

## Checklist
- [ ] Tests pass
- [ ] Documentation updated
- [ ] Lint clean
- [ ] Changelog updated (if needed)
```

3. **Keep PRs focused**: One feature/fix per PR
4. **Respond to reviews**: Address feedback promptly
5. **Squash commits**: Clean history before merge

### Review Process

1. At least one maintainer review required
2. All CI checks must pass
3. No merge conflicts
4. Documentation for public API changes
5. Tests for new functionality

## Community Guidelines

### Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Provide constructive feedback
- Focus on the code, not the person
- Assume good intentions

### Getting Help

- **Issues**: Bug reports and feature requests
- **Discussions**: Questions and ideas
- **Discord**: Real-time chat (if available)
- **Documentation**: Check docs first

### Ways to Contribute

- **Code**: Features, bug fixes, optimizations
- **Documentation**: Guides, examples, API docs
- **Tests**: Increase coverage, add edge cases
- **Examples**: Showcase Fusabi capabilities
- **Translations**: Help internationalize
- **Design**: Improve error messages, CLI UX

## License

By contributing to Fusabi, you agree that your contributions will be licensed under the project's license (MIT).

## Questions?

If this guide doesn't answer your question, please:
1. Check existing issues/discussions
2. Ask in the community chat
3. Open a new issue with the question label

Thank you for contributing to Fusabi! 🚀