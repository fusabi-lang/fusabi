# RFC-003: Multi-File Module System

**Status**: Draft
**Author**: Claude (AI Assistant)
**Created**: 2025-12-14
**Requires**: RFC-001 (Computation Expressions), Module System

## Summary

This RFC proposes adding `#load` directive support to Fusabi, enabling code organization across multiple `.fsx` files. This is critical for building larger applications like TUI frameworks where widgets, views, and utilities need to be organized in separate files.

## Motivation

### Current Limitation

Fusabi currently only supports single-file modules:

```fsharp
// Everything must be in one file
module Widgets =
    let button label = ...
    let list items = ...
    // 500+ lines...

module Views =
    let dashboard state = ...
    let settings state = ...
    // 500+ lines...

// main.fsx becomes unwieldy
```

### Problems Solved

1. **Code Organization**: Split large applications into manageable files
2. **Reusability**: Share utilities across multiple scripts
3. **Team Collaboration**: Multiple developers can work on different files
4. **Hot Reload**: Only reload changed files, not entire application
5. **Package Management**: FPM needs multi-file support for packages

### Use Cases

#### TUI Widget Library
```fsharp
// widgets/button.fsx
module Button =
    let create label onClick = ...

// widgets/list.fsx
module List =
    let create items = ...

// app.fsx
#load "widgets/button.fsx"
#load "widgets/list.fsx"

let view state = ...
```

#### Shared Utilities
```fsharp
// utils/http.fsx
module Http =
    let get url = ...
    let post url body = ...

// Multiple apps can #load "utils/http.fsx"
```

## Proposed Syntax

### `#load` Directive

Load and evaluate another `.fsx` file:

```fsharp
#load "path/to/file.fsx"
```

### Path Resolution

1. **Relative paths**: Resolved from current file's directory
2. **Absolute paths**: Used as-is
3. **Package paths**: Resolved via FPM (future)

```fsharp
// Relative to current file
#load "utils.fsx"
#load "./components/button.fsx"
#load "../shared/http.fsx"

// Absolute path
#load "/home/user/libs/mylib.fsx"

// Package path (future FPM integration)
#load "pkg:fusabi-tui-widgets/button.fsx"
```

### Multiple Loads

Files can be loaded multiple times in different files - they're only evaluated once:

```fsharp
// a.fsx
#load "utils.fsx"  // Evaluates utils.fsx

// b.fsx
#load "utils.fsx"  // Uses cached result

// main.fsx
#load "a.fsx"      // Loads a.fsx (and utils.fsx)
#load "b.fsx"      // Loads b.fsx (utils.fsx already loaded)
```

### Conditional Loading (Future)

```fsharp
#if SCARAB_PLUGIN
#load "renderers/scarab.fsx"
#else
#load "renderers/crossterm.fsx"
#endif
```

## Semantics

### Evaluation Order

1. Directives are processed top-to-bottom
2. Each `#load` blocks until the loaded file is fully evaluated
3. Loaded modules/bindings become available immediately after

```fsharp
#load "a.fsx"      // A's bindings now available
let x = A.func ()  // OK

#load "b.fsx"      // B's bindings now available
let y = B.func ()  // OK
```

### Scope Rules

Loaded files introduce their top-level bindings and modules into scope:

```fsharp
// math.fsx
module Math =
    let add x y = x + y

let PI = 3.14159

// main.fsx
#load "math.fsx"

let result = Math.add 1 2  // Module access
let area = PI * r * r       // Top-level binding access
```

### Dependency Graph

The loader builds a dependency graph to:
1. Detect circular dependencies (error)
2. Ensure correct evaluation order (topological sort)
3. Cache loaded files (load once, use many)

```
main.fsx
├── widgets/button.fsx
│   └── utils/style.fsx
├── widgets/list.fsx
│   └── utils/style.fsx  (already loaded, cached)
└── views/dashboard.fsx
    ├── widgets/button.fsx  (already loaded, cached)
    └── widgets/list.fsx    (already loaded, cached)
```

### Error Handling

