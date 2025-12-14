# Fusabi TUI Ecosystem Overview

## The Vision

A unified framework where Fusabi applications can:
1. Run **standalone** via terminal (crossterm)
2. Run as **Scarab plugins** via shared memory
3. **Hot-reload** during development
4. Share **widgets and themes** via package registry

## Ecosystem Map

```
                    ┌─────────────────────────────────────────┐
                    │           Package Registry              │
                    │        packages.fusabi.dev              │
                    │                                         │
                    │  Widgets │ Themes │ Apps │ Plugins      │
                    └─────────────────┬───────────────────────┘
                                      │
                    ┌─────────────────┼───────────────────────┐
                    │                 │                       │
                    ▼                 ▼                       ▼
          ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
          │   Scryforge     │ │     Phage       │ │   Sigilforge    │
          │   (Info Agg)    │ │ (Context Comp)  │ │ (Credentials)   │
          └────────┬────────┘ └────────┬────────┘ └────────┬────────┘
                   │                   │                   │
                   └───────────────────┼───────────────────┘
                                       │
                                       ▼
          ┌────────────────────────────────────────────────────────┐
          │                 fusabi-tui-runtime                     │
          │                                                        │
          │  ┌──────────────────────────────────────────────────┐  │
          │  │ Core Types (from ratatui-core, ~2K LOC)          │  │
          │  │ Cell, Buffer, Rect, Constraint, Layout, Style    │  │
          │  └──────────────────────────────────────────────────┘  │
          │                                                        │
          │  ┌──────────────────────────────────────────────────┐  │
          │  │ Widget System                                    │  │
          │  │ RustWidget (perf) │ FusabiWidget (hot-reload)    │  │
          │  └──────────────────────────────────────────────────┘  │
          │                                                        │
          │  ┌──────────────────────────────────────────────────┐  │
          │  │ Renderer Abstraction                             │  │
          │  │ CrosstermRenderer │ ScarabRenderer │ TestRenderer│  │
          │  └──────────────────────────────────────────────────┘  │
          │                                                        │
          │  ┌──────────────────────────────────────────────────┐  │
          │  │ Dashboard Engine                                 │  │
          │  │ Hot-reload │ State preservation │ Dev overlay    │  │
          │  └──────────────────────────────────────────────────┘  │
          │                                                        │
          └────────────────────────┬───────────────────────────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
                    ▼              ▼              ▼
          ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐
          │ Standalone  │ │   Scarab    │ │  Testing        │
          │ Terminal    │ │   Plugin    │ │  (ratatui-      │
          │ (crossterm) │ │ (SharedMem) │ │   testlib)      │
          └─────────────┘ └─────────────┘ └─────────────────┘
```

## Project Relationships

### Core Infrastructure

| Project | Role | Status |
|---------|------|--------|
| **fusabi** | F# language runtime | Production |
| **fusabi-tui-runtime** | TUI framework | New (this project) |
| **scarab** | Terminal emulator | Active development |
| **ratatui-testlib** | Testing infrastructure | Production |

### Applications (Migrate to fusabi-tui-runtime)

| Project | Current State | Migration Path |
|---------|---------------|----------------|
| **Scryforge** | 50K LOC, has fusabi-tui-* crates | Extract/merge crates |
| **Phage** | 25K LOC, Fusabi config | Port TUI to runtime |
| **Sigilforge** | 5K LOC, CLI only | Write TUI in Fusabi |
| **Hibana** | hibana-top dashboard | Template for dashboard system |
| **Tolaria** | Complex multi-view TUI | Template for BRP pattern |

### Support Projects

| Project | Role |
|---------|------|
| **ratatui-testlib** | TUI testing (PTY, graphics, IPC) |
| **fusabi-host** | Fusabi embedding patterns |
| **fusabi-plugin-runtime** | Hot-reload infrastructure |

---

## Key Patterns Discovered

### 1. Shadow World Sync (from Tolaria)

Client maintains synchronized copy of server state:

```fsharp
// Fusabi script receives sync updates
let onSync newState =
    state.services <- newState.services
    state.metrics <- newState.metrics
    scheduleRender()
```

**Use case**: Remote TUI clients, Scarab daemon-client communication

### 2. Topic-Based Event Routing (from Hibana)

Decouple data producers from consumers:

```fsharp
// Subscribe to specific data streams
Events.subscribe "metrics/*" (fun evt ->
    updateMetricsPanel evt.data
)

Events.subscribe "logs/error" (fun evt ->
    showErrorNotification evt.data
)
```

**Use case**: Multi-source dashboards, plugin communication

### 3. Lock-Free Metrics (from Hibana)

Use atomics for non-blocking reads:

```rust
pub struct Metrics {
    events_in: AtomicU64,
    events_out: AtomicU64,
}
```

**Use case**: TUI reads metrics without blocking hot path

### 4. BRP Introspection (from Tolaria)

