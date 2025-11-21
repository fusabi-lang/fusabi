# Issue #016: List Support

## Overview
Implement list data structures with cons-cell representation and standard list operations.

## Labels
- `feature`, `phase-2: features`, `priority: high`, `parallel-safe`, `component: frontend`, `effort: s` (2-3 days)

## Milestone
Phase 2.2: Data Structures (Week 5)

## Track
Frontend (Developer 1)

## Dependencies
- Phase 1 complete

## Blocks
- #018 (Pattern Matching) - list patterns

## Parallel-Safe
✅ **YES** - Can work simultaneously with #015, #017

## Acceptance Criteria
- [ ] List syntax: `[1; 2; 3]` and `[]`
- [ ] Cons operator: `x :: xs`
- [ ] Pattern matching: `| [] -> ...` and `| x :: xs -> ...`
- [ ] `Value::List` variant (cons-cell)
- [ ] Standard operations: `head`, `tail`, `length`
- [ ] 30+ list tests

## Technical Specification

### AST Extension
```rust
// fusabi-frontend/src/ast.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    List(Vec<Expr>),           // [1; 2; 3]
    Cons { head: Box<Expr>, tail: Box<Expr> },  // x :: xs
    Nil,                       // []
    // ...
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    List(Vec<Pattern>),
    Cons { head: Box<Pattern>, tail: Box<Pattern> },
    Nil,
    // ...
}
```

### Value Extension
```rust
// fusabi-vm/src/value.rs
#[derive(Debug, Clone)]
pub enum Value {
    List(Rc<ListNode>),
    Nil,
    // ...
}

pub enum ListNode {
    Cons { head: Value, tail: Rc<ListNode> },
    Nil,
}
```

### Bytecode
```rust
pub enum Instruction {
    MakeList(u16),   // Create list with N elements
    Cons,            // Cons: pop tail, pop head, push cons cell
    Head,            // Get head of list
    Tail,            // Get tail of list
    IsNil,           // Check if list is empty
    // ...
}
```

## Testing Requirements

```rust
#[test]
fn test_empty_list() {
    let code = "[]";
    assert_eq!(compile_and_run(code), Value::Nil);
}

#[test]
fn test_list_literal() {
    let code = "[1; 2; 3]";
    let result = compile_and_run(code);
    assert!(matches!(result, Value::List(_)));
}

#[test]
fn test_cons_operator() {
    let code = r#"
        let x = 1
        let xs = [2; 3]
        in x :: xs
    "#;
    // Should equal [1; 2; 3]
}

#[test]
fn test_list_pattern_match() {
    let code = r#"
        let rec length list =
            match list with
            | [] -> 0
            | _ :: xs -> 1 + length xs
        in length [1; 2; 3; 4; 5]
    "#;
    assert_eq!(compile_and_run(code), Value::Int(5));
}

#[test]
fn test_list_map() {
    let code = r#"
        let rec map f list =
            match list with
            | [] -> []
            | x :: xs -> (f x) :: (map f xs)

        let double x = x * 2
        in map double [1; 2; 3]
    "#;
    // Should equal [2; 4; 6]
}
```

## Implementation Steps
1. Day 1: AST and parser for list syntax
2. Day 2: Value type, bytecode, and VM
3. Day 3: Testing and examples

## Estimated Effort
**2-3 days** (Small)

## Notes
- Immutable cons-cell lists (F# style)
- Empty list is `Nil` value
- Cons is right-associative: `1 :: 2 :: 3 :: []`
