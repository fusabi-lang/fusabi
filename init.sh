# Create directories
mkdir -p docs
mkdir -p src/host
mkdir -p src/runtime
mkdir -p src/transpiler-extensions
mkdir -p examples
mkdir -p .github/ISSUE_TEMPLATE

####################
# README.md
####################
cat > README.md << 'EOF'
# F#-to-Rust Embedded Script Engine

## Overview
This project enables authoring script modules in **F# syntax**, transpiling them via **Fable’s Rust backend**, and embedding them inside a **Rust host application** with first-class integration (load, call, hot-reload, host-interop).

## Architecture
1. F# source files (`.fs` / `.fsx`)
2. Transpile using Fable with the `--lang rust` backend into Rust code
3. Compile generated Rust code via Cargo
4. Rust host application:
   - Loads script modules as dynamic crates or static modules
   - Invokes script-defined functions
   - Exposes host-side functions/types to scripts
   - Supports hot-reloading of script modules

## Supported Features (initial)
- Let-bindings, functions, modules
- Basic types: int, bool, string, list/array
- Calling script functions from Rust host
- Registering host functions callable from scripts
- Hot-reload of script modules

## Out of Scope (initial)
- Full F# type system: interfaces, generics, computation expressions
- Full async workflows
- Reflection
- Full .NET BCL compatibility

## Quickstart
### Transpile F# to Rust:
```bash
dotnet fable MyScript.fsx --lang rust