# Issue #014: Currying and Partial Application

## Overview
Implement currying and partial application support, enabling multi-parameter functions to be called with fewer arguments and creating specialized functions.

## Labels
- `feature`
- `phase-2: features`
- `priority: high`
- `requires-coordination`
- `component: integration`
- `effort: m` (3-4 days)

## Milestone
Phase 2.1: Functions & Closures (Week 4)

## Track
Integration (Developer 3)

## Dependencies
- Phase 1 MVP complete (#001-#009)
- #012 (Closures) - soft dependency (uses closures for partial application)
- #013 (Let-Rec) - soft dependency (curried recursive functions)

## Blocks
None (enhances existing features)

## Parallel-Safe
⚠️ **REQUIRES COORDINATION** - Works across frontend and VM, needs coordination with #012 and #013

## Acceptance Criteria
- [ ] Multi-parameter function syntax: `let add x y = x + y`
- [ ] Curried function calls work: `let inc = add 1`
- [ ] Partial application creates new closures
- [ ] Fully applied functions execute normally
- [ ] Type inference works with curried functions
- [ ] 30+ unit tests for currying
- [ ] Example: Higher-order functions with partial application

## Technical Specification

### File Locations
- `rust/crates/fusabi-frontend/src/parser.rs` - Multi-parameter syntax
- `rust/crates/fusabi-frontend/src/compiler.rs` - Currying compilation
- `rust/tests/currying_tests.rs` - Integration tests

### Parser Extension

```rust
// rust/crates/fusabi-frontend/src/parser.rs

impl Parser {
    /// Parse function definition with multiple parameters
    /// Syntax: `let f x y z = body` desugars to `let f = fun x -> fun y -> fun z -> body`
    fn parse_let_binding(&mut self) -> Result<Expr, ParseError> {
        let name = self.expect_identifier()?;

        // Collect parameters
        let mut params = Vec::new();
        while self.peek_is_identifier() && !self.check_token(Token::Equals) {
            params.push(self.expect_identifier()?);
        }

        self.expect_token(Token::Equals)?;
        let body = self.parse_expr()?;

        // Desugar to nested lambdas
        let value = if params.is_empty() {
            body
        } else {
            self.desugar_curried_function(params, body)
        };

        self.expect_keyword("in")?;
        let body = self.parse_expr()?;

        Ok(Expr::Let {
            name,
            value: Box::new(value),
            body: Box::new(body),
        })
    }

    /// Desugar `fun x y z -> body` to `fun x -> fun y -> fun z -> body`
    fn desugar_curried_function(&self, params: Vec<String>, body: Expr) -> Expr {
        params.into_iter().rev().fold(body, |acc, param| {
            Expr::Lambda {
                param,
                body: Box::new(acc),
            }
        })
    }
}
```

### Curried Function Calls

```rust
// Partial application is automatic through closure creation

// Example: let add x y = x + y
// Compiles to: let add = fun x -> fun y -> x + y

// When called with one argument:
// add 10  =>  (fun x -> fun y -> x + y) 10  =>  fun y -> 10 + y

// This works automatically through the closure mechanism from #012
```

### Compiler Strategy

```rust
// rust/crates/fusabi-frontend/src/compiler.rs

// Currying is handled at parse time (desugaring)
// No special compiler support needed beyond closure support

impl Compiler {
    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            // Multi-parameter functions are already desugared to nested lambdas
            Expr::Lambda { param, body } => {
                self.compile_lambda(param, body)
            }

            // Function application handles partial application automatically
            Expr::App { func, arg } => {
                self.compile_application(func, arg)
            }

            // ... other cases ...
        }
    }

    fn compile_application(&mut self, func: &Expr, arg: &Expr) -> Result<(), CompileError> {
        // Compile function expression (may return closure)
        self.compile_expr(func)?;

        // Compile argument
        self.compile_expr(arg)?;

        // Call the function (if it's a closure expecting more args, returns another closure)
        self.emit(Instruction::Call(1));

        Ok(())
    }
}
```

## Implementation Steps

### Step 1: Parser Multi-Parameter Syntax (Day 1)
- [ ] Parse `let f x y z = body` syntax
- [ ] Desugar to nested lambdas
- [ ] Handle `fun x y z -> body` syntax
- [ ] Add parser tests

### Step 2: Compiler Integration (Day 2)
- [ ] Ensure desugared lambdas compile correctly
- [ ] Test partial application compilation
- [ ] Verify closure creation for partial apps
- [ ] Add compiler tests

### Step 3: VM Testing (Day 2-3)
- [ ] Test partial application execution
- [ ] Test full application execution
- [ ] Test nested partial applications
- [ ] Performance testing

### Step 4: Examples & Documentation (Day 3-4)
- [ ] Simple currying examples
- [ ] Partial application examples
- [ ] Higher-order function examples
- [ ] Real-world use cases
- [ ] Edge cases

## Testing Requirements

### Unit Tests - Parser

```rust
// rust/crates/fusabi-frontend/src/parser.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multi_param_function() {
        let code = "let add x y = x + y in add 1 2";
        let expr = parse_expr(code).unwrap();

        // Should desugar to nested lambdas
        match expr {
            Expr::Let { value, .. } => {
                assert!(matches!(*value, Expr::Lambda { .. }));
            }
            _ => panic!("Expected Let"),
        }
    }

    #[test]
    fn test_parse_three_param_function() {
        let code = "let f x y z = x + y + z in f 1 2 3";
        let expr = parse_expr(code).unwrap();
        // Verify desugaring to triple-nested lambda
    }
}
```

### Integration Tests

```rust
// rust/tests/currying_tests.rs

#[test]
fn test_simple_currying() {
    let code = r#"
        let add x y = x + y
        in add 10 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(15));
}

#[test]
fn test_partial_application() {
    let code = r#"
        let add x y = x + y
        let add10 = add 10
        in add10 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(15));
}

#[test]
fn test_multiple_partial_applications() {
    let code = r#"
        let add x y z = x + y + z
        let add10 = add 10
        let add10_20 = add10 20
        in add10_20 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(35));
}

#[test]
fn test_higher_order_with_currying() {
    let code = r#"
        let map f list =
            match list with
            | [] -> []
            | x :: xs -> (f x) :: (map f xs)

        let add x y = x + y
        let add10 = add 10

        in map add10 [1; 2; 3]
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, list![Value::Int(11), Value::Int(12), Value::Int(13)]);
}

#[test]
fn test_curried_recursive_function() {
    let code = r#"
        let rec foldr f acc list =
            match list with
            | [] -> acc
            | x :: xs -> f x (foldr f acc xs)

        let add x y = x + y
        in foldr add 0 [1; 2; 3; 4; 5]
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(15));
}

#[test]
fn test_function_composition() {
    let code = r#"
        let compose f g = fun x -> f (g x)
        let add10 x = x + 10
        let mul2 x = x * 2

        let transform = compose add10 mul2
        in transform 5
    "#;
    let result = compile_and_run(code);
    assert_eq!(result, Value::Int(20)); // (5 * 2) + 10
}
```

### Example Scripts

```fsharp
// examples/currying/simple.fsx

// Basic currying
let add x y = x + y
let add10 = add 10

add10 5  // Returns 15
add10 20 // Returns 30

// Three-parameter function
let addThree x y z = x + y + z
let add10 = addThree 10
let add10_20 = add10 20

add10_20 5  // Returns 35
```

```fsharp
// examples/currying/higher-order.fsx

// Map with curried functions
let map f list =
    match list with
    | [] -> []
    | x :: xs -> (f x) :: (map f xs)

let multiply x y = x * y
let double = multiply 2

map double [1; 2; 3; 4; 5]
// Returns [2; 4; 6; 8; 10]

// Filter with curried predicates
let filter pred list =
    match list with
    | [] -> []
    | x :: xs ->
        if pred x then x :: (filter pred xs)
        else filter pred xs

let greaterThan threshold value = value > threshold
let greaterThan10 = greaterThan 10

filter greaterThan10 [5; 15; 8; 20; 12]
// Returns [15; 20; 12]
```

```fsharp
// examples/currying/composition.fsx

// Function composition
let compose f g = fun x -> f (g x)
let pipe f g = fun x -> g (f x)

let add10 x = x + 10
let mul2 x = x * 2
let negate x = -x

// Compose functions
let transform = compose add10 (compose mul2 negate)
transform 5
// (-5 * 2) + 10 = 0

// Pipeline operator (left-to-right)
let transform2 = pipe negate (pipe mul2 add10)
transform2 5
// Same result

// Practical use case: data transformation pipeline
let processUser =
    pipe getUserName
    (pipe normalizeString
    (pipe validateFormat
         sendNotification))
```

## Estimated Effort
**3-4 days** (Medium)

### Breakdown:
- Day 1: Parser multi-parameter syntax (6-8 hours)
- Day 2: Compiler and VM integration (6-8 hours)
- Day 3: Testing and validation (6-8 hours)
- Day 4: Examples, documentation, edge cases (4-6 hours)

## Related Issues
- Uses #012 (Closures) - partial application creates closures
- Works with #013 (Let-Rec) - curried recursive functions
- Enhances #020 (Type Inference) - curried function types

## Notes

### Design Decisions
- **Desugaring Approach**: Multi-parameter functions desugar to nested lambdas
- **Automatic Currying**: All functions are curried by default (F# style)
- **No Special Instructions**: Leverage closure mechanism from #012
- **Type Inference**: Curried types represented as `T1 -> T2 -> T3`

### Implementation References
- **F# currying**: Reference implementation
- **Haskell currying**: All functions are curried
- **OCaml currying**: Similar desugaring approach
- **ML currying**: Classic functional language pattern

### Future Extensions (Phase 3+)
- [ ] Uncurried functions (tupled parameters) as optimization
- [ ] Partial application optimization (avoid creating closures)
- [ ] Better error messages for arity mismatches
- [ ] Function composition operators (`>>`, `<<`)

### Parallel Work Opportunity
⚠️ **REQUIRES COORDINATION**:
- Works across frontend (parser) and VM (closures)
- Needs coordination with #012 (closure calling convention)
- Needs coordination with #013 (curried recursive functions)

**Coordination Points**:
- Day 1-2: Review closure API from #012
- Day 2: Test with recursive functions from #013
- Daily sync: Ensure compatibility

### Critical Path
⚠️ **IMPORTANT**: Enhances functional programming capabilities, not blocking

### Success Metrics
- [ ] All currying tests pass (30+ tests)
- [ ] Partial application working correctly
- [ ] Higher-order functions with currying
- [ ] Function composition working
- [ ] No performance regression vs direct calls
- [ ] Clear examples demonstrating practical use
