use std::io;
use std::path::PathBuf;

/// Errors that can occur during configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    /// A parse error (invalid YAML/JSON format).
    #[error("Parse error: {0}")]
    ParseError(String),

    /// A storage error from the underlying storage layer.
    #[error("Storage error: {0}")]
    StorageError(#[from] perust_storage::StorageError),

    /// The requested configuration was not found.
    #[error("Not found: {0}")]
    NotFound(PathBuf),
}

impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
        ConfigError::ParseError(format!("JSON parse error: {}", e))
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(e: serde_yaml::Error) -> Self {
        ConfigError::ParseError(format!("YAML parse error: {}", e))
    }
}
