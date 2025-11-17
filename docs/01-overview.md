# fsrs Overview

`fsrs` is a **Mini‑F# dialect + Rust VM** primarily intended for embedded scripting in a Rust host application, such as a terminal emulator.

The language:

- Feels like a small subset of F#:
  - Records, discriminated unions, pattern matching.
  - `let` bindings, functions, `if/then/else`, `match`.
  - Modules and pipelines (`|>`, `>>`, `<<`).
  - Optional computation expression (CE) sugar for domain‑specific DSLs.

The runtime:

- A **Lua‑style bytecode interpreter** implemented in Rust.
- Eager, call‑by‑value evaluation.
- Tagged `Value` representation for ints, bools, strings, lists, records, variants, closures.
- Simple mark‑and‑sweep GC, tuned for:
  - A few long‑lived configs,
  - Many short‑lived values in callbacks.

The target use case is to replace Lua in something like WezTerm:

- Configuration: written in the Mini‑F# dialect instead of Lua.
- Runtime scripting: callbacks (tab formatting, key handling) implemented in the dialect.
- Host: Rust, exposing an API surface into the VM (similar to `wezterm.*` in Lua).

## Architectural components

1. **Front‑end (`fsrs-frontend` crate)**

   - Tokenizer and parser for the Mini‑F# syntax subset.
   - Hindley‑Milner‑style type inference for:
     - Ints, floats, bools, strings, unit.
     - Lists, arrays, tuples, options.
     - Records and discriminated unions.
   - Desugaring passes:
     - Computation expressions → builder calls.
     - Pipelines → nested function calls.
     - Pattern matching → simpler core decision trees.
   - Core AST suitable for compilation into bytecode.

2. **VM / runtime (`fsrs-vm` crate)**

   - `Value` enum representing runtime values.
   - `Instruction` enum and bytecode `Chunk` representation.
   - `Vm` struct managing:
     - Call stack and frames,
     - Globals and upvalues,
     - GC roots and heap allocation.
   - Built‑in functions and host interop stubs:
     - You will later expose host APIs (terminal actions, logging, etc.).

3. **Host demo (`fsrs-demo` crate)**

   - Small binary that:
     - Loads a `.fsrs` file from `examples/`.
     - Uses `fsrs-frontend` to parse/compile it to bytecode.
     - Executes the bytecode in `fsrs-vm`.
   - Initialize host → script wiring:
     - Register a few built‑in functions (e.g. `print`, `add`).
     - Demonstrate calling script functions from Rust and vice versa.

## Phased development strategy

To keep the project tractable, develop in phases:

### Phase 1: Core language + interpreter skeleton

- Define the core AST.
- Implement tokenizer + parser for:
  - Literals, identifiers, `let`, functions, application.
  - Simple `if/then/else`.
- Define `Value`, `Instruction`, and `Vm`.
- Implement a minimal interpreter that can:
  - Evaluate integer arithmetic,
  - Call a few built‑in functions,
  - Run a trivial script.

### Phase 2: Types, records, and unions

- Add a simple Hindley‑Milner typechecker.
- Introduce:
  - Records and DUs,
  - Pattern matching over them.
- Extend bytecode and interpreter to handle records/variants.

### Phase 3: Modules, DSLs, and host embedding

- Add basic module support to the front‑end.
- Introduce computation expressions for domain‑specific DSLs (e.g. layouts).
- Flesh out the embedding API on the Rust side (similar to `mlua` but strongly typed).

Each phase is described in more detail in the other docs; this file is the high‑level overview.
