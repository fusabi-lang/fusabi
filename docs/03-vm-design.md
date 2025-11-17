# VM Design (Draft)

This document outlines the core VM for the Mini‑F# dialect.

The VM is:

- A **stack‑based bytecode interpreter**.
- Designed for **embedding** in Rust applications.
- Intended to be **good enough** performance‑wise to replace Lua for configs + callbacks.

## 1. Runtime value representation

A single `Value` enum represents all runtime values:

```rust
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Gc<String>),
    List(Gc<ListCell>),
    Array(Gc<Vec<Value>>),
    Record(Gc<RecordInstance>),
    Variant(Gc<VariantInstance>),
    Closure(Gc<Closure>),
    BuiltinFn(BuiltinFnPtr),
    Unit,
}
```

Where:

- `Gc<T>` is a simple GC handle (e.g. arena index or `Rc<RefCell<T>>` for phase 1).
- `RecordInstance` contains:
  - `type_id: TypeId`,
  - `fields: Vec<Value>` indexed by `FieldId`.
- `VariantInstance` contains:
  - `type_id: TypeId`,
  - `variant_tag: VariantTag`,
  - `payload: Vec<Value>` (0 or more fields).

## 2. Bytecode

Bytecode is organised in **chunks**, one per function:

```rust
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>, // literals, function refs, etc.
}
```

### 2.1 Instructions (first pass)

```rust
pub enum Instruction {
    LoadConst(u16),        // push constants[idx]
    LoadLocal(u8),         // push locals[idx]
    StoreLocal(u8),        // pop -> locals[idx]
    LoadUpvalue(u8),       // push captured upvalue
    StoreUpvalue(u8),      // pop -> captured upvalue
    Pop,                   // pop1

    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Not,

    MakeTuple(u8),         // pop N -> tuple
    MakeList(u8),
    MakeArray(u8),

    MakeRecord(TypeId, u8), // field_count
    GetField(FieldId),

    MakeVariant(TypeId, VariantTag, u8), // arg_count

    MatchTag(TypeId, VariantTag, i16), // if top is not matching tag, jump offset

    Jump(i16),
    JumpIfFalse(i16),

    Call(u8),              // arg_count
    TailCall(u8),          // optional optimisation
    Return,
}
```

You can extend this later with specialised list ops, string ops, etc.

## 3. Call frames and VM loop

A `Frame` holds:

```rust
pub struct Frame {
    pub chunk: ChunkId,      // which function
    pub ip: usize,           // instruction pointer
    pub base: usize,         // base index into the VM value stack
}
```

The VM maintains:

```rust
pub struct Vm {
    pub stack: Vec<Value>,
    pub frames: Vec<Frame>,
    pub globals: Vec<Value>,
    pub heap: Heap,          // GC arena, depending on implementation
}
```

Pseudo‑code for the interpreter loop:

```rust
loop {
    let instr = current_frame.fetch_next();
    match instr {
        Instruction::LoadConst(idx) => { push(constants[idx]); }
        Instruction::Add => { let b = pop_int(); let a = pop_int(); push_int(a + b); }
        Instruction::Call(arg_count) => { setup_new_frame(func_value, arg_count); }
        Instruction::Return => { tear_down_frame(); if frames.is_empty() { break; } }
        // ...
    }
}
```

Start simple; optimise later.

## 4. Compilation pipeline

Front‑end steps:

1. Parse source into AST.
2. Typecheck (HM) and annotate AST with concrete types.
3. Desugar:
   - Pipelines → nested function calls.
   - CEs → builder calls.
   - Pattern matches → decision trees.
4. Compile typed AST to bytecode:
   - Map each function to a `Chunk`.
   - Build constant pools.
   - Emit instruction sequences with jumps.

### 4.1 Pattern match compilation

Compile:

```fsharp
match v with
| Left -> e1
| Right -> e2
| _ -> e3
```

Rough strategy:

1. Evaluate `v` → stack top.
2. Emit:

   ```text
   MatchTag(Direction, Left, L1)
   // matched Left:
   drop scrutinee or bind variables
   code for e1
   Jump L_end

   L1:
   MatchTag(Direction, Right, L2)
   // matched Right:
   code for e2
   Jump L_end

   L2:
   // default:
   code for e3

   L_end:
   ```

For records and tuples, you add `GetField` / `DESTRUCT` instructions as needed.

## 5. GC plan

Phase 1: **simplest thing that works**:

- Use `Rc<RefCell<T>>` and accept non‑moving semantics and minor leaks when cycles happen.
- This is enough to get correctness while you develop front‑end and VM.

Phase 2: introduce a proper arena + mark‑and‑sweep:

- Heap stores objects in vectors keyed by an index type.
- On GC:
  - Start from stack, globals, upvalues as roots.
  - Mark reachable heap objects.
  - Sweep unreached objects.

Tuning will depend on benchmarks (e.g., config size, callback frequency).

## 6. Host interop and built‑ins

The VM exposes built‑ins as `Value::BuiltinFn`:

```rust
pub type BuiltinFnPtr = fn(&mut Vm, &[Value]) -> Result<Value, VmError>;
```

The host registers built‑ins by name:

```rust
engine.register_builtin("print", builtin_print);
engine.register_builtin("add", builtin_add);
```

Front‑end resolves identifiers like `print` to a global index that refers to the built‑in value.

In a terminal use case, you might expose:

- `term.log : string -> unit`
- `term.action.split : Direction -> Action`
- `term.action.sendKeys : string -> Action`
- And others, all as built‑ins that construct records/DUs for the host.

## 7. Performance notes

To approach Lua‑class performance:

- Avoid heavy dynamic features:
  - No reflection.
  - No dynamic field lookup by string; use `FieldId` indices.
- After functionality is stable:
  - Consider NaN‑boxing for numeric/bool/ptr values.
  - Inline hot opcodes (e.g., arithmetic, comparisons).
  - Add simple bytecode peephole optimisations.

However, for the intended use (configs + callbacks), a straightforward Rust interpreter should already be sufficient.
