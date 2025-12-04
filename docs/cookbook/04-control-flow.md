# Control Flow

Fusabi provides two main control flow mechanisms: `if-then-else` expressions and `match` expressions. Both are expressions that return values.

## If-Then-Else

The `if` expression evaluates a condition and returns one of two values:

```fsharp
// examples/conditionals.fsx
let x = 42 in
let y = 17 in
let max = if x > y then x else y in
max  // 42
```

### Chained Conditions

```fsharp
let describe n =
    if n < 0 then "negative"
    else if n = 0 then "zero"
    else "positive"

describe (-5)  // "negative"
describe 0     // "zero"
describe 10    // "positive"
```

### If as Expression

Since `if` is an expression, you can use it anywhere a value is expected:

```fsharp
let status = if isActive then "running" else "stopped"

let value = 10 + (if condition then 5 else 0)

let message = "Count: " + (if n = 1 then "one" else "many")
```

## Pattern Matching

Pattern matching is more powerful than `if-else`, allowing you to match on structure and bind variables:

```fsharp
// examples/pattern_matching_basic.fsx
let describe n =
    match n with
    | 0 -> "zero"
    | 1 -> "one"
    | 2 -> "two"
    | _ -> "many"

describe 0  // "zero"
describe 5  // "many"
```

### Literal Patterns

Match exact values:

```fsharp
// Integer patterns
match x with
| 0 -> "none"
| 1 -> "one"
| 42 -> "the answer"
| _ -> "other"

// Boolean patterns
let bool_to_int b =
    match b with
    | true -> 1
    | false -> 0

// String patterns
let greet lang =
    match lang with
    | "en" -> "Hello"
    | "es" -> "Hola"
    | "fr" -> "Bonjour"
    | _ -> "Hi"
```

### Variable Patterns

Bind matched values to names:

```fsharp
let process value =
    match value with
    | 0 -> "zero"
    | n -> "got: " + n  // 'n' is bound to the value
```

### Wildcard Pattern

The underscore `_` matches anything without binding:

```fsharp
let is_zero x =
    match x with
    | 0 -> true
    | _ -> false  // Catch-all
```

### Tuple Patterns

Destructure tuples in patterns:

```fsharp
// examples/pattern_matching_tuples.fsx
let classify_point p =
    match p with
    | (0, 0) -> "origin"
    | (0, y) -> "on y-axis"
    | (x, 0) -> "on x-axis"
    | (x, y) -> "in quadrant"

classify_point (0, 0)  // "origin"
classify_point (3, 0)  // "on x-axis"
classify_point (3, 4)  // "in quadrant"
```

### Nested Patterns

Patterns can be nested for complex matching:

```fsharp
// examples/pattern_matching_nested.fsx
let describe_nested t =
    match t with
    | (0, (0, 0)) -> "zero with origin"
    | (x, (0, z)) -> "y is zero"
    | (x, (y, z)) -> "general case"
```

### Pattern Matching in Functions

Use `match` to define functions that behave differently based on input:

```fsharp
// examples/pattern_matching_functions.fsx
let rec length list =
    match list with
    | [] -> 0
    | head :: tail -> 1 + length tail

length [1; 2; 3; 4]  // 4
```

## Combining Patterns

### Calculate with Destructuring

```fsharp
let add_pair p =
    match p with
    | (a, b) -> a + b

add_pair (3, 4)  // 7

let distance_squared p =
    match p with
    | (x, y) -> x * x + y * y

distance_squared (3, 4)  // 25
```

### Transform Data

```fsharp
let swap p =
    match p with
    | (x, y) -> (y, x)

swap (1, 2)  // (2, 1)
```

## Pattern Matching vs If-Else

Pattern matching is often clearer than nested if-else:

```fsharp
// Verbose if-else
let describe n =
    if n = 0 then "zero"
    else if n = 1 then "one"
    else if n = 2 then "two"
    else "many"

// Cleaner with match
let describe n =
    match n with
    | 0 -> "zero"
    | 1 -> "one"
    | 2 -> "two"
    | _ -> "many"
```

## Option Matching

Match on `Option` values safely:

```fsharp
let describe_option opt =
    match opt with
    | Some x -> "has value: " + x
    | None -> "empty"

describe_option (Some 42)  // "has value: 42"
describe_option None       // "empty"
```

### Safe Value Access

```fsharp
let get_or_default opt default =
    match opt with
    | Some x -> x
    | None -> default

// Or use the built-in
Option.defaultValue 0 (Some 42)  // 42
Option.defaultValue 0 None       // 0
```

## Result Matching

Handle success and error cases:

```fsharp
let handle_result r =
    match r with
    | Ok value -> "success: " + value
    | Error msg -> "failed: " + msg
```

## Common Patterns

### Exhaustive Matching

Always handle all cases (use `_` for catch-all):

```fsharp
let safe_classify n =
    match n with
    | 0 -> "zero"
    | 1 -> "one"
    | _ -> "other"  // Handles all remaining cases
```

### Guard-like Behavior

Until guards are added, use nested if:

```fsharp
let classify_age age =
    match age with
    | 0 -> "newborn"
    | n -> if n < 13 then "child"
           else if n < 20 then "teen"
           else "adult"
```

## Next Steps

Now let's explore [collections](05-collections.md) - lists, arrays, and maps.
