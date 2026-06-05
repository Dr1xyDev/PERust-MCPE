//! The main Player struct representing a connected player.

use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use uuid::Uuid;

use perust_inventory::{ContainerInventory, PlayerInventory};
use perust_protocol::packets::LoginPacket;
use perust_protocol::types::{AdventureSettingsFlags, Difficulty, Dimension, GameMode, PlayerAction};
use perust_utils::math::{BlockPos, Vector3f};

use crate::error::PlayerError;
use crate::login_state::LoginState;
use crate::permission::PlayerPermissions;

/// The main player struct representing a connected player on the server.
///
/// Contains all state related to a player: identity, connection info,
/// world position, inventory, health, permissions, and more.
pub struct Player {
    // === Identity ===
    /// Runtime entity ID (assigned by EntityManager, unique per session).
    pub runtime_id: u64,
    /// Unique persistent entity ID.
    pub unique_id: i64,
    /// The player's UUID (from Xbox Live or offline).
    pub uuid: Uuid,
    /// The player's username.
    pub username: String,
    /// The player's display name (may differ from username).
    pub display_name: String,
    /// The player's Xbox User ID (XUID), if available.
    pub xuid: Option<String>,
    /// The client's unique ID from the login packet.
    pub client_id: i64,

    // === Connection ===
    /// The player's network address.
    pub address: SocketAddr,
    /// The current login state of this player.
    pub login_state: LoginState,
    /// The protocol version the client is using.
    pub protocol: u32,
    /// The device OS the client is running on.
    pub device_os: u32,
    /// The device model string from the client.
    pub device_model: String,
    /// The language code of the client.
    pub language: String,

    // === World state ===
    /// The player's position in world space.
    pub position: Vector3f,
    /// The player's spawn position.
    pub spawn_position: BlockPos,
    /// The player's current game mode.
    pub gamemode: GameMode,
    /// The world difficulty.
    pub difficulty: Difficulty,
    /// The dimension the player is in.
    pub dimension: Dimension,
    /// Whether the player is on the ground.
    pub on_ground: bool,
    /// The player's yaw rotation (horizontal).
    pub yaw: f32,
    /// The player's pitch rotation (vertical).
    pub pitch: f32,
    /// The player's head yaw rotation.
    pub head_yaw: f32,

    // === Inventory ===
    /// The player's main inventory.
    pub inventory: PlayerInventory,
    /// The currently held item slot index (0-8 for hotbar).
    pub held_item_slot: usize,
    /// The currently open container inventory, if any.
    pub open_inventory: Option<ContainerInventory>,

    // === Health & stats ===
    /// The player's current health.
    pub health: f32,
    /// The player's maximum health.
    pub max_health: f32,
    /// The player's food level (0-20).
    pub food: i32,
    /// The player's food saturation level.
    pub saturation: f32,
    /// The player's food exhaustion level.
    pub exhaustion: f32,
    /// The player's experience points.
    pub experience: i32,
    /// The player's experience level.
    pub experience_level: i32,

    // === Flags ===
    /// Whether the player is a server operator.
    pub is_op: bool,
    /// Whether the player is whitelisted.
    pub is_whitelisted: bool,
    /// Whether the player is banned.
    pub is_banned: bool,
    /// Whether the player is sprinting.
    pub is_sprinting: bool,
    /// Whether the player is sneaking.
    pub is_sneaking: bool,
    /// Whether the player is flying.
    pub is_flying: bool,
    /// Whether the player is gliding (elytra).
    pub is_gliding: bool,
    /// Whether the player is sleeping in a bed.
    pub is_sleeping: bool,

    // === Adventure settings ===
    /// The player's adventure settings flags.
    pub adventure_settings: AdventureSettingsFlags,

    // === Chunk loading ===
    /// The requested chunk render radius.
    pub chunk_radius: u32,
    /// Set of currently loaded chunk coordinates for this player.
    pub loaded_chunks: HashSet<(i32, i32)>,
    /// The last chunk position the player was known to be in.
    pub last_chunk_position: (i32, i32),

    // === Timing ===
    /// The time of the last movement packet from this player.
    pub last_move_time: Instant,
    /// The player's current ping (round-trip time).
    pub ping: Duration,
    /// The time when this player joined the server.
    pub join_time: Instant,