#### Circular Dependency
```fsharp
// a.fsx
#load "b.fsx"

// b.fsx
#load "a.fsx"  // ERROR: Circular dependency detected: a.fsx -> b.fsx -> a.fsx
```

#### File Not Found
```fsharp
#load "nonexistent.fsx"  // ERROR: File not found: nonexistent.fsx
```

#### Parse/Compile Error in Loaded File
```fsharp
#load "broken.fsx"  // ERROR: In broken.fsx:5:10: Unexpected token '}'
```

## Implementation Strategy

### Phase 1: Lexer & Parser (fusabi-frontend)

#### Lexer Changes

```rust
// crates/fusabi-frontend/src/lexer.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ... existing tokens ...

    // Directives
    LoadDirective(String),  // #load "path"
}

impl Lexer {
    fn scan_directive(&mut self) -> Result<Token, LexError> {
        self.advance(); // consume '#'
        let directive = self.scan_identifier()?;

        match directive.as_str() {
            "load" => {
                self.skip_whitespace();
                let path = self.scan_string()?;
                Ok(Token::LoadDirective(path))
            }
            _ => Err(LexError::UnknownDirective(directive))
        }
    }
}
```

#### Parser Changes

```rust
// crates/fusabi-frontend/src/parser.rs

pub struct LoadDirective {
    pub path: String,
    pub span: Span,
}

pub struct Program {
    pub directives: Vec<LoadDirective>,
    pub modules: Vec<ModuleDef>,
    pub imports: Vec<Import>,
    pub main_expr: Option<Expr>,
}

impl Parser {
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut directives = Vec::new();
        let mut modules = Vec::new();
        let mut imports = Vec::new();

        // Parse directives first
        while let Some(Token::LoadDirective(path)) = self.peek() {
            directives.push(LoadDirective {
                path: path.clone(),
                span: self.current_span(),
            });
            self.advance();
        }

        // Parse modules and imports
        // ... existing logic ...

        Ok(Program { directives, modules, imports, main_expr })
    }
}
```

### Phase 2: File Loader (fusabi-frontend)

```rust
// crates/fusabi-frontend/src/loader.rs

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct FileLoader {
    /// Cache of already-loaded files (path -> compiled result)
    cache: HashMap<PathBuf, LoadedFile>,

    /// Currently loading files (for cycle detection)
    loading: HashSet<PathBuf>,

    /// Base directory for relative paths
    base_dir: PathBuf,
}

pub struct LoadedFile {
    pub path: PathBuf,
    pub modules: Vec<CompiledModule>,
    pub bindings: Vec<(String, Value)>,
}

#[derive(Debug)]
pub enum LoadError {
    FileNotFound(PathBuf),
    CircularDependency(Vec<PathBuf>),
    ParseError(PathBuf, ParseError),
    CompileError(PathBuf, CompileError),
    IoError(std::io::Error),
}

impl FileLoader {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            cache: HashMap::new(),
            loading: HashSet::new(),
            base_dir,
        }
    }

    pub fn load(&mut self, path: &str, from_file: &Path) -> Result<&LoadedFile, LoadError> {
        let resolved = self.resolve_path(path, from_file)?;

        // Check cache first
        if self.cache.contains_key(&resolved) {
            return Ok(self.cache.get(&resolved).unwrap());
        }

        // Check for cycles
        if self.loading.contains(&resolved) {
            return Err(LoadError::CircularDependency(
                self.loading.iter().cloned().collect()
            ));
        }

        // Mark as loading
        self.loading.insert(resolved.clone());

        // Read and parse file
        let source = std::fs::read_to_string(&resolved)
            .map_err(LoadError::IoError)?;

        let program = Parser::new(&source)
            .parse_program()
            .map_err(|e| LoadError::ParseError(resolved.clone(), e))?;

        // Recursively load dependencies
        for directive in &program.directives {
            self.load(&directive.path, &resolved)?;
        }

        // Compile the file
        let compiled = Compiler::new()
            .with_loader(self)
            .compile_program(&program)
            .map_err(|e| LoadError::CompileError(resolved.clone(), e))?;

        // Remove from loading, add to cache
        self.loading.remove(&resolved);
        self.cache.insert(resolved.clone(), compiled);

        Ok(self.cache.get(&resolved).unwrap())
    }

    fn resolve_path(&self, path: &str, from_file: &Path) -> Result<PathBuf, LoadError> {
        let resolved = if path.starts_with('/') {
            // Absolute path
            PathBuf::from(path)
        } else if path.starts_with("pkg:") {
            // Package path (future)
            todo!("Package path resolution")
        } else {
            // Relative to from_file's directory
            from_file.parent()
                .unwrap_or(&self.base_dir)
                .join(path)
        };

        let canonical = resolved.canonicalize()
            .map_err(|_| LoadError::FileNotFound(resolved.clone()))?;

        Ok(canonical)
    }
}
```

