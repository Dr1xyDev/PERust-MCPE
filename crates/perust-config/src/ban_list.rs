use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

use crate::error::ConfigError;
use perust_storage::JsonStorage;

/// Intermediate JSON structure for persisting ban lists.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct BanListData {
    #[serde(default)]
    banned_players: Vec<String>,
    #[serde(default)]
    banned_ips: Vec<String>,
    #[serde(default)]
    banned_cids: Vec<String>,
}

/// Manages player, IP, and Client ID (CID) bans.
pub struct BanList {
    /// Set of banned player names.
    pub banned_players: HashSet<String>,
    /// Set of banned IP addresses.
    pub banned_ips: HashSet<String>,
    /// Set of banned client IDs.
    pub banned_cids: HashSet<String>,
}

impl Default for BanList {
    fn default() -> Self {
        BanList {
            banned_players: HashSet::new(),
            banned_ips: HashSet::new(),
            banned_cids: HashSet::new(),
        }
    }
}

impl BanList {
    /// Loads ban lists from the data directory.
    ///
    /// Reads from `banned_players.json`, `banned_ips.json`, and `banned_cids.json`
    /// within the given directory.
    pub fn load(data_dir: &Path) -> Result<Self, ConfigError> {
        let banned_players = Self::load_string_set(data_dir.join("banned_players.json"))?;
        let banned_ips = Self::load_string_set(data_dir.join("banned_ips.json"))?;
        let banned_cids = Self::load_string_set(data_dir.join("banned_cids.json"))?;

        Ok(BanList {
            banned_players,
            banned_ips,
            banned_cids,
        })
    }

    /// Saves all ban lists to the data directory.
    pub fn save(&self, data_dir: &Path) -> Result<(), ConfigError> {
        Self::save_string_set(&self.banned_players, data_dir.join("banned_players.json"))?;
        Self::save_string_set(&self.banned_ips, data_dir.join("banned_ips.json"))?;
        Self::save_string_set(&self.banned_cids, data_dir.join("banned_cids.json"))?;
        Ok(())
    }

    /// Returns `true` if the player is banned.
    pub fn is_banned(&self, name: &str) -> bool {
        self.banned_players
            .iter()
            .any(|p| p.eq_ignore_ascii_case(name))
    }

    /// Returns `true` if the IP address is banned.
    pub fn is_ip_banned(&self, ip: &str) -> bool {
        self.banned_ips.contains(ip)
    }

    /// Bans a player by name.
    pub fn ban_player(&mut self, name: &str) {
        self.banned_players.insert(name.to_lowercase());
    }

    /// Unbans a player by name.
    pub fn unban_player(&mut self, name: &str) {
        self.banned_players.remove(&name.to_lowercase());
    }

    /// Bans an IP address.
    pub fn ban_ip(&mut self, ip: &str) {
        self.banned_ips.insert(ip.to_string());
    }

    /// Unbans an IP address.
    pub fn unban_ip(&mut self, ip: &str) {
        self.banned_ips.remove(ip);
    }

    /// Helper: loads a set of strings from a JSON file.
    fn load_string_set(path: std::path::PathBuf) -> Result<HashSet<String>, ConfigError> {
        let data: BanListData = JsonStorage::read_or_default(&path)?;
        Ok(data.banned_players.into_iter().collect())
    }

    /// Helper: saves a set of strings to a JSON file.
    fn save_string_set(set: &HashSet<String>, path: std::path::PathBuf) -> Result<(), ConfigError> {
        let data = BanListData {
            banned_players: set.iter().cloned().collect(),
            banned_ips: Vec::new(),
            banned_cids: Vec::new(),
        };
        JsonStorage::write(&path, &data)?;
        Ok(())
    }
}
