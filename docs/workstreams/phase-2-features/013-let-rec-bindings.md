# Issue #013: Let-Rec Bindings

## Overview
Implement recursive let-bindings (`let rec`) to enable recursive and mutually recursive function definitions. This extends the Phase 1 simple let-bindings to support self-referential functions.

## Labels
- `feature`
- `phase-2: features`
- `priority: high`
- `parallel-safe`
- `component: frontend`
- `effort: m` (3-4 days)

## Milestone
Phase 2.1: Functions & Closures (Week 4)

## Track
Frontend (Developer 1)

## Dependencies
- Phase 1 MVP complete (#001-#009)
- AST and parser working (#001, #003)

## Blocks
- #020 (Type Inference) - soft dependency (recursive types)

## Parallel-Safe
✅ **YES** - Frontend track, works on AST/parser, no conflicts with VM (#012) or integration (#014)

## Acceptance Criteria
- [ ] `LetRec` variant added to AST `Expr` enum
- [ ] Parser supports `let rec` syntax
- [ ] Compiler emits bytecode for recursive bindings
- [ ] Simple recursive functions work (factorial, fibonacci)
- [ ] Mutually recursive functions work
- [ ] 30+ unit tests for let-rec
- [ ] Example: Recursive list processing

## Technical Specification

### File Locations
- `rust/crates/fsrs-frontend/src/ast.rs` - AST extension
- `rust/crates/fsrs-frontend/src/parser.rs` - Parser support
- `rust/crates/fsrs-frontend/src/compiler.rs` - Compilation

### AST Extension

```rust
// rust/crates/fsrs-frontend/src/ast.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // ... existing variants ...

    /// Recursive let binding: `let rec f x = ... in body`
    LetRec {
        name: String,
        /// For now, this is always a Lambda
        /// Later: can be any expression
        value: Box<Expr>,
        body: Box<Expr>,
    },

    /// Mutually recursive bindings: `let rec f = ... and g = ... in body`
    LetRecMutual {
        bindings: Vec<(String, Expr)>,
        body: Box<Expr>,
    },
}
```

### Parser Extension

```rust
// rust/crates/fsrs-frontend/src/parser.rs

impl Parser {
    fn parse_let(&mut self) -> Result<Expr, ParseError> {
        self.expect_keyword("let")?;

        // Check for 'rec' keyword
        if self.check_keyword("rec") {
            self.parse_let_rec()
        } else {
            self.parse_let_simple()
        }
    }

    fn parse_let_rec(&mut self) -> Result<Expr, ParseError> {
        // Parse: let rec name = expr in body
        let name = self.expect_identifier()?;
        self.expect_token(Token::Equals)?;
        let value = self.parse_expr()?;

        // Check for 'and' for mutual recursion
        let mut bindings = vec![(name.clone(), value)];
        while self.check_keyword("and") {
            self.advance(); // consume 'and'
            let name = self.expect_identifier()?;
            self.expect_token(Token::Equals)?;
            let value = self.parse_expr()?;
            bindings.push((name, value));
        }

        self.expect_keyword("in")?;
        let body = self.parse_expr()?;

        if bindings.len() == 1 {
            Ok(Expr::LetRec {
                name: bindings[0].0.clone(),
                value: Box::new(bindings[0].1.clone()),
                body: Box::new(body),
            })
        } else {
            Ok(Expr::LetRecMutual {
                bindings,
                body: Box::new(body),
            })
        }
    }
}
```

### Compiler Extension

```rust
// rust/crates/fsrs-frontend/src/compiler.rs

impl Compiler {
    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::LetRec { name, value, body } => {
                self.compile_let_rec(name, value, body)
            }

            Expr::LetRecMutual { bindings, body } => {
                self.compile_let_rec_mutual(bindings, body)
            }

            // ... other cases ...
        }
    }

    fn compile_let_rec(
        &mut self,
        name: &str,
        value: &Expr,
        body: &Expr,
    ) -> Result<(), CompileError> {
        // Strategy: Create a placeholder, compile the function with
        // the name in scope, then patch the binding

        // 1. Push placeholder (will be replaced by closure)
        self.emit(Instruction::LoadConst(self.add_constant(Value::Unit)));
        let slot = self.locals.len();
        self.locals.push(name.to_string());

        // 2. Compile the value (usually a lambda) with name in scope
        // The lambda can now reference itself
        self.compile_expr(value)?;

        // 3. Update the stack slot with the actual closure
        self.emit(Instruction::SetLocal(slot as u16));
        self.emit(Instruction::Pop); // Pop the set result

        // 4. Push the value again for use in body
        self.emit(Instruction::GetLocal(slot as u16));

        // 5. Compile body
        self.compile_expr(body)?;

        // 6. Clean up local
        self.locals.pop();
        self.emit(Instruction::Pop); // Pop the binding

        Ok(())
    }

    fn compile_let_rec_mutual(
        &mut self,
        bindings: &[(String, Expr)],
        body: &Expr,
    ) -> Result<(), CompileError> {
        // Strategy: Create placeholders for all bindings, then fill them in

        // 1. Push placeholders and record slots
        let mut slots = Vec::new();
        for (name, _) in bindings {
            self.emit(Instruction::LoadConst(self.add_constant(Value::Unit)));
            let slot = self.locals.len();
            self.locals.push(name.clone());
            slots.push(slot);
        }

        // 2. Compile each value (with all names in scope)
        for (i, (_name, value)) in bindings.iter().enumerate() {
            self.compile_expr(value)?;
            self.emit(Instruction::SetLocal(slots[i] as u16));
            self.emit(Instruction::Pop);
        }

        // 3. Compile body
        self.compile_expr(body)?;

        // 4. Clean up locals
        for _ in bindings {
            self.locals.pop();
        }
        for _ in bindings {
            self.emit(Instruction::Pop);
        }

        Ok(())
    }
}
```

## Implementation Steps

### Step 1: Extend AST (Day 1 - Morning)
- [ ] Add `LetRec` variant to `Expr` enum
- [ ] Add `LetRecMutual` variant for mutual recursion
- [ ] Update AST tests
- [ ] Update Display/Debug implementations

### Step 2: Update Parser (Day 1 - Afternoon)
- [ ] Parse `let rec name = expr in body`
- [ ] Parse `let rec f = ... and g = ... in body`
- [ ] Handle syntax errors gracefully
- [ ] Add parser tests

### Step 3: Compiler Support (Day 2)
- [ ] Implement simple let-rec compilation
- [ ] Handle self-reference in function bodies
- [ ] Implement mutual recursion compilation
- [ ] Add compiler tests

### Step 4: VM Integration (Day 2-3)
- [ ] Ensure VM handles recursive calls correctly
- [ ] Test with increasing recursion depths
- [ ] Add stack overflow protection
- [ ] Performance testing

### Step 5: Examples & Testing (Day 3-4)
- [ ] Factorial example
- [ ] Fibonacci example
- [ ] List processing (recursive length, sum, etc.)
- [ ] Mutually recursive functions (even/odd)
- [ ] Edge cases and error handling

## Testing Requirements

### Unit Tests - AST

```rust
// rust/crates/fsrs-frontend/src/ast.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_rec_ast() {
        let expr = Expr::LetRec {
            name: "fact".to_string(),
            value: Box::new(Expr::Lambda {
                param: "n".to_string(),
                body: Box::new(Expr::Var("n".to_string())),
            }),
            body: Box::new(Expr::Var("fact".to_string())),
        };
        assert!(matches!(expr, Expr::LetRec { .. }));
    }
}
```

### Unit Tests - Parser

```rust
// rust/crates/fsrs-frontend/src/parser.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let_rec_simple() {
        let code = "let rec fact = fun n -> n in fact";
        let expr = parse_expr(code).unwrap();
        assert!(matches!(expr, Expr::LetRec { .. }));
    }

    #[test]
    fn test_parse_let_rec_mutual() {
        let code = r#"
            let rec even = fun n -> if n = 0 then true else odd (n - 1)
            and odd = fun n -> if n = 0 then false else even (n - 1)
            in even 10
        "#;
        let expr = parse_expr(code).unwrap();
        assert!(matches!(expr, Expr::LetRecMutual { .. }));
    }
}
```

### Integration Tests

```rust
// rust/tests/let_rec_tests.rs

#[test]
fn test_factorial() {
    let code = r#"
        let rec fact = fun n ->
            if n <= 1 then 1
            else n * fact (n - 1)
        in fact 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(120));
}

#[test]
fn test_fibonacci() {
    let code = r#"
        let rec fib = fun n ->
            if n <= 1 then n
            else fib (n - 1) + fib (n - 2)
        in fib 10
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(55));
}

#[test]
fn test_list_length() {
    let code = r#"
        let rec length = fun list ->
            match list with
            | [] -> 0
            | _ :: xs -> 1 + length xs
        in length [1; 2; 3; 4; 5]
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(5));
}

#[test]
fn test_mutual_recursion() {
    let code = r#"
        let rec even = fun n ->
            if n = 0 then true
            else odd (n - 1)
        and odd = fun n ->
            if n = 0 then false
            else even (n - 1)
        in even 10
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_recursive_closure() {
    let code = r#"
        let x = 10
        let rec f = fun n ->
            if n <= 0 then x
            else n + f (n - 1)
        in f 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(25)); // 5 + 4 + 3 + 2 + 1 + 10
}
```

### Example Scripts

```fsharp
// examples/recursion/factorial.fsrs

// Classic recursive factorial
let rec factorial n =
    if n <= 1 then 1
    else n * factorial (n - 1)

factorial 5  // 120
factorial 10 // 3628800

// Tail-recursive factorial (more efficient)
let rec factorialTail n acc =
    if n <= 1 then acc
    else factorialTail (n - 1) (n * acc)

let factorial n = factorialTail n 1
```

```fsharp
// examples/recursion/list-processing.fsrs

// Recursive list length
let rec length list =
    match list with
    | [] -> 0
    | _ :: xs -> 1 + length xs

// Recursive list sum
let rec sum list =
    match list with
    | [] -> 0
    | x :: xs -> x + sum xs

// Recursive map
let rec map f list =
    match list with
    | [] -> []
    | x :: xs -> (f x) :: (map f xs)

// Recursive filter
let rec filter pred list =
    match list with
    | [] -> []
    | x :: xs ->
        if pred x then x :: (filter pred xs)
        else filter pred xs
```

```fsharp
// examples/recursion/mutual.fsrs

// Mutually recursive even/odd
let rec even n =
    if n = 0 then true
    else odd (n - 1)
and odd n =
    if n = 0 then false
    else even (n - 1)

even 10  // true
odd 10   // false
```

## Estimated Effort
**3-4 days** (Medium)

### Breakdown:
- Day 1: AST and parser extensions (6-8 hours)
- Day 2: Compiler implementation (6-8 hours)
- Day 3: VM integration and testing (6-8 hours)
- Day 4: Examples, documentation, edge cases (4-6 hours)

## Related Issues
- Works with #012 (Closures) - recursive closures
- Soft blocks #020 (Type Inference) - recursive types
- Independent of #014 (Currying)

## Notes

### Design Decisions
- **Placeholder Strategy**: Use placeholder values during compilation
- **Mutual Recursion**: Support via `let rec ... and ... and ...`
- **Stack Overflow**: Runtime stack limit protection
- **Tail-Call Optimization**: Deferred to Phase 3 (performance)

### Implementation References
- **F# let rec**: Reference implementation
- **OCaml recursive bindings**: Letrec semantics
- **Scheme letrec**: Classic recursive binding strategy

### Future Extensions (Phase 3+)
- [ ] Tail-call optimization for recursive functions
- [ ] Mutual recursion across modules
- [ ] Recursive type definitions
- [ ] Better error messages for invalid recursion

### Parallel Work Opportunity
✅ **PARALLEL-SAFE**: Frontend track, no conflicts

**Coordination Points**:
- Day 2-3: Ensure compiler changes compatible with #012 (closures)
- Day 3: Share recursive function examples with #014 (currying)

### Critical Path
⚠️ **IMPORTANT**: Enables recursive programming, soft dependency for type inference

### Success Metrics
- [ ] All recursive function tests pass (30+ tests)
- [ ] Factorial, Fibonacci working correctly
- [ ] Mutual recursion working
- [ ] Stack overflow protection in place
- [ ] Clear error messages for invalid let-rec
