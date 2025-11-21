# The Omakase 🍣

**"I'll leave it up to you."**

Welcome to the Omakase - chef-selected Fusabi examples that showcase what this language does best: embedding functional scripting into Rust applications.

These aren't random code snippets. They're carefully curated patterns that demonstrate real-world use cases.

## Philosophy

*Omakase* (お任せ) means "I'll leave it up to you" in Japanese. At a sushi bar, it's when you trust the chef to select the best dishes. Here, trust us to show you the best of Fusabi.

Small bites. Big flavor. Zero filler.

## The Menu

### 🍵 Appetizers (Simple One-Liners)

Quick bites to taste Fusabi's syntax:

#### String Manipulation
```fsharp
// examples/appetizers/string_ops.fsx
"Hello, Fusabi!"
|> String.toUpper
|> String.split ","
|> List.map String.trim
// ["HELLO", "FUSABI!"]
```

**What it shows**: Pipeline operator, standard library, functional composition

#### Math Operations
```fsharp
// examples/appetizers/math_demo.fsx
[1..100]
|> List.filter (fun x -> x % 15 = 0)
|> List.sum
// 315 (sum of numbers divisible by 15)
```

**What it shows**: List comprehensions, lambdas, higher-order functions

#### Regex Matching
```fsharp
// examples/appetizers/regex.fsx
Regex.matches @"\b[A-Z]\w+" "Hello World From Fusabi"
// ["Hello", "World", "From", "Fusabi"]
```

**What it shows**: Built-in regex, verbatim strings, pattern matching

#### Type Inference
```fsharp
// examples/appetizers/types.fsx
let add x y = x + y        // int -> int -> int (inferred)
let concat s1 s2 = s1 + s2 // string -> string -> string (inferred)
let map f xs = List.map f xs // ('a -> 'b) -> 'a list -> 'b list (generic)
```

**What it shows**: Hindley-Milner type inference, polymorphism

---

### 🍱 Main Courses (Full Applications)

Complete integrations with popular Rust frameworks:

#### 1. Bevy Game Scripting
**Path**: `examples/main_courses/bevy_scripting/`

**Use Case**: Hot-reload entity behaviors without recompiling your game.

```rust
// Rust: Load and run behavior scripts
let engine = fusabi::Engine::new();
for entity in entities.iter() {
    let script = fs::read_to_string(&entity.behavior_path)?;
    let result = engine.eval(&script)?;
    entity.apply_behavior(result);
}
```

```fsharp
// F#: behavior.fsx - Define entity movement
let speed = time * 2.0
let radius = 5.0
let x = radius * cos speed
let y = radius * sin speed
(x, y) // Return new position
```

**What it shows**: Game loop scripting, hot-reload workflows, tuple returns

#### 2. Ratatui Terminal Layout
**Path**: `examples/main_courses/ratatui_layout/`

**Use Case**: Define TUI layouts in functional style.

```fsharp
// layout.fsx - Declarative terminal UI
{ direction = Vertical
  constraints = [
    Percentage 20  // Header
    Min 10         // Content
    Length 3       // Footer
  ]
  widgets = [
    Text "Fusabi Dashboard"
    List items
    Text "Press 'q' to quit"
  ]
}
```

**What it shows**: Record syntax, structured configuration, UI composition

#### 3. Axum Web Server Validation
**Path**: `examples/main_courses/web_server/`

**Use Case**: Business logic validation without rebuilding the server.

```rust
// Rust: Register validation handler
let validator = fusabi::Engine::new()
    .load_script("validation.fsx")?;

app.post("/users", |req| {
    let result = validator.call("validate_user", &req)?;
    match result {
        Ok(user) => Json(user),
        Err(msg) => StatusCode::BAD_REQUEST,
    }
});
```

```fsharp
// F#: validation.fsx - Define validation rules
let validate_user user =
    if user.age < 18 then
        Error "Must be 18 or older"
    else if not (user.email |> String.contains "@") then
        Error "Invalid email format"
    else if String.length user.name < 2 then
        Error "Name too short"
    else
        Ok user
```

**What it shows**: Result type, if-then-else chains, string operations

#### 4. Burn Neural Net Configuration
**Path**: `examples/main_courses/burn_config/`

**Use Case**: Define model architectures with type safety.

```fsharp
// model.fsx - Typed neural network config
{ layers = [
    Linear { input = 784; output = 128 }
    ReLU
    Dropout { rate = 0.2 }
    Linear { input = 128; output = 10 }
  ]
  optimizer = Adam {
    learningRate = 0.001
    beta1 = 0.9
    beta2 = 0.999
  }
  epochs = 50
  batchSize = 32
}
```

**What it shows**: Discriminated unions, nested records, hyperparameter tuning

---

### 🔥 Fusion (Advanced Rust Interop)

Mixing Fusabi and Rust at a deep level:

#### 1. Host Function Callbacks
**Path**: `examples/fusion/host_callbacks/`

