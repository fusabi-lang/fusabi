# FSRS Phase 2 Feature Workstreams

This directory contains all GitHub issues for Phase 2: Language Features (Weeks 4-7).

## Overview

**Objective**: Extend the language to support essential functional programming features: closures, data structures, pattern matching, and type inference.

Phase 2 builds on the Phase 1 MVP to create a production-ready functional scripting language with F#-style ergonomics.

## Issue Organization

```
docs/workstreams/phase-2-features/
├── README.md                     # This file
├── DEPENDENCIES.md               # Dependency graph
├── PARALLELIZATION.md            # Parallel work guide
├── 012-closure-support.md        # Closure implementation
├── 013-let-rec-bindings.md       # Recursive functions
├── 014-currying-partial-app.md   # Currying and partial application
├── 015-tuple-support.md          # Tuple data structures
├── 016-list-support.md           # List data structures
├── 017-array-support.md          # Array data structures (optional)
├── 018-pattern-matching.md       # Pattern matching expressions
├── 019-pattern-compiler.md       # Pattern match compilation
├── 020-type-inference.md         # Hindley-Milner type inference
└── 021-type-checker.md           # Type checking and validation
```

## Phase 2 Goals

### Success Criteria
- Execute closures with proper variable capture
- Run recursive and mutually recursive functions
- Use tuples, lists, and arrays in scripts
- Pattern match over data structures
- Infer types for polymorphic functions
- Reject ill-typed programs before compilation
- 150+ total unit tests passing
- All examples from language spec working

## Milestone Breakdown

| Milestone | Duration | Issues | Description |
|-----------|----------|--------|-------------|
| **2.1: Functions & Closures** | Week 4 | #012-#014 | Closures, let-rec, currying |
| **2.2: Data Structures** | Week 5 | #015-#017 | Tuples, lists, arrays |
| **2.3: Pattern Matching** | Week 6 | #018-#019 | Match expressions, compilation |
| **2.4: Type System** | Week 7 | #020-#021 | Type inference, checking |

**Total**: 10 issues across 4 weeks

## Parallel Tracks (3 Developers)

Phase 2 is organized into 3 parallel development tracks that minimize conflicts:

### Track A: Frontend Extensions (Dev 1)
**Focus**: AST, Parser, Type System

**Week 4**:
- #013: Let-Rec Bindings (3-4 days)
- Support #012 with parser changes

**Week 5**:
- #015: Tuple Support (3-4 days)
- #016: List Support (2-3 days - parallel end)

**Week 6**:
- #018: Pattern Matching (4-5 days - AST/Parser)

**Week 7**:
- #020: Type Inference (5-6 days - CRITICAL)

**Skills**: Parser design, type theory, AST transformations

---

### Track B: VM Runtime (Dev 2)
**Focus**: VM, Bytecode, Value System

**Week 4**:
- #012: Closure Support (4-5 days - CRITICAL)

**Week 5**:
- #017: Array Support (2-3 days)
- Support #015, #016 with value types

**Week 6**:
- #019: Pattern Compiler (4-5 days - Bytecode generation)

**Week 7**:
- #021: Type Checker (3-4 days)

**Skills**: VM implementation, bytecode design, runtime optimization

---

### Track C: Integration & Optimization (Dev 3)
**Focus**: Currying, Integration, Testing

**Week 4**:
- #014: Currying & Partial Application (3-4 days)

**Week 5**:
- Integration testing for data structures
- Performance benchmarks

**Week 6**:
- Pattern matching integration tests
- Example scripts

**Week 7**:
- Type system integration
- End-to-end validation
- Documentation updates

**Skills**: Integration testing, optimization, documentation

---

## Week-by-Week Timeline

### Week 4: Functions & Closures
```
Dev 1: #013 Let-Rec          ████████░░░░ (3-4d)
Dev 2: #012 Closures         ████████████ (4-5d) CRITICAL
Dev 3: #014 Currying         ████████░░░░ (3-4d)
```

**Deliverables**: Recursive functions, closures, partial application

---

### Week 5: Data Structures
```
Dev 1: #015 Tuples           ████████░░░░ (3-4d)
       #016 Lists            ██████░░░░░░ (2-3d)
Dev 2: #017 Arrays           ██████░░░░░░ (2-3d)
       Integration Support   ████░░░░░░░░ (2d)
Dev 3: Testing & Examples    ████████████ (full week)
```

**Deliverables**: Tuples, lists, arrays, data structure examples

---

### Week 6: Pattern Matching
```
Dev 1: #018 Match AST        ████████████ (4-5d)
Dev 2: #019 Pattern Compiler ████████████ (4-5d)
Dev 3: Integration & Tests   ████████████ (full week)
```

**Deliverables**: Full pattern matching support, decision trees

---

### Week 7: Type System
```
Dev 1: #020 Type Inference   ████████████ (5-6d) CRITICAL
Dev 2: #021 Type Checker     ████████░░░░ (3-4d)
Dev 3: Integration & Polish  ████████████ (full week)
```

**Deliverables**: Hindley-Milner inference, type checking, Phase 2 complete

---

## Parallelization Strategy

### Parallelizable Work (8/10 issues can parallel)

