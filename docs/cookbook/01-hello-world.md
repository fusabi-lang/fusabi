# Hello World

Every programming journey starts with Hello World. In Fusabi, it's remarkably simple.

## The Simplest Program

```fsharp
// examples/hello.fsx
"Hello, Fusabi!"
```

In Fusabi, any expression at the end of a script becomes its result. Strings are valid expressions, so this one-liner is a complete program.

## Using Print

For explicit output, use the `print` function:

```fsharp
print "Hello, World!"
print "\n"
```

Or use `printfn` for automatic newlines:

```fsharp
printfn "Hello, World!"
```

## String Concatenation

Combine strings with the `+` operator:

```fsharp
let name = "Fusabi" in
let greeting = "Hello, " + name + "!" in
print greeting
```

## String Formatting

For more complex formatting, use `String.concat`:

```fsharp
let name = "Alice" in
let age = 30 in
let message = String.concat ["Hello, "; name; "! You are "; "30"; " years old."] in
print message
```

## Comments

Fusabi supports single-line comments:

```fsharp
// This is a comment
let x = 42  // Comments can appear at end of lines
```

## Running the Example

Save your code to a `.fsx` file and run:

```bash
fusabi run hello.fsx
```

## Key Takeaways

- Fusabi scripts are expressions; the last expression is the result
- Use `print` for output, `printfn` for output with newline
- Strings use double quotes: `"Hello"`
- Comments start with `//`
- String concatenation uses `+` or `String.concat`

## Next Steps

Now that you've written your first Fusabi program, let's explore the [primitive types](02-primitives.md) available in the language.
