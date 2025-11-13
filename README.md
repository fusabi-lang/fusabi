# F#-to-Rust Embedded Script Engine (FSRS)

## Overview
This project enables authoring script modules in **F# syntax**, transpiling them via **Fable's Rust backend**, and embedding them inside a **Rust host application** with first-class integration (load, call, hot-reload, host-interop).

## Architecture
1. F# source files (`.fs` / `.fsx`)
2. Transpile using Fable with the `--lang rust` backend into Rust code
3. Compile generated Rust code via Cargo
4. Rust host application:
   - Loads script modules as dynamic crates or static modules
   - Invokes script-defined functions
   - Exposes host-side functions/types to scripts
   - Supports hot-reloading of script modules

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs) (latest stable)
- [.NET SDK](https://dotnet.microsoft.com/download) (6.0 or later)
- [Nushell](https://nushell.sh) (for automation scripts)
- [Just](https://github.com/casey/just) (command runner)

### Installation

```bash
# Clone the repository
git clone https://github.com/raibid-labs/fsrs.git
cd fsrs

# Run setup script
just setup

# Build the project
just build

# Run an example
just example hello
```

## Using Just Commands

FSRS uses [Just](https://github.com/casey/just) as a command runner for common tasks:

```bash
# View all available commands
just

# Development
just dev              # Start watch mode with hot reload
just build            # Build all components
just test             # Run test suite
just check            # Run all quality checks

# Code Quality
just fmt              # Format code
just lint             # Run linters
just audit            # Security audit

# Examples
just example hello    # Run hello example
just run script.fsx   # Run a specific script

# Documentation
just docs             # Generate and open docs

# Utilities
just clean            # Clean build artifacts
just version          # Show version info
just ci               # Run CI checks locally
```

## Project Structure

```
fsrs/
├── src/
│   ├── host/                 # Rust host application
│   ├── runtime/              # Script runtime and integration
│   └── transpiler-extensions/# Fable extensions
├── tests/
│   ├── unit/                 # Unit tests
│   └── integration/          # Integration tests
├── examples/                 # Example F# scripts
├── scripts/                  # Nushell automation scripts
├── docs/                     # Documentation
├── justfile                  # Just command definitions
└── CLAUDE.md                 # Claude Code configuration

## Nushell Scripts

All automation is powered by Nushell scripts in `/scripts/`:

- `setup.nu` - Environment setup
- `build.nu` - Build orchestration
- `test.nu` - Test automation
- `transpile.nu` - F#-to-Rust transpilation
- `dev.nu` - Development workflows
- `run.nu` - Script execution
- `format.nu` - Code formatting
- `lint.nu` - Linting
- `check.nu` - Quality checks
- `clean.nu` - Cleanup
- `docs.nu` - Documentation generation
- `bench.nu` - Benchmarking
- `install.nu` - Installation
- `version.nu` - Version info
- `ci.nu` - CI simulation
- `init.nu` - Project initialization
- `repl.nu` - Interactive REPL
- `profile.nu` - Performance profiling
- `release.nu` - Release preparation

## Development Workflow

### 1. Create a New F# Script

```bash
# Initialize a new script project
just init my-script

# Edit the script
nano examples/my-script/my-script.fsx

# Run it
just example my-script
```

### 2. Watch Mode Development

```bash
# Start watch mode for automatic rebuilds
just watch

# Or watch with automatic testing
just watch-test
```

### 3. Transpile F# to Rust

```bash
# Transpile a specific file
just transpile examples/hello.fsx

# Output is in target/transpiled/
```

### 4. Testing

```bash
# Run all tests
just test

# Run only unit tests
just test-unit

# Run only integration tests
just test-integration

# Generate coverage report
just coverage
```

### 5. Quality Assurance

```bash
# Run all checks before committing
just check

# Individual checks
just fmt-check
just lint
just audit
```

## Supported Features (Initial)
- Let-bindings, functions, modules
- Basic types: int, bool, string, list/array
- Calling script functions from Rust host
- Registering host functions callable from scripts
- Hot-reload of script modules

## Out of Scope (Initial)
- Full F# type system: interfaces, generics, computation expressions
- Full async workflows
- Reflection
- Full .NET BCL compatibility

## Manual Transpilation Example

If you want to transpile manually without using the scripts:

```bash
dotnet fable MyScript.fsx --lang rust
