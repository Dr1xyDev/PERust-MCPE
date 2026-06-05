use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::StorageError;

/// YAML file storage utility.
///
/// Provides static methods for reading and writing YAML files using serde.
pub struct YamlStorage;

impl YamlStorage {
    /// Reads a YAML file and deserializes it into type `T`.
    ///
    /// # Errors
    ///
    /// Returns `StorageError::FileNotFound` if the file does not exist,
    /// `StorageError::YamlError` if deserialization fails.
    pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T, StorageError> {
        if !path.exists() {
            return Err(StorageError::FileNotFound(path.to_path_buf()));
        }
        let data = fs::read_to_string(path)?;
        let value: T = serde_yaml::from_str(&data)?;
        Ok(value)
    }

    /// Serializes data as YAML and writes it to a file.
    ///
    /// Creates parent directories if they do not exist.
    pub fn write<T: Serialize>(path: &Path, data: &T) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(data)?;
        fs::write(path, yaml)?;
        Ok(())
    }

    /// Reads a YAML file, or returns the default value if the file does not exist.
    ///
    /// If the file exists but cannot be parsed, an error is returned.
    pub fn read_or_default<T: DeserializeOwned + Default>(path: &Path) -> Result<T, StorageError> {
        if !path.exists() {
            return Ok(T::default());
        }
        Self::read(path)
    }
}