    // === Skin data ===
    /// The player's skin ID.
    pub skin_id: String,
    /// The player's skin image data.
    pub skin_data: Vec<u8>,

    // === Permissions ===
    /// The player's permission system.
    pub permissions: PlayerPermissions,
}

impl Player {
    /// Creates a new player with the given runtime ID and network address.
    ///
    /// All fields are initialized to sensible defaults. The player starts
    /// in the `Connecting` login state.
    pub fn new(runtime_id: u64, address: SocketAddr) -> Self {
        Self {
            // Identity
            runtime_id,
            unique_id: runtime_id as i64,
            uuid: Uuid::nil(),
            username: String::new(),
            display_name: String::new(),
            xuid: None,
            client_id: 0,

            // Connection
            address,
            login_state: LoginState::Connecting,
            protocol: 0,
            device_os: 0,
            device_model: String::new(),
            language: String::new(),

            // World state
            position: Vector3f::new(0.0, 64.0, 0.0),
            spawn_position: BlockPos::new(0, 64, 0),
            gamemode: GameMode::Survival,
            difficulty: Difficulty::Normal,
            dimension: Dimension::Overworld,
            on_ground: false,
            yaw: 0.0,
            pitch: 0.0,
            head_yaw: 0.0,

            // Inventory
            inventory: PlayerInventory::new(),
            held_item_slot: 0,
            open_inventory: None,

            // Health & stats
            health: 20.0,
            max_health: 20.0,
            food: 20,
            saturation: 5.0,
            exhaustion: 0.0,
            experience: 0,
            experience_level: 0,

            // Flags
            is_op: false,
            is_whitelisted: true,
            is_banned: false,
            is_sprinting: false,
            is_sneaking: false,
            is_flying: false,
            is_gliding: false,
            is_sleeping: false,

            // Adventure settings
            adventure_settings: AdventureSettingsFlags::empty(),

            // Chunk loading
            chunk_radius: 8,
            loaded_chunks: HashSet::new(),
            last_chunk_position: (0, 0),

            // Timing
            last_move_time: Instant::now(),
            ping: Duration::from_millis(0),
            join_time: Instant::now(),

            // Skin data
            skin_id: String::new(),
            skin_data: Vec::new(),

            // Permissions
            permissions: PlayerPermissions::new(false),
        }
    }

    /// Processes a login packet, extracting player identity and connection info.
    ///
    /// Updates the player's state from the login packet data, including
    /// protocol version, username, UUID, and skin information.
    pub fn handle_login(&mut self, login_packet: &LoginPacket) -> Result<(), PlayerError> {
        if self.login_state != LoginState::Connecting {
            return Err(PlayerError::InvalidState(format!(
                "Expected Connecting state, got {:?}",
                self.login_state
            )));
        }

        self.protocol = login_packet.protocol;

        // Try to extract identity from the JWT chain
        for chain in &login_packet.chain_data {
            if let Ok(display_name) = chain.extract_display_name() {
                self.username = display_name.clone();
                self.display_name = display_name;
            }
            if let Ok(identity_str) = chain.extract_identity() {
                if let Ok(uuid) = Uuid::parse_str(&identity_str) {
                    self.uuid = uuid;
                }
            }
        }

        // Try to extract display name from client data
        if let Ok(name) = login_packet.client_data.display_name() {
            if !name.is_empty() {
                self.display_name = name;
            }
        }

        // Parse client data for device info
        if let Ok(client_data) = login_packet.client_data.parse() {
            if let Some(os) = client_data.get("DeviceOS").and_then(|v| v.as_i64()) {
                self.device_os = os as u32;
            }
            if let Some(model) = client_data.get("DeviceModel").and_then(|v| v.as_str()) {
                self.device_model = model.to_string();
            }
            if let Some(lang) = client_data.get("LanguageCode").and_then(|v| v.as_str()) {
                self.language = lang.to_string();
            }
            if let Some(client_id) = client_data.get("ClientId").and_then(|v| v.as_i64()) {
                self.client_id = client_id;
            }
        }

        self.login_state = LoginState::LoggingIn;

        log::info!(
            "Player {} ({}) logging in from {} with protocol {}",
            self.username,
            self.uuid,
            self.address,
            self.protocol
        );

        Ok(())
    }

