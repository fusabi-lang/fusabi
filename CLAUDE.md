# Claude Code Configuration - FSRS (F#-to-Rust Script Engine)

## 🚨 CRITICAL: CONCURRENT EXECUTION & FILE MANAGEMENT

**ABSOLUTE RULES**:
1. ALL operations MUST be concurrent/parallel in a single message
2. **NEVER save working files, text/mds and tests to the root folder**
3. ALWAYS organize files in appropriate subdirectories
4. **USE CLAUDE CODE'S TASK TOOL** for spawning agents concurrently, not just MCP

### ⚡ GOLDEN RULE: "1 MESSAGE = ALL RELATED OPERATIONS"

**MANDATORY PATTERNS:**
- **TodoWrite**: ALWAYS batch ALL todos in ONE call (5-10+ todos minimum)
- **Task tool (Claude Code)**: ALWAYS spawn ALL agents in ONE message with full instructions
- **File operations**: ALWAYS batch ALL reads/writes/edits in ONE message
- **Bash commands**: ALWAYS batch ALL terminal operations in ONE message
- **Memory operations**: ALWAYS batch ALL memory store/retrieve in ONE message

## Project Overview

**FSRS** is an F#-to-Rust embedded script engine that enables:
- Authoring script modules in F# syntax
- Transpiling via Fable's Rust backend
- Embedding in Rust host applications with first-class integration
- Hot-reloading, host-interop, and dynamic function calls

### Tech Stack
- **F#**: Script authoring language
- **Fable**: F#-to-Rust transpiler (--lang rust)
- **Rust**: Host runtime and generated code
- **Nushell**: Build automation and scripting
- **Just**: Command runner for common tasks

## 📁 File Organization Rules

**Directory Structure:**
```
/src
  /host           - Rust host application code
  /runtime        - Script runtime and integration layer
  /transpiler-extensions - Fable extensions and plugins
/tests           - Test suites (Rust + F# scripts)
/examples        - Example F# scripts and usage demos
/docs            - Documentation and architecture guides
/scripts         - Nushell automation scripts
/.github         - CI/CD workflows and templates
```

**NEVER save to root folder. Use these directories:**
- `/src` - Source code (host, runtime, transpiler)
- `/tests` - All test files
- `/docs` - Documentation and markdown files
- `/scripts` - Nushell automation scripts
- `/examples` - Example F# scripts and demos

## 🛠️ Just Commands (via justfile)

### Quick Reference
```bash
just           # Show all available commands
just build     # Build all components
just test      # Run test suite
just dev       # Start development mode
just clean     # Clean build artifacts
just fmt       # Format all code
just lint      # Run linters
just example NAME  # Run an example script
```

### Development Workflow
```bash
just setup     # Initial project setup
just watch     # Watch mode for hot reload
just transpile FILE  # Transpile F# to Rust
just run SCRIPT      # Run a script in the host
just bench           # Run benchmarks
```

### Quality Assurance
```bash
just check     # Run all checks (fmt, lint, test)
just coverage  # Generate test coverage report
just audit     # Security audit dependencies
just docs      # Generate documentation
```

## 🐚 Nushell Scripts

All automation scripts are in `/scripts/*.nu`:
- `build.nu` - Build orchestration
- `test.nu` - Testing automation
- `transpile.nu` - F#-to-Rust transpilation
- `dev.nu` - Development workflows
- `deploy.nu` - Deployment automation

## Code Style & Best Practices

### Rust Code
- Use `rustfmt` for formatting
- Follow Rust API guidelines
- Keep modules focused and small
- Document public APIs with `///`
- Use `clippy` for lints

### F# Scripts
- Follow F# style guide
- Keep scripts modular
- Use type annotations for clarity
- Document script functions

### General
- **Modular Design**: Files under 500 lines
- **Environment Safety**: Never hardcode secrets
- **Test-First**: Write tests before implementation
- **Clean Architecture**: Separate concerns
- **Documentation**: Keep updated

## 🚀 Available Agents (Use Task Tool)