Expose ECS state via Bevy Remote Protocol:

```rust
#[derive(Reflect)]
pub struct DashboardState {
    pub active_tab: usize,
    pub metrics: Vec<f64>,
}
```

**Use case**: Remote debugging, state-based testing

### 5. Capability-Based Security (from Scarab)

Plugins declare required permissions:

```toml
[capabilities]
terminal-control = true
filesystem = false
network = false
```

**Use case**: Safe plugin ecosystem

---

## Fusabi Language Gaps

### Critical (Block TUI Development)

| Gap | Impact | Solution |
|-----|--------|----------|
| Mutable refs (`ref`) | Widget state | Implement `ref`/`!`/`:=` |
| Async Tokio | Non-blocking I/O | Integrate with tokio runtime |

### High Priority (Improve DX)

| Gap | Impact | Solution |
|-----|--------|----------|
| Type providers | Widget schemas | Implement JSON Schema provider |
| Widget DSL | Declarative UI | Computation expression builder |
| LSP completions | Dev experience | Integrate with type providers |

### Medium Priority (Ecosystem)

| Gap | Impact | Solution |
|-----|--------|----------|
| Package registry | Distribution | Deploy packages.fusabi.dev |
| Workspace support | Monorepos | Implement in fpm |

---

## Testing Strategy

### Unit Tests (Fusabi Scripts)

```fsharp
test "renders metrics correctly" {
    let state = { metrics = [1.0; 2.0; 3.0] }
    let buffer = render state

    assert (buffer.contains "1.00")
    assert (buffer.contains "2.00")
}
```

### Integration Tests (ratatui-testlib)

```rust
#[test]
fn test_dashboard_navigation() {
    let harness = TuiTestHarness::new(80, 24)?;
    harness.spawn(CommandBuilder::new("my-dashboard"))?;

    harness.wait_for_text("Overview")?;
    harness.send_key("2")?;  // Switch to tab 2
    harness.wait_for_text("Metrics")?;

    assert!(harness.screen_contents().contains("Metrics"));
}
```

### Scarab Plugin Tests

```rust
#[test]
fn test_plugin_in_scarab() {
    let harness = ScarabTestHarness::connect()?;

    harness.load_plugin("my-dashboard")?;
    harness.wait_for_text("Dashboard loaded")?;

    let grid = harness.grid_contents()?;
    assert!(grid.contains("GPU"));
}
```

---

## Migration Timeline

### Phase 1: Foundation (Weeks 1-4)

1. Create `fusabi-tui-runtime` crate
2. Port core types from ratatui-core
3. Implement CrosstermRenderer
4. Implement ScarabRenderer
5. Basic widget system

### Phase 2: Integration (Weeks 5-8)

6. Hot-reload infrastructure
7. Dashboard engine
8. Sigilforge TUI (clean slate)
9. Hibana-top port (validation)

### Phase 3: Migration (Weeks 9-12)

10. Scryforge migration
11. Phage migration
12. Package registry setup
13. Documentation and examples

### Phase 4: Polish (Weeks 13-16)

14. Type providers for widgets
15. LSP integration
16. Advanced testing patterns
17. Performance optimization

---

## Success Metrics

| Metric | Target |
|--------|--------|
| LOC for fusabi-tui-runtime | < 10K |
| Time to create new TUI app | < 5 minutes |
| Hot-reload latency | < 500ms |
| Frame render time | < 16ms (60 FPS) |
| Scarab plugin overhead | < 1ms per frame |
| Test coverage | > 80% |

---

## Open Questions

1. **Should we support Bevy ECS in fusabi-tui-runtime?**
   - Pro: Tolaria pattern is proven
   - Con: Adds complexity, not all apps need ECS

2. **How to handle Fusabi's `Rc<RefCell>` thread-safety?**
   - Option A: Widgets render in single thread
   - Option B: Message-passing between threads
   - Option C: Wait for Fusabi to support `Arc<Mutex>`

3. **Web target (WASM)?**
   - Tolaria has Ratzilla web UI
   - Would need WebRenderer implementation
   - Lower priority than standalone + Scarab

4. **Graphics protocols (Sixel, Kitty)?**
   - ratatui-testlib has full support
   - fusabi-tui-runtime should support via widgets
   - Image widgets would use host bindings

---

## Related Documents

- [OVERVIEW.md](./architecture/OVERVIEW.md) - Architecture overview
- [RATATUI_ANALYSIS.md](./architecture/RATATUI_ANALYSIS.md) - Ratatui deep dive
- [FUSABI_ENHANCEMENTS.md](./design/FUSABI_ENHANCEMENTS.md) - Language features
- [HOT_RELOAD_DASHBOARD.md](./design/HOT_RELOAD_DASHBOARD.md) - Dashboard system
- [PACKAGE_MANAGEMENT.md](./design/PACKAGE_MANAGEMENT.md) - DX design
