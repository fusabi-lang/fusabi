# Audit Summary & Implementation Plan

**Date**: 2025-12-02
**Status**: Audit Complete

## Findings
1.  **Gap Analysis**: Fusabi has a working core (List, String, Option) but misses significant portions of the F# standard library (Array module entirely, List.filter/fold, etc.).
2.  **VM Capabilities**: The VM fully supports Higher-Order Functions (HOFs). `List.map` is already implemented, proving feasibility.
3.  **Documentation**: Docs are currently manual. A strategy to extract them from Rust source comments is proposed.

## Instructions for Claude (Implementation)

Please execute the following tasks to upgrade the Fusabi Standard Library:

### Task 1: Fix Misleading Examples
- **File**: `examples/stdlib_demo.fsx`
- **Action**: Remove "Not implemented yet" comments for `List.map`. It works. Verify `List.map` usage in the script.

### Task 2: Implement Array Module
- **Files**: Create `rust/crates/fusabi-vm/src/stdlib/array.rs`.
- **Register**: Update `rust/crates/fusabi-vm/src/stdlib/mod.rs` to register the module and its functions.
- **Functions to Implement**:
  - `Array.length`
  - `Array.isEmpty`
  - `Array.get` (safe access)
  - `Array.set` (safe access)
  - `Array.ofList`
  - `Array.toList`
  - `Array.init`
  - `Array.create`

### Task 3: Expand List Module
- **File**: `rust/crates/fusabi-vm/src/stdlib/list.rs`
- **Implement**:
  - `list_filter` (using `vm.call_value`)
  - `list_fold` (using `vm.call_value`)
  - `list_iter`
- **Register**: Add to `rust/crates/fusabi-vm/src/stdlib/mod.rs`.

### Task 4: Register "Implicit" Globals
- **File**: `rust/crates/fusabi-vm/src/stdlib/mod.rs`
- **Action**: Register `print` (if available in host/vm) or implement a basic stdout wrapper.

### Task 5: Documentation
- **Action**: Ensure all new functions have `/// Name : Sig` comments.

## Note on Testing
For every new function, add a corresponding test case in the `mod tests` section of the Rust file (unit test) OR a new `.fsx` script in `examples/` (integration test).
