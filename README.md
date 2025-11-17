# fsrs

`fsrs` is an experimental Mini‑F# dialect plus embeddable Rust VM designed to replace Lua‑style scripting in tools like terminal emulators (e.g. WezTerm).

The goal:

- F#-style developer ergonomics (records, DUs, pattern matching, pipelines, simple modules).
- A small, eager, expression‑oriented language suitable for **embedded scripting and configs**.
- A Lua‑class bytecode VM implemented in Rust (no .NET, no LLVM, no WASM required).
- Designed for use in a Rust host (terminal emulator, CLI, etc.) with hot‑path callbacks.

This repository is **bootstrap scaffolding**, design docs, and example configs to get the project started.

## Layout

- `README.md` — high‑level overview and goals (this file).
- `docs/` — design docs for the language, VM, and embedding.
- `examples/` — example Mini‑F# (`.fsrs`) configs and callback modules.
- `scripts/` — helper scripts (Nushell) to scaffold the Rust workspace.
- `rust/` — initial Rust workspace skeleton (front‑end, VM, demo host).

## Getting started

### 1. Prereqs

- Rust (latest stable) with `cargo`.
- Nushell (for the optional bootstrap script): <https://www.nushell.sh>
- A code assistant such as Claude Code in your editor (optional but recommended).

### 2. Bootstrap the Rust workspace (optional)

If you want to recreate the workspace from scratch, you can run:

```nu
use scripts/bootstrap.nu *
bootstrap
```

This will:

- Create/refresh the `rust/` Cargo workspace.
- Create crates:
  - `fsrs-frontend` — parser, typechecker, desugaring for the Mini‑F# dialect.
  - `fsrs-vm` — bytecode VM, value representation, GC, and built‑ins.
  - `fsrs-demo` — small binary demonstrating embedding the language in a host program.

You can also inspect the script and just follow its steps manually.

### 3. Build and run the demo host

From the `rust/` directory:

```bash
cd rust
cargo build
cargo run -p fsrs-demo
```

Initially, `fsrs-demo` just:

- Loads an example `.fsrs` script from `../examples/`.
- Pretends to parse/compile it (stubbed).
- Prints placeholder information to show the integration points you will fill in.

### 4. How to use these files with Claude Code

See `docs/CLAUDE_CODE_NOTES.md` for detailed prompts and task breakdowns tailored for Claude Code.

High‑level flow:

1. Start in `rust/crates/fsrs-frontend/src/lib.rs` and follow the **Phase 1** tasks in `docs/CLAUDE_CODE_NOTES.md` to:
   - Define the core AST,
   - Implement a minimal tokenizer and parser for the Mini‑F# subset,
   - Add basic error reporting.

2. Move on to the `fsrs-vm` crate and implement:
   - `Value`, `Instruction`, `Chunk`, `Vm` structs,
   - A minimal interpreter loop,
   - A few built‑ins (ints, bools, strings, simple arithmetic).

3. Wire the two together in `fsrs-demo`:
   - Parse & compile `examples/minifs_config.fsrs`,
   - Execute it in the VM,
   - Extract data for a pretend “terminal config”.

Each step is broken down in the design docs to be friendly to an AI code assistant.

---
