# Issue #021: Type Checker Integration

## Overview
Integrate type inference into the compilation pipeline and implement type checking that rejects ill-typed programs before bytecode generation.

## Labels
- `feature`, `phase-2: features`, `priority: high`, `component: types`, `effort: m` (3-4 days)

## Milestone
Phase 2.4: Type System (Week 7)

## Track
VM (Developer 2)

## Dependencies
- #020 (Type Inference) - HARD (needs inference engine)

## Blocks
None (completes Phase 2)

## Parallel-Safe
❌ **NO** - Depends on #020 completion

## Acceptance Criteria
- [ ] Type checking integrated into compiler
- [ ] Reject ill-typed programs before compilation
- [ ] Clear type error messages with source location
- [ ] Type annotations optional but checked if present
- [ ] All Phase 2 features type-checked correctly
- [ ] 40+ type checking tests

## Technical Specification

### Compiler Integration
```rust
// fusabi-frontend/src/compiler.rs

impl Compiler {
    pub fn compile_with_types(expr: &Expr) -> Result<Chunk, CompileError> {
        // Phase 1: Type inference
        let mut inference = TypeInference::new();
        let ty = inference.infer_expr(expr, &TypeEnv::new())?;
        let subst = inference.solve_constraints()?;
        let final_ty = inference.apply_subst(&ty, &subst);

        // Annotate AST with types (optional)
        let typed_expr = Self::annotate_types(expr, &final_ty, &subst)?;

        // Phase 2: Compile to bytecode
        let mut compiler = Compiler::new();
        compiler.compile_expr(&typed_expr)?;
        compiler.chunk.emit(Instruction::Return);

        Ok(compiler.chunk)
    }

    fn annotate_types(
        expr: &Expr,
        ty: &Type,
        subst: &Substitution,
    ) -> Result<TypedExpr, CompileError> {
        // Optionally annotate AST with inferred types
        // Useful for debugging and error messages
        todo!()
    }
}
```

### Type Error Messages
```rust
// fusabi-frontend/src/types.rs

#[derive(Debug, Clone)]
pub enum TypeError {
    TypeMismatch {
        expected: Type,
        found: Type,
        span: Span,
    },
    UnboundVariable {
        name: String,
        span: Span,
    },
    OccursCheck {
        var: TypeVar,
        ty: Type,
        span: Span,
    },
    ArityMismatch {
        expected: usize,
        found: usize,
        span: Span,
    },
}

impl TypeError {
    pub fn format_error(&self, source: &str) -> String {
        match self {
            TypeError::TypeMismatch { expected, found, span } => {
                format!(
                    "Type mismatch at {}:\n  Expected: {}\n  Found: {}\n\n{}",
                    span,
                    expected,
                    found,
                    Self::format_span(source, span)
                )
            }
            // ... other cases ...
        }
    }

    fn format_span(source: &str, span: &Span) -> String {
        // Extract source lines and highlight error location
        // Similar to Rust compiler error format
        todo!()
    }
}
```

## Testing Requirements

```rust
#[test]
fn test_type_check_valid_program() {
    let code = r#"
        let add x y = x + y
        in add 10 20
    "#;
    let result = compile_with_types(code);
    assert!(result.is_ok());
}

#[test]
fn test_reject_type_mismatch() {
    let code = "1 + true";
    let result = compile_with_types(code);
    assert!(matches!(result, Err(CompileError::TypeError(_))));
}

#[test]
fn test_reject_unbound_variable() {
    let code = "x + 1";
    let result = compile_with_types(code);
    assert!(matches!(
        result,
        Err(CompileError::TypeError(TypeError::UnboundVariable { .. }))
    ));
}

#[test]
fn test_type_check_closure() {
    let code = r#"
        let x = 10
        let f = fun y -> x + y
        in f 5
    "#;
    let result = compile_with_types(code);
    assert!(result.is_ok());
}

#[test]
fn test_type_check_polymorphic() {
    let code = r#"
        let id x = x
        let a = id 1
        let b = id true
        in (a, b)
    "#;
    let result = compile_with_types(code);
    assert!(result.is_ok());
}
```

## Implementation Steps
1. **Day 1**: Integrate inference into compiler pipeline
2. **Day 2**: Implement type error formatting
3. **Day 3**: Test with all Phase 2 features
4. **Day 4**: Polish error messages, final integration

## Estimated Effort
**3-4 days** (Medium)

## Notes
- Builds on #020 (Type Inference)
- Focus on clear error messages
- Reference: Elm, Rust error messages for inspiration
- Complete Phase 2 milestone

## Success Metrics
- [ ] All valid Phase 2 programs type-check
- [ ] All invalid programs rejected with clear errors
- [ ] Error messages include source location
- [ ] Type checking adds < 10% compilation overhead
- [ ] Phase 2 COMPLETE
