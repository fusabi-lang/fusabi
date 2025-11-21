# Issue #012: Closure Support

## Overview
Implement closure support in the VM with proper variable capture via upvalues. This enables functions to capture and reference variables from their enclosing scope.

## Labels
- `feature`
- `phase-2: features`
- `priority: critical`
- `foundational`
- `parallel-safe`
- `component: vm`
- `effort: l` (4-5 days)

## Milestone
Phase 2.1: Functions & Closures (Week 4)

## Track
VM (Developer 2)

## Dependencies
- Phase 1 MVP complete (#001-#009)
- Value representation extensible (#004)
- VM interpreter working (#006)

## Blocks
- #018 (Pattern Matching) - indirectly
- #020 (Type Inference) - needs closures for function types

## Parallel-Safe
✅ **YES** - Works on VM crate, no conflicts with Week 4 frontend/integration work

## Acceptance Criteria
- [ ] `Value::Closure` variant with upvalue storage
- [ ] Upvalue capture mechanism (open/closed system)
- [ ] `MakeClosure` bytecode instruction
- [ ] `ClosureCall` instruction for calling closures
- [ ] Closures can capture variables from parent scopes
- [ ] Nested closures work correctly
- [ ] Upvalues properly closed when scope exits
- [ ] 40+ unit tests for closures
- [ ] Example: Higher-order functions with closures

## Technical Specification

### File Locations
- `rust/crates/fusabi-vm/src/value.rs` - Closure value type
- `rust/crates/fusabi-vm/src/closure.rs` - Closure implementation (NEW)
- `rust/crates/fusabi-vm/src/bytecode.rs` - New instructions
- `rust/crates/fusabi-vm/src/vm.rs` - Closure execution

### Core Types

```rust
// rust/crates/fusabi-vm/src/closure.rs

use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::bytecode::Chunk;

/// An upvalue - a captured variable from an enclosing scope
#[derive(Debug, Clone)]
pub struct Upvalue {
    /// Location of the variable:
    /// - Some(index) = on stack (open upvalue)
    /// - None = closed (value moved to `closed`)
    pub location: Option<usize>,

    /// Closed value (when variable goes out of scope)
    pub closed: Rc<RefCell<Option<Value>>>,
}

impl Upvalue {
    /// Create new open upvalue pointing to stack slot
    pub fn new_open(stack_index: usize) -> Self {
        Upvalue {
            location: Some(stack_index),
            closed: Rc::new(RefCell::new(None)),
        }
    }

    /// Close the upvalue, capturing the current value
    pub fn close(&mut self, value: Value) {
        self.location = None;
        *self.closed.borrow_mut() = Some(value);
    }

    /// Get the upvalue's current value
    pub fn get(&self, stack: &[Value]) -> Value {
        match self.location {
            Some(idx) => stack[idx].clone(),
            None => self.closed.borrow().as_ref().unwrap().clone(),
        }
    }

    /// Set the upvalue's value
    pub fn set(&mut self, value: Value, stack: &mut [Value]) {
        match self.location {
            Some(idx) => stack[idx] = value,
            None => *self.closed.borrow_mut() = Some(value),
        }
    }
}

/// A closure - a function with captured variables
#[derive(Debug, Clone)]
pub struct Closure {
    /// The function's bytecode chunk
    pub chunk: Rc<Chunk>,

    /// Captured upvalues from enclosing scopes
    pub upvalues: Vec<Rc<RefCell<Upvalue>>>,

    /// Number of parameters the function takes
    pub arity: u8,

    /// Function name (for debugging/error messages)
    pub name: Option<String>,
}

impl Closure {
    pub fn new(chunk: Chunk, arity: u8, name: Option<String>) -> Self {
        Closure {
            chunk: Rc::new(chunk),
            upvalues: Vec::new(),
            arity,
            name,
        }
    }

    /// Add an upvalue to this closure
    pub fn add_upvalue(&mut self, upvalue: Rc<RefCell<Upvalue>>) {
        self.upvalues.push(upvalue);
    }
}
```

### Value Extension

```rust
// rust/crates/fusabi-vm/src/value.rs

use crate::closure::Closure;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Unit,

    /// Closure with captured variables
    Closure(Rc<Closure>),
}

impl Value {
    pub fn is_closure(&self) -> bool {
        matches!(self, Value::Closure(_))
    }

    pub fn as_closure(&self) -> Option<&Rc<Closure>> {
        match self {
            Value::Closure(c) => Some(c),
            _ => None,
        }
    }
}
```

### Bytecode Instructions

```rust
// rust/crates/fusabi-vm/src/bytecode.rs

#[derive(Debug, Clone)]
pub enum Instruction {
    // ... existing instructions ...

    /// Create a closure from a function chunk
    /// Operand: constant pool index of the function chunk
    MakeClosure(u16),

    /// Capture an upvalue
    /// Operands: is_local (1 byte), index (1 byte)
    /// - is_local=1: capture from current stack frame
    /// - is_local=0: capture from parent closure's upvalues
    CaptureUpvalue { is_local: bool, index: u8 },

    /// Get value from upvalue
    /// Operand: upvalue index
    GetUpvalue(u8),

    /// Set value to upvalue
    /// Operand: upvalue index
    SetUpvalue(u8),

    /// Close upvalues at and above stack position
    /// Operand: stack position
    CloseUpvalue(u16),

    /// Call a closure
    /// Operand: number of arguments
    Call(u8),
}
```

### VM Implementation

```rust
// rust/crates/fusabi-vm/src/vm.rs

use crate::closure::{Closure, Upvalue};
use std::rc::Rc;
use std::cell::RefCell;

pub struct CallFrame {
    pub closure: Rc<Closure>,
    pub ip: usize,
    pub stack_base: usize,
}

pub struct VM {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    open_upvalues: Vec<Rc<RefCell<Upvalue>>>,
}

impl VM {
    fn execute_instruction(&mut self, instr: &Instruction) -> Result<(), RuntimeError> {
        match instr {
            Instruction::MakeClosure(const_idx) => {
                // Get function chunk from constant pool
                let chunk = self.get_constant(*const_idx)?;
                let closure = Rc::new(Closure::new(chunk, arity, name));
                self.stack.push(Value::Closure(closure));
                Ok(())
            }

            Instruction::CaptureUpvalue { is_local, index } => {
                let upvalue = if *is_local {
                    // Capture from current stack frame
                    let stack_idx = self.current_frame().stack_base + *index as usize;
                    self.capture_upvalue(stack_idx)
                } else {
                    // Capture from parent closure
                    self.current_frame().closure.upvalues[*index as usize].clone()
                };

                // Add to the closure being constructed (top of stack)
                if let Value::Closure(closure) = self.stack.last_mut().unwrap() {
                    Rc::get_mut(closure).unwrap().add_upvalue(upvalue);
                }
                Ok(())
            }

            Instruction::GetUpvalue(idx) => {
                let upvalue = &self.current_frame().closure.upvalues[*idx as usize];
                let value = upvalue.borrow().get(&self.stack);
                self.stack.push(value);
                Ok(())
            }

            Instruction::SetUpvalue(idx) => {
                let value = self.stack.pop().unwrap();
                let upvalue = &self.current_frame().closure.upvalues[*idx as usize];
                upvalue.borrow_mut().set(value, &mut self.stack);
                Ok(())
            }

            Instruction::CloseUpvalue(stack_pos) => {
                self.close_upvalues(*stack_pos as usize);
                Ok(())
            }

            Instruction::Call(arg_count) => {
                let closure = self.stack[self.stack.len() - *arg_count as usize - 1]
                    .as_closure()
                    .ok_or(RuntimeError::NotCallable)?
                    .clone();

                if *arg_count != closure.arity {
                    return Err(RuntimeError::WrongArity {
                        expected: closure.arity,
                        got: *arg_count,
                    });
                }

                let frame = CallFrame {
                    closure,
                    ip: 0,
                    stack_base: self.stack.len() - *arg_count as usize - 1,
                };

                self.frames.push(frame);
                Ok(())
            }

            // ... other instructions ...
        }
    }

    /// Capture a stack slot as an upvalue
    fn capture_upvalue(&mut self, stack_index: usize) -> Rc<RefCell<Upvalue>> {
        // Check if upvalue already exists for this stack slot
        for upvalue in &self.open_upvalues {
            let uv = upvalue.borrow();
            if uv.location == Some(stack_index) {
                return upvalue.clone();
            }
        }

        // Create new upvalue
        let upvalue = Rc::new(RefCell::new(Upvalue::new_open(stack_index)));
        self.open_upvalues.push(upvalue.clone());
        upvalue
    }

    /// Close all upvalues at or above the given stack position
    fn close_upvalues(&mut self, from_index: usize) {
        self.open_upvalues.retain(|upvalue_ref| {
            let mut upvalue = upvalue_ref.borrow_mut();
            if let Some(loc) = upvalue.location {
                if loc >= from_index {
                    // Close this upvalue
                    let value = self.stack[loc].clone();
                    upvalue.close(value);
                    return false; // Remove from open_upvalues
                }
            }
            true // Keep in open_upvalues
        });
    }
}
```

## Implementation Steps

### Step 1: Define Closure Types (Day 1)
- [ ] Create `closure.rs` module
- [ ] Implement `Upvalue` struct
- [ ] Implement `Closure` struct
- [ ] Add `Value::Closure` variant
- [ ] Unit tests for basic closure creation

### Step 2: Add Bytecode Instructions (Day 1-2)
- [ ] Add `MakeClosure` instruction
- [ ] Add `CaptureUpvalue` instruction
- [ ] Add `GetUpvalue` / `SetUpvalue` instructions
- [ ] Add `CloseUpvalue` instruction
- [ ] Add `Call` instruction
- [ ] Unit tests for instruction encoding

### Step 3: Implement VM Support (Day 2-3)
- [ ] Add `CallFrame` for function calls
- [ ] Implement upvalue capture logic
- [ ] Implement upvalue closing logic
- [ ] Handle closure calls
- [ ] Unit tests for VM execution

### Step 4: Compiler Integration (Day 3-4)
- [ ] Update compiler to emit closure instructions
- [ ] Track captured variables during compilation
- [ ] Emit upvalue capture instructions
- [ ] Handle nested closures
- [ ] Integration tests

### Step 5: Testing & Examples (Day 4-5)
- [ ] Simple closure capturing one variable
- [ ] Nested closures
- [ ] Multiple upvalues
- [ ] Upvalue mutation
- [ ] Higher-order functions
- [ ] Edge cases and error handling

## Testing Requirements

### Unit Tests

```rust
// rust/crates/fusabi-vm/src/closure.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upvalue_open() {
        let mut stack = vec![Value::Int(42)];
        let upvalue = Upvalue::new_open(0);
        assert_eq!(upvalue.get(&stack), Value::Int(42));
    }

    #[test]
    fn test_upvalue_close() {
        let stack = vec![Value::Int(42)];
        let mut upvalue = Upvalue::new_open(0);
        upvalue.close(Value::Int(42));

        // After closing, value is independent of stack
        assert_eq!(upvalue.get(&vec![]), Value::Int(42));
    }

    #[test]
    fn test_closure_creation() {
        let chunk = Chunk::new();
        let closure = Closure::new(chunk, 1, Some("test".to_string()));
        assert_eq!(closure.arity, 1);
        assert_eq!(closure.name, Some("test".to_string()));
    }
}
```

### Integration Tests

```rust
// rust/tests/closure_tests.rs

#[test]
fn test_simple_closure() {
    let code = r#"
        let x = 10
        let f = fun y -> x + y
        f 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(15));
}

#[test]
fn test_closure_captures_multiple_vars() {
    let code = r#"
        let x = 10
        let y = 20
        let f = fun z -> x + y + z
        f 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(35));
}

#[test]
fn test_nested_closures() {
    let code = r#"
        let x = 10
        let outer = fun y ->
            let inner = fun z -> x + y + z
            inner
        let f = outer 5
        f 3
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(18));
}

#[test]
fn test_closure_mutation() {
    let code = r#"
        let x = 10
        let inc = fun () -> x := x + 1; x
        inc ()
        inc ()
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(12));
}

#[test]
fn test_higher_order_function() {
    let code = r#"
        let map = fun f list ->
            match list with
            | [] -> []
            | x :: xs -> (f x) :: (map f xs)

        let add10 = fun x -> x + 10
        map add10 [1; 2; 3]
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, list![Value::Int(11), Value::Int(12), Value::Int(13)]);
}
```

### Example Scripts

```fsharp
// examples/closures/simple.fsx

// Simple closure capturing one variable
let x = 42
let getX = fun () -> x
getX ()  // Returns 42

// Closure with parameters and capture
let makeAdder = fun n ->
    fun x -> x + n

let add10 = makeAdder 10
add10 5  // Returns 15

// Nested closures
let makeCounter = fun () ->
    let count = 0
    fun () ->
        count := count + 1
        count

let counter = makeCounter ()
counter ()  // Returns 1
counter ()  // Returns 2
counter ()  // Returns 3
```

## Estimated Effort
**4-5 days** (Large)

### Breakdown:
- Day 1: Closure types and basic instructions (6-8 hours)
- Day 2: VM upvalue capture logic (6-8 hours)
- Day 3: Upvalue closing and call frames (6-8 hours)
- Day 4: Compiler integration and testing (6-8 hours)
- Day 5: Examples, documentation, edge cases (4-6 hours)

## Related Issues
- Blocks #018 (Pattern Matching) - indirectly
- Blocks #020 (Type Inference) - needs closure types
- Works with #013 (Let-Rec) - recursive closures
- Works with #014 (Currying) - partial application with closures

## Notes

### Design Decisions
- **Upvalue Strategy**: Open/closed upvalue system (Lua-style)
- **GC Strategy**: Use `Rc<RefCell<>>` for Phase 2 (proper GC in Phase 3)
- **Arity Checking**: Runtime arity checking for closures
- **Nested Closures**: Full support for arbitrary nesting depth

### Implementation References
- **Lua closures**: Proven upvalue design
- **Crafting Interpreters (Ch. 25)**: Closure implementation guide
- **OCaml ZINC machine**: Closure representation
- **Python closures**: Cell objects for captured variables

### Future Extensions (Phase 3+)
- [ ] Proper garbage collection for closures
- [ ] Closure optimization (inline small closures)
- [ ] Tail-call optimization for recursive closures
- [ ] Closure debugging support

### Parallel Work Opportunity
✅ **PARALLEL-SAFE**: VM track, no conflicts with frontend (#013) or integration (#014)

**Coordination Points**:
- Day 3-4: Share closure API with #014 (Currying)
- Daily sync: Update on closure calling conventions

### Critical Path
⚠️ **CRITICAL**: Foundational for advanced features, blocks type inference

### Success Metrics
- [ ] All upvalue tests pass (20+ tests)
- [ ] Nested closures work (5+ levels deep)
- [ ] Higher-order functions work
- [ ] No memory leaks in closure tests
- [ ] Clear error messages for closure arity mismatches
