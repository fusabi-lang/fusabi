# Collections

Fusabi provides three main collection types: Lists (immutable, linked), Arrays (mutable, indexed), and Maps (key-value dictionaries).

## Lists

Lists are immutable, singly-linked sequences. They're the primary collection type in Fusabi.

### Creating Lists

```fsharp
// examples/lists_basic.fsx
let empty = []
let single = [42]
let numbers = [1; 2; 3; 4; 5]
let strings = ["apple"; "banana"; "cherry"]
```

### The Cons Operator (::)

Build lists by prepending elements:

```fsharp
// Build a list
let list1 = 1 :: 2 :: 3 :: []  // [1; 2; 3]

// Prepend to existing list
let numbers = [2; 3; 4] in
let extended = 1 :: numbers    // [1; 2; 3; 4]
```

### List Operations

```fsharp
// examples/stdlib_demo.fsx
let numbers = [1; 2; 3; 4; 5] in

// Basic operations
let count = List.length numbers     // 5
let first = List.head numbers       // 1
let rest = List.tail numbers        // [2; 3; 4; 5]
let backwards = List.reverse numbers // [5; 4; 3; 2; 1]
let isEmpty = List.isEmpty []       // true

// Combining lists
let more = [6; 7; 8] in
let combined = List.append numbers more  // [1; 2; 3; 4; 5; 6; 7; 8]

// Flatten nested lists
let nested = [[1; 2]; [3; 4]; [5]] in
let flat = List.concat nested  // [1; 2; 3; 4; 5]
```

### Transforming Lists

```fsharp
// Map: transform each element
let doubled = List.map (fun x -> x * 2) [1; 2; 3]  // [2; 4; 6]

// Filter: keep elements matching predicate
let evens = List.filter (fun x -> x % 2 = 0) [1; 2; 3; 4]  // [2; 4]

// Exists: check if any element matches
let hasLarge = List.exists (fun x -> x > 3) [1; 2; 3; 4; 5]  // true

// Find: get first matching element
let firstLarge = List.find (fun x -> x > 3) [1; 2; 3; 4; 5]  // 4

// TryFind: safely find (returns Option)
let maybe = List.tryFind (fun x -> x > 10) [1; 2; 3]  // None
```

### Recursive List Processing

```fsharp
// examples/lists_recursive.fsx
let rec sum list =
    match list with
    | [] -> 0
    | head :: tail -> head + sum tail

sum [1; 2; 3; 4; 5]  // 15

let rec map f list =
    match list with
    | [] -> []
    | head :: tail -> f head :: map f tail
```

## Arrays

Arrays are mutable, indexed collections with O(1) access.

### Creating Arrays

```fsharp
// examples/arrays_basic.fsx
let empty = [||]
let numbers = [|1; 2; 3; 4; 5|]
let matrix = [|[|1; 2|]; [|3; 4|]|]  // Nested arrays
```

### Array Indexing

```fsharp
let arr = [|10; 20; 30; 40; 50|] in
let first = arr.[0]   // 10
let third = arr.[2]   // 30

// Nested indexing
let matrix = [|[|1; 2|]; [|3; 4|]|] in
let element = matrix.[1].[0]  // 3
```

### Array Updates

Updates are immutable - they return new arrays:

```fsharp
// examples/arrays_updates.fsx
let arr = [|1; 2; 3; 4; 5|] in
let updated = arr.[1] <- 99 in
// arr is still [|1; 2; 3; 4; 5|]
// updated is [|1; 99; 3; 4; 5|]
```

### Array Operations

```fsharp
let arr = Array.ofList [1; 2; 3; 4; 5] in

// Basic operations
let len = Array.length arr      // 5
let empty = Array.isEmpty arr   // false
let elem = Array.get 2 arr      // 3

// Create arrays
let zeros = Array.create 5 0    // [|0; 0; 0; 0; 0|]
let squares = Array.init 5 (fun i -> i * i)  // [|0; 1; 4; 9; 16|]

// Convert back to list
let asList = Array.toList arr   // [1; 2; 3; 4; 5]
```

## Maps

Maps are immutable dictionaries for key-value storage.

### Creating and Using Maps

```fsharp
// examples/stdlib_demo.fsx
let emptyMap = Map.empty () in

// Add entries
let step1 = Map.add "city" "NYC" emptyMap in
let myMap = Map.add "name" "Alice" step1 in

// Find values
let name = Map.find "name" myMap  // "Alice"

// Safe find (returns Option)
let maybeName = Map.tryFind "age" myMap  // None

// Check existence
let hasName = Map.containsKey "name" myMap  // true

// Get size
let size = Map.count myMap  // 2
```

### Transforming Maps

```fsharp
// Transform all values
let upperMap = Map.map String.toUpper myMap
```

## Pipelines with Collections

The pipeline operator makes collection processing readable:

```fsharp
// examples/pipeline_demo.fsx
let result = [1; 2; 3; 4; 5]
    |> List.filter (fun x -> x > 2)
    |> List.map (fun x -> x * 2)
    // [6; 8; 10]

// String processing
let wordCount = "  HELLO FUSABI WORLD  "
    |> String.trim
    |> String.toLower
    |> String.split " "
    |> List.length  // 3
```

## Records

Records are named tuples with field access:

```fsharp
// examples/records_basic.fsx
let person = { name = "Alice"; age = 30; active = true }

// Field access
let name = person.name  // "Alice"
let age = person.age    // 30

// Record update (creates new record)
let older = { person with age = 31 }

// Nested records
let address = { street = "123 Main"; city = "Boston" }
let personWithAddr = { name = "Bob"; address = address }
let city = personWithAddr.address.city  // "Boston"
```

## Common Patterns

### Safe Head with Option

```fsharp
let safeHead list =
    if List.isEmpty list then None
    else Some (List.head list)
```

### Building Lookup Tables

```fsharp
let codes = Map.empty ()
    |> Map.add "US" "United States"
    |> Map.add "UK" "United Kingdom"
    |> Map.add "DE" "Germany"

let country = Map.find "US" codes  // "United States"
```

### Processing CSV-like Data

```fsharp
let csvLine = "Alice,30,Engineer" in
let fields = String.split "," csvLine in
let name = List.head fields  // "Alice"
```

## Next Steps

Learn how to organize your code with [modules](06-modules.md).