**Week 4**: All 3 tracks independent
- Frontend: Let-rec (AST changes)
- VM: Closures (Value/VM changes)
- Integration: Currying (uses both)

**Week 5**: Parallel data structure work
- Tuples, Lists, Arrays can be developed simultaneously
- Different value types, minimal conflicts

**Week 6**: Coordinated pattern matching
- AST/Parser work (Track A)
- Bytecode compilation (Track B)
- Clear interface between tracks

**Week 7**: Sequential type system
- Type inference is critical path
- Type checker depends on inference
- Integration work throughout

### File Ownership

| Track | Primary Files | Crates |
|-------|--------------|--------|
| **Frontend** | `ast.rs`, `parser.rs`, `types.rs` | `fsrs-frontend` |
| **VM** | `value.rs`, `bytecode.rs`, `vm.rs` | `fsrs-vm` |
| **Integration** | `compiler.rs`, `main.rs`, tests | `fsrs-frontend`, `fsrs-demo` |

### Conflict Prevention
- Each track owns specific modules
- Coordination on shared interfaces
- Daily standups for dependency updates
- PR reviews across tracks

## Critical Path Analysis

### Longest Path: 24 days
```
#012 Closures (5d) → #018 Pattern AST (5d) → #020 Type Inference (6d) → #021 Type Checker (4d) → Integration (4d)
```

**With 3 developers parallelizing**: ~28 days (4 weeks)

### Bottlenecks
1. **#012 (Closures)**: Foundational for all advanced features
2. **#020 (Type Inference)**: Complex, requires closures + data structures
3. **Pattern matching**: Needs coordination between frontend and VM

### Mitigation Strategies
- Start #012 immediately in Week 4
- Prepare type inference design in Week 5-6
- Incremental integration throughout

## Label System

### Type Labels
- `feature` - New feature implementation
- `enhancement` - Improvement to existing feature
- `infrastructure` - Build/tooling updates

### Priority Labels
- `priority: critical` - Blocking, must complete
- `priority: high` - Important for milestone
- `priority: medium` - Normal priority

### Status Labels
- `blocked` - Blocked by another issue
- `in-progress` - Currently being worked on
- `ready-for-review` - Awaiting code review

### Phase Labels
- `phase-2: features` - Phase 2 work

### Component Labels
- `component: frontend` - Parser/AST work
- `component: vm` - VM runtime work
- `component: types` - Type system work
- `component: integration` - Cross-component work

### Effort Labels
- `effort: s` - 1-2 days
- `effort: m` - 2-4 days
- `effort: l` - 4-7 days

### Dependency Labels
- `parallel-safe` - Can work in parallel
- `foundational` - Others depend on this
- `requires-coordination` - Cross-track coordination needed

## Development Workflow

### Branch Naming
```
feat/issue-012-closure-support
feat/issue-020-type-inference
fix/issue-018-pattern-bug
```

### Commit Message Format
```
feat(vm): implement closure support (#012)

- Add Closure value type with upvalues
- Implement upvalue capture mechanism
- Add ClosureCall instruction
- Include comprehensive closure tests

Closes #012
```

### PR Requirements
- All tests pass (`just test`)
- Clippy clean (`just lint`)
- Formatted (`just fmt`)
- Documentation updated
- No merge conflicts
- Cross-track review if touching shared code

## Getting Started

### Prerequisites
- Phase 1 MVP complete
- Rust 1.70+
- Nushell 0.90+
- Just command runner

### Workflow
1. Pick issue from assigned track
2. Check dependencies are complete
3. Create feature branch
4. Implement with TDD
5. Open draft PR early
6. Coordinate with other tracks
7. Request review when ready
8. Merge and delete branch

## Common Commands

```bash
# Development
just dev                  # Watch mode
just test                 # Run all tests
just test-crate fsrs-frontend
just test-crate fsrs-vm

# Quality
just check                # fmt + lint + test
just fmt                  # Format code
just lint                 # Run clippy

# Examples
just example closures     # Run closure examples
just example pattern      # Run pattern matching examples
```

## Success Metrics

### Technical Metrics
- 150+ total unit tests (Phase 1 + Phase 2)
- All language spec examples working
- No performance regressions
- Type inference < 100ms for typical scripts

### Quality Metrics
- Zero clippy warnings
- 100% of public APIs documented
- Clear error messages for type errors
- Comprehensive pattern matching coverage

### Integration Metrics
- End-to-end scripts demonstrating all features
- Successful host interop with complex types
- Hot-reload working with new features

## Resources

- **[ROADMAP.md](../../ROADMAP.md)** - Overall project roadmap
- **[02-language-spec.md](../../02-language-spec.md)** - Language specification
- **[03-vm-design.md](../../03-vm-design.md)** - VM architecture
- **[Phase 1 Workstreams](../phase-1-mvp/README.md)** - Phase 1 reference

## Support

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Design discussions, Q&A
- **CLAUDE.md**: Claude Code workflow guidance

---

**Phase Duration**: 4 weeks (Weeks 4-7)
**Total Issues**: 10
**Parallel Capacity**: 3 simultaneous tracks
**Target Completion**: End of Week 7
**Estimated Team Velocity**: 2.5 issues/week with parallelization
