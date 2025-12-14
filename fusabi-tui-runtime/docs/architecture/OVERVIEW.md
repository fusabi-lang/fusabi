# Fusabi TUI Runtime Architecture Overview

## Executive Summary

`fusabi-tui-runtime` is a unified TUI framework that enables Fusabi applications to run **standalone** (via terminal) OR as **Scarab plugins** (via shared memory) using the same codebase.

## Problem Statement

The raibid-labs ecosystem has multiple TUI applications:
- **Scryforge** - Information aggregator (50K LOC, Ratatui-based)
- **Phage** - Context composition engine (25K LOC, Ratatui-based)
- **Sigilforge** - Credential management (5K LOC, CLI-only)

These applications share common patterns but have no unified runtime that supports both standalone execution and Scarab plugin integration.

## Solution: Dual-Mode Runtime

```
                         Application Layer
  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐
  │   Scryforge   │  │    Phage      │  │  Sigilforge   │
  │   (Fusabi)    │  │   (Fusabi)    │  │   (Fusabi)    │
  └───────┬───────┘  └───────┬───────┘  └───────┬───────┘
          │                  │                  │
          ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│                  fusabi-tui-runtime                     │
│  ┌────────────────────────────────────────────────────┐ │
│  │ Core Types (from ratatui-core, ~2K LOC)           │ │
│  │  • Cell, Buffer, Rect, Constraint, Layout, Style  │ │
│  └────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────┐ │
│  │ Widget System (Fusabi-first)                      │ │
│  │  • FusabiWidget - renders via .fsx/.fzb           │ │
│  │  • RustWidget - high-performance Rust widgets     │ │
│  └────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────┐ │
│  │ Renderer Abstraction                              │ │
│  │  • CrosstermRenderer (standalone)                 │ │
│  │  • ScarabRenderer (plugin mode)                   │ │
│  │  • TestRenderer (unit tests)                      │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
          │                                    │
          ▼                                    ▼
┌──────────────────────┐        ┌──────────────────────┐
│   Standalone Mode    │        │  Scarab Plugin Mode  │
│  • Terminal I/O      │        │  • SharedState IPC   │
│  • Crossterm events  │        │  • Zero-copy render  │
│  • Escape sequences  │        │  • Unix socket input │
└──────────────────────┘        └──────────────────────┘
```

## Design Principles

### 1. Borrow, Don't Fork

Instead of forking ratatui's 80K LOC codebase:
- **Keep**: Cell, Buffer, Rect, Layout, Constraint, Style (~2K LOC from ratatui-core)
- **Discard**: Terminal, Backend, Frame (terminal-specific)
- **Create**: New Renderer abstraction supporting both modes

### 2. Fusabi-First Widget Model

Widgets can be defined in:
- **Fusabi (.fsx)**: Hot-reloadable, declarative, UI logic
- **Rust**: Performance-critical rendering primitives

### 3. Same Code, Two Modes

```fsharp
// This code runs in BOTH modes unchanged
let render state =
    Layout.horizontal [
        StreamList.render state.streams, Constraint.Percentage 20
        ItemList.render state.items, Constraint.Fill 1
    ]
```

The framework detects `SCARAB_PLUGIN` environment variable to choose renderer.

## Core Components

### 1. Core Types (from ratatui-core)

| Type | Purpose | Source |
|------|---------|--------|
| `Cell` | Character + style + modifiers | ratatui-core/buffer/cell.rs |
| `Buffer` | Grid of cells | ratatui-core/buffer/buffer.rs |
| `Rect` | Area rectangle | ratatui-core/layout/rect.rs |
| `Constraint` | Layout constraint enum | ratatui-core/layout/constraint.rs |
| `Layout` | Cassowary constraint solver | ratatui-core/layout/layout.rs |
| `Style` | Color + modifiers | ratatui-core/style/ |

### 2. Renderer Trait

```rust
pub trait Renderer: Send + Sync {
    type Error: std::error::Error;

    fn render(&mut self, buffer: &Buffer) -> Result<(), Self::Error>;
    fn poll_event(&mut self, timeout: Duration) -> Option<Event>;
    fn size(&self) -> Size;
    fn cleanup(&mut self) -> Result<(), Self::Error>;
}
```

### 3. Widget Traits

```rust
// Rust-defined widgets
pub trait RustWidget {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

// Fusabi-defined widgets
pub trait FusabiWidget {
    fn render(&self, area: Rect, vm: &FusabiVm) -> Buffer;
}
```

## Renderer Implementations

### CrosstermRenderer (Standalone Mode)

- Uses crossterm for terminal I/O
- Handles keyboard/mouse events
- Renders via ANSI escape sequences
- ~400 LOC (borrowed from ratatui-crossterm)

### ScarabRenderer (Plugin Mode)

- Writes to `SharedState` via shared memory
- Receives events via Unix domain socket
- Zero-copy cell transfer
- Uses atomic sequence numbers for synchronization
- ~300 LOC

### TestRenderer (Testing)

- In-memory buffer capture
- Event injection for testing
- Snapshot testing support
- ~100 LOC

## Comparison with Alternatives

| Approach | LOC | Pros | Cons |
|----------|-----|------|------|
| **Fork ratatui** | 80K+ | Full widget library | Maintenance burden, terminal-coupled |
| **Use ratatui directly** | 0 | Mature | Can't integrate with Scarab |
| **fusabi-tui-runtime** | ~8K | Dual-mode, lightweight | New development |
| **Write from scratch** | 15K+ | Full control | Reinventing wheels |

## Implementation Phases

### Phase 1: Core Types
- Port Cell, Buffer, Rect, Style from ratatui-core
- Implement Constraint and Layout with Cassowary solver
- Target: 2K LOC

### Phase 2: Renderer Abstraction
- Define Renderer trait
- Implement CrosstermRenderer
- Implement ScarabRenderer
- Target: 800 LOC

### Phase 3: Widget System
- Define RustWidget and FusabiWidget traits
- Port essential widgets (Block, Paragraph, List, Table)
- Create Fusabi bindings
- Target: 3K LOC

### Phase 4: Application Framework
- Event loop abstraction
- State management patterns
- Hot-reload integration
- Target: 2K LOC

## Related Documents

- [RATATUI_ANALYSIS.md](./RATATUI_ANALYSIS.md) - Deep dive into ratatui architecture
- [RENDERER_DESIGN.md](../design/RENDERER_DESIGN.md) - Renderer trait specification
- [WIDGET_SYSTEM.md](../design/WIDGET_SYSTEM.md) - Widget trait and bindings
- [MIGRATION_GUIDE.md](../MIGRATION_GUIDE.md) - Migrating existing apps
