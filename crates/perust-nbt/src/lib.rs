//! # perust-nbt
//!
//! NBT (Named Binary Tag) serialization/deserialization for Minecraft Bedrock Edition.
//!
//! This crate provides:
//! - **Tag**: NBT tag types and their encoding/decoding
//! - **NbtReader**: Reader supporting BigEndian, LittleEndian, and Network (VarInt) formats
//! - **NbtWriter**: Writer supporting the same formats
//! - **NbtError**: Error types for NBT operations
//! - **Endianness**: Byte order specification for different NBT formats

pub mod tag;
pub mod reader;
pub mod writer;
pub mod error;

pub use tag::{Tag, NamedTag};
pub use reader::NbtReader;
pub use writer::NbtWriter;
pub use error::NbtError;

/// Byte order used for NBT serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    /// Big-endian byte order (Java/disk format).
    BigEndian,
    /// Little-endian byte order (Bedrock LevelDB format).
    LittleEndian,
    /// Network format using VarInt for lengths (Bedrock network format).
    Network,
}