### Phase 3: Compiler Integration

```rust
// crates/fusabi-frontend/src/compiler.rs

impl Compiler {
    pub fn with_loader(mut self, loader: &FileLoader) -> Self {
        self.loader = Some(loader);
        self
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<CompiledProgram, CompileError> {
        // First, ensure all dependencies are loaded
        if let Some(loader) = &mut self.loader {
            for directive in &program.directives {
                let loaded = loader.load(&directive.path, &self.current_file)?;

                // Import bindings from loaded file
                for (name, value) in &loaded.bindings {
                    self.env.define(name.clone(), value.clone());
                }

                // Register loaded modules
                for module in &loaded.modules {
                    self.module_registry.register(module.clone());
                }
            }
        }

        // Continue with normal compilation
        // ... existing logic ...
    }
}
```

### Phase 4: Hot Reload Support

```rust
// crates/fusabi-frontend/src/hot_reload.rs

pub struct HotReloader {
    loader: FileLoader,
    watchers: HashMap<PathBuf, FileWatcher>,
    on_reload: Box<dyn Fn(&LoadedFile) + Send>,
}

impl HotReloader {
    pub fn watch(&mut self, path: &Path) -> Result<(), WatchError> {
        let watcher = FileWatcher::new(path, move |event| {
            if event.kind == EventKind::Modify {
                // Invalidate cache for this file and dependents
                self.loader.invalidate(path);

                // Reload the file
                if let Ok(loaded) = self.loader.load(path.to_str().unwrap(), path) {
                    (self.on_reload)(loaded);
                }
            }
        })?;

        self.watchers.insert(path.to_path_buf(), watcher);
        Ok(())
    }

    pub fn invalidate_dependents(&mut self, path: &Path) {
        // Find all files that depend on this one and invalidate them too
        let dependents = self.loader.find_dependents(path);
        for dep in dependents {
            self.loader.cache.remove(&dep);
        }
    }
}
```

## Examples

### Example 1: Simple Widget Library

```fsharp
// widgets/base.fsx
module Widget =
    type Props = { id: string; style: Style }
    let make props render = { props = props; render = render }

// widgets/button.fsx
#load "base.fsx"

module Button =
    let create label onClick =
        Widget.make
            { id = "button"; style = Style.default }
            (fun () -> renderButton label onClick)

// widgets/list.fsx
#load "base.fsx"

module List =
    let create items onSelect =
        Widget.make
            { id = "list"; style = Style.default }
            (fun () -> renderList items onSelect)

// app.fsx
#load "widgets/button.fsx"
#load "widgets/list.fsx"

let view state =
    Layout.vertical [
        Button.create "Click Me" (fun () -> update state)
        List.create state.items (fun i -> select state i)
    ]
```

### Example 2: Shared Configuration

```fsharp
// config.fsx
module Config =
    let apiUrl = "https://api.example.com"
    let timeout = 5000
    let retries = 3

// http.fsx
#load "config.fsx"

module Http =
    let get url =
        httpRequest Config.apiUrl url Config.timeout Config.retries

// api.fsx
#load "config.fsx"
#load "http.fsx"

module Api =
    let fetchUser id = Http.get (sprintf "/users/%d" id)
```

