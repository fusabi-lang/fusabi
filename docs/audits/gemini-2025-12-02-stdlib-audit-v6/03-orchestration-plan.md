# Parallel Orchestration Plan V6

**Date**: 2025-12-03
**Status**: Active

## Work Streams

Based on Audit V6 findings, the following work can be executed in parallel:

### Stream A: List Module Polish - PR #172
**Branch**: `feat/list-nth-mapi`
**Deliverable**: `rust/crates/fusabi-vm/src/stdlib/list.rs` updates

Implement missing List functions required by commander.fsx:
- `List.nth : int -> 'a list -> 'a option` - Get element at index
- `List.mapi : (int -> 'a -> 'b) -> 'a list -> 'b list` - Map with index

**Implementation**:
1. Add functions to `list.rs`
2. Register in `mod.rs` (host registry + globals)
3. Add unit tests
4. Update documentation

**Dependencies**: None

---

### Stream B: Console Module - PR #173
**Branch**: `feat/console-module`
**Deliverable**: New `rust/crates/fusabi-vm/src/stdlib/console.rs`

Create Console module for user input:
- `Console.readLine : unit -> string` - Blocking line input
- `Console.readKey : unit -> string` - Blocking single key input
- `Console.isKeyAvailable : unit -> bool` - Non-blocking key check

**Implementation**:
1. Create `console.rs` module
2. Register in `mod.rs`
3. Add unit tests
4. Update gen-docs.nu

**Dependencies**: None

---

### Stream C: Commander TUI Update - PR #174
**Branch**: `feat/commander-interactive`
**Deliverable**: Updated `examples/commander.fsx`

Update commander.fsx to use real interactive input:
- Replace demo command list with actual Console.readKey loop
- Implement proper event loop with user input
- Add raw mode terminal handling

**Dependencies**: Streams A and B must complete first

---

## Execution Order

```
┌─────────────────────────────────────────────────────────────┐
│                    PARALLEL EXECUTION                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Stream A ─────────────────────►                            │
│  (List.nth + List.mapi)         │                           │
│                                 ├──► Stream C               │
│  Stream B ─────────────────────►│   (Commander Update)      │
│  (Console Module)               │                           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## PR Merge Strategy

1. Merge Streams A and B in parallel (no dependencies)
2. Merge Stream C after A and B are merged
3. Each PR should include tests

## Success Criteria

- [ ] `List.nth` and `List.mapi` implemented with tests
- [ ] `Console` module provides readLine/readKey
- [ ] `commander.fsx` runs interactively (not simulation)
- [ ] All PRs pass CI and are merged to main
