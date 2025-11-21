# Issue #015: Tuple Support

## Overview
Implement tuple data structures for grouping heterogeneous values together.

## Labels
- `feature`, `phase-2: features`, `priority: high`, `parallel-safe`, `component: frontend`, `effort: m` (3-4 days)

## Milestone
Phase 2.2: Data Structures (Week 5)

## Track
Frontend (Developer 1)

## Dependencies
- Phase 1 complete

## Blocks
- #018 (Pattern Matching) - tuple patterns

## Parallel-Safe
✅ **YES** - Frontend work, can parallel with #016, #017

## Acceptance Criteria
- [ ] Tuple syntax: `(1, "hello", true)`
- [ ] Tuple destructuring: `let (x, y) = pair`
- [ ] Tuple indexing (optional): `fst`, `snd` functions
- [ ] `Value::Tuple` variant in VM
- [ ] 30+ tuple tests

## Technical Specification

### AST Extension
```rust
// fusabi-frontend/src/ast.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Tuple construction: (expr1, expr2, ...)
    Tuple(Vec<Expr>),
    // ...
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Tuple(Vec<Pattern>),
    // ...
}
```

### Value Extension
```rust
// fusabi-vm/src/value.rs
#[derive(Debug, Clone)]
pub enum Value {
    Tuple(Vec<Value>),
    // ...
}
```

### Bytecode
```rust
// fusabi-vm/src/bytecode.rs
pub enum Instruction {
    MakeTuple(u8),  // Create tuple with N elements
    GetTupleElement(u8),  // Get element at index
    // ...
}
```

## Testing Requirements

```rust
#[test]
fn test_simple_tuple() {
    let code = r#"(1, "hello", true)"#;
    let result = compile_and_run(code);
    assert!(matches!(result, Value::Tuple(_)));
}

#[test]
fn test_tuple_destructuring() {
    let code = r#"
        let (x, y) = (10, 20) in x + y
    "#;
    assert_eq!(compile_and_run(code), Value::Int(30));
}

#[test]
fn test_nested_tuples() {
    let code = r#"
        let nested = ((1, 2), (3, 4))
        let ((a, b), (c, d)) = nested
        in a + b + c + d
    "#;
    assert_eq!(compile_and_run(code), Value::Int(10));
}
```

## Implementation Steps
1. Day 1: AST and parser for tuple syntax
2. Day 2: Value type and bytecode instructions
3. Day 3: Compiler and VM execution
4. Day 4: Testing and examples

## Estimated Effort
**3-4 days** (Medium)

## Notes
- Tuples are immutable
- Zero-indexed internally
- Pattern matching comes in #018
