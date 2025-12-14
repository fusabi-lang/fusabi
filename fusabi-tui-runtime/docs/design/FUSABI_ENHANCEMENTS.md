# Fusabi Language Enhancements for TUI Runtime

## Executive Summary

This document outlines Fusabi language features needed to fully support the `fusabi-tui-runtime` framework. Based on analysis of the current Fusabi implementation and patterns from Hibana, Tolaria, Scryforge, and Phage.

## Current State Analysis

### Available Features (Production Ready)

| Feature | Status | Notes |
|---------|--------|-------|
| Core Language | Production | Let bindings, lambdas, pattern matching, records, unions |
| Type Inference | Production | Hindley-Milner Algorithm W |
| Hot Reload | Production | PluginWatcher with debounce, backoff |
| Host Bindings | Production | `register_fn1/2/3`, trait-based providers |
| Bytecode VM | Production | `.fzb` compilation, 1-2M instructions/sec |
| Terminal Control | Production | `TerminalControlProvider` trait |

### Partially Implemented (Near-Term)

| Feature | Status | Blocker |
|---------|--------|---------|
| Async/Await | RFC-002 Design | Needs Tokio integration for real I/O |
| Type Providers | Architecture Done | JSON Schema provider implementation |
| Package Management | Design Complete | Registry infrastructure |

### Not Yet Implemented (Medium-Term)

| Feature | Priority | Impact on TUI |
|---------|----------|---------------|
| Mutable References (`ref`) | High | Widget state management |
| Generics | Medium | Polymorphic widgets |
| Async I/O | High | Non-blocking event loops |
| Module Exports | Low | API surface control |

---

## Recommended Enhancements

### 1. Async Computation Expressions (Priority: Critical)

**Current State**: RFC-002 exists with Free Monad implementation.

**What Works**:
```fsharp
let fetchData url = async {
    do! sleep 100
    return "Content"
}

let res = Async.RunSynchronously main
```

**What's Missing**:
- Real async I/O (currently uses shell commands for sleep)
- Tokio integration for production use
- `Async.Parallel` and `Async.Sequential` combinators

**Recommended Enhancement**:

```rust
// In fusabi-vm/src/stdlib/async_ops.rs
pub fn register_async_io(vm: &mut Vm, runtime: &tokio::runtime::Handle) {
    // Non-blocking sleep via Tokio
    vm.register_async("Async.sleep", move |ms| {
        runtime.spawn(async move {
            tokio::time::sleep(Duration::from_millis(ms)).await;
        })
    });

    // HTTP fetch via reqwest
    vm.register_async("Async.httpGet", move |url| {
        runtime.spawn(async move {
            reqwest::get(&url).await?.text().await
        })
    });
}
```

**TUI Impact**: Essential for non-blocking event loops.

---

### 2. Type Providers for Widget Schemas (Priority: High)

**Current State**: `fusabi-type-providers` crate exists with `TypeProvider` trait.

**Recommended**: JSON Schema Type Provider for Widget Definitions

```fsharp
// Define widget schema once
type Widgets = JsonProvider<"./widgets.schema.json">

// Get typed widgets at compile time
let button: Widgets.Button = {
    label = "Click Me"
    onClick = fun () -> notify "Clicked!"
    style = Widgets.ButtonStyle.Primary
}

let list: Widgets.List = {
    items = ["Item 1"; "Item 2"]
    selected = 0
    onSelect = fun idx -> state.selectedIndex <- idx
}
```

**Schema Example** (`widgets.schema.json`):
```json
{
  "definitions": {
    "Button": {
      "type": "object",
      "properties": {
        "label": { "type": "string" },
        "style": { "enum": ["Primary", "Secondary", "Danger"] },
        "onClick": { "type": "function" }
      },
      "required": ["label"]
    },
    "List": {
      "type": "object",
      "properties": {
        "items": { "type": "array", "items": { "type": "string" } },
        "selected": { "type": "integer" },
        "onSelect": { "type": "function" }
      }
    }
  }
}
```

**Benefits**:
- Compile-time validation of widget props
- Auto-completion in LSP
- Self-documenting widget API
- Schema changes auto-propagate

---

### 3. Mutable References (`ref`) (Priority: High)

**Current State**: Not implemented (immutable-first design).

**Why Needed**: Widget state management requires mutation.

**Proposed Syntax**:
```fsharp
// Create mutable reference
let counter = ref 0

// Read value
let current = !counter

// Write value
counter := !counter + 1

// Modify in place
incr counter
```

**Implementation**:
```rust
// In fusabi-vm/src/value.rs
pub enum Value {
    // ... existing variants
    Ref(Rc<RefCell<Value>>),
}

// In stdlib
fn ref_create(value: Value) -> Value {
    Value::Ref(Rc::new(RefCell::new(value)))
}

fn ref_deref(r: Value) -> Value {
    match r {
        Value::Ref(rc) => rc.borrow().clone(),
        _ => panic!("Expected ref"),
    }
}

fn ref_set(r: Value, v: Value) {
    match r {
        Value::Ref(rc) => *rc.borrow_mut() = v,
        _ => panic!("Expected ref"),
    }
}
```

