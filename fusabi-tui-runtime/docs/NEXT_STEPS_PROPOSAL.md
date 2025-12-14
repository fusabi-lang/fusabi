# Next Steps Proposal: fusabi-tui-runtime Implementation

**Date**: 2025-12-14
**Status**: Ready for Implementation
**Prerequisites Completed**: RFC-003 (Multi-file modules), RFC-004 (Async Tokio)

## Executive Summary

With the multi-file module system and async Tokio integration now available in Fusabi v0.35.0, we are ready to proceed with the fusabi-tui-runtime implementation. This proposal outlines the phased approach to building a unified TUI framework for the Raibid Labs ecosystem.

---

## Phase 1: Core Foundation (Target: v0.36.0)

### 1.1 Create fusabi-tui-core Crate

**Goal**: Extract and adapt core primitives from ratatui-core

**Components to Borrow (~2K LOC)**:
- `Cell` - Single terminal cell (char + style)
- `Buffer` - 2D grid of cells
- `Rect` - Position and size
- `Constraint` - Layout constraints
- `Layout` - Constraint solver
- `Style` - Text styling (fg, bg, modifiers)

**Implementation**:
```rust
// crates/fusabi-tui-core/src/lib.rs
pub mod buffer;   // Cell, Buffer
pub mod layout;   // Rect, Constraint, Layout
pub mod style;    // Style, Color, Modifier
pub mod symbols;  // Box-drawing characters
```

### 1.2 Renderer Abstraction Layer

**Goal**: Abstract terminal backend for dual-mode operation

```rust
pub trait Renderer: Send + Sync {
    fn draw(&mut self, buffer: &Buffer) -> Result<(), RenderError>;
    fn flush(&mut self) -> Result<(), RenderError>;
    fn size(&self) -> Rect;
    fn clear(&mut self) -> Result<(), RenderError>;
}

// Implementations
pub struct CrosstermRenderer { ... }  // Standalone mode
pub struct ScarabRenderer { ... }     // Plugin mode (shared memory)
pub struct TestRenderer { ... }       // Testing via ratatui-testlib
```

### 1.3 Fusabi Bindings

**Goal**: Expose core types to Fusabi scripts

```fsharp
// cell.fsx
module Cell =
    type t = { char: char; fg: Color; bg: Color; mods: Modifier }
    let empty = { char = ' '; fg = White; bg = Black; mods = None }

// buffer.fsx
module Buffer =
    let create width height = ...
    let set x y cell buffer = ...
    let get x y buffer = ...
```

---

## Phase 2: Widget Framework (Target: v0.37.0)

### 2.1 Base Widget Trait

```rust
pub trait Widget {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

pub trait StatefulWidget {
    type State;
    fn render(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

### 2.2 Core Widgets

**Priority Order**:
1. `Block` - Bordered container with title
2. `Paragraph` - Text display with wrapping
3. `List` - Scrollable list with selection
4. `Table` - Tabular data display
5. `Gauge` - Progress/percentage bar
6. `Sparkline` - Inline mini-chart
7. `Tabs` - Tab navigation

### 2.3 Fusabi Widget System

```fsharp
// widgets.fsx
#load "cell.fsx"
#load "buffer.fsx"

module Widget =
    type t = {
        render: Rect -> Buffer -> unit
    }

    let block title children = ...
    let paragraph text = ...
    let list items = ...
```

---

## Phase 3: Hot Reload Engine (Target: v0.38.0)

### 3.1 DashboardEngine

**Goal**: Orchestrate hot-reloadable TUI dashboards

```rust
pub struct DashboardEngine {
    renderer: Box<dyn Renderer>,
    loader: FileLoader,
    watcher: FileWatcher,
    state: DashboardState,
}

impl DashboardEngine {
    pub fn load(&mut self, path: &Path) -> Result<(), Error>;
    pub fn reload(&mut self) -> Result<(), Error>;
    pub fn render(&mut self) -> Result<(), Error>;
    pub fn handle_event(&mut self, event: Event) -> Result<(), Error>;
}
```

### 3.2 File Watching

```rust
// Uses notify crate for cross-platform file watching
pub struct FileWatcher {
    rx: Receiver<DebouncedEvent>,
    watched: HashMap<PathBuf, WatchId>,
}
```

### 3.3 Incremental Recompilation

- Track dependency graph via FileLoader
- Only recompile changed files and dependents
- Preserve state across reloads when possible

---

## Phase 4: Scarab Integration (Target: v0.39.0)

### 4.1 Shared Memory Protocol

```rust
// ScarabRenderer writes to SharedState
pub struct ScarabRenderer {
    shm: SharedMemory,
    sequence: AtomicU64,
}

