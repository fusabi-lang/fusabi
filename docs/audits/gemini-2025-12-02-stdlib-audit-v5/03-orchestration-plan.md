# Parallel Orchestration Plan

**Date**: 2025-12-03
**Status**: Active

## Work Streams

Based on Audit V5 findings, the following work can be executed in parallel:

### Stream A: Commander TUI (Priority 1) - PR #169
**Branch**: `feat/commander-tui`
**Deliverable**: `examples/commander.fsx`

A proof-of-concept TUI file explorer demonstrating:
- Event loop using `Events` module
- Terminal rendering using `TerminalControl`
- Keyboard interaction handling
- Simple MVU (Model-View-Update) architecture

**Dependencies**: None (uses existing stdlib)

---

### Stream B: Computation Expressions Spec (Priority 2) - PR #170
**Branch**: `docs/ce-design-spec`
**Deliverable**: `docs/design/computation-expressions.md`

Research and design document covering:
- Proposed syntax for `builder { ... }` blocks
- Desugaring rules for `let!`, `do!`, `return`, `return!`
- Builder interface requirements
- Example: `async`, `result`, `option` builders
- Implementation roadmap for compiler changes

**Dependencies**: None (research/documentation only)

---

### Stream C: Package Management Foundations (Priority 3) - PR #171
**Branch**: `docs/package-management-spec`
**Deliverable**: `docs/design/package-management.md`

Design specification for:
- `fusabi.toml` manifest format
- Dependency resolution algorithm
- Package registry architecture
- `fpm` CLI command structure

**Dependencies**: None (research/documentation only)

---

## Execution Order

```
┌─────────────────────────────────────────────────────────────┐
│                    PARALLEL EXECUTION                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Stream A ─────────────────────────────────────────────►    │
│  (Commander TUI - MAIN PRIORITY)                            │
│                                                             │
│  Stream B ─────────────────────────►                        │
│  (CE Spec - Documentation)                                  │
│                                                             │
│  Stream C ─────────────────────────►                        │
│  (Package Mgmt Spec - Documentation)                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## PR Merge Strategy

1. Merge documentation PRs (B, C) first as they have no code dependencies
2. Merge Commander TUI (A) after thorough testing
3. Each PR should be self-contained and independently reviewable

## Success Criteria

- [ ] `examples/commander.fsx` runs and demonstrates working TUI
- [ ] CE design spec provides clear implementation guidance
- [ ] Package management spec defines complete manifest format
- [ ] All PRs pass CI and are merged to main
