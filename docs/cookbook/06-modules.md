# Modules

Modules organize code into logical namespaces. They group related functions, types, and values together.

## Defining Modules

Use the `module` keyword to create a module:

```fsharp
// examples/modules_basic.fsx
module Math =
    let add x y = x + y
    let multiply x y = x * y
    let square x = multiply x x
```

## Accessing Module Members

### Qualified Access

Use dot notation to access module members:

```fsharp
let sum = Math.add 10 20      // 30
let product = Math.multiply 3 4  // 12
let squared = Math.square 5   // 25
```

### Opening Modules

The `open` keyword imports all module members into scope:

```fsharp
open Math

// Now use functions without qualification
let result = square (add 3 4)  // square(7) = 49
```

### Mixed Access

You can use both qualified and unqualified access:

```fsharp
module Math =
    let add x y = x + y
    let multiply x y = x * y
    let square x = multiply x x

open Math

let result1 = square (add 3 4)   // Unqualified
let result2 = Math.add 10 20     // Qualified still works
```

## Nested Modules

Modules can contain other modules:

```fsharp
// examples/modules_nested.fsx
module Geometry =
    module Circle =
        let pi = 3.14159
        let area r = pi * r * r
        let circumference r = 2.0 * pi * r

    module Rectangle =
        let area w h = w * h
        let perimeter w h = 2 * (w + h)

// Access nested modules
let circleArea = Geometry.Circle.area 5.0
let rectArea = Geometry.Rectangle.area 3 4
```

## Module Values and Constants

Modules can contain values, not just functions:

```fsharp
module Constants =
    let pi = 3.14159
    let e = 2.71828
    let goldenRatio = 1.61803

let area = Constants.pi * radius * radius
```

## Standard Library Modules

Fusabi provides several built-in modules:

### List Module

```fsharp
List.length [1; 2; 3]           // 3
List.head [1; 2; 3]             // 1
List.tail [1; 2; 3]             // [2; 3]
List.reverse [1; 2; 3]          // [3; 2; 1]
List.map (fun x -> x * 2) [1; 2; 3]  // [2; 4; 6]
List.filter (fun x -> x > 1) [1; 2; 3]  // [2; 3]
```

### String Module

```fsharp
String.length "hello"           // 5
String.toUpper "hello"          // "HELLO"
String.toLower "WORLD"          // "world"
String.trim "  hi  "            // "hi"
String.split " " "a b c"        // ["a"; "b"; "c"]
String.concat ["a"; "b"; "c"]   // "abc"
```

### Array Module

```fsharp
Array.length [|1; 2; 3|]        // 3
Array.get 0 [|1; 2; 3|]         // 1
Array.create 5 0                // [|0; 0; 0; 0; 0|]
Array.init 3 (fun i -> i * 2)   // [|0; 2; 4|]
```

### Option Module

```fsharp
Option.isSome (Some 42)         // true
Option.isNone None              // true
Option.defaultValue 0 (Some 42) // 42
Option.defaultValue 0 None      // 0
Option.map (fun x -> x * 2) (Some 5)  // Some 10
```

### Map Module

```fsharp
let m = Map.empty ()
let m2 = Map.add "key" "value" m
Map.find "key" m2               // "value"
Map.tryFind "missing" m2        // None
Map.containsKey "key" m2        // true
Map.count m2                    // 1
```

## Module Organization Patterns

### Helper Module

```fsharp
module Helpers =
    let isEven x = x % 2 = 0
    let isOdd x = x % 2 <> 0
    let clamp min max x =
        if x < min then min
        else if x > max then max
        else x

open Helpers
let clamped = clamp 0 100 150  // 100
```

### Domain Module

```fsharp
module User =
    let create name email = { name = name; email = email }
    let updateEmail newEmail user = { user with email = newEmail }
    let displayName user = user.name

let user = User.create "Alice" "alice@example.com"
let name = User.displayName user  // "Alice"
```

### Utility Collection

```fsharp
module StringUtils =
    let isEmpty s = String.length s = 0
    let isBlank s = String.trim s |> isEmpty
    let truncate maxLen s =
        if String.length s <= maxLen then s
        else s  // Would need substring support

module ListUtils =
    let safeLast list =
        if List.isEmpty list then None
        else Some (list |> List.reverse |> List.head)
```

## Best Practices

1. **Group related functionality**: Keep functions that work together in the same module
2. **Use descriptive names**: Module names should indicate their purpose
3. **Prefer qualified access**: Makes code easier to understand at a glance
4. **Open sparingly**: Only open modules you use heavily

```fsharp
// Good: clear what each function does
let result = Math.add (Geometry.Circle.area 5.0) 10.0

// Less clear after opening many modules
open Math
open Geometry.Circle
let result = add (area 5.0) 10.0
```

## Next Steps

Learn about [async computation expressions](07-async.md) for handling asynchronous operations.
