# Fusabi by Example

Welcome to **Fusabi by Example**, a collection of runnable examples that illustrate various Fusabi concepts and standard library features.

## What is Fusabi?

Fusabi is a Mini-F# dialect with a Rust VM, designed for embedded scripting in Rust host applications. It features:

- **F#-style syntax**: `let` bindings, pattern matching, pipelines, modules
- **Functional-first**: Immutable data, higher-order functions, currying
- **Expression-oriented**: Everything is an expression that returns a value
- **Async support**: Computation expressions for async workflows
- **Rich standard library**: List, String, Array, Map, Option, and more

## Running Examples

All examples in this cookbook can be run with the Fusabi CLI:

```bash
fusabi run examples/hello.fsx
```

Or try them interactively in the REPL:

```bash
fusabi repl
```

## Table of Contents

1. [Hello World](01-hello-world.md) - Your first Fusabi program
2. [Primitives](02-primitives.md) - Types, values, and operators
3. [Functions](03-functions.md) - Functions, lambdas, and currying
4. [Control Flow](04-control-flow.md) - Conditionals and pattern matching
5. [Collections](05-collections.md) - Lists, arrays, and maps
6. [Modules](06-modules.md) - Organizing code with modules
7. [Async](07-async.md) - Async computation expressions
8. [IO](08-io.md) - File and terminal operations

## Philosophy

Fusabi embraces the principle of **expression-oriented programming**. Unlike imperative languages where statements perform actions, in Fusabi every construct is an expression that evaluates to a value:

```fsharp
// if-else is an expression
let status = if x > 0 then "positive" else "non-positive"

// match is an expression
let description = match n with
    | 0 -> "zero"
    | 1 -> "one"
    | _ -> "many"

// let bindings chain with 'in'
let result =
    let a = 10 in
    let b = 20 in
    a + b
```

This makes code more composable and easier to reason about.

## Getting Help

- [Language Specification](../02-language-spec.md) - Complete syntax reference
- [Standard Library Reference](../STDLIB_REFERENCE.md) - All built-in functions
- [GitHub Issues](https://github.com/fusabi-lang/fusabi/issues) - Report bugs or request features