**TUI Impact**: Essential for widget state (selection, scroll position, input buffers).

---

### 4. Widget DSL via Computation Expressions (Priority: Medium)

**Leverage existing CE infrastructure for declarative UI**:

```fsharp
// Widget computation expression builder
let view state = ui {
    let! header = Block.new "Dashboard"
    let! sidebar = ui {
        let! list = List.new state.items
        list.selected <- state.selectedIndex
        return list
    }
    let! content = Paragraph.new state.content

    return Layout.horizontal [
        sidebar, Constraint.Percentage 20
        content, Constraint.Fill 1
    ]
}
```

**Builder Implementation**:
```fsharp
type UiBuilder() =
    member _.Bind(widget, f) =
        let rendered = widget.render()
        f rendered
    member _.Return(layout) = layout
    member _.Zero() = Layout.empty

let ui = UiBuilder()
```

---

### 5. Enhanced Host Bindings for TUI (Priority: High)

**Extend `TerminalControlProvider` and `TerminalInfoProvider`**:

```rust
pub trait WidgetProvider: Send + Sync {
    // Core widget rendering
    fn render_block(&self, title: &str, area: Rect) -> Buffer;
    fn render_paragraph(&self, text: &str, style: Style, area: Rect) -> Buffer;
    fn render_list(&self, items: &[String], selected: Option<usize>, area: Rect) -> Buffer;
    fn render_table(&self, rows: &[Vec<String>], headers: &[String], area: Rect) -> Buffer;
    fn render_gauge(&self, ratio: f64, label: &str, area: Rect) -> Buffer;

    // Layout
    fn compute_layout(&self, constraints: &[Constraint], area: Rect) -> Vec<Rect>;

    // Events
    fn subscribe_event(&self, event_type: &str, callback_id: u32);
    fn unsubscribe_event(&self, callback_id: u32);
}

pub trait StateProvider: Send + Sync {
    // State management (for ref cells)
    fn create_state(&self, initial: Value) -> StateId;
    fn get_state(&self, id: StateId) -> Value;
    fn set_state(&self, id: StateId, value: Value);
    fn subscribe_state(&self, id: StateId, callback_id: u32);
}
```

---

### 6. Package Management DX (Priority: Medium)

**Leverage existing FPM design**:

**Manifest for TUI Plugins** (`fusabi.toml`):
```toml
[package]
name = "my-dashboard"
version = "0.1.0"
type = "tui-plugin"

[dependencies]
fusabi-tui-widgets = "0.1.0"
fusabi-tui-core = "0.1.0"

[tui]
entry = "dashboard.fsx"
hot-reload = true
renderers = ["crossterm", "scarab"]

[capabilities]
terminal-control = true
terminal-info = true
network = false
filesystem = false
```

**CLI Commands**:
```bash
# Create new TUI plugin
fpm new --template tui-plugin my-dashboard

# Install dependencies
fpm install

# Run in development mode (hot-reload)
fpm dev

# Build for production
fpm build --release

# Publish to registry
fpm publish
```

---

### 7. Testing Infrastructure (Priority: Medium)

**Leverage ratatui-testlib patterns**:

```fsharp
// Test framework for Fusabi TUI scripts
open FusabiTui.Testing

let tests = test {
    test "renders list correctly" {
        let harness = TestHarness.new (80, 24)
        let app = App.load "dashboard.fsx"

        harness.render app

        assert (harness.contains "Item 1")
        assert (harness.cell_at(0, 0).fg = Color.White)
    }

    test "handles key input" {
        let harness = TestHarness.new (80, 24)
        let app = App.load "dashboard.fsx"

        harness.send_key "j"  // Move down
        harness.render app

        assert (app.state.selectedIndex = 1)
    }
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-4)

1. **Mutable References** - Enable widget state
2. **Async Tokio Integration** - Real async I/O
3. **WidgetProvider Trait** - Core widget bindings

### Phase 2: Type Safety (Weeks 5-8)

4. **JSON Schema Type Provider** - Widget schema validation
5. **Widget DSL Builder** - Computation expression for UI
6. **Enhanced Error Messages** - Better debugging

### Phase 3: Ecosystem (Weeks 9-12)

7. **Package Management** - FPM registry
8. **Testing Framework** - TUI testing DSL
9. **LSP Integration** - Auto-complete for widgets

---

## Compatibility Matrix

| Feature | Standalone Mode | Scarab Plugin | Hot Reload |
|---------|-----------------|---------------|------------|
| Async CE | Yes | Yes | Yes |
| Type Providers | Yes | Yes | No (compile-time) |
| Mutable Refs | Yes | Yes | Yes (state preserved) |
| Widget DSL | Yes | Yes | Yes |
| Host Bindings | CrosstermProvider | ScarabProvider | Varies |

---

## Summary

The Fusabi language has a solid foundation. The critical enhancements for TUI support are:

1. **Mutable references** - For widget state
2. **Async Tokio integration** - For non-blocking I/O
3. **Type providers** - For typed widget schemas
4. **Widget host bindings** - For rendering primitives

These build on existing infrastructure (computation expressions, host bindings, hot reload) rather than requiring fundamental language changes.