    /// Handles a movement update from the player.
    ///
    /// Updates position, rotation, and ground state. Also tracks the
    /// time of the last movement for timeout detection.
    pub fn handle_move(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        yaw: f32,
        pitch: f32,
        head_yaw: f32,
        on_ground: bool,
    ) {
        self.position = Vector3f::new(x, y, z);
        self.yaw = yaw;
        self.pitch = pitch;
        self.head_yaw = head_yaw;
        self.on_ground = on_ground;
        self.last_move_time = Instant::now();

        // Update chunk position
        let chunk_x = self.position.x.floor() as i32 >> 4;
        let chunk_z = self.position.z.floor() as i32 >> 4;
        self.last_chunk_position = (chunk_x, chunk_z);
    }

    /// Handles a player action packet.
    ///
    /// Updates player flags based on the action type (sprinting, sneaking, etc.).
    pub fn handle_player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::StartSprint => {
                self.is_sprinting = true;
            }
            PlayerAction::StopSprint => {
                self.is_sprinting = false;
            }
            PlayerAction::StartSneak => {
                self.is_sneaking = true;
            }
            PlayerAction::StopSneak => {
                self.is_sneaking = false;
            }
            PlayerAction::StartGlide => {
                self.is_gliding = true;
            }
            PlayerAction::StopGlide => {
                self.is_gliding = false;
            }
            PlayerAction::StopSleeping => {
                self.is_sleeping = false;
            }
            PlayerAction::Jump => {
                // Track jump for exhaustion
            }
            _ => {
                log::debug!("Unhandled player action: {:?}", action);
            }
        }
    }

    /// Handles an inventory transaction.
    ///
    /// This is a placeholder that logs the transaction. Actual validation
    /// and application should be done at a higher level with access to
    /// the full game state.
    pub fn handle_inventory_transaction(
        &mut self,
        transaction: &perust_inventory::Transaction,
    ) {
        log::trace!(
            "Inventory transaction: inventory_id={}, slot={}, old={:?}, new={:?}",
            transaction.inventory_id,
            transaction.slot,
            transaction.old_item,
            transaction.new_item
        );
    }

    /// Sets the player's game mode and updates related flags.
    pub fn set_gamemode(&mut self, gamemode: GameMode) {
        self.gamemode = gamemode;

        // Update adventure settings based on gamemode
        match gamemode {
            GameMode::Creative => {
                self.adventure_settings |= AdventureSettingsFlags::ALLOW_FLIGHT;
                self.adventure_settings |= AdventureSettingsFlags::WORLD_BUILDER;
            }
            GameMode::Spectator => {
                self.adventure_settings |= AdventureSettingsFlags::ALLOW_FLIGHT;
                self.adventure_settings |= AdventureSettingsFlags::NO_CLIP;
                self.adventure_settings |= AdventureSettingsFlags::WORLD_IMMUTABLE;
            }
            GameMode::Survival | GameMode::Adventure => {
                self.adventure_settings -= AdventureSettingsFlags::ALLOW_FLIGHT;
                self.adventure_settings -= AdventureSettingsFlags::NO_CLIP;
                self.adventure_settings -= AdventureSettingsFlags::WORLD_IMMUTABLE;
                if gamemode == GameMode::Adventure {
                    self.adventure_settings |= AdventureSettingsFlags::WORLD_IMMUTABLE;
                }
            }
        }
    }

    /// Teleports the player to the given position.
    pub fn teleport(&mut self, position: Vector3f) {
        self.position = position;
        self.on_ground = false;

        // Update chunk position after teleport
        let chunk_x = self.position.x.floor() as i32 >> 4;
        let chunk_z = self.position.z.floor() as i32 >> 4;
        self.last_chunk_position = (chunk_x, chunk_z);
    }

    /// Kicks the player with the given reason.
    ///
    /// Sets the login state to Disconnected and logs the kick.
    pub fn kick(&mut self, reason: &str) {
        log::info!("Kicking player {} for: {}", self.username, reason);
        self.login_state = LoginState::Disconnected;
    }

    /// Sends a chat message to the player.
    ///
    /// This is a placeholder; actual sending requires the network layer.
    pub fn send_message(&mut self, _message: &str) {
        // The actual sending is handled by the network layer.
        // This method is provided as a convenient API for the server code.
        log::trace!("Sending message to {}: {}", self.username, _message);
    }

    /// Sends a tip message to the player (shown briefly above the hotbar).
    pub fn send_tip(&mut self, _message: &str) {
        log::trace!("Sending tip to {}: {}", self.username, _message);
    }

    /// Sends a popup message to the player (shown in the center of the screen).
    pub fn send_popup(&mut self, _message: &str) {
        log::trace!("Sending popup to {}: {}", self.username, _message);
    }

    /// Checks if the player needs more chunks to be sent.
    ///
    /// Returns `true` if the player's chunk position has changed since
    /// the last batch of chunks was sent.
    pub fn needs_chunks(&self) -> bool {
        let current_chunk = self.get_chunk_position();
        current_chunk != self.last_chunk_position || self.loaded_chunks.is_empty()
    }

    /// Gets the list of chunk coordinates that need to be loaded for this player.
    ///
    /// Returns all chunks within the player's render radius that haven't
    /// been loaded yet.
    pub fn get_chunks_needed(&self) -> Vec<(i32, i32)> {
        let (center_x, center_z) = self.get_chunk_position();
        let radius = self.chunk_radius as i32;
        let mut needed = Vec::new();

        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let chunk_x = center_x + dx;
                let chunk_z = center_z + dz;
                if !self.loaded_chunks.contains(&(chunk_x, chunk_z)) {
                    needed.push((chunk_x, chunk_z));
                }
            }
        }

        needed
    }

    /// Returns the chunk position based on the player's current world position.
    pub fn get_chunk_position(&self) -> (i32, i32) {
        let chunk_x = self.position.x.floor() as i32 >> 4;
        let chunk_z = self.position.z.floor() as i32 >> 4;
        (chunk_x, chunk_z)
    }

    /// Marks a chunk as loaded for this player.
    pub fn mark_chunk_loaded(&mut self, chunk_x: i32, chunk_z: i32) {
        self.loaded_chunks.insert((chunk_x, chunk_z));
    }

    /// Removes chunks that are outside the player's render radius.
    ///
    /// Returns the list of chunks that were unloaded.
    pub fn unload_distant_chunks(&mut self) -> Vec<(i32, i32)> {
        let (center_x, center_z) = self.get_chunk_position();
        let radius = self.chunk_radius as i32;

        let to_remove: Vec<(i32, i32)> = self
            .loaded_chunks
            .iter()
            .filter(|&(cx, cz)| {
                let dx = *cx - center_x;
                let dz = *cz - center_z;
                dx.abs() > radius || dz.abs() > radius
            })
            .copied()
            .collect();

        for chunk in &to_remove {
            self.loaded_chunks.remove(chunk);
        }

        to_remove
    }

    /// Returns `true` if this player is fully connected and in-game.
    pub fn is_playing(&self) -> bool {
        self.login_state == LoginState::Playing
    }

    /// Returns `true` if this player is connected (not disconnected).
    pub fn is_connected(&self) -> bool {
        self.login_state != LoginState::Disconnected
    }

    /// Damages the player by the given amount.
    ///
    /// Returns `true` if the player died from this damage.
    pub fn damage(&mut self, amount: f32) -> bool {
        if self.gamemode == GameMode::Creative || self.gamemode == GameMode::Spectator {
            return false;
        }
        self.health = (self.health - amount).max(0.0);
        self.health <= 0.0
    }

    /// Heals the player by the given amount.
    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Sets the player's food level, clamping to valid range.
    pub fn set_food(&mut self, food: i32) {
        self.food = food.clamp(0, 20);
    }

    /// Adds exhaustion to the player, reducing saturation.
    pub fn add_exhaustion(&mut self, amount: f32) {
        self.exhaustion += amount;
        while self.exhaustion >= 4.0 {
            self.exhaustion -= 4.0;
            if self.saturation > 0.0 {
                self.saturation = (self.saturation - 1.0).max(0.0);
            } else {
                self.food = (self.food - 1).max(0);
            }
        }
    }

    /// Checks if the player has a specific permission.
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.has_permission(permission)
    }

    /// Sets a permission for this player.
    pub fn set_permission(&mut self, permission: &str, value: bool) {
        self.permissions.set_permission(permission.to_string(), value);
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player{{name={}, uuid={}, address={}, state={}}}",
            self.username, self.uuid, self.address, self.login_state
        )
    }
}
