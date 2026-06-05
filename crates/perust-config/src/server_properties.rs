use serde::{Deserialize, Serialize};

use crate::error::ConfigError;
use perust_storage::YamlStorage;

// === Default value functions ===

fn default_motd() -> String {
    "A PeRust Server".to_string()
}

fn default_port() -> u16 {
    19132
}

fn default_max_players() -> u32 {
    20
}

fn default_view_distance() -> u32 {
    10
}

fn default_gamemode() -> u32 {
    1 // Survival
}

fn default_difficulty() -> u32 {
    2 // Normal
}

fn default_level_name() -> String {
    "world".to_string()
}

fn default_level_type() -> String {
    "DEFAULT".to_string()
}

fn default_true() -> bool {
    true
}

fn default_spawn_protection() -> u32 {
    16
}

fn default_chunk_radius() -> u32 {
    8
}

fn default_language() -> String {
    "eng".to_string()
}

/// Main server configuration properties.
///
/// This struct maps to the `server.properties` YAML file and controls
/// all major aspects of server behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerProperties {
    // === General ===
    /// Message of the Day shown in the server list.
    #[serde(default = "default_motd")]
    pub motd: String,

    /// The port the server listens on.
    #[serde(default = "default_port")]
    pub server_port: u16,

    /// The IP address to bind to. Empty string means all interfaces.
    #[serde(default)]
    pub server_ip: String,

    /// Maximum number of concurrent players.
    #[serde(default = "default_max_players")]
    pub max_players: u32,

    /// View distance in chunks.
    #[serde(default = "default_view_distance")]
    pub view_distance: u32,

    // === Gameplay ===
    /// Default game mode: 0=Survival, 1=Creative, 2=Adventure, 3=Spectator.
    #[serde(default = "default_gamemode")]
    pub gamemode: u32,

    /// Difficulty: 0=Peaceful, 1=Easy, 2=Normal, 3=Hard.
    #[serde(default = "default_difficulty")]
    pub difficulty: u32,

    /// Whether hardcore mode is enabled.
    #[serde(default)]
    pub hardcore: bool,

    /// Whether players are forced back to the default gamemode on join.
    #[serde(default)]
    pub force_gamemode: bool,

    // === World ===
    /// Name of the default world/level.
    #[serde(default = "default_level_name")]
    pub level_name: String,

    /// Level type: DEFAULT, FLAT, or VOID.
    #[serde(default = "default_level_type")]
    pub level_type: String,

    /// Whether the Nether dimension is enabled.
    #[serde(default)]
    pub nether_enabled: bool,

    /// Whether the End dimension is enabled.
    #[serde(default)]
    pub ender_enabled: bool,

    // === Security ===
    /// Whether the whitelist is enabled.
    #[serde(default)]
    pub white_list: bool,

    /// Spawn protection radius in blocks (0 = disabled).
    #[serde(default = "default_spawn_protection")]
    pub spawn_protection: u32,

    /// Whether PvP is enabled.
    #[serde(default)]
    pub pvp: bool,

    // === Features ===
    /// Whether weather cycles are enabled.
    #[serde(default = "default_true")]
    pub weather_enabled: bool,

    /// Whether hunger/food mechanics are enabled.
    #[serde(default = "default_true")]
    pub food_enabled: bool,

    /// Whether experience mechanics are enabled.
    #[serde(default = "default_true")]
    pub experience_enabled: bool,

    /// Whether players keep inventory on death.
    #[serde(default)]
    pub keep_inventory: bool,

    /// Whether redstone mechanics are enabled.
    #[serde(default = "default_true")]
    pub redstone_enabled: bool,

    /// Whether the anvil is enabled.
    #[serde(default = "default_true")]
    pub anvil_enabled: bool,

    /// Whether the enchanting table is enabled.
    #[serde(default = "default_true")]
    pub enchanting_table_enabled: bool,

    // === Performance ===
    /// Whether auto-save is enabled.
    #[serde(default)]
    pub auto_save: bool,

    /// Chunk radius for initial loading around players.
    #[serde(default = "default_chunk_radius")]
    pub chunk_radius: u32,

    // === Network ===
    /// Whether the query protocol is enabled.
    #[serde(default)]
    pub enable_query: bool,

    /// Whether RCON (remote console) is enabled.
    #[serde(default)]
    pub enable_rcon: bool,

    /// Password for RCON access.
    #[serde(default)]
    pub rcon_password: String,

    // === Language ===
    /// Server language code (e.g. "eng", "jpn").
    #[serde(default = "default_language")]
    pub language: String,
}

impl Default for ServerProperties {
    fn default() -> Self {
        ServerProperties {
            motd: default_motd(),
            server_port: default_port(),
            server_ip: String::new(),
            max_players: default_max_players(),
            view_distance: default_view_distance(),
            gamemode: default_gamemode(),
            difficulty: default_difficulty(),
            hardcore: false,
            force_gamemode: false,
            level_name: default_level_name(),
            level_type: default_level_type(),
            nether_enabled: false,
            ender_enabled: false,
            white_list: false,
            spawn_protection: default_spawn_protection(),
            pvp: false,
            weather_enabled: true,
            food_enabled: true,
            experience_enabled: true,
            keep_inventory: false,
            redstone_enabled: true,
            anvil_enabled: true,
            enchanting_table_enabled: true,
            auto_save: false,
            chunk_radius: default_chunk_radius(),
            enable_query: false,
            enable_rcon: false,
            rcon_password: String::new(),
            language: default_language(),
        }
    }
}

impl ServerProperties {
    /// Loads server properties from a YAML file, or creates defaults if missing.
    pub fn load(path: &std::path::Path) -> Result<Self, ConfigError> {
        YamlStorage::read_or_default(path).map_err(ConfigError::from)
    }

    /// Saves server properties to a YAML file.
    pub fn save(&self, path: &std::path::Path) -> Result<(), ConfigError> {
        YamlStorage::write(path, self).map_err(ConfigError::from)
    }

    /// Returns the game mode name for the numeric value.
    pub fn gamemode_name(&self) -> &'static str {
        match self.gamemode {
            0 => "Survival",
            1 => "Creative",
            2 => "Adventure",
            3 => "Spectator",
            _ => "Unknown",
        }
    }

    /// Returns the difficulty name for the numeric value.
    pub fn difficulty_name(&self) -> &'static str {
        match self.difficulty {
            0 => "Peaceful",
            1 => "Easy",
            2 => "Normal",
            3 => "Hard",
            _ => "Unknown",
        }
    }
}
