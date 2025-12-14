# Hot-Reloadable TUI Dashboard System

## Overview

This document designs a hot-reloadable dashboard system that runs within the `fusabi-tui-runtime` framework AND can be used alongside it as a development tool.

## Inspiration from Existing Projects

### Hibana `hibana-top`
- Ratatui-based dashboard with 3 tabs
- 500ms refresh rate (configurable)
- Lock-free metrics via AtomicU64
- Remote gRPC mode for distributed monitoring

### Tolaria `tolaria-lens`
- Shadow World sync pattern (10Hz from Foundry)
- 7 integrated views with tab switching
- BRP (Bevy Remote Protocol) for introspection
- State-based assertions for testing

### Scryforge `fusabi-tui-widgets`
- Widget abstraction layer
- Formatting utilities
- Theme system

---

## Architecture

### Three Deployment Modes

```
┌─────────────────────────────────────────────────────────────────┐
│                     Hot-Reload Dashboard                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Mode 1: Standalone Development Tool                            │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ $ fusabi-dashboard --watch ./my-app.fsx                 │   │
│  │                                                         │   │
│  │ ┌─────────────────────────────────────────────────────┐ │   │
│  │ │           Live Preview Panel                        │ │   │
│  │ │   (Renders your .fsx app in real-time)             │ │   │
│  │ └─────────────────────────────────────────────────────┘ │   │
│  │ ┌─────────────────────────────────────────────────────┐ │   │
│  │ │           Diagnostics Panel                         │ │   │
│  │ │   • Type errors  • Render time  • Memory usage     │ │   │
│  │ └─────────────────────────────────────────────────────┘ │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Mode 2: Embedded in fusabi-tui-runtime App                     │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ // In your app                                          │   │
│  │ let app = App.new()                                     │   │
│  │     .with_hot_reload(true)                              │   │
│  │     .with_dev_overlay(cfg!(debug_assertions))           │   │
│  │     .run("dashboard.fsx")                               │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  Mode 3: Scarab Plugin with Hot-Reload                          │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ Scarab Daemon                                           │   │
│  │  └─ Dashboard Plugin (.fzb)                             │   │
│  │      └─ Watches: ~/.config/scarab/dashboard.fsx         │   │
│  │      └─ Reloads on change (debounced 500ms)            │   │
│  │                                                         │   │
│  │ Scarab Client                                           │   │
│  │  └─ Renders updated dashboard immediately              │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. DashboardEngine

The central coordination component:

```rust
pub struct DashboardEngine {
    /// Fusabi VM for script execution
    vm: FusabiVm,

    /// Current dashboard state
    state: DashboardState,

    /// File watcher for hot-reload
    watcher: Option<PluginWatcher>,

    /// Renderer abstraction
    renderer: Box<dyn Renderer>,

    /// Event queue
    events: EventQueue,

    /// Metrics collector
    metrics: DashboardMetrics,
}

impl DashboardEngine {
    pub fn new() -> Self { /* ... */ }

    pub fn with_hot_reload(mut self, enabled: bool) -> Self {
        if enabled {
            self.watcher = Some(PluginWatcher::new(
                WatchConfig::new()
                    .with_debounce(Duration::from_millis(500))
                    .with_extensions(&["fsx", "toml"])
            ));
        }
        self
    }

    pub fn with_renderer<R: Renderer + 'static>(mut self, renderer: R) -> Self {
        self.renderer = Box::new(renderer);
        self
    }

    pub async fn run(&mut self, script_path: &Path) -> Result<()> {
        self.load_script(script_path)?;

        loop {
            // Check for script changes
            if let Some(ref mut watcher) = self.watcher {
                if watcher.has_changes() {
                    self.reload_script()?;
                }
            }

            // Handle events
            while let Some(event) = self.events.poll() {
                self.handle_event(event)?;
            }

            // Render frame
            let buffer = self.render_frame()?;
            self.renderer.render(&buffer)?;

            // Collect metrics
            self.metrics.record_frame();
        }
    }
}
```

### 2. DashboardState (Preserved Across Reloads)

```rust
#[derive(Default, Serialize, Deserialize)]
pub struct DashboardState {
    /// User-defined state (preserved on reload)
    pub user_state: HashMap<String, Value>,

