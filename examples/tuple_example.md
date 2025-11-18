# Tuple Examples for FSRS

This file demonstrates the tuple functionality in FSRS (F#-to-Rust Script Engine).

## Basic Tuple Creation

```fsharp
// Empty tuple
()

// Pair
(1, 2)

// Triple
(1, 2, 3)

// Mixed types
(42, "hello", true)
```

## Tuples in Let Bindings

```fsharp
// Store a tuple in a variable
let pair = (1, 2) in pair
// Result: (1, 2)

// Multiple variables creating a tuple
let x = 10 in
let y = 20 in
(x, y)
// Result: (10, 20)
```

## Tuples with Expressions

```fsharp
// Tuple of computed values
(1 + 2, 3 * 4)
// Result: (3, 12)

// Complex expression
let x = 10 in
let y = 20 in
(x + y, x * y)
// Result: (30, 200)
```

## Nested Tuples

```fsharp
// Simple nesting
(1, (2, 3))
// Result: (1, (2, 3))

// Deep nesting
((1, 2), (3, 4))
// Result: ((1, 2), (3, 4))
```

## Tuple Comparisons

```fsharp
// Equality
(1, 2) == (1, 2)
// Result: true

// Inequality
(1, 2) != (1, 3)
// Result: true
```

## Tuples in Conditionals

```fsharp
// Conditional returning tuples
if true then (1, 2) else (3, 4)
// Result: (1, 2)
```

## Implementation Details

### Runtime Support (Layer 3)

The tuple implementation includes:

1. **Value Type**: `Value::Tuple(Vec<Value>)` - Runtime representation
2. **Instructions**:
   - `MakeTuple(u16)` - Create tuple from N stack values
   - `GetTupleField(u8)` - Extract field by index (for future use)
3. **Compiler**: Emits `MakeTuple` after compiling all elements
4. **VM Execution**:
   - Pops N values from stack
   - Creates tuple maintaining left-to-right order
   - Pushes tuple back to stack

### Features

- **Element-wise equality**: Tuples compare elements recursively
- **Heterogeneous types**: Mix integers, booleans, strings, and nested tuples
- **Arbitrary size**: Up to 65,535 elements (u16::MAX)
- **Display format**: Pretty-prints as `(v1, v2, ...)`
- **Truthiness**: Empty tuples are falsy, non-empty are truthy

### Test Coverage

- 141 VM tests (including 13 tuple tests)
- 210 frontend tests (including 12 tuple compiler tests)
- 13 integration tests
- Total: 477 tests passing

## Future Enhancements (Layer 4+)

- Tuple pattern matching
- Tuple destructuring in let bindings
- First-class tuple field access via `GetTupleField`
- Type-safe tuple indexing
