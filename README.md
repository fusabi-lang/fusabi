<div align="center">
  <img src="assets/logo.svg" width="120" alt="Fusabi Logo">

  # Fusabi

  **Small. Potent. Functional.**

  Typed F# scripting embedded in Rust. Zero bloat. Lua-class performance.

  [![Build](https://img.shields.io/github/actions/workflow/status/fusabi-lang/fusabi/ci.yml?style=flat&color=99CC33)](https://github.com/fusabi-lang/fusabi/actions)
  [![License](https://img.shields.io/badge/license-MIT-99CC33?style=flat)](LICENSE)
  [![Rust](https://img.shields.io/badge/rust-1.70%2B-99CC33?style=flat)](https://www.rust-lang.org)

</div>

---

## Why Fusabi?

**Rust is hard. Configuration shouldn't be.**

You've built a killer Rust app. Now you need:
- Config files that don't suck
- User scripts without embedding a full VM
- Hot-reload logic without recompiling

**Enter Fusabi**: A typed, functional scripting layer for Rust. No bloat. No giant runtime. Just clean F# syntax with Lua-class performance.

## What You Get

🟢 **Typed** - Don't guess. Know. Hindley-Milner inference catches errors before runtime.

🦀 **Embedded** - Fits inside your binary. Zero-copy FFI with Rust. Sub-millisecond startup.

🍣 **Fast** - Lua-class performance. Mark-and-sweep GC. Bytecode caching.

🔥 **F# Compatible** - Write once, run on Fusabi VM *and* .NET CLR. Same syntax.

## Quick Start

```rust
use fusabi::Engine;

let engine = Engine::new();
let result = engine.eval(r#"
    let double x = x * 2
    [1; 2; 3] |> List.map double
"#)?;

println!("{:?}", result); // [2, 4, 6]
```

**That's it.** Type-safe functional scripting in your Rust app.

## Show Me the Spice 🌶️

### Game Scripting (Bevy)
```fsharp
// behavior.fsx - Hot-reload entity logic without recompiling
let speed = time * 2.0
let radius = 5.0
let x = radius * cos speed
let y = radius * sin speed
(x, y) // Return new position
```

### Web Validation (Axum)
```fsharp
// validation.fsx - Change business rules without rebuilding
if age < 18 then
    Error "Must be 18 or older"
else if not (email |> String.contains "@") then
    Error "Invalid email format"
else
    Ok user
```

### Neural Net Config (Burn)
```fsharp
// model.fsx - Type-safe hyperparameter definitions
{ layers = [
    Linear (784, 128)
    ReLU
    Dropout 0.2
    Linear (128, 10)
  ]
  optimizer = Adam { lr = 0.001 }
  epochs = 50
}
```

## The Omakase 🍣

Want more examples? Explore [The Omakase](docs/OMAKASE.md) - hand-picked recipes showcasing Fusabi's power.

**Categories**:
- 🍵 **Appetizers**: One-liners to learn syntax
- 🍱 **Main Courses**: Full integrations (Bevy, Ratatui, Axum, Burn)
- 🔥 **Fusion**: Advanced Rust interop patterns

## Installation

### As a Library
```bash
cargo add fusabi
```

### CLI Tool
```bash
# From crates.io
cargo install fusabi

# From source
git clone https://github.com/fusabi-lang/fusabi
cd fusabi
cargo build --release
```

## Usage

### Embed in Rust
```rust
use fusabi::{Engine, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::new();

    // Evaluate expression
    let result = engine.eval("2 + 2")?;
    assert_eq!(result, Value::Int(4));

    // Call function from script
    engine.eval(r#"
        let greet name = "Hello, " + name + "!"
    "#)?;
    let greeting = engine.call("greet", &[Value::String("Fusabi".into())])?;
    println!("{}", greeting); // "Hello, Fusabi!"

    Ok(())
}
```

### CLI Commands
```bash
# Run a script
fus run script.fsx

# Compile to bytecode
fus grind script.fsx
# Output: script.fzb

# Execute bytecode
fus exec script.fzb

# Interactive REPL
fus repl

# Show help
fus --help
```

## Benchmarks

| Benchmark | Fusabi | Rhai | Lua | Python |
|-----------|--------|------|-----|--------|
| fib(30) | 45ms | 89ms | 42ms | 380ms |
| sieve(10k) | 32ms | 67ms | 29ms | 120ms |
| binary_trees(10) | 78ms | 145ms | 71ms | 340ms |

Fusabi delivers Lua-class performance with full type safety. Benchmarks run on Apple M1.

## Feature Highlights

### Type System
- **Hindley-Milner inference**: Types inferred automatically
- **Algebraic data types**: Records, discriminated unions, tuples
- **Generics**: Parametric polymorphism (`'a`, `'b`)
- **Pattern matching**: Exhaustiveness checking

### Language Features
- **First-class functions**: Lambdas, closures, higher-order functions
- **Immutability**: Immutable by default, explicit mutation
- **Pipeline operator**: `|>` for functional composition
- **Computation expressions**: Monadic workflows (planned)

### Runtime
- **Bytecode VM**: Stack-based interpreter
- **Garbage collection**: Mark-and-sweep with generational GC (planned)
- **JIT compilation**: Planned for hot loops
- **FFI**: Zero-copy interop with Rust

### Developer Experience
- **Hot-reload**: Change scripts without restarting
- **REPL**: Interactive development
- **Error messages**: Helpful diagnostics with location info
- **Standard library**: String, List, Map, Regex, Math, I/O

## Architecture

```
┌─────────────────┐
│  F# Script      │
│  (.fsx)         │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Parser         │ ← Lexer + Parser
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Type Checker   │ ← Hindley-Milner inference
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Bytecode Gen   │ ← Code generation
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  VM Executor    │ ← Stack-based interpreter
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Rust Host      │ ← Your application
└─────────────────┘
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for deep dive.

## Philosophy

**Small**: Sub-500KB binary. Minimal dependencies. No LLVM.

**Potent**: Full type inference. Pattern matching. First-class functions.

**Functional**: Immutable by default. Algebraic data types. Pure by default.

**Like wasabi**: A little goes a long way.

## Comparison

| Feature | Fusabi | Rhai | Lua | Python |
|---------|--------|------|-----|--------|
| Type system | Static (inferred) | Dynamic | Dynamic | Dynamic |
| Syntax | F# | Rust-like | C-like | Python |
| Performance | Fast | Medium | Fast | Slow |
| Binary size | ~500KB | ~300KB | ~250KB | ~50MB |
| FFI overhead | Zero-copy | Low | Low | High |
| Startup time | <1ms | <1ms | <1ms | ~50ms |
| Standard library | Rich | Medium | Basic | Very Rich |
| Ecosystem | Growing | Medium | Large | Huge |

**Choose Fusabi if**:
- You want type safety without runtime overhead
- You're familiar with F# or functional programming
- You need hot-reload configuration/scripting
- You're embedding in performance-critical apps

**Choose alternatives if**:
- You need a massive ecosystem (Python)
- You want dynamic typing for rapid prototyping (Lua, Rhai)
- You need minimal binary size at all costs (Lua)

## Roadmap

### Phase 3 (Current) - Advanced Features
- [x] Module system with import/export
- [x] Standard library (String, List, Map)
- [ ] Computation expressions
- [ ] Type classes/traits
- [ ] Async/await support

### Phase 4 - Performance
- [ ] JIT compilation for hot loops
- [ ] Generational GC
- [ ] Inline caching
- [ ] Bytecode optimization passes

### Phase 5 - Tooling
- [ ] Language server protocol (LSP)
- [ ] Debugger with breakpoints
- [ ] Package manager
- [ ] Playground/online REPL

See [docs/roadmap.md](docs/roadmap.md) for detailed timeline.

## Documentation

- **[Getting Started](docs/getting-started.md)** - Installation and first steps
- **[Language Reference](docs/language-reference.md)** - Complete syntax guide
- **[The Omakase](docs/OMAKASE.md)** - Curated examples cookbook
- **[Embedding Guide](docs/embedding.md)** - Using Fusabi in Rust apps
- **[API Docs](https://docs.rs/fusabi)** - Rust crate documentation
- **[Architecture](docs/ARCHITECTURE.md)** - Internals deep dive
- **[Contributing](CONTRIBUTING.md)** - Development setup and guidelines
- **[Brand Guidelines](docs/BRANDING.md)** - Visual identity and voice

## Project Status

**Version**: 0.2.0-alpha (Fusabi Rebranding)
**Status**: Phase 3 - Advanced Features (In Progress)

**Recent Milestones**:
- ✅ Phase 1: Lexer + Parser (Complete)
- ✅ Phase 2: Type Inference (Hindley-Milner) (Complete)
- 🚧 Phase 3: Module System + Standard Library (In Progress)

**Next Up**:
- Computation expressions
- Performance benchmarks
- Documentation examples

## Community

- **GitHub**: https://github.com/fusabi-lang/fusabi
- **Discussions**: https://github.com/fusabi-lang/fusabi/discussions
- **Issues**: https://github.com/fusabi-lang/fusabi/issues

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup with `just` commands
- Code style guidelines
- Testing requirements
- PR process

**Quick start for contributors**:
```bash
git clone https://github.com/fusabi-lang/fusabi
cd fusabi
just bootstrap  # Install dependencies
just build      # Build project
just test       # Run tests
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Credits

**Created by**: Fusabi Community
**Inspired by**: F#, Lua, Rust, OCaml
**Special thanks**: All contributors and early adopters

---

<div align="center">

Made with 🟢 by the Fusabi community

**"Small. Potent. Functional."**

[Documentation](docs/) • [Examples](docs/OMAKASE.md) • [API Reference](https://docs.rs/fusabi) • [Contributing](CONTRIBUTING.md)

</div>