impl Renderer for ScarabRenderer {
    fn draw(&mut self, buffer: &Buffer) -> Result<(), RenderError> {
        // Convert Buffer to SharedState grid format
        // Increment sequence number atomically
    }
}
```

### 4.2 Plugin API

```rust
// scarab-plugin-api integration
pub struct TuiPlugin {
    engine: DashboardEngine,
}

impl ScarabPlugin for TuiPlugin {
    fn on_init(&mut self, ctx: &PluginContext) -> Result<(), Error>;
    fn on_render(&mut self, ctx: &RenderContext) -> Result<(), Error>;
    fn on_input(&mut self, event: InputEvent) -> Result<(), Error>;
}
```

### 4.3 Capability Permissions

```toml
# plugin.toml
[capabilities]
shared_memory = true
file_watch = true
async_runtime = true
```

---

## Phase 5: Ecosystem Integration (Target: v0.40.0)

### 5.1 Sigilforge TUI

- OAuth token management dashboard
- Credential status indicators
- Login flow UI

### 5.2 Scryforge Migration

- Extract fusabi-tui-core and fusabi-tui-widgets
- Replace with fusabi-tui-runtime dependency
- Maintain API compatibility

### 5.3 Phage Context TUI

- Context visualization
- Memory usage monitoring
- Topic-based event display

### 5.4 Tolaria Dashboard

- DGX K8s cluster monitoring
- Shadow World sync visualization
- BRP introspection UI

### 5.5 Hibana Observability

- GPU metrics dashboard
- Lock-free metrics display
- Event routing visualization

---

## Future Enhancements

### Language Features

| Feature | Priority | Impact | Effort |
|---------|----------|--------|--------|
| Mutable references (`ref`) | High | Widget state management | Medium |
| Type providers | Medium | Typed widget schemas | High |
| Pattern matching guards | Low | Complex conditionals | Low |
| Module signatures | Low | API contracts | Medium |

### FPM Package Manager

```bash
# Future CLI
fpm install fusabi-tui-widgets
fpm publish my-dashboard
```

### Testing Infrastructure

- Integration with ratatui-testlib
- Snapshot testing for TUI output
- Event simulation for input testing

---

## Implementation Order

```
Phase 1 (Foundation)
├── 1.1 fusabi-tui-core crate
├── 1.2 Renderer abstraction
└── 1.3 Fusabi bindings

Phase 2 (Widgets)
├── 2.1 Widget traits
├── 2.2 Core widgets
└── 2.3 Fusabi widget system

Phase 3 (Hot Reload)
├── 3.1 DashboardEngine
├── 3.2 File watching
└── 3.3 Incremental recompilation

Phase 4 (Scarab)
├── 4.1 Shared memory protocol
├── 4.2 Plugin API
└── 4.3 Capability permissions

Phase 5 (Ecosystem)
├── 5.1 Sigilforge TUI
├── 5.2 Scryforge migration
├── 5.3 Phage context TUI
├── 5.4 Tolaria dashboard
└── 5.5 Hibana observability
```

---

## Success Criteria

1. **fusabi-tui-runtime** available as Rust crate with:
   - Dual-mode rendering (standalone + Scarab plugin)
   - Hot-reloadable `.fsx` dashboards
   - Core widget library

2. **Ecosystem adoption**:
   - Scarab using for terminal UI
   - At least 2 other projects migrated

3. **Performance**:
   - <16ms render latency for 60fps
   - <100ms hot-reload time
   - Zero-copy shared memory for Scarab mode

4. **Developer experience**:
   - Write TUI in `.fsx` files
   - See changes instantly on save
   - Type-safe widget composition

---

## Recommended Next Action

**Start with Phase 1.1**: Create the `fusabi-tui-core` crate by:

1. Fork/copy relevant code from ratatui-core (MIT licensed)
2. Adapt for Fusabi's needs (remove unnecessary dependencies)
3. Create Fusabi bindings module
4. Write comprehensive tests
5. Document the API

This establishes the foundation for all subsequent work.

---

## References

- [fusabi-tui-runtime Work Log](./WORK_LOG.md)
- [Ecosystem Overview](./ECOSYSTEM_OVERVIEW.md)
- [Architecture Overview](./architecture/OVERVIEW.md)
- [Ratatui Analysis](./architecture/RATATUI_ANALYSIS.md)
- [RFC-003: Multi-file Modules](../../fusabi/docs/design/RFC-003-MULTIFILE-MODULES.md)
- [RFC-004: Async Tokio](../../fusabi/docs/design/RFC-004-ASYNC-TOKIO.md)
