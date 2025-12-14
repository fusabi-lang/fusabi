# Package Management DX for fusabi-tui-runtime

## Overview

This document designs the developer experience for creating, publishing, and consuming TUI plugins and dashboards built with `fusabi-tui-runtime`.

## Goals

1. **Zero-friction onboarding** - Create a working TUI app in < 5 minutes
2. **Consistent structure** - Standard project layouts
3. **Hot-reload by default** - Development mode watches files
4. **Multi-target builds** - Standalone AND Scarab plugin from same source
5. **Registry integration** - Share and discover plugins

---

## Project Templates

### Template: `tui-app`

Full standalone TUI application:

```bash
$ fpm new --template tui-app my-dashboard
Creating my-dashboard...

my-dashboard/
├── fusabi.toml           # Package manifest
├── src/
│   ├── main.fsx          # Entry point
│   ├── views/
│   │   ├── mod.fsx       # View module
│   │   ├── overview.fsx  # Overview view
│   │   └── metrics.fsx   # Metrics view
│   └── components/
│       ├── mod.fsx       # Component module
│       └── status_bar.fsx
├── assets/
│   └── theme.toml        # Theme configuration
├── tests/
│   └── views_test.fsx    # View tests
└── README.md

Done! Run `cd my-dashboard && fpm dev` to start.
```

### Template: `tui-plugin`

Scarab plugin with TUI:

```bash
$ fpm new --template tui-plugin my-plugin
Creating my-plugin...

my-plugin/
├── fusabi.toml
├── plugin.toml           # Scarab plugin manifest
├── src/
│   ├── lib.fsx           # Plugin entry
│   └── ui/
│       └── overlay.fsx   # TUI overlay
└── README.md
```

### Template: `tui-widget`

Reusable widget library:

```bash
$ fpm new --template tui-widget my-widgets
Creating my-widgets...

my-widgets/
├── fusabi.toml
├── src/
│   ├── lib.fsx           # Library entry
│   ├── button.fsx
│   ├── list.fsx
│   └── chart.fsx
├── examples/
│   └── demo.fsx
└── README.md
```

---

## Manifest Format

### `fusabi.toml` for TUI Apps

```toml
[package]
name = "my-dashboard"
version = "0.1.0"
authors = ["Developer <dev@example.com>"]
license = "MIT"
description = "A hot-reloadable GPU monitoring dashboard"
repository = "https://github.com/user/my-dashboard"
keywords = ["tui", "dashboard", "gpu", "monitoring"]

# Package type determines build behavior
type = "tui-app"  # or "tui-plugin", "tui-widget"

[dependencies]
fusabi-tui-core = "0.1.0"
fusabi-tui-widgets = "0.1.0"
hibana-client = { version = "0.1.0", optional = true }

[dev-dependencies]
fusabi-tui-testing = "0.1.0"

[features]
default = []
hibana = ["hibana-client"]
scarab = []

[tui]
entry = "src/main.fsx"

# Renderer configuration
[tui.renderers]
crossterm = { default = true }
scarab = { optional = true, feature = "scarab" }

# Hot-reload configuration
[tui.dev]
hot-reload = true
refresh-rate-ms = 100
overlay = true

# Build targets
[tui.build]
targets = ["standalone", "scarab-plugin"]
optimize = true

[scarab]
# Only if type = "tui-plugin" or targets includes "scarab-plugin"
plugin-name = "my-dashboard"
capabilities = ["terminal-info", "ui-overlay"]
menu-items = [
    { label = "Open Dashboard", action = "open" },
    { label = "Settings", action = "settings" }
]
```

### `plugin.toml` for Scarab Plugins

```toml
[plugin]
name = "my-dashboard"
version = "0.1.0"
description = "GPU monitoring dashboard"
emoji = "📊"
color = "#89b4fa"
catchphrase = "Keep an eye on your GPUs"

api-version = "0.1.0"
min-scarab-version = "0.2.0"

[capabilities]
terminal-info = true
ui-overlay = true
menu-registration = true
status-bar = true

[hooks]
on-load = true
on-resize = true
on-remote-command = true

[fusabi]
entry = "src/lib.fsx"
bytecode = "dist/plugin.fzb"
hot-reload = true
```

---

## CLI Commands

### Development Workflow

