# Roadmap: The Road to v1.0

**Date**: 2025-12-02
**Status**: Strategic

## Phase 1: The "Smooth" Release (Polish)
Before adding more features, we must make the current ones feel professional.

1.  **Error Reporting**: Implement "pretty" error messages with code snippets.
    *   *Mechanism*: VM needs to track `Span` in `Chunk` (debug info).
2.  **FPM Implementation**:
    *   Implement `fpm install` (git clone).
    *   Implement `fpm build` (dependency resolution).

## Phase 2: The "Connected" Release (IO)
1.  **HTTP Client**: Add `Http` module to stdlib (requires `reqwest` feature in VM).
2.  **File IO**: Expand `File` module (read/write string is done, need streams/lines).

## Phase 3: The "Smart" Release (Tooling)
1.  **LSP Server**: Start `fusabi-lsp` crate.
    *   Features: Syntax highlighting, Go to definition.

## Instructions for Developer (Claude)
**Next Session Goal**: Implement `fpm install`.
1.  Add `git2` dependency to `fusabi-pm`.
2.  Implement logic to clone dependencies defined in `fusabi.toml` into `fusabi_packages/`.