**What it shows**: Calling Rust from F#, calling F# from Rust, higher-order functions across the FFI boundary.

```rust
// Rust: Register host function
engine.register_function("http_get", |url: String| {
    reqwest::blocking::get(&url)?.text()
});
```

```fsharp
// F#: Use host function from script
let urls = ["https://api.github.com"; "https://rust-lang.org"]
let responses = urls |> List.map http_get
responses |> List.iter (printfn "%s")
```

#### 2. .NET Compatibility Layer
**Path**: `examples/fusion/interop_net/`

**What it shows**: Same script runs on Fusabi VM and .NET CLR - proof of syntax compatibility.

```fsharp
// shared.fsx - Compatible with both runtimes
module Math =
    let rec fib n =
        if n <= 1 then n
        else fib (n - 1) + fib (n - 2)

let result = Math.fib 10
printfn "Fibonacci(10) = %d" result
```

```bash
# Run on Fusabi
fus run shared.fsx

# Run on .NET
dotnet fsi shared.fsx

# Both produce: Fibonacci(10) = 55
```

#### 3. Computation Expressions
**Path**: `examples/fusion/computations/`

**What it shows**: Custom DSLs with builder patterns, monadic workflows in embedded scripting.

```fsharp
// query.fsx - Computation expression for SQL-like queries
query {
    for user in users do
    where (user.age > 18)
    select user.name
}
// Generates: SELECT name FROM users WHERE age > 18
```

---

## Serving Suggestions

Each example includes:
- 📖 **README**: Explanation of the pattern and use case
- ✅ **Working Code**: All examples compile and run
- 🎯 **Clear Goal**: One concept demonstrated per example
- 🧪 **Tests**: Validation that it works as expected

### Running Examples

```bash
# Run a single example
fus run examples/appetizers/string_ops.fsx

# Compile to bytecode
fus grind examples/appetizers/math_demo.fsx
# Output: math_demo.fzb

# Run bytecode
fus exec math_demo.fzb
```

### Testing Examples

```bash
# Run tests for all examples
cargo test --package fusabi-examples

# Run tests for specific category
cargo test --package fusabi-examples appetizers
```

## Example Structure

```
examples/
├── appetizers/           # 1-10 line snippets
│   ├── string_ops.fsx
│   ├── math_demo.fsx
│   ├── regex.fsx
│   └── types.fsx
├── main_courses/         # Full integrations
│   ├── bevy_scripting/
│   │   ├── src/main.rs   # Rust host
│   │   ├── behavior.fsx  # F# script
│   │   └── README.md
│   ├── ratatui_layout/
│   ├── web_server/
│   └── burn_config/
└── fusion/               # Advanced interop
    ├── host_callbacks/
    ├── interop_net/
    └── computations/
```

## Categories Explained

### 🍵 Appetizers
**Purpose**: Learn Fusabi syntax quickly
**Time**: 1-5 minutes per example
**Complexity**: Beginner-friendly
**Goal**: Copy-paste and experiment

### 🍱 Main Courses
**Purpose**: See real-world integration patterns
**Time**: 15-30 minutes per example
**Complexity**: Intermediate (requires Rust knowledge)
**Goal**: Template for your own projects

### 🔥 Fusion
**Purpose**: Advanced techniques and edge cases
**Time**: 30-60 minutes per example
**Complexity**: Advanced (deep FFI knowledge)
**Goal**: Push the boundaries of what's possible

## Contribution Guide

Have a spicy pattern to share? Here's how to add it to the Omakase:

1. **Choose a category**: Appetizer, Main Course, or Fusion
2. **Create a directory**: `examples/<category>/<your_example>/`
3. **Include files**:
   - `README.md` - Explain the pattern and use case
   - Working code (`.fsx` for scripts, `src/` for Rust host)
   - Tests if applicable
4. **Follow the style**:
   - Punchy, minimal explanations
   - Self-documenting code
   - One clear concept per example
5. **Submit a PR**: Tag it with `examples` label

### Example Checklist

- [ ] Code compiles and runs without errors
- [ ] README explains *why*, not just *what*
- [ ] Follows Fusabi style guide (see `docs/BRANDING.md`)
- [ ] Includes usage instructions
- [ ] Tests pass (if applicable)
- [ ] No external dependencies unless necessary
- [ ] Meaningful example, not just "hello world"

## Learn More

- **Language Reference**: `docs/language-reference.md`
- **Embedding Guide**: `docs/embedding.md`
- **API Documentation**: https://docs.rs/fusabi
- **Source Code**: https://github.com/fusabi-lang/fusabi

## Support

Questions about an example? Open a discussion:
https://github.com/fusabi-lang/fusabi/discussions

Found a bug in an example? Open an issue:
https://github.com/fusabi-lang/fusabi/issues

---

**Fusabi**: Small. Potent. Functional. 🟢

*Enjoy the Omakase.*
