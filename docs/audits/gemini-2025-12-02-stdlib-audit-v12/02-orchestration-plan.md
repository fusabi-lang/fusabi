# Parallel Orchestration Plan V12

**Date**: 2025-12-03
**Status**: Active

## Work Streams

Based on Audit V12 findings, noting that Async Parser was completed in V11:

### Stream A: Connect Package Manager to CLI - PR #193
**Branch**: `feat/fpm-cli-integration`
**Deliverable**: Updated `rust/crates/fusabi/src/main.rs`

Wire up `fus root` (rename to `fus pm`) to delegate to `fpm`:
- Rename "root" command to "pm" (more descriptive)
- Execute `fpm` subprocess with passed arguments
- Update help text to document `fpm` commands
- Fallback message if `fpm` binary not found

**Dependencies**: None

---

## Execution Order

```
┌─────────────────────────────────────────────────────────────┐
│                    SINGLE STREAM                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Stream A ─────────────────────►                            │
│  (FPM CLI Integration)                                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Technical Notes

### Why subprocess delegation?
`fusabi-pm` depends on `fusabi` for compilation/execution. If `fusabi` depended
on `fusabi-pm`, we'd have a circular dependency. Instead, `fus pm` executes
the `fpm` binary directly, similar to how `cargo` delegates to `rustc`.

### Command mapping
- `fus pm init` → `fpm init`
- `fus pm build` → `fpm build`
- `fus pm run` → `fpm run`
- `fus pm add <pkg>` → `fpm add <pkg>`

## Success Criteria

- [ ] `fus pm init` creates new Fusabi package
- [ ] `fus pm build` compiles package to bytecode
- [ ] `fus pm run` executes package
- [ ] Help text documents package manager commands
- [ ] All PRs merged, no open issues
- [ ] Release v0.29.0
