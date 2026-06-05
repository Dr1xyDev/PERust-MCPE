//! Error types for world operations.

use thiserror::Error;

/// Errors that can occur during world operations.
#[derive(Debug, Error)]
pub enum WorldError {
    /// The requested chunk was not found.
    #[error("chunk not found at ({0}, {1})")]
    ChunkNotFound(i32, i32),

    /// The requested region file was not found.
    #[error("region file not found for region ({0}, {1})")]
    RegionNotFound(i32, i32),

    /// The chunk data is invalid or corrupted.
    #[error("invalid chunk data: {0}")]
    InvalidChunkData(String),

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// An NBT serialization/deserialization error occurred.
    #[error("NBT error: {0}")]
    NbtError(#[from] perust_nbt::NbtError),

    /// A terrain generation error occurred.
    #[error("generator error: {0}")]
    GeneratorError(String),
}
