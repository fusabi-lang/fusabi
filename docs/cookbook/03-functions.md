# Functions

Functions are the core building blocks of Fusabi programs. All functions are first-class values that can be passed around, returned, and partially applied.

## Defining Functions

Use `let` to define named functions:

```fsharp
let add x y = x + y

let greet name = "Hello, " + name + "!"

let square x = x * x
```

## Calling Functions

Function application uses juxtaposition (no parentheses needed):

```fsharp
let result = add 3 4      // 7
let message = greet "Bob" // "Hello, Bob!"
let n = square 5          // 25
```

Use parentheses for nested calls or to group expressions:

```fsharp
let result = add (square 3) (square 4)  // 9 + 16 = 25
```

## Multi-Parameter Functions

Functions with multiple parameters are automatically curried:

```fsharp
// examples/currying_simple.fsx
let add x y = x + y

// Partial application creates specialized functions
let add10 = add 10
let add20 = add 20

// Use the partially applied functions
let result1 = add10 5   // 15
let result2 = add20 5   // 25
```

### Three or More Parameters

```fsharp
let addThree x y z = x + y + z

// Multiple levels of partial application
let add10 = addThree 10
let add10_20 = add10 20

add10_20 5  // 35
```

## Anonymous Functions (Lambdas)

Use `fun` for inline anonymous functions:

```fsharp
let double = fun x -> x * 2

// Commonly used with higher-order functions
let doubled = List.map (fun x -> x * 2) [1; 2; 3]  // [2; 4; 6]

let evens = List.filter (fun x -> x % 2 = 0) [1; 2; 3; 4]  // [2; 4]
```

## Recursive Functions

Use `let rec` for recursive definitions:

```fsharp
// examples/fibonacci.fsx
let rec fib n =
    if n <= 1 then n
    else fib (n - 1) + fib (n - 2)

fib 10  // 55
```

### Tail Recursion

For better performance, use tail-recursive style with an accumulator:

```fsharp
let rec factorial_acc n acc =
    if n <= 1 then acc
    else factorial_acc (n - 1) (n * acc)

let factorial n = factorial_acc n 1

factorial 5  // 120
```

## Higher-Order Functions

Functions can take other functions as arguments:

```fsharp
// examples/currying_higher_order.fsx
let apply_twice f x = f (f x)

let double x = x * 2
let result = apply_twice double 3  // double (double 3) = 12
```

Functions can also return functions:

```fsharp
let make_adder n =
    fun x -> x + n

let add5 = make_adder 5
add5 10  // 15
```

## Pipeline Operator

The `|>` operator passes a value to a function, enabling readable data flows:

```fsharp
// examples/pipeline_demo.fsx
let result = 5
    |> double
    |> addTen
    |> double  // 40

// Instead of: double (addTen (double 5))
```

### String Processing Pipeline

```fsharp
let cleaned = "  HELLO FUSABI WORLD  "
    |> String.trim
    |> String.toLower
    |> String.split " "
    |> List.length  // 3
```

### List Processing Pipeline

```fsharp
let result = [1; 2; 3; 4; 5]
    |> List.reverse
    |> List.tail
    |> List.head  // 4
```

## Function Composition

Compose functions with `>>` (forward) and `<<` (backward):

```fsharp
let double x = x * 2
let addOne x = x + 1

// Forward composition: apply double first, then addOne
let doubleAndAddOne = double >> addOne
doubleAndAddOne 5  // 11

// Backward composition: apply addOne first, then double
let addOneAndDouble = double << addOne
addOneAndDouble 5  // 12
```

## Local Functions

Define helper functions inside other functions:

```fsharp
let processData data =
    let helper x = x * 2 in
    let transform x = helper x + 1 in
    List.map transform data

processData [1; 2; 3]  // [3; 5; 7]
```

## Common Patterns

### Identity Function

```fsharp
let id x = x  // Built-in as 'id'
```

### Ignore Result

```fsharp
let ignored = ignore (some_side_effect ())  // Returns unit
```

### Constant Function

```fsharp
let always x = fun _ -> x
let alwaysZero = always 0
alwaysZero "anything"  // 0
```

## Next Steps

Now that you can define functions, let's explore [control flow](04-control-flow.md) with conditionals and pattern matching.
