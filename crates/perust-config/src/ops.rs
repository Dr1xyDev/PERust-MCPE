use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::ConfigError;
use perust_storage::JsonStorage;

/// List of server operators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsList {
    /// Names of operators.
    pub ops: Vec<String>,
}

impl Default for OpsList {
    fn default() -> Self {
        OpsList { ops: Vec::new() }
    }
}

impl OpsList {
    /// Loads the ops list from a JSON file, or returns default if missing.
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        JsonStorage::read_or_default(path).map_err(ConfigError::from)
    }

    /// Saves the ops list to a JSON file.
    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        JsonStorage::write(path, self).map_err(ConfigError::from)
    }

    /// Returns `true` if the given player name is an operator.
    pub fn is_op(&self, name: &str) -> bool {
        self.ops.iter().any(|op| op.eq_ignore_ascii_case(name))
    }

    /// Adds a player as an operator. No-op if already an operator.
    pub fn add_op(&mut self, name: &str) {
        if !self.is_op(name) {
            self.ops.push(name.to_string());
        }
    }

    /// Removes a player from the operator list. No-op if not an operator.
    pub fn remove_op(&mut self, name: &str) {
        self.ops.retain(|op| !op.eq_ignore_ascii_case(name));
    }
}
