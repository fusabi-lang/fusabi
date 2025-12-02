# Fusabi Standard Library Roadmap (v2)

**Date**: 2025-12-02
**Status**: Pending Implementation

## Phase 1: The Array Foundation (CRITICAL)
The language has syntax for arrays (`[| |]`) but no standard library support. This is the highest priority gap.

### 1.1 Create `stdlib/array.rs`
Implement the following native functions. Use `Value::Array` (which is `Arc<Mutex<Vec<Value>>>`) for storage.
- `Array.length : 'a array -> int`
- `Array.isEmpty : 'a array -> bool`
- `Array.get : int -> 'a array -> 'a` (Safe indexing, throws if OOB)
- `Array.set : int -> 'a -> 'a array -> unit` (Mutable update in place? Or copy? *Note: Language spec says `<-` creates new array, but `set` func usually implies mutation. Stick to F# semantics: `Array.set` mutates.*)
- `Array.ofList : 'a list -> 'a array`
- `Array.toList : 'a array -> 'a list`
- `Array.init : int -> (int -> 'a) -> 'a array` (Requires `vm.call_value`)
- `Array.create : int -> 'a -> 'a array`

### 1.2 Register Array Module
Update `stdlib/mod.rs`:
- Add `pub mod array;`
- In `register_stdlib`, add `Array` record to `vm.globals`.
- Register all functions in `HostRegistry`.

## Phase 2: Higher-Order Functions (HOF) Completeness
Leverage `vm.call_value` to complete the functional programming toolkit.

### 2.1 List Module Expansion
Update `stdlib/list.rs`:
- `List.iter : ('a -> unit) -> 'a list -> unit`
- `List.filter : ('a -> bool) -> 'a list -> 'a list`
- `List.fold : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a`
- `List.exists : ('a -> bool) -> 'a list -> bool`
- `List.find : ('a -> bool) -> 'a list -> 'a` (Throw error if not found)
- `List.tryFind : ('a -> bool) -> 'a list -> 'a option`

### 2.2 Map Module Expansion
Update `stdlib/map.rs`:
- `Map.map : ('a -> 'b) -> 'k map -> 'b map`
- `Map.iter : ('k -> 'v -> unit) -> 'k map -> unit`

## Phase 3: Cleanup & Polish

### 3.1 Global Functions
In `stdlib/mod.rs`:
- Ensure `print` and `printfn` are registered in the global scope (not just as part of a module).

### 3.2 Documentation & Examples
- Update `examples/stdlib_demo.fsx`:
  - Remove false "unimplemented" comments.
  - Add examples for new Array and List functions.
- Generate updated `docs/stdlib-summary.md`.

## Instructions for Developer/Agent
1.  **Start with Phase 1**. Create `array.rs`. This is a new file and won't conflict with existing code.
2.  **Proceed to Phase 2**. Add the missing functions to `list.rs`.
3.  **Verify**. Run `cargo test` in `fusabi-vm`.