```bash
# Create new project
fpm new --template tui-app my-app
cd my-app

# Install dependencies
fpm install

# Start development server with hot-reload
fpm dev
# → Watching src/**/*.fsx
# → Server running at http://localhost:3000 (web preview)
# → Press 'q' to quit, 'r' to force reload

# Run tests
fpm test

# Type check without running
fpm check

# Format code
fpm fmt
```

### Build Workflow

```bash
# Build for all configured targets
fpm build
# → Building standalone target...
# → Building scarab-plugin target...
# → Output: dist/my-app, dist/my-app.fzb

# Build specific target
fpm build --target standalone
fpm build --target scarab-plugin

# Build with optimizations
fpm build --release

# Build with specific features
fpm build --features hibana,scarab
```

### Publishing

```bash
# Login to registry
fpm login

# Publish package
fpm publish
# → Publishing my-dashboard@0.1.0...
# → Validating manifest...
# → Building artifacts...
# → Uploading to packages.fusabi.dev...
# → Published! https://packages.fusabi.dev/my-dashboard

# Publish specific version
fpm publish --version 0.2.0

# Dry run (validate without publishing)
fpm publish --dry-run
```

### Installation

```bash
# Install from registry
fpm add fusabi-tui-widgets

# Install specific version
fpm add fusabi-tui-widgets@0.1.0

# Install from git
fpm add --git https://github.com/user/widgets

# Install from local path
fpm add --path ../my-widgets

# Install as dev dependency
fpm add --dev fusabi-tui-testing

# Install globally (for CLI tools)
fpm install -g fusabi-dashboard
```

### Scarab Integration

```bash
# Install as Scarab plugin
fpm scarab install
# → Installing to ~/.config/scarab/plugins/my-dashboard/
# → Registering in plugins.toml...
# → Done! Restart Scarab to load.

# Uninstall from Scarab
fpm scarab uninstall

# List Scarab plugins
fpm scarab list

# Update Scarab plugin
fpm scarab update
```

---

## Registry Structure

### Package Index

```
packages.fusabi.dev/
├── api/v1/
│   ├── packages/
│   │   ├── fusabi-tui-core/
│   │   │   ├── index.json        # Package metadata
│   │   │   └── versions/
│   │   │       ├── 0.1.0.json    # Version metadata
│   │   │       └── 0.1.0.tar.gz  # Package archive
│   │   └── ...
│   ├── search?q=tui              # Search endpoint
│   └── categories/
│       ├── tui.json              # TUI packages
│       ├── widgets.json          # Widget libraries
│       └── scarab-plugins.json   # Scarab plugins
```

### Package Archive Contents

```
my-dashboard-0.1.0.tar.gz/
├── fusabi.toml
├── src/
│   └── *.fsx
├── dist/
│   ├── standalone/
│   │   └── my-dashboard.fzb
│   └── scarab-plugin/
│       └── plugin.fzb
├── README.md
├── LICENSE
└── CHANGELOG.md
```

---

## Discovery and Search

### Web Interface

```
packages.fusabi.dev/

┌─────────────────────────────────────────────────────────────────┐
│ Fusabi Package Registry                    [Search packages...] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Categories: [TUI Apps] [Widgets] [Scarab Plugins] [Libraries]   │
│                                                                 │
│ Popular TUI Packages                                            │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ fusabi-tui-widgets    v0.1.0    ★ 234    ↓ 12.3k           │ │
│ │ Essential TUI widgets for Fusabi applications               │ │
│ │ [widgets] [tui] [ui]                                        │ │
│ ├─────────────────────────────────────────────────────────────┤ │
│ │ gpu-dashboard          v0.2.1    ★ 156    ↓ 8.7k           │ │
│ │ Real-time GPU monitoring dashboard                          │ │
│ │ [dashboard] [gpu] [monitoring] [scarab-plugin]              │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### CLI Search

```bash
$ fpm search dashboard
Searching packages.fusabi.dev...

NAME                 VERSION  DOWNLOADS  DESCRIPTION
gpu-dashboard        0.2.1    8.7k       Real-time GPU monitoring
k8s-dashboard        0.1.0    3.2k       Kubernetes cluster monitor
hibana-top           0.3.0    2.1k       Observability dashboard
system-monitor       0.1.2    1.8k       System resource monitor

