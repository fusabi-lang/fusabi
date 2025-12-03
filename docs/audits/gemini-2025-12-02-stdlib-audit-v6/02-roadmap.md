# Roadmap V6: The Interactive Era

**Date**: 2025-12-02
**Status**: Active

The "Commander" demo revealed critical gaps in input handling and list operations.

## Phase 1: Standard Library Polish (List)
The demo script assumes functions that might not exist.
1.  **Audit `stdlib/list.rs`**: Check for `nth` and `mapi`.
2.  **Implement**:
    - `List.nth : int -> 'a list -> 'a option`
    - `List.mapi : (int -> 'a -> 'b) -> 'a list -> 'b list`
    - Register in `mod.rs` and update `gen-docs.nu`.

## Phase 2: True Interactivity (Input)
We cannot build a TUI without reading keys.
1.  **Create `stdlib/console.rs`**:
    - `Console.readLine : unit -> string` (Blocking standard input read)
    - `Console.readKey : unit -> string` (Blocking single key read - requires raw mode support in VM or host).
2.  **Update `commander.fsx`**: Replace the "simulated loop" with a real `while` loop using `Console.readKey`.

## Phase 3: Computation Expressions (Design)
Start RFC for CEs.

## Instructions
1.  **Implement `List.nth` and `List.mapi`**. This unblocks the demo script.
2.  **Implement `Console` module**. This enables the demo script.
