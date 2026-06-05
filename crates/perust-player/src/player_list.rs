//! Player list manager for the tab list.

use dashmap::DashMap;
use uuid::Uuid;

/// Thread-safe player list manager.
///
/// Manages the list of players visible in the tab list (player list).
/// Each entry contains the player's identity, skin info, and platform data.
pub struct PlayerList {
    players: DashMap<Uuid, PlayerListEntry>,
}

/// An entry in the player list (tab list).
///
/// Contains the information needed to display a player in the tab list
/// and to construct the PlayerListPacket.
#[derive(Debug, Clone)]
pub struct PlayerListEntry {
    /// The player's UUID.
    pub uuid: Uuid,
    /// The player's username.
    pub username: String,
    /// The player's skin ID.
    pub skin_id: String,
    /// The player's skin image data.
    pub skin_data: Vec<u8>,
    /// The player's Xbox User ID.
    pub xuid: String,
    /// The player's platform chat ID.
    pub platform_chat_id: String,
}

impl PlayerList {
    /// Creates a new empty player list.
    pub fn new() -> Self {
        Self {
            players: DashMap::new(),
        }
    }

    /// Adds a player to the list.
    ///
    /// If a player with the same UUID already exists, their entry is updated.
    pub fn add(&self, entry: PlayerListEntry) {
        self.players.insert(entry.uuid, entry);
    }

    /// Removes a player from the list by UUID.
    ///
    /// Returns the removed entry, if any.
    pub fn remove(&self, uuid: &Uuid) -> Option<PlayerListEntry> {
        self.players.remove(uuid).map(|(_, v)| v)
    }

    /// Gets a player entry by UUID.
    pub fn get(&self, uuid: &Uuid) -> Option<dashmap::mapref::one::Ref<'_, Uuid, PlayerListEntry>> {
        self.players.get(uuid)
    }

    /// Returns all player entries.
    pub fn get_all(&self) -> Vec<PlayerListEntry> {
        self.players.iter().map(|r| r.value().clone()).collect()
    }

    /// Returns the number of players in the list.
    pub fn count(&self) -> usize {
        self.players.len()
    }

    /// Returns `true` if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    /// Checks if a player with the given UUID is in the list.
    pub fn contains(&self, uuid: &Uuid) -> bool {
        self.players.contains_key(uuid)
    }

    /// Clears all entries from the player list.
    pub fn clear(&self) {
        self.players.clear();
    }
}

impl Default for PlayerList {
    fn default() -> Self {
        Self::new()
    }
}
