// Fusabi VM - Bytecode Virtual Machine Runtime

pub mod chunk;
pub mod closure;
pub mod conversions;
pub mod host;
pub mod instruction;
pub mod stdlib;
pub mod value;
pub mod vm;

pub use chunk::{Chunk, ChunkBuilder};
pub use closure::{Closure, Upvalue};
pub use host::{HostFn, HostRegistry};
pub use instruction::Instruction;
pub use stdlib::StdlibRegistry;
pub use value::Value;
pub use vm::{Frame, Vm, VmError};

// ===== Bytecode Serialization (.fzb format) =====

/// Magic bytes for .fzb file format
pub const FZB_MAGIC: &[u8] = b"FZB\x01";

/// Current bytecode format version
pub const FZB_VERSION: u8 = 1;

#[cfg(feature = "serde")]
/// Serialize a Chunk to .fzb bytecode format
///
/// Format:
/// - 4 bytes: Magic bytes "FZB\x01"
/// - 1 byte: Version number
/// - N bytes: Bincode-serialized Chunk data
pub fn serialize_chunk(chunk: &Chunk) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();

    // Write magic bytes
    bytes.extend_from_slice(FZB_MAGIC);

    // Write version
    bytes.push(FZB_VERSION);

    // Serialize chunk with bincode
    let chunk_bytes = bincode::serialize(chunk)?;
    bytes.extend_from_slice(&chunk_bytes);

    Ok(bytes)
}

#[cfg(feature = "serde")]
/// Deserialize a Chunk from .fzb bytecode format
///
/// Validates magic bytes and version before deserializing
pub fn deserialize_chunk(bytes: &[u8]) -> Result<Chunk, Box<dyn std::error::Error>> {
    // Check minimum length
    if bytes.len() < 5 {
        return Err("File too small to be a valid .fzb file".into());
    }

    // Check magic bytes
    if !bytes.starts_with(FZB_MAGIC) {
        return Err("Invalid magic bytes - not a .fzb file".into());
    }

    // Check version
    if bytes[4] != FZB_VERSION {
        return Err(format!("Unsupported .fzb version: {} (expected {})", bytes[4], FZB_VERSION).into());
    }

    // Deserialize chunk
    let chunk: Chunk = bincode::deserialize(&bytes[5..])?;
    Ok(chunk)
}

#[cfg(all(test, feature = "serde"))]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_empty_chunk() {
        let chunk = Chunk::new();
        let bytes = serialize_chunk(&chunk).unwrap();
        let restored = deserialize_chunk(&bytes).unwrap();

        assert_eq!(chunk.instructions.len(), restored.instructions.len());
        assert_eq!(chunk.constants.len(), restored.constants.len());
        assert_eq!(chunk.name, restored.name);
    }

    #[test]
    fn test_serialize_deserialize_chunk_with_data() {
        let mut chunk = Chunk::with_name("test");
        chunk.add_constant(Value::Int(42));
        chunk.add_constant(Value::Str("hello".to_string()));
        chunk.emit(Instruction::LoadConst(0));
        chunk.emit(Instruction::LoadConst(1));
        chunk.emit(Instruction::Return);

        let bytes = serialize_chunk(&chunk).unwrap();
        assert!(bytes.starts_with(FZB_MAGIC));
        assert_eq!(bytes[4], FZB_VERSION);

        let restored = deserialize_chunk(&bytes).unwrap();
        assert_eq!(chunk.name, restored.name);
        assert_eq!(chunk.instructions.len(), restored.instructions.len());
        assert_eq!(chunk.constants.len(), restored.constants.len());

        // Check instructions
        assert_eq!(chunk.instructions[0], restored.instructions[0]);
        assert_eq!(chunk.instructions[1], restored.instructions[1]);
        assert_eq!(chunk.instructions[2], restored.instructions[2]);

        // Check constants
        assert_eq!(chunk.constants[0], restored.constants[0]);
        assert_eq!(chunk.constants[1], restored.constants[1]);
    }

    #[test]
    fn test_invalid_magic_bytes() {
        let bad_bytes = b"BAD\x01\x00extra data";
        let result = deserialize_chunk(bad_bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid magic bytes"));
    }

    #[test]
    fn test_unsupported_version() {
        let mut bad_bytes = Vec::new();
        bad_bytes.extend_from_slice(FZB_MAGIC);
        bad_bytes.push(99); // Wrong version
        bad_bytes.extend_from_slice(b"extra data");

        let result = deserialize_chunk(&bad_bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported .fzb version"));
    }

    #[test]
    fn test_file_too_small() {
        let bad_bytes = b"FZB";
        let result = deserialize_chunk(bad_bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("File too small"));
    }
}
