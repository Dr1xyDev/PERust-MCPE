use std::io;
use std::path::PathBuf;

/// Errors that can occur during storage operations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    /// A JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// A YAML serialization/deserialization error.
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// An NBT error.
    #[error("NBT error: {0}")]
    NbtError(#[from] perust_nbt::NbtError),

    /// A region file format error.
    #[error("Region error: {0}")]
    RegionError(String),

    /// The requested file was not found.
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    /// The data has an invalid format.
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}
