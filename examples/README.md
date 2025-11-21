# Fusabi Examples

Welcome to The Omakase - hand-rolled, chef-selected examples that showcase Fusabi's power.

## Quick Links

For the complete cookbook with detailed explanations, see [The Omakase](../docs/OMAKASE.md).

## What's Here

### 🍵 Appetizers
Quick bites to learn Fusabi syntax:
- String manipulation with pipelines
- List operations and higher-order functions
- Pattern matching and type inference
- Regex and built-in libraries

### 🍱 Main Courses
Full application integrations:
- **Bevy Game Scripting**: Hot-reload entity behaviors
- **Ratatui TUI Layouts**: Functional terminal UI composition
- **Axum Web Validation**: Business logic in F# scripts
- **Burn Neural Nets**: Type-safe ML configuration

### 🔥 Fusion
Advanced Rust interop:
- Host function callbacks across FFI
- .NET compatibility (same script, dual runtimes)
- Computation expressions and custom DSLs

## Running Examples

```bash
# Run any example directly
fus run examples/<category>/<example>.fsx

# With disassembly output
fus run --disasm examples/appetizers/string_ops.fsx

# Compile to bytecode (coming soon)
fus grind examples/main_courses/bevy_scripting/behavior.fsx
```

## Philosophy

**"Omakase"** (お任せ) means "I'll leave it up to you" - trust the chef's selection. These aren't random snippets. They're carefully curated patterns demonstrating real-world use cases.

Small. Potent. Functional. 🟢

## Contributing

Have a spicy pattern? See [The Omakase](../docs/OMAKASE.md#contribution-guide) for how to add your recipe.

---

For complete documentation, see: [docs/](../docs/)
