# Issue #020: Type Inference (Hindley-Milner)

## Overview
Implement Hindley-Milner type inference algorithm to infer types for all expressions and reject ill-typed programs before compilation.

## Labels
- `feature`, `phase-2: features`, `priority: critical`, `foundational`, `component: types`, `effort: l` (5-6 days)

## Milestone
Phase 2.4: Type System (Week 7)

## Track
Frontend (Developer 1)

## Dependencies
- #012 (Closures) - HARD (function types)
- #013 (Let-Rec) - SOFT (recursive types)
- #015 (Tuples) - HARD (tuple types)
- #016 (Lists) - HARD (list types)

## Blocks
- #021 (Type Checker) - HARD

## Parallel-Safe
❌ **NO** - Critical path, complex algorithm

## Acceptance Criteria
- [ ] Hindley-Milner unification algorithm
- [ ] Type inference for all expressions
- [ ] Polymorphic type support: `'a -> 'a`
- [ ] Let-polymorphism
- [ ] Type variables and substitution
- [ ] Clear type error messages
- [ ] 60+ type inference tests

## Technical Specification

### Type System
```rust
// fusabi-frontend/src/types.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Type variable: 'a, 'b, ...
    Var(TypeVar),

    /// Concrete types
    Int,
    Bool,
    String,
    Unit,

    /// Function type: T1 -> T2
    Arrow(Box<Type>, Box<Type>),

    /// Tuple type: T1 * T2 * ...
    Tuple(Vec<Type>),

    /// List type: T list
    List(Box<Type>),

    /// Array type: T array
    Array(Box<Type>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(u32);

pub type Substitution = HashMap<TypeVar, Type>;
```

### Type Inference Engine
```rust
// fusabi-frontend/src/inference.rs

pub struct TypeInference {
    next_var: u32,
    constraints: Vec<Constraint>,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub lhs: Type,
    pub rhs: Type,
    pub span: Span,  // For error reporting
}

impl TypeInference {
    pub fn infer_expr(
        &mut self,
        expr: &Expr,
        env: &TypeEnv,
    ) -> Result<Type, TypeError> {
        match expr {
            Expr::Lit(lit) => Ok(self.infer_literal(lit)),

            Expr::Var(name) => {
                env.lookup(name)
                    .ok_or_else(|| TypeError::UnboundVariable(name.clone()))
            }

            Expr::Lambda { param, body } => {
                let param_ty = self.fresh_var();
                let mut new_env = env.clone();
                new_env.insert(param.clone(), param_ty.clone());

                let body_ty = self.infer_expr(body, &new_env)?;

                Ok(Type::Arrow(Box::new(param_ty), Box::new(body_ty)))
            }

            Expr::App { func, arg } => {
                let func_ty = self.infer_expr(func, env)?;
                let arg_ty = self.infer_expr(arg, env)?;
                let result_ty = self.fresh_var();

                // Add constraint: func_ty = arg_ty -> result_ty
                self.add_constraint(
                    func_ty,
                    Type::Arrow(Box::new(arg_ty), Box::new(result_ty.clone())),
                )?;

                Ok(result_ty)
            }

            Expr::Let { name, value, body } => {
                // Infer value type
                let value_ty = self.infer_expr(value, env)?;

                // Generalize (let-polymorphism)
                let scheme = self.generalize(value_ty, env);

                // Add to environment
                let mut new_env = env.clone();
                new_env.insert(name.clone(), scheme);

                // Infer body
                self.infer_expr(body, &new_env)
            }

            // ... other cases ...
        }
    }

    fn fresh_var(&mut self) -> Type {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        Type::Var(var)
    }

    fn add_constraint(&mut self, lhs: Type, rhs: Type) -> Result<(), TypeError> {
        self.constraints.push(Constraint { lhs, rhs, span: Span::default() });
        Ok(())
    }

    pub fn solve_constraints(&self) -> Result<Substitution, TypeError> {
        let mut subst = Substitution::new();

        for constraint in &self.constraints {
            let lhs = self.apply_subst(&constraint.lhs, &subst);
            let rhs = self.apply_subst(&constraint.rhs, &subst);

            let new_subst = self.unify(lhs, rhs)?;
            subst = self.compose_subst(subst, new_subst);
        }

        Ok(subst)
    }

    fn unify(&self, t1: Type, t2: Type) -> Result<Substitution, TypeError> {
        match (t1, t2) {
            (Type::Var(v), t) | (t, Type::Var(v)) => {
                if self.occurs(v, &t) {
                    Err(TypeError::OccursCheck)
                } else {
                    Ok(Substitution::from([(v, t)]))
                }
            }

            (Type::Arrow(l1, r1), Type::Arrow(l2, r2)) => {
                let s1 = self.unify(*l1, *l2)?;
                let s2 = self.unify(
                    self.apply_subst(&r1, &s1),
                    self.apply_subst(&r2, &s1),
                )?;
                Ok(self.compose_subst(s1, s2))
            }

            (Type::Tuple(ts1), Type::Tuple(ts2)) if ts1.len() == ts2.len() => {
                self.unify_list(ts1, ts2)
            }

            (t1, t2) if t1 == t2 => Ok(Substitution::new()),

            _ => Err(TypeError::TypeMismatch { expected: t1, found: t2 }),
        }
    }

    fn generalize(&self, ty: Type, env: &TypeEnv) -> TypeScheme {
        let free_vars = self.free_vars(&ty);
        let env_free_vars = env.free_vars();
        let quantified = free_vars.difference(&env_free_vars).copied().collect();

        TypeScheme { quantified, ty }
    }
}
```

## Testing Requirements

```rust
#[test]
fn test_infer_simple_int() {
    let code = "42";
    let ty = infer_type(code).unwrap();
    assert_eq!(ty, Type::Int);
}

#[test]
fn test_infer_lambda() {
    let code = "fun x -> x";
    let ty = infer_type(code).unwrap();
    // Should be: 'a -> 'a
    assert!(matches!(ty, Type::Arrow(_, _)));
}

#[test]
fn test_infer_let_polymorphism() {
    let code = r#"
        let id = fun x -> x
        in (id 1, id true)
    "#;
    let ty = infer_type(code).unwrap();
    assert_eq!(ty, Type::Tuple(vec![Type::Int, Type::Bool]));
}

#[test]
fn test_reject_type_error() {
    let code = "1 + true";
    let result = infer_type(code);
    assert!(result.is_err());
}

#[test]
fn test_infer_recursive_function() {
    let code = r#"
        let rec length list =
            match list with
            | [] -> 0
            | _ :: xs -> 1 + length xs
        in length
    "#;
    // Should infer: 'a list -> int
}
```

## Implementation Steps
1. **Days 1-2**: Research and design (Hindley-Milner algorithm)
2. **Days 3-4**: Implement type inference engine
3. **Days 4-5**: Constraint solving and unification
4. **Days 5-6**: Let-polymorphism and generalization
5. **Day 6**: Testing, error messages, integration

## Estimated Effort
**5-6 days** (Large) - CRITICAL PATH

## Notes
- Most complex issue in Phase 2
- Reference: TAPL book, miniml implementations
- Start with simple types, add polymorphism incrementally
- Clear error messages essential
