# Fusabi ABI Specification

## Table of Contents

1. [Value Representation](#value-representation)
2. [Memory Management](#memory-management)
3. [Bytecode Format (.fzb)](#bytecode-format-fzb)
4. [Instruction Encoding](#instruction-encoding)
5. [Constant Pool](#constant-pool)
6. [Call Convention](#call-convention)
7. [Interop Protocol](#interop-protocol)

## Value Representation

Fusabi uses a tagged union (enum) for runtime values, optimized for both performance and flexibility.

### Core Value Type

```rust
pub enum Value {
    Int(i64),                              // 8 bytes + discriminant
    Bool(bool),                            // 1 byte + discriminant
    Str(String),                           // 24 bytes (ptr + len + cap) + discriminant
    Unit,                                  // 0 bytes + discriminant
    Tuple(Vec<Value>),                     // 24 bytes (Vec) + discriminant
    Cons {
        head: Box<Value>,
        tail: Box<Value>
    },                                     // 16 bytes (2 pointers) + discriminant
    Nil,                                   // 0 bytes + discriminant
    Array(Rc<RefCell<Vec<Value>>>),        // 8 bytes (Rc ptr) + discriminant
    Record(Rc<RefCell<HashMap<String, Value>>>), // 8 bytes (Rc ptr) + discriminant
    Variant {
        type_name: String,                 // 24 bytes
        variant_name: String,              // 24 bytes
        fields: Vec<Value>,                // 24 bytes
    },                                     // 72 bytes + discriminant
}
```

### Memory Layout

The enum discriminant occupies 1 byte (can represent up to 256 variants). Values are aligned to 8-byte boundaries for optimal performance on 64-bit systems.

#### Size Analysis:
- **Minimum size**: 16 bytes (discriminant + padding + smallest variant)
- **Primitive values**: 16 bytes (Int, Bool, Unit)
- **String values**: 32 bytes (discriminant + String struct)
- **Heap references**: 16 bytes (discriminant + Rc pointer)
- **Variant values**: 80 bytes (largest variant)

### Tagged Union Discriminants

| Discriminant | Value Type | Description |
|-------------|------------|-------------|
| 0x00 | Int | 64-bit signed integer |
| 0x01 | Bool | Boolean value |
| 0x02 | Str | Heap-allocated string |
| 0x03 | Unit | Unit/void type |
| 0x04 | Tuple | Product type |
| 0x05 | Cons | List cons cell |
| 0x06 | Nil | Empty list |
| 0x07 | Array | Mutable array |
| 0x08 | Record | Field-value mapping |
| 0x09 | Variant | Discriminated union |

## Memory Management

### Reference Counting

Heap-allocated values use `Rc<RefCell<T>>` for shared ownership:

```rust
// Arrays and Records use interior mutability
Array(Rc<RefCell<Vec<Value>>>)
Record(Rc<RefCell<HashMap<String, Value>>>)
```

#### Rc<RefCell<T>> Behavior:
- **Rc**: Reference counting for shared ownership
  - Cloning increments reference count
  - Dropping decrements reference count
  - Deallocated when count reaches 0
- **RefCell**: Runtime borrow checking
  - Allows mutation of shared data
  - Panics on borrow rule violations

### Garbage Collection

While Rc handles acyclic structures, cycles require additional collection:

1. **Mark Phase**: Traverse from root set (stack, globals)
2. **Sweep Phase**: Collect unreachable Rc cycles
3. **Trigger**: On allocation threshold or memory pressure

### Value Lifecycle

```
Creation -> Stack/Heap Allocation -> Sharing (Rc clone) -> Collection (Rc drop/GC)
```

## Bytecode Format (.fzb)

Fusabi bytecode files use the `.fzb` extension and follow this binary format:

### File Structure

```
┌─────────────────────────┐
│   Magic Bytes (4)       │  "FZB\x01"
├─────────────────────────┤
│   Version (1)           │  Format version
├─────────────────────────┤
│   Flags (4)             │  Feature flags
├─────────────────────────┤
│   Metadata Length (4)   │  Size of metadata section
├─────────────────────────┤
│   Metadata              │  Module info, dependencies
├─────────────────────────┤
│   Constant Pool Size (4)│  Number of constants
├─────────────────────────┤
│   Constant Pool         │  Serialized constants
├─────────────────────────┤
│   Code Size (4)         │  Bytecode length
├─────────────────────────┤
│   Bytecode              │  Instruction stream
├─────────────────────────┤
│   Debug Info Size (4)   │  Optional debug data size
├─────────────────────────┤
│   Debug Info            │  Source maps, symbols
└─────────────────────────┘
```

### Header Format

```rust
struct FzbHeader {
    magic: [u8; 4],        // "FZB\x01"
    version: u8,           // Format version (currently 1)
    flags: u32,            // Bit flags for features
}
```

#### Feature Flags:
- Bit 0: Has debug information
- Bit 1: Compressed bytecode
- Bit 2: Has source map
- Bit 3: Optimized code
- Bits 4-31: Reserved

### Metadata Section

Encoded using bincode, containing:

```rust
struct Metadata {
    module_name: String,
    source_hash: [u8; 32],    // SHA-256 of source
    timestamp: u64,            // Unix timestamp
    dependencies: Vec<String>, // Required modules
    exports: Vec<String>,      // Exported symbols
}
```

## Instruction Encoding

Each instruction consists of an opcode byte followed by operands:

### Instruction Format

```
┌────────┬──────────────────────┐
│ Opcode │     Operands          │
│ (1 byte)│   (0-8 bytes)        │
└────────┴──────────────────────┘
```

### Opcode Table

| Opcode | Instruction | Operands | Description |
|--------|-------------|----------|-------------|
| 0x00 | Nop | 0 | No operation |
| 0x01 | LoadConst | u16 | Load constant[u16] |
| 0x02 | LoadLocal | u8 | Load local[u8] |
| 0x03 | StoreLocal | u8 | Store to local[u8] |
| 0x04 | LoadUpvalue | u8 | Load upvalue[u8] |
| 0x05 | StoreUpvalue | u8 | Store to upvalue[u8] |
| 0x06 | Pop | 0 | Pop stack top |
| 0x07 | Dup | 0 | Duplicate stack top |
| 0x10 | Add | 0 | Integer addition |
| 0x11 | Sub | 0 | Integer subtraction |
| 0x12 | Mul | 0 | Integer multiplication |
| 0x13 | Div | 0 | Integer division |
| 0x20 | Eq | 0 | Equality comparison |
| 0x21 | Neq | 0 | Inequality comparison |
| 0x22 | Lt | 0 | Less than |
| 0x23 | Lte | 0 | Less than or equal |
| 0x24 | Gt | 0 | Greater than |
| 0x25 | Gte | 0 | Greater than or equal |
| 0x30 | Jump | i16 | Unconditional jump |
| 0x31 | JumpIfFalse | i16 | Conditional jump |
| 0x32 | Call | u8 | Function call |
| 0x33 | Return | 0 | Return from function |
| 0x40 | MakeTuple | u8 | Create tuple |
| 0x41 | MakeCons | 0 | Create cons cell |
| 0x42 | MakeArray | u16 | Create array |
| 0x43 | MakeRecord | u8 | Create record |
| 0x44 | MakeVariant | 0 | Create variant |

### Operand Encoding

- **u8**: 1 byte unsigned
- **u16**: 2 bytes little-endian unsigned
- **i16**: 2 bytes little-endian signed
- **i64**: 8 bytes little-endian signed

## Constant Pool

Constants are stored in a pool and referenced by index:

### Constant Entry Format

```rust
enum Constant {
    Int(i64),
    Bool(bool),
    String(String),
    Symbol(String),  // For identifiers
}
```

### Serialization

Constants are serialized using bincode:

```
┌─────────────────┐
│  Type (1 byte)  │
├─────────────────┤
│  Length (4 bytes)│  (for variable-length types)
├─────────────────┤
│     Data        │
└─────────────────┘
```

## Call Convention

### Function Calls

Stack layout during function call:

```
Before CALL:          After CALL:
┌─────────────┐      ┌─────────────┐
│   Arg N     │      │  Return Val │
├─────────────┤      └─────────────┘
│   Arg N-1   │
├─────────────┤
│     ...     │
├─────────────┤
│   Arg 1     │
├─────────────┤
│  Function   │
└─────────────┘
```

### Call Frame

```rust
struct CallFrame {
    function: Function,
    ip: usize,           // Instruction pointer
    stack_base: usize,   // Stack frame base
    locals: Vec<Value>,  // Local variables
}
```

## Interop Protocol

### Host Function Interface

Host functions receive arguments and return values as Fusabi Values:

```rust
type HostFunction = Arc<dyn Fn(Vec<Value>) -> Result<Value, String>>;
```

### Value Conversion

#### From Rust to Fusabi:

```rust
impl From<i64> for Value { ... }        // Int
impl From<bool> for Value { ... }       // Bool
impl From<String> for Value { ... }     // Str
impl From<()> for Value { ... }         // Unit
impl<T> From<Vec<T>> for Value { ... }  // Array/List
```

#### From Fusabi to Rust:

```rust
impl Value {
    pub fn as_int(&self) -> Option<i64> { ... }
    pub fn as_bool(&self) -> Option<bool> { ... }
    pub fn as_str(&self) -> Option<&str> { ... }
    pub fn as_tuple(&self) -> Option<&Vec<Value>> { ... }
}
```

### FFI Boundary

When crossing the FFI boundary:

1. **Arguments**: Converted from Value to Rust types
2. **Execution**: Host function runs with native types
3. **Return**: Result converted back to Value
4. **Errors**: String errors become runtime exceptions

### Async Interop

Async host functions use a future-based protocol:

```rust
type AsyncHostFunction = Arc<dyn Fn(Vec<Value>) -> BoxFuture<'static, Result<Value, String>>>;
```

The VM suspends execution until the future completes.

## Binary Compatibility

### Version Matrix

| Bytecode Version | VM Version | Compatible |
|-----------------|------------|------------|
| 1 | 1.x | ✓ |
| 1 | 2.x | ✓ (backward compat) |
| 2 | 1.x | ✗ |
| 2 | 2.x | ✓ |

### Stability Guarantees

- **Major version**: May break bytecode compatibility
- **Minor version**: Backward compatible
- **Patch version**: No bytecode changes

## Security Considerations

### Bytecode Validation

Before execution, bytecode is validated for:

1. **Magic bytes**: Must match expected signature
2. **Version**: Must be supported by VM
3. **Instruction bounds**: All jumps within code section
4. **Stack depth**: No stack underflow possible
5. **Constant indices**: All indices within pool

### Resource Limits

Planned (not yet implemented):

- Maximum stack depth
- Maximum heap allocation
- Instruction count limits
- Recursion depth limits

## Future Extensions

### Planned Features

1. **Compressed bytecode**: zstd compression for code section
2. **Incremental loading**: Stream bytecode execution
3. **Module caching**: Shared bytecode between VMs
4. **JIT compilation**: Hot path optimization
5. **Debug protocol**: Step debugging support

### Reserved Opcodes

Opcodes 0x80-0xFF are reserved for future extensions.

## Examples

### Creating a .fzb file

```rust
use fusabi::compiler::Compiler;
use std::fs::File;

let source = "let x = 42 in x + 1";
let chunk = Compiler::compile(source)?;
let file = File::create("output.fzb")?;
chunk.serialize(&file)?;
```

### Loading a .fzb file

```rust
use fusabi::vm::{Vm, Chunk};
use std::fs::File;

let file = File::open("output.fzb")?;
let chunk = Chunk::deserialize(&file)?;
let mut vm = Vm::new();
let result = vm.execute(chunk)?;
```

## References

- [Source: value.rs](../rust/crates/fusabi-vm/src/value.rs)
- [Source: instruction.rs](../rust/crates/fusabi-vm/src/instruction.rs)
- [Source: chunk.rs](../rust/crates/fusabi-vm/src/chunk.rs)
- [Source: vm.rs](../rust/crates/fusabi-vm/src/vm.rs)