### Core Development
`coder`, `reviewer`, `tester`, `planner`, `researcher`, `rust-pro`

### Specialized
- `backend-dev` - Rust backend development
- `system-architect` - System design
- `code-analyzer` - Code quality analysis
- `tester` - Test automation
- `debugger` - Bug investigation

### SPARC Methodology
`sparc-coord`, `sparc-coder`, `specification`, `pseudocode`, `architecture`, `refinement`

## 🎯 Development Workflow

### 1. Feature Development
```bash
just dev              # Start watch mode
just transpile script.fsx  # Transpile F# to Rust
just test             # Run tests
just check            # Run all checks
```

### 2. Testing
```bash
just test             # Run all tests
just test unit        # Unit tests only
just test integration # Integration tests
just coverage         # Coverage report
```

### 3. Quality Checks
```bash
just fmt              # Format code
just lint             # Run linters
just audit            # Security audit
just check            # All checks
```

## 🤖 Agent Execution Pattern

When using Claude Code's Task tool for parallel agent execution:

```javascript
// Single message with all agents spawned concurrently
[Parallel Agent Execution]:
  Task("Rust architect", "Design host runtime API and module loading system", "system-architect")
  Task("F# expert", "Create example scripts demonstrating features", "coder")
  Task("Transpiler specialist", "Enhance Fable Rust backend integration", "backend-dev")
  Task("Test engineer", "Create comprehensive test suite", "tester")
  Task("Documentation writer", "Document API and usage patterns", "researcher")

  // Batch ALL todos
  TodoWrite { todos: [...5-10 todos...] }

  // Batch file operations
  Write "src/host/module_loader.rs"
  Write "examples/hello_world.fsx"
  Write "tests/integration_test.rs"
```

## 📋 Project-Specific Guidelines

### F# Script Development
- Keep scripts in `/examples` or user workspace
- Use `.fsx` for script files, `.fs` for modules
- Document script requirements and dependencies
- Provide clear examples of host interop

### Rust Host Development
- Host code in `/src/host`
- Runtime code in `/src/runtime`
- Use traits for extensibility
- Support hot-reload via dynamic loading

### Transpiler Extensions
- Extensions in `/src/transpiler-extensions`
- Document Fable integration points
- Test transpilation thoroughly
- Provide error handling for edge cases

### Testing Strategy
- Unit tests for Rust runtime
- Integration tests for F# script execution
- Performance benchmarks for hot-reload
- Example scripts as validation tests

## 🔧 Build System

### Cargo Workspaces
```toml
[workspace]
members = ["src/host", "src/runtime"]
```

### Just + Nushell Integration
- `justfile` defines high-level commands
- Nushell scripts implement complex workflows
- Cross-platform compatibility
- Rich error handling and reporting

## 🚀 Getting Started

```bash
# Initial setup
./init.sh         # Run initial setup script
just setup        # Install dependencies

# Development
just dev          # Start development mode
just example hello  # Run hello world example

# Build and test
just build        # Build all components
just test         # Run test suite
just check        # Run all quality checks
```

## 📚 Documentation

- Architecture: `/docs/architecture.md`
- API Reference: `/docs/api.md`
- Examples: `/examples/README.md`
- Contributing: `/docs/contributing.md`

## 🎯 Supported Features

### Current
- Let-bindings, functions, modules
- Basic types: int, bool, string, list/array
- Script function calls from Rust
- Host function registration
- Hot-reload of script modules

### Future
- Full F# type system support
- Async workflows
- Advanced interop patterns
- Performance optimizations

## 🔗 Integration Tips

1. Use justfile for common tasks
2. Leverage nushell for complex automation
3. Follow raibid-labs org patterns
4. Use Task tool for parallel agent execution
5. Batch all operations in single messages
6. Organize files in proper directories

## Support & Resources

- Repository: https://github.com/raibid-labs/fsrs
- Issues: Use GitHub Issues
- Discussions: GitHub Discussions

---

**Remember**: All operations in parallel, proper file organization, Task tool for agents!