    /// UI state (tabs, selections)
    pub ui_state: UiState,

    /// Last render timestamp
    pub last_render: Instant,

    /// Error state (for error overlay)
    pub last_error: Option<DashboardError>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UiState {
    pub active_tab: usize,
    pub scroll_positions: HashMap<String, usize>,
    pub selections: HashMap<String, Option<usize>>,
    pub input_buffers: HashMap<String, String>,
}
```

### 3. Hot-Reload Lifecycle

```
File Change Detected
        │
        ▼
┌───────────────────┐
│ Save Current State │
│ (serialize to JSON)│
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│ Compile New Script │
│ (parse .fsx)       │
└─────────┬─────────┘
          │
    ┌─────┴─────┐
    │           │
    ▼           ▼
Success      Failure
    │           │
    ▼           ▼
┌───────────┐ ┌───────────────┐
│ Load New  │ │ Show Error    │
│ Script    │ │ Keep Old      │
└─────┬─────┘ │ Script Running│
      │       └───────────────┘
      ▼
┌───────────────────┐
│ Restore State     │
│ (deserialize)     │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│ Continue Rendering │
│ (no frame drop)   │
└───────────────────┘
```

---

## Dashboard Script Format

### Basic Dashboard (.fsx)

```fsharp
// dashboard.fsx
module Dashboard

open FusabiTui

// State type
type State = {
    metrics: float list
    selectedTab: int
    lastUpdate: DateTime
}

// Initial state
let initialState = {
    metrics = []
    selectedTab = 0
    lastUpdate = DateTime.now()
}

// Event handlers
let handleKey state key =
    match key with
    | "1" -> { state with selectedTab = 0 }
    | "2" -> { state with selectedTab = 1 }
    | "3" -> { state with selectedTab = 2 }
    | "q" -> App.quit()
    | _ -> state

let handleTick state =
    let newMetrics = Metrics.collect()
    { state with
        metrics = newMetrics :: state.metrics |> List.take 100
        lastUpdate = DateTime.now() }

// Render function
let render state =
    let tabs = Tabs.new ["Overview"; "Metrics"; "Logs"]
        |> Tabs.select state.selectedTab

    let content =
        match state.selectedTab with
        | 0 -> renderOverview state
        | 1 -> renderMetrics state
        | 2 -> renderLogs state
        | _ -> Paragraph.new "Unknown tab"

