use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::ConfigError;
use perust_storage::JsonStorage;

/// Player whitelist configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Whitelist {
    /// Whether the whitelist is enforced.
    pub enabled: bool,
    /// Names of whitelisted players.
    pub players: Vec<String>,
}

impl Default for Whitelist {
    fn default() -> Self {
        Whitelist {
            enabled: false,
            players: Vec::new(),
        }
    }
}

impl Whitelist {
    /// Loads the whitelist from a JSON file, or returns default if missing.
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        JsonStorage::read_or_default(path).map_err(ConfigError::from)
    }

    /// Saves the whitelist to a JSON file.
    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        JsonStorage::write(path, self).map_err(ConfigError::from)
    }

    /// Returns `true` if the given player name is on the whitelist.
    ///
    /// If the whitelist is disabled, all players are considered whitelisted.
    pub fn is_whitelisted(&self, name: &str) -> bool {
        if !self.enabled {
            return true;
        }
        self.players
            .iter()
            .any(|p| p.eq_ignore_ascii_case(name))
    }

    /// Adds a player to the whitelist. No-op if already present.
    pub fn add(&mut self, name: &str) {
        if !self.players.iter().any(|p| p.eq_ignore_ascii_case(name)) {
            self.players.push(name.to_string());
        }
    }

    /// Removes a player from the whitelist. No-op if not present.
    pub fn remove(&mut self, name: &str) {
        self.players.retain(|p| !p.eq_ignore_ascii_case(name));
    }
}
