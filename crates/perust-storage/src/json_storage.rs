use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::StorageError;

/// JSON file storage utility.
///
/// Provides static methods for reading and writing JSON files using serde.
pub struct JsonStorage;

impl JsonStorage {
    /// Reads a JSON file and deserializes it into type `T`.
    ///
    /// # Errors
    ///
    /// Returns `StorageError::FileNotFound` if the file does not exist,
    /// `StorageError::JsonError` if deserialization fails.
    pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T, StorageError> {
        if !path.exists() {
            return Err(StorageError::FileNotFound(path.to_path_buf()));
        }
        let data = fs::read_to_string(path)?;
        let value: T = serde_json::from_str(&data)?;
        Ok(value)
    }

    /// Serializes data as JSON and writes it to a file.
    ///
    /// Creates parent directories if they do not exist.
    /// The output is pretty-printed for human readability.
    pub fn write<T: Serialize>(path: &Path, data: &T) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(data)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Reads a JSON file, or returns the default value if the file does not exist.
    ///
    /// If the file exists but cannot be parsed, an error is returned.
    pub fn read_or_default<T: DeserializeOwned + Default>(path: &Path) -> Result<T, StorageError> {
        if !path.exists() {
            return Ok(T::default());
        }
        Self::read(path)
    }
}
