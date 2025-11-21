# Issue #019: Pattern Matching Compiler

## Overview
Implement pattern matching compilation from AST patterns to bytecode using decision trees for efficient matching.

## Labels
- `feature`, `phase-2: features`, `priority: critical`, `requires-coordination`, `component: vm`, `effort: l` (4-5 days)

## Milestone
Phase 2.3: Pattern Matching (Week 6)

## Track
VM (Developer 2)

## Dependencies
- #018 (Pattern AST) - HARD (needs pattern definition)

## Blocks
- #020 (Type Inference) - SOFT

## Parallel-Safe
❌ **NO** - Depends on #018 interface definition (can parallel after Day 1 coordination)

## Acceptance Criteria
- [ ] Compile match expressions to bytecode
- [ ] Decision tree optimization
- [ ] Pattern matching instructions in bytecode
- [ ] Efficient matching (no backtracking where possible)
- [ ] Runtime pattern matching in VM
- [ ] 40+ pattern matching tests

## Technical Specification

### Bytecode Instructions
```rust
// fusabi-vm/src/bytecode.rs

pub enum Instruction {
    // Pattern matching
    MatchTag(u8),               // Check discriminant/tag
    MatchLit(u16),              // Match literal (constant pool)
    Destruct(u8),               // Destructure (tuple/list)
    GetField(u8),               // Get field from structure
    JumpIfNotMatch(i16),        // Jump if pattern doesn't match
    BindLocal(u16),             // Bind pattern variable
    // ...
}
```

### Decision Tree Compilation
```rust
// fusabi-frontend/src/compiler.rs

impl Compiler {
    fn compile_match(
        &mut self,
        scrutinee: &Expr,
        arms: &[MatchArm],
    ) -> Result<(), CompileError> {
        // Compile scrutinee
        self.compile_expr(scrutinee)?;

        // Build decision tree
        let tree = self.build_decision_tree(arms)?;

        // Compile decision tree to bytecode
        self.compile_decision_tree(&tree)?;

        Ok(())
    }

    fn build_decision_tree(&self, arms: &[MatchArm]) -> Result<DecisionTree, CompileError> {
        // Strategy: Build efficient decision tree
        // - Group by pattern type
        // - Optimize common prefixes
        // - Generate jump table for literals

        todo!("Decision tree construction")
    }

    fn compile_decision_tree(&mut self, tree: &DecisionTree) -> Result<(), CompileError> {
        match tree {
            DecisionTree::Leaf(body) => {
                self.compile_expr(body)?;
            }
            DecisionTree::Switch { cases, default } => {
                // Compile switch with jump table
                for (pattern, subtree) in cases {
                    // Emit pattern test
                    // Jump to subtree if match
                    self.compile_decision_tree(subtree)?;
                }
                if let Some(default_tree) = default {
                    self.compile_decision_tree(default_tree)?;
                }
            }
        }
        Ok(())
    }
}

enum DecisionTree {
    Leaf(Expr),
    Switch {
        cases: Vec<(Pattern, Box<DecisionTree>)>,
        default: Option<Box<DecisionTree>>,
    },
}
```

## Testing Requirements

```rust
#[test]
fn test_match_literals() {
    let code = r#"
        match 2 with
        | 1 -> "one"
        | 2 -> "two"
        | _ -> "other"
    "#;
    assert_eq!(compile_and_run(code), Value::Str("two".into()));
}

#[test]
fn test_match_list() {
    let code = r#"
        let rec length list =
            match list with
            | [] -> 0
            | _ :: xs -> 1 + length xs
        in length [1; 2; 3]
    "#;
    assert_eq!(compile_and_run(code), Value::Int(3));
}

#[test]
fn test_match_tuple() {
    let code = r#"
        match (1, 2) with
        | (x, y) -> x + y
    "#;
    assert_eq!(compile_and_run(code), Value::Int(3));
}

#[test]
fn test_nested_patterns() {
    let code = r#"
        match ((1, 2), [3; 4]) with
        | ((a, b), x :: xs) -> a + b + x
        | _ -> 0
    "#;
    assert_eq!(compile_and_run(code), Value::Int(6));
}
```

## Implementation Steps
1. **Day 1 MORNING**: Coordination with #018 on pattern AST
2. **Day 1 AFTERNOON**: Define bytecode instructions
3. **Day 2-3**: Implement decision tree compilation
4. **Day 4**: VM pattern matching execution
5. **Day 5**: Testing and optimization

## Estimated Effort
**4-5 days** (Large)

## Coordination Protocol
**Day 1 MANDATORY**: Attend design session with Dev 1
- Understand pattern AST structure
- Define compilation strategy
- Agree on pattern intermediate representation

## Notes
- Decision trees for efficiency
- Simple strategy first, optimize later
- Reference: OCaml pattern compiler, Maranget's algorithm
