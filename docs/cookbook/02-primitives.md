# Primitives

Fusabi has a small set of built-in primitive types that form the foundation of all programs.

## Integers

Integers are whole numbers:

```fsharp
let x = 42
let negative = -17
let zero = 0
```

### Arithmetic Operations

```fsharp
// examples/arithmetic.fsx
let a = 10 in
let b = 5 in
let result = a * 2 + b - 3 in
result  // 17
```

Operators follow standard precedence: `*`, `/` before `+`, `-`.

```fsharp
let sum = 10 + 5       // 15
let diff = 10 - 3      // 7
let product = 4 * 5    // 20
let quotient = 20 / 4  // 5
let modulo = 17 % 5    // 2
```

## Floats

Floating-point numbers for decimal values:

```fsharp
let pi = 3.14159
let half = 0.5
let negative = -2.5
```

The `Math` module provides common operations:

```fsharp
let angle = Math.pi () / 4.0 in
let sine = Math.sin angle in
let cosine = Math.cos angle in
let root = Math.sqrt 2.0 in
let power = Math.pow 2.0 10.0  // 1024.0
```

## Booleans

Boolean values are `true` and `false`:

```fsharp
let isActive = true
let isDone = false
```

### Comparison Operators

```fsharp
let x = 10 in
let y = 20 in
let less = x < y        // true
let greater = x > y     // false
let equal = x = y       // false (note: single = for equality)
let notEqual = x <> y   // true
let lessEq = x <= y     // true
let greaterEq = x >= y  // false
```

### Boolean Operators

```fsharp
// examples/boolean_logic.fsx
let x = 10 in
let y = 20 in
let less = x < y in
let greater = x > y in
let and_result = less && greater in   // false (both must be true)
let or_result = greater || less in    // true (either can be true)
if and_result || or_result then 0 else 100
```

## Strings

Strings are double-quoted text:

```fsharp
let greeting = "Hello, World!"
let empty = ""
let withQuotes = "She said \"Hi\""
```

### String Operations

```fsharp
// Length
let len = String.length "hello"  // 5

// Case conversion
let upper = String.toUpper "hello"  // "HELLO"
let lower = String.toLower "WORLD"  // "world"

// Trimming whitespace
let clean = String.trim "  hello  "  // "hello"

// Splitting and joining
let words = String.split " " "hello world"  // ["hello"; "world"]
let joined = String.concat ["a"; "b"; "c"]  // "abc"

// Searching
let hasHello = String.contains "hello" "hello world"  // true
let starts = String.startsWith "hello" "hello world"  // true
let ends = String.endsWith "world" "hello world"      // true
```

## Unit

The unit type `()` represents "no meaningful value":

```fsharp
let nothing = ()
```

Functions that perform side effects typically return unit:

```fsharp
let logMessage msg =
    print msg
    print "\n"
    ()  // Returns unit
```

## Tuples

Tuples group multiple values of potentially different types:

```fsharp
// examples/tuples_basic.fsx
let pair = (1, 2)
let triple = (42, "hello", true)
let nested = ((1, 2), (3, 4))
```

Access tuple elements with `fst` and `snd` (for pairs) or pattern matching:

```fsharp
let point = (10, 20) in
let x = fst point  // 10
let y = snd point  // 20

// Or with pattern matching
match point with
| (x, y) -> x + y  // 30
```

## Type Summary

| Type | Examples | Description |
|------|----------|-------------|
| `int` | `42`, `-3`, `0` | Whole numbers |
| `float` | `3.14`, `-0.5` | Decimal numbers |
| `bool` | `true`, `false` | Boolean values |
| `string` | `"hello"` | Text |
| `unit` | `()` | No value |
| tuple | `(1, "a")` | Fixed-size grouping |

## Next Steps

Now that you understand the basic types, let's see how to [define and use functions](03-functions.md).
