# FSRS Setup Guide

## Quick Setup

The fastest way to get started:

```bash
just setup
```

This will:
1. Check all prerequisites
2. Create directory structure
3. Install Rust dependencies
4. Install F# and Fable tools
5. Setup git hooks

## Prerequisites

### Required

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **.NET SDK** (6.0 or later)
   - Download from: https://dotnet.microsoft.com/download

3. **Nushell** (for automation)
   ```bash
   # macOS
   brew install nushell

   # Linux
   cargo install nu

   # Windows
   winget install nushell
   ```

4. **Just** (command runner)
   ```bash
   cargo install just
   ```

### Optional but Recommended

- **cargo-watch** (for development watch mode)
  ```bash
  cargo install cargo-watch
  ```

- **cargo-edit** (for dependency management)
  ```bash
  cargo install cargo-edit
  ```

- **cargo-tarpaulin** (for code coverage)
  ```bash
  cargo install cargo-tarpaulin
  ```

- **cargo-audit** (for security audits)
  ```bash
  cargo install cargo-audit
  ```

- **cargo-flamegraph** (for profiling)
  ```bash
  cargo install cargo-flamegraph
  ```

## Manual Setup

If you prefer to set things up manually:

### 1. Create Directory Structure

```bash
mkdir -p src/{host,runtime,transpiler-extensions}
mkdir -p tests/{unit,integration}
mkdir -p examples docs scripts
```

### 2. Install Rust Components

```bash
rustup component add rustfmt clippy
```

### 3. Install Fable

```bash
dotnet tool install -g fable
```

### 4. Install Cargo Tools

```bash
cargo install cargo-watch cargo-edit cargo-audit
```

### 5. Setup Git Hooks

Create `.git/hooks/pre-commit`:

```bash
#!/bin/sh
just fmt-check && just lint
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

## Verification

After setup, verify everything is working:

```bash
# Check versions
just version

# Run a quick build
just build

# Run tests
just test
```

## Troubleshooting

### Rust Not Found

If `rustc` or `cargo` commands are not found:

1. Make sure Rust is installed: https://rustup.rs
2. Ensure `~/.cargo/bin` is in your PATH:
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

### .NET SDK Not Found

If `dotnet` command is not found:

1. Install .NET SDK: https://dotnet.microsoft.com/download
2. Verify installation: `dotnet --version`

### Fable Not Found

If `fable` command is not found:

1. Install as global tool: `dotnet tool install -g fable`
2. Ensure .NET tools are in PATH:
   ```bash
   echo 'export PATH="$HOME/.dotnet/tools:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

### Permission Denied on Scripts

If you get permission errors running scripts:

```bash
chmod +x scripts/*.nu
```

### Just Command Not Working

Make sure Just is installed and in your PATH:

```bash
cargo install just
which just  # Should show the path
```

## Next Steps

After setup is complete:

1. Read the [Architecture Guide](architecture.md)
2. Check out the [Examples](../examples/)
3. Try the development workflow: `just dev`
4. Create your first script: `just init my-script`

## Environment Variables

FSRS respects these environment variables:

- `CARGO_HOME` - Cargo installation directory (default: `~/.cargo`)
- `DOTNET_ROOT` - .NET installation directory
- `RUST_LOG` - Logging level (debug, info, warn, error)

## IDE Setup

### VS Code

Recommended extensions:
- Rust Analyzer
- Ionide-fsharp
- Even Better TOML

### JetBrains

- IntelliJ Rust
- Rider (for F#)

## Updating

To update dependencies:

```bash
just update
```

This will update both Rust crates and F# tools.