### Example 3: Dashboard with Hot Reload

```fsharp
// views/overview.fsx
module Overview =
    let render metrics =
        Block.new "Overview" [
            Gauge.new "CPU" metrics.cpu
            Gauge.new "Memory" metrics.memory
        ]

// views/details.fsx
module Details =
    let render selected =
        Block.new "Details" [
            Paragraph.new selected.description
        ]

// dashboard.fsx
#load "views/overview.fsx"
#load "views/details.fsx"

let view state =
    Layout.horizontal [
        Overview.render state.metrics, Constraint.Percentage 30
        Details.render state.selected, Constraint.Fill 1
    ]

// Changes to overview.fsx trigger hot-reload of just that view
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_simple_file() {
        let loader = FileLoader::new(PathBuf::from("/tmp"));

        std::fs::write("/tmp/test.fsx", "let x = 42").unwrap();

        let loaded = loader.load("test.fsx", Path::new("/tmp")).unwrap();
        assert_eq!(loaded.bindings.len(), 1);
        assert_eq!(loaded.bindings[0].0, "x");
    }

    #[test]
    fn test_circular_dependency_detection() {
        let loader = FileLoader::new(PathBuf::from("/tmp"));

        std::fs::write("/tmp/a.fsx", "#load \"b.fsx\"\nlet a = 1").unwrap();
        std::fs::write("/tmp/b.fsx", "#load \"a.fsx\"\nlet b = 2").unwrap();

        let result = loader.load("a.fsx", Path::new("/tmp"));
        assert!(matches!(result, Err(LoadError::CircularDependency(_))));
    }

    #[test]
    fn test_caching() {
        let mut loader = FileLoader::new(PathBuf::from("/tmp"));

        std::fs::write("/tmp/cached.fsx", "let x = 42").unwrap();

        loader.load("cached.fsx", Path::new("/tmp")).unwrap();
        loader.load("cached.fsx", Path::new("/tmp")).unwrap();

        // Should only compile once
        assert_eq!(loader.cache.len(), 1);
    }
}
```

### Integration Tests

```bash
# test/multifile/run_tests.sh

# Test 1: Basic load
fus run test/multifile/basic/main.fsx
assert_output "42"

# Test 2: Nested loads
fus run test/multifile/nested/main.fsx
assert_output "success"

# Test 3: Circular dependency error
fus run test/multifile/circular/main.fsx 2>&1 | grep "Circular dependency"
assert_exit_code 1
```

## Migration Path

### From Single-File to Multi-File

1. Identify logical groupings in existing code
2. Extract modules to separate files
3. Add `#load` directives
4. Test incrementally

### Tooling Support

- **fpm**: Automatically handles `#load` in packages
- **LSP**: Go-to-definition works across files
- **Hot Reload**: Watches all loaded files

## Open Questions

1. **Should `#load` support URLs?** (e.g., `#load "https://..."`)
2. **Namespace pollution**: Should loaded files use implicit or explicit imports?
3. **Bytecode caching**: Should `.fzb` files cache dependency info?
4. **Parallel loading**: Can independent files be loaded in parallel?

## Alternatives Considered

### Import Statements (like ES6)
```fsharp
import { Button, List } from "widgets.fsx"
```
- Pro: More explicit about what's imported
- Con: Different from F# syntax, harder to implement

### Module Paths (like Rust)
```fsharp
mod widgets;  // loads widgets.fsx or widgets/mod.fsx
```
- Pro: Cleaner syntax
- Con: Requires directory conventions

### Compilation Units (like C#)
- Pro: Familiar to .NET developers
- Con: Requires project files, more complex

## References

- [F# Interactive #load](https://docs.microsoft.com/en-us/dotnet/fsharp/tools/fsharp-interactive/)
- [Fusabi Module System](./module_system.md)
- [RFC-001: Computation Expressions](./computation-expressions.md)
- [RFC-002: Async Computation Expressions](./RFC-002-ASYNC-CE.md)