$ fpm search --category scarab-plugins
...
```

---

## Dependency Resolution

### Lock File (`fusabi.lock`)

```toml
# This file is automatically generated by fpm.
# Manual edits may be overwritten.

[[package]]
name = "fusabi-tui-core"
version = "0.1.0"
source = "registry+https://packages.fusabi.dev"
checksum = "sha256:abc123..."

[[package]]
name = "fusabi-tui-widgets"
version = "0.1.0"
source = "registry+https://packages.fusabi.dev"
checksum = "sha256:def456..."
dependencies = [
    "fusabi-tui-core 0.1.0"
]

[[package]]
name = "hibana-client"
version = "0.1.0"
source = "git+https://github.com/raibid-labs/hibana#abc123"
```

### Resolution Algorithm

Uses Minimal Version Selection (MVS):
1. Parse all dependency specifications
2. Find compatible version ranges
3. Select minimum satisfying version for each
4. Verify no conflicts
5. Generate lock file

---

## Workspace Support

### Multi-Package Workspaces

```toml
# workspace/fusabi.toml
[workspace]
members = [
    "packages/core",
    "packages/widgets",
    "packages/dashboard",
    "examples/*"
]

[workspace.package]
version = "0.1.0"
authors = ["Team <team@example.com>"]
license = "MIT"

[workspace.dependencies]
fusabi-tui-core = { path = "packages/core" }
fusabi-tui-widgets = { path = "packages/widgets" }
```

### Workspace Commands

```bash
# Build all packages
fpm build --workspace

# Test all packages
fpm test --workspace

# Publish all packages (respects dependency order)
fpm publish --workspace

# Run specific package
fpm dev -p dashboard
```

---

## Version Management

### Semantic Versioning

```bash
# Bump patch version (0.1.0 → 0.1.1)
fpm version patch

# Bump minor version (0.1.1 → 0.2.0)
fpm version minor

# Bump major version (0.2.0 → 1.0.0)
fpm version major

# Set specific version
fpm version 1.0.0-beta.1

# Show current version
fpm version
```

### Changelog Generation

```bash
# Generate changelog from commits
fpm changelog

# Generate for specific version range
fpm changelog --from v0.1.0 --to v0.2.0
```

---

## Security

### Package Verification

```bash
# Verify package integrity
fpm verify my-package

# Audit dependencies for vulnerabilities
fpm audit

# Update vulnerable dependencies
fpm audit --fix
```

### Capability Enforcement

For Scarab plugins, capabilities are verified at install:

```bash
$ fpm scarab install suspicious-plugin
Warning: This plugin requests the following capabilities:
  - filesystem (read/write files)
  - network (make HTTP requests)
  - process-spawn (run external commands)

These capabilities could be used maliciously.
Do you want to continue? [y/N]
```

---

## IDE Integration

### LSP Support

```json
// .vscode/settings.json
{
    "fusabi.packageManager": "fpm",
    "fusabi.autoInstall": true,
    "fusabi.formatOnSave": true
}
```

### Auto-Complete for Dependencies

```toml
# fusabi.toml
[dependencies]
fusabi-tui-  # LSP suggests: fusabi-tui-core, fusabi-tui-widgets, ...
```

---

## Migration Guides

### From Standalone to Scarab Plugin

```bash
# Add Scarab target
fpm target add scarab-plugin

# This adds to fusabi.toml:
# [tui.build]
# targets = ["standalone", "scarab-plugin"]

# Creates plugin.toml with defaults

# Build both targets
fpm build
```

### From Ratatui Direct

```bash
# Initialize from existing Ratatui app
fpm init --from-ratatui ./my-ratatui-app

# Generates:
# - fusabi.toml with dependencies
# - Wrapper .fsx files calling existing Rust code
# - Migration guide for gradual port
```

---

## Summary

The package management DX provides:

| Feature | Command |
|---------|---------|
| Create project | `fpm new --template tui-app` |
| Install deps | `fpm install` |
| Development | `fpm dev` |
| Build | `fpm build` |
| Test | `fpm test` |
| Publish | `fpm publish` |
| Scarab install | `fpm scarab install` |
| Search | `fpm search <query>` |
| Audit | `fpm audit` |

Key principles:
- **Convention over configuration** - Sensible defaults
- **Progressive disclosure** - Simple for basic, powerful for advanced
- **Multi-target support** - Same source, multiple outputs
- **Registry integration** - Discover and share packages