    Layout.vertical [
        tabs, Constraint.Length 3
        content, Constraint.Fill 1
        StatusBar.render state, Constraint.Length 1
    ]

// Export app configuration
let app = {
    initialState = initialState
    render = render
    onKey = handleKey
    onTick = Some (handleTick, Duration.fromMs 1000)
    onResize = None
}
```

### Advanced Dashboard with Multiple Panels

```fsharp
// advanced-dashboard.fsx
module AdvancedDashboard

open FusabiTui
open FusabiTui.Widgets
open FusabiTui.Layout

type Panel =
    | Metrics
    | Logs
    | Topology
    | Config

type State = {
    panels: Map<string, Panel>
    layout: LayoutConfig
    data: DataStore
}

// Declarative layout definition
let layoutConfig = layout {
    split Horizontal [
        panel "sidebar" (Constraint.Percentage 20) {
            widget (List.new ["Metrics"; "Logs"; "Topology"; "Config"])
        }
        split Vertical [
            panel "main" (Constraint.Fill 1) {
                dynamic (fun state -> renderPanel state.activePanel state)
            }
            panel "status" (Constraint.Length 1) {
                widget (StatusBar.new)
            }
        ]
    ]
}

// Data binding
let bindings = [
    "metrics", DataSource.poll Metrics.collect (Duration.fromMs 500)
    "logs", DataSource.stream Logs.subscribe
    "topology", DataSource.once Topology.discover
]

let app = App.create()
    |> App.withLayout layoutConfig
    |> App.withBindings bindings
    |> App.withTheme Theme.dark
```

---

## Development Overlay

When running in development mode, an overlay shows:

```
┌─────────────────────────────────────────────────────────────────┐
│ [DEV] dashboard.fsx │ Last reload: 2.3s ago │ Render: 0.8ms     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                      Your Dashboard Here                        │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ [F1] Help │ [F2] State │ [F3] Metrics │ [F4] Errors │ [F5] Hide │
└─────────────────────────────────────────────────────────────────┘

[F2] State Inspector:
┌─────────────────────────────────────────────────────────────────┐
│ State Inspector                                      [x] Close  │
├─────────────────────────────────────────────────────────────────┤
│ selectedTab: 0                                                  │
│ metrics: [72.3, 71.8, 73.1, ...]                               │
│ lastUpdate: 2024-01-15T10:32:45Z                               │
│                                                                 │
│ [Edit] [Reset] [Export JSON]                                    │
└─────────────────────────────────────────────────────────────────┘

[F4] Error Panel (on compile error):
┌─────────────────────────────────────────────────────────────────┐
│ Compilation Error                                    [x] Close  │
├─────────────────────────────────────────────────────────────────┤
│ dashboard.fsx:42:15                                             │
│ Error: Type mismatch                                            │
│   Expected: int                                                 │
│   Got: string                                                   │
│                                                                 │
│   41 │ let count = state.items |> List.length                   │
│   42 │ let label = count + " items"                             │
│      │               ^^^^^^^^^^^^                               │
│   43 │ Paragraph.new label                                      │
│                                                                 │
│ [Previous script still running]                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Integration with Scarab

### As a Scarab Plugin

```rust
// In scarab-plugin-dashboard
pub struct DashboardPlugin {
    engine: DashboardEngine,
    config: DashboardConfig,
}

impl Plugin for DashboardPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "dashboard".to_string(),
            version: "0.1.0".to_string(),
            description: "Hot-reloadable TUI dashboard".to_string(),
            emoji: Some("📊".to_string()),
            ..Default::default()
        }
    }

    async fn on_load(&mut self, ctx: &mut PluginContext) -> Result<()> {
        // Load dashboard script
        let script_path = ctx.get_env("DASHBOARD_SCRIPT")
            .unwrap_or("~/.config/scarab/dashboard.fsx".to_string());

        self.engine = DashboardEngine::new()
            .with_hot_reload(true)
            .with_renderer(ScarabRenderer::new(ctx));

        self.engine.load_script(&script_path)?;
        Ok(())
    }

    async fn on_resize(&mut self, cols: u16, rows: u16, ctx: &PluginContext) -> Result<()> {
        self.engine.handle_resize(cols, rows)?;
        Ok(())
    }

    fn get_menu() -> Vec<MenuItem> {
        vec![
            MenuItem::action("Reload Dashboard", "reload"),
            MenuItem::action("Edit Script", "edit"),
            MenuItem::action("Toggle Dev Overlay", "toggle-dev"),
        ]
    }
}
```

### Dashboard Script for Scarab

```fsharp
// ~/.config/scarab/dashboard.fsx
module ScarabDashboard

open FusabiTui
open Scarab

let render state =
    let terminalInfo = TerminalInfo.get()
    let processInfo = TerminalInfo.getForegroundProcess()

    Layout.vertical [
        // Terminal info bar
        Block.new "Terminal" {
            Paragraph.new (sprintf "Size: %dx%d" terminalInfo.cols terminalInfo.rows)
        }, Constraint.Length 3

        // Current process
        Block.new "Process" {
            match processInfo with
            | Some p -> Paragraph.new (sprintf "%s (PID: %d)" p.name p.pid)
            | None -> Paragraph.new "No foreground process"
        }, Constraint.Length 3

        // Custom content
        Block.new "Notes" {
            Paragraph.new state.notes
        }, Constraint.Fill 1
    ]

let app = {
    initialState = { notes = "" }
    render = render
    onKey = fun state key ->
        match key with
        | _ -> state  // Handle input
    onTick = None
}
```

---

## Metrics Collection

### Built-in Metrics

```rust
pub struct DashboardMetrics {
    /// Frame render times
    render_times: RingBuffer<Duration>,

    /// Script compilation times
    compile_times: RingBuffer<Duration>,

    /// Memory usage
    memory_usage: AtomicU64,

    /// Reload count
    reload_count: AtomicU64,

    /// Error count
    error_count: AtomicU64,
}

impl DashboardMetrics {
    pub fn record_frame(&self, duration: Duration) {
        self.render_times.push(duration);
    }

    pub fn summary(&self) -> MetricsSummary {
        MetricsSummary {
            avg_render_ms: self.render_times.average().as_millis(),
            p99_render_ms: self.render_times.percentile(99).as_millis(),
            memory_mb: self.memory_usage.load(Ordering::Relaxed) / 1_000_000,
            reloads: self.reload_count.load(Ordering::Relaxed),
            errors: self.error_count.load(Ordering::Relaxed),
        }
    }
}
```

### Exposed to Fusabi Scripts

```fsharp
// Access metrics in dashboard script
let metrics = Dashboard.metrics()

let renderMetricsPanel () =
    Block.new "Performance" {
        Table.new [
            ["Avg Render", sprintf "%.2f ms" metrics.avgRenderMs]
            ["P99 Render", sprintf "%.2f ms" metrics.p99RenderMs]
            ["Memory", sprintf "%.1f MB" metrics.memoryMb]
            ["Reloads", string metrics.reloads]
            ["Errors", string metrics.errors]
        ]
    }
```

---

## CLI Tool

```bash
# Create new dashboard project
fusabi-dashboard new my-dashboard

# Run with hot-reload
fusabi-dashboard dev my-dashboard/dashboard.fsx

# Run in production mode (no hot-reload)
fusabi-dashboard run my-dashboard/dashboard.fsx

# Build to bytecode
fusabi-dashboard build my-dashboard/dashboard.fsx -o dashboard.fzb

# Install as Scarab plugin
fusabi-dashboard install --scarab my-dashboard/
```

---

## Configuration

### Dashboard Config (`dashboard.toml`)

```toml
[dashboard]
entry = "dashboard.fsx"
hot-reload = true
refresh-rate-ms = 100

[dev]
overlay = true
state-inspector = true
error-panel = true

[renderer]
# "crossterm" for standalone, "scarab" for plugin
type = "auto"

[watch]
debounce-ms = 500
extensions = ["fsx", "toml"]
exclude = ["*.tmp", ".git/*"]

[state]
# Preserve state across reloads
preserve = true
# State file for persistence across restarts
persist-file = ".dashboard-state.json"

[theme]
name = "dark"
# Or custom theme
# [theme.custom]
# primary = "#89b4fa"
# secondary = "#a6e3a1"
```

---

## Summary

The hot-reloadable dashboard system provides:

1. **Three deployment modes**: Standalone tool, embedded in apps, Scarab plugin
2. **State preservation**: User state survives reloads
3. **Development overlay**: Real-time debugging
4. **Graceful error handling**: Keep running on compile errors
5. **Metrics collection**: Performance monitoring built-in
6. **CLI tooling**: Easy project creation and deployment

This builds on existing infrastructure:
- `fusabi-plugin-runtime` for hot-reload
- `fusabi-tui-runtime` for rendering
- Patterns from Hibana and Tolaria dashboards
