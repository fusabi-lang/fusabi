# Ratatui Analysis for fusabi-tui-runtime

## Executive Summary

This document analyzes ratatui v0.30.0 to determine what components to borrow for `fusabi-tui-runtime`. The recommendation is to **borrow core types, discard terminal layer**.

## Ratatui Architecture Overview

### Crate Organization (v0.30.0+)

```
ratatui (workspace)
├── ratatui-core (21,751 LOC)     # Core types - no_std compatible
├── ratatui-widgets (27,539 LOC)  # 14 built-in widgets
├── ratatui-crossterm (879 LOC)   # Crossterm backend
├── ratatui-termion              # Unix backend
├── ratatui-termwiz              # Advanced backend
├── ratatui-macros               # Declarative macros
└── ratatui (2,221 LOC)          # Re-export crate
```

**Total**: ~80K LOC

### What We BORROW (from ratatui-core)

| Component | LOC | Purpose |
|-----------|-----|---------|
| `Cell` | 200 | Character + style + modifiers |
| `Buffer` | 500 | Grid of cells |
| `Rect` | 100 | Area rectangle |
| `Constraint` | 150 | Layout constraint enum |
| `Layout` | 800 | Cassowary constraint solver |
| `Style` | 300 | Color + modifiers |

**Total borrowed: ~2K LOC**

### What We DISCARD

| Component | Reason |
|-----------|--------|
| `Terminal` | Synchronous dual-buffer diffing - blocks render thread |
| `Backend` trait | Too tied to terminal escape sequences |
| `Frame` | Designed for terminal workflows |
| `CrosstermBackend` | We need shared memory, not escape sequences |

## Key Types to Port

### Cell

```rust
pub struct Cell {
    pub symbol: CompactString,  // Unicode grapheme
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
    pub skip: bool,  // For differential rendering
}
```

**Changes for fusabi-tui-runtime**:
- Remove `skip` (we use sequence numbers instead)
- Add semantic tags for hyperlinks, selections

### Buffer

```rust
pub struct Buffer {
    pub area: Rect,
    pub content: Vec<Cell>,  // Flat array: [y * width + x]
}

impl Buffer {
    pub fn set_string(&mut self, x: u16, y: u16, string: &str, style: Style);
    pub fn set_style(&mut self, area: Rect, style: Style);
    pub fn merge(&mut self, other: &Buffer);
}
```

**Changes for fusabi-tui-runtime**:
- Add `#[repr(C)]` for shared memory compatibility
- Add `bytemuck::Pod` derive for zero-copy

### Constraint

```rust
pub enum Constraint {
    Min(u16),          // Minimum size
    Max(u16),          // Maximum size
    Length(u16),       // Exact size
    Percentage(u16),   // % of available
    Ratio(u32, u32),   // Proportional ratio
    Fill(u16),         // Fill remaining
}
```

**Keep unchanged** - This is well-designed.

### Layout

```rust
pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
    margin: Margin,
    spacing: Spacing,
    flex: Flex,
}

impl Layout {
    pub fn split(self, area: Rect) -> Rc<[Rect]>;
}
```

**Changes for fusabi-tui-runtime**:
- Use `kasuari` crate directly (Cassowary solver)
- Cache layout results with LRU

## Widget Trait Comparison

### Ratatui Widget

```rust
pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}

pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}
```

### fusabi-tui-runtime Widgets

```rust
// Rust widgets (performance-critical)
pub trait RustWidget {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

// Fusabi widgets (hot-reloadable)
pub trait FusabiWidget {
    fn render(&self, area: Rect, vm: &FusabiVm) -> Buffer;
}
```

**Key difference**: Fusabi widgets return owned Buffer, allowing VM isolation.

## Backend Abstraction Comparison

### Ratatui Backend

```rust
pub trait Backend {
    type Error;
    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn hide_cursor(&mut self) -> Result<(), Self::Error>;
    fn show_cursor(&mut self) -> Result<(), Self::Error>;
    fn get_cursor_position(&mut self) -> Result<Position, Self::Error>;
    fn set_cursor_position(&mut self, position: Position) -> Result<(), Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn size(&self) -> Result<Size, Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}
```

### fusabi-tui-runtime Renderer

```rust
pub trait Renderer: Send + Sync {
    type Error: std::error::Error;

    /// Render entire buffer (not iterator-based)
    fn render(&mut self, buffer: &Buffer) -> Result<(), Self::Error>;

    /// Poll for events (non-blocking)
    fn poll_event(&mut self, timeout: Duration) -> Option<Event>;

    /// Get current size
    fn size(&self) -> Size;

    /// Cleanup
    fn cleanup(&mut self) -> Result<(), Self::Error>;
}
```

**Key differences**:
1. `Send + Sync` required (for async compatibility)
2. `render()` takes full buffer, not iterator
3. `poll_event()` included (ratatui doesn't handle events)
4. No cursor management (handled by client)

## Performance Considerations

### Ratatui's Dual-Buffer Diffing

```
Frame N:   [Terminal draws to Buffer A]
           [Compare A vs B, send only changes]
Frame N+1: [Terminal draws to Buffer B]
           [Compare B vs A, send only changes]
```

**Problem**: Comparison happens on render thread (blocking).

### fusabi-tui-runtime's Sequence Number Approach

```
Daemon:    [Write Buffer to SharedMemory]
           [Increment sequence_number atomically]

Client:    [Poll sequence_number]
           [If changed, read Buffer]
           [Render entire Buffer via GPU]
```

**Advantage**: Non-blocking, zero-copy, GPU-friendly.

## Migration Path

### Step 1: Port Core Types

```bash
# Copy from ratatui-core (adapt for no_std + repr(C))
src/types/
├── cell.rs      # From ratatui-core/buffer/cell.rs
├── buffer.rs    # From ratatui-core/buffer/buffer.rs
├── rect.rs      # From ratatui-core/layout/rect.rs
├── style.rs     # From ratatui-core/style/
├── constraint.rs # From ratatui-core/layout/constraint.rs
└── layout.rs    # From ratatui-core/layout/layout.rs
```

### Step 2: Implement Renderers

```bash
src/renderers/
├── mod.rs         # Renderer trait
├── crossterm.rs   # Standalone mode
├── scarab.rs      # Plugin mode (SharedState)
└── test.rs        # Unit testing
```

### Step 3: Port Essential Widgets

```bash
src/widgets/
├── block.rs       # From ratatui-widgets
├── paragraph.rs   # From ratatui-widgets
├── list.rs        # From ratatui-widgets
├── table.rs       # From ratatui-widgets
└── gauge.rs       # From ratatui-widgets
```

## Compatibility with Ratatui Ecosystem

### Ratatui Examples

The ~30 ratatui examples can be ported by:
1. Replacing `Terminal` with our `App` abstraction
2. Replacing `Backend` with `Renderer`
3. Keeping widget code unchanged (same render pattern)

### Third-Party Widgets

Ratatui-compatible widgets can be wrapped:

```rust
impl<W: ratatui::Widget> RustWidget for RatatuiWrapper<W> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.inner.render(area, buf);
    }
}
```

## Summary

| Aspect | Ratatui | fusabi-tui-runtime |
|--------|---------|-------------------|
| Core types | Keep | Port 2K LOC |
| Widgets | Keep | Port or wrap |
| Terminal | Discard | Replace with Renderer |
| Backend | Discard | Replace with Renderer |
| Event handling | None | Built-in |
| Async | None | Native |
| Plugin mode | None | Native |
