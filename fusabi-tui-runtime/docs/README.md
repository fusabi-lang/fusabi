# fusabi-tui-runtime Documentation

A unified TUI framework enabling Fusabi applications to run standalone OR as Scarab plugins.

## Documentation Structure

```
docs/
├── README.md                      # This file
├── ECOSYSTEM_OVERVIEW.md          # High-level ecosystem map
├── architecture/
│   ├── OVERVIEW.md               # Architecture overview
│   └── RATATUI_ANALYSIS.md       # Ratatui analysis and borrowing strategy
├── design/
│   ├── FUSABI_ENHANCEMENTS.md    # Language features needed
│   ├── HOT_RELOAD_DASHBOARD.md   # Hot-reloadable dashboard system
│   └── PACKAGE_MANAGEMENT.md     # Package management DX
└── api/                          # (Future) API reference
```

## Quick Links

| Document | Purpose |
|----------|---------|
| [Ecosystem Overview](./ECOSYSTEM_OVERVIEW.md) | Understand the full picture |
| [Architecture](./architecture/OVERVIEW.md) | Core architecture decisions |
| [Ratatui Analysis](./architecture/RATATUI_ANALYSIS.md) | What we borrow vs. build |
| [Fusabi Enhancements](./design/FUSABI_ENHANCEMENTS.md) | Language gaps and solutions |
| [Dashboard System](./design/HOT_RELOAD_DASHBOARD.md) | Hot-reload architecture |
| [Package Management](./design/PACKAGE_MANAGEMENT.md) | Developer experience |

## Key Decisions

1. **Borrow from ratatui-core** (~2K LOC): Cell, Buffer, Rect, Layout, Constraint, Style
2. **Replace Terminal layer**: New Renderer trait supporting crossterm + Scarab shared memory
3. **Fusabi-first widgets**: Hot-reloadable UI defined in F# scripts
4. **Dual-mode runtime**: Same `.fsx` code runs standalone or as Scarab plugin

## Getting Started

```bash
# Create new TUI app
fpm new --template tui-app my-dashboard

# Run with hot-reload
cd my-dashboard
fpm dev
```

## Project Status

| Component | Status |
|-----------|--------|
| Documentation | Complete |
| Core Types | Design complete |
| Renderers | Design complete |
| Widget System | Design complete |
| Implementation | Not started |

## Related Projects

- [Fusabi](https://github.com/fusabi-lang/fusabi) - F# language for Rust
- [Scarab](../../../raibid-labs/scarab) - Terminal emulator
- [Scryforge](../../../raibid-labs/scryforge) - Info aggregator (migration target)
- [Phage](../../../raibid-labs/phage) - Context engine (migration target)
- [ratatui-testlib](../../../raibid-labs/ratatui-testlib) - TUI testing
