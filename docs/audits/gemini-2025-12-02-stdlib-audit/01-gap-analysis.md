# Fusabi vs F# Core: Gap Analysis

**Date**: 2025-12-02
**Auditor**: Gemini Agent

## Executive Summary
The Fusabi standard library has established a solid foundation with `List`, `String`, `Option`, and `Map` modules. However, it significantly lags behind F# Core in breadth. The most critical omission is the entire `Array` module, despite `[| ... |]` syntax and `Value::Array` support in the VM. Additionally, while Higher-Order Function (HOF) support exists in the VM, it is underutilized in the standard library (missing `filter`, `fold`, etc.).

## Module-by-Module Analysis

### 1. List Module
**Status**: Partial Implementation
**Existing**: `length`, `head`, `tail`, `reverse`, `isEmpty`, `append`, `concat`, `map`.

**Missing High-Priority (F# Core Parity)**:
- `List.filter` : `('a -> bool) -> 'a list -> 'a list`
- `List.fold` : `('a -> 'b -> 'a) -> 'a -> 'b list -> 'a`
- `List.foldBack` : `('a -> 'b -> 'b) -> 'a list -> 'b -> 'b`
- `List.iter` : `('a -> unit) -> 'a list -> unit`
- `List.exists` : `('a -> bool) -> 'a list -> bool`
- `List.forall` : `('a -> bool) -> 'a list -> bool`
- `List.contains` : `'a -> 'a list -> bool`
- `List.find` : `('a -> bool) -> 'a list -> 'a`
- `List.tryFind` : `('a -> bool) -> 'a list -> 'a option`
- `List.sort` : `'a list -> 'a list`
- `List.sortBy` : `('a -> 'b) -> 'a list -> 'a list`
- `List.collect` : `('a -> 'b list) -> 'a list -> 'b list`
- `List.init` : `int -> (int -> 'a) -> 'a list`

### 2. String Module
**Status**: Good Basic Coverage
**Existing**: `length`, `trim`, `toLower`, `toUpper`, `split`, `concat`, `contains`, `startsWith`, `endsWith`, `format` (and `sprintf`).

**Missing High-Priority**:
- `String.collect` : `(char -> string) -> string -> string`
- `String.init` : `int -> (int -> string) -> string`
- `String.replicate` : `int -> string -> string`
- `String.concat` (overload with separator): Currently `String.concat` takes a list, F# `String.concat` takes `sep` and `list`. Fusabi's `concat` is effectively `String.Concat` (Join "" list). Need `String.join` or update `concat` to match F# `String.concat sep list`.

### 3. Option Module
**Status**: Partial Implementation
**Existing**: `isSome`, `isNone`, `defaultValue`, `defaultWith`, `map`, `bind`, `iter`, `map2`, `orElse`.
**Constructors**: `Some`, `None` (Global).

**Missing High-Priority**:
- `Option.count` : `'a option -> int`
- `Option.fold` : `('a -> 'b -> 'a) -> 'a -> 'b option -> 'a`
- `Option.exists` : `('a -> bool) -> 'a option -> bool`
- `Option.forall` : `('a -> bool) -> 'a option -> bool`
- `Option.filter` : `('a -> bool) -> 'a option -> 'a option`
- `Option.toArray` : `'a option -> 'a array`
- `Option.toList` : `'a option -> 'a list`

### 4. Array Module
**Status**: ❌ MISSING
**Observation**: The `Value::Array` variant exists, and the language spec details array syntax (`[| |]`), indexing (`.[i]`), and mutation (`<-`). However, `stdlib/mod.rs` does **not** register an `Array` module or any array functions.

**Required Implementation**:
- `Array.length`
- `Array.isEmpty`
- `Array.get` (likely handled by `.[i]` op, but function useful for pipelines)
- `Array.set` (likely handled by `<-` op)
- `Array.map`
- `Array.map2`
- `Array.iter`
- `Array.iter2`
- `Array.filter`
- `Array.fold`
- `Array.create`
- `Array.init`
- `Array.ofList`
- `Array.toList`
- `Array.append`
- `Array.concat` (flatten)

### 5. Map Module
**Status**: Good Basic Coverage
**Existing**: `empty`, `add`, `remove`, `find`, `tryFind`, `containsKey`, `isEmpty`, `count`, `ofList`, `toList`.

**Missing**:
- `Map.iter`
- `Map.map`
- `Map.fold`
- `Map.filter`
- `Map.exists`
- `Map.forall`

### 6. Core / Global Namespace
**Status**: Unknown/Implicit
**Missing from Registration**:
- `print` / `printfn`: Referenced in docs but not found in `stdlib/mod.rs`. Likely registered in `main.rs` or a different layer (e.g., `cli`). **Action**: Move to `stdlib` or `prelude` to ensure availability in all embedding contexts.
- `failwith` / `invalidOp`: Standard F# error helpers.
- `int`, `float`, `string` conversion functions (e.g. `int "42"`).

## Recommendation
The priority should be:
1.  **Array Module**: Implement basic Array support to match language syntax.
2.  **List HOFs**: Complete `fold`, `filter`, `iter`.
3.  **Core Globals**: Ensure `print`, `failwith` are standard.
