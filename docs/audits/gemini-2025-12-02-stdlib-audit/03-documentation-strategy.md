# Documentation Strategy

**Date**: 2025-12-02
**Auditor**: Gemini Agent

## Current State
- **Source of Truth**: The standard library is implemented in Rust (`fusabi-vm/src/stdlib/*.rs`).
- **Inline Docs**: The Rust code contains standard `///` doc comments with signatures (e.g., `/// List.length : 'a list -> int`).
- **User Docs**: `docs/stdlib-implementation.md` exists but is manual.

## Strategy: "Single Source of Truth"
We should not maintain separate markdown documentation that risks drifting from the code. Instead, we should generate user-facing documentation directly from the Rust implementation files.

### Proposed Workflow
1.  **Standardize Comments**: Ensure every exported native function in `src/stdlib/*.rs` has a `///` comment block formatted as:
    ```rust
    /// FunctionSignature : type -> type
    /// Description...
    ```
2.  **Extraction Tool**: Build a simple CLI command (or script in `scripts/`) that:
    - Scans `rust/crates/fusabi-vm/src/stdlib/*.rs`.
    - Parses `///` comments associated with `pub fn`.
    - Extracts the function name from the registry string (e.g., `registry.register("List.length", ...)`). *Note: This is harder because registration is in `mod.rs` but implementation is in `list.rs`. Map them by convention.*
    - **Better Approach**: Put the docs on the *implementation functions* in `list.rs`, `string.rs` etc., and have the tool scan those files. The tool can look for `/// Name : Sig` patterns.

### Implementation Plan (MVP)
Create a script `scripts/gen-stdlib-docs.nu` (NuShell) or `rs` that:
1.  Reads `list.rs`, `string.rs`, `option.rs`.
2.  Matches regex: `///\s*(.*) : (.*)\n///\s*(.*)` (simplified).
3.  Generates a `STDLIB_REFERENCE.md` file.

### Future "F# Style" XML Docs
In the future, when the Fusabi compiler is self-hosted or more mature, we might support `///` comments in `.fsx` files for user code. For now, focusing on the Rust-hosted stdlib is sufficient.

