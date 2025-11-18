# Issue #017: Array Support

## Overview
Implement mutable array data structures with efficient indexed access.

## Labels
- `feature`, `phase-2: features`, `priority: medium`, `parallel-safe`, `component: vm`, `effort: s` (2-3 days)

## Milestone
Phase 2.2: Data Structures (Week 5)

## Track
VM (Developer 2)

## Dependencies
- Phase 1 complete

## Blocks
- #018 (Pattern Matching) - soft (array patterns optional)

## Parallel-Safe
✅ **YES** - VM work, can parallel with #015, #016

## Acceptance Criteria
- [ ] Array creation: `[|1; 2; 3|]`
- [ ] Array indexing: `arr.[0]`
- [ ] Array update: `arr.[0] <- 42`
- [ ] Array length: `Array.length arr`
- [ ] `Value::Array` variant
- [ ] 20+ array tests

## Technical Specification

### AST Extension
```rust
// fsrs-frontend/src/ast.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Array(Vec<Expr>),              // [|1; 2; 3|]
    ArrayIndex { array: Box<Expr>, index: Box<Expr> },
    ArrayUpdate { array: Box<Expr>, index: Box<Expr>, value: Box<Expr> },
    // ...
}
```

### Value Extension
```rust
// fsrs-vm/src/value.rs
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Array(Rc<RefCell<Vec<Value>>>),  // Mutable array
    // ...
}
```

### Bytecode
```rust
pub enum Instruction {
    MakeArray(u16),     // Create array with N elements
    ArrayGet,           // Get element: pop index, pop array, push element
    ArraySet,           // Set element: pop value, pop index, pop array
    ArrayLength,        // Get array length
    // ...
}
```

## Testing Requirements

```rust
#[test]
fn test_array_creation() {
    let code = "[|1; 2; 3|]";
    let result = compile_and_run(code);
    assert!(matches!(result, Value::Array(_)));
}

#[test]
fn test_array_indexing() {
    let code = r#"
        let arr = [|10; 20; 30|]
        in arr.[1]
    "#;
    assert_eq!(compile_and_run(code), Value::Int(20));
}

#[test]
fn test_array_update() {
    let code = r#"
        let arr = [|1; 2; 3|]
        arr.[1] <- 42
        arr.[1]
    "#;
    assert_eq!(compile_and_run(code), Value::Int(42));
}

#[test]
fn test_array_length() {
    let code = r#"
        let arr = [|1; 2; 3; 4; 5|]
        in Array.length arr
    "#;
    assert_eq!(compile_and_run(code), Value::Int(5));
}
```

## Implementation Steps
1. Day 1: AST and array value type
2. Day 2: Bytecode and VM execution
3. Day 3: Testing and examples

## Estimated Effort
**2-3 days** (Small)

## Notes
- Mutable arrays (unlike immutable lists)
- Zero-indexed
- Bounds checking at runtime
- Use `RefCell` for interior mutability
