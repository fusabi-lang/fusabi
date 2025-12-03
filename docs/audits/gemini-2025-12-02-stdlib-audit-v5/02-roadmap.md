# Roadmap V5: The "Dogfooding" Phase

**Date**: 2025-12-02
**Status**: Active

We have a rich standard library but untested integration. Before adding major language features like Computation Expressions, we must prove the platform's viability by building a real application.

## Phase 1: The "Commander" TUI (Proof of Concept)
Build a simple TUI file explorer/process manager entirely in Fusabi (`examples/commander.fsx`).

**Goal**: Prove `TerminalControl`, `Events`, `Process`, and `List` work together.

**Requirements**:
1.  **Loop**: An event loop listening for keypresses.
2.  **Render**: Draw a list of files or processes to the screen using `TerminalControl`.
3.  **Interact**: Arrow keys to move selection, Enter to "action" (e.g., print file path or kill process).
4.  **Architecture**: Use a simple Model-View-Update (MVU) architecture if possible with current functions.

## Phase 2: Computation Expressions (Research & Design)
Start the design work for CEs.
1.  **Spec**: Define the syntax for `builder { ... }` in Fusabi.
2.  **Desugaring**: Map out how `let!` translates to `builder.Bind`.
3.  **Implementation**: This is a compiler task, not a stdlib task.

## Phase 3: Package Management (Foundations)
1.  **`fusabi.toml`**: Define a project manifest format.
2.  **`fpm` (Fusabi Package Manager)**: A simple CLI tool (written in Fusabi?) to manage dependencies.

## Instructions for Developer (Claude)
**Immediate Task**: Execute Phase 1.
1.  Create `examples/commander.fsx`.
2.  Implement a basic interactive loop using the `Events` and `TerminalControl` modules.
3.  Report any bugs or missing APIs discovered in the low-level terminal modules during this process.
