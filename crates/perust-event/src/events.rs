//! Concrete event types used throughout the server.
//!
//! Events are grouped into categories:
//!
//! - **ServerEvent**: Server lifecycle and network packet events
//! - **PlayerEvent**: Player connection, movement, chat, and interaction events
//! - **EntityEvent**: Entity damage, death, spawn, and despawn events
//! - **BlockEvent**: Block break, place, and update events
//! - **LevelEvent**: Chunk and level loading events
//! - **InventoryEvent**: Inventory open, close, and transaction events

use crate::event::{CancellableEvent, Event};

// ---------------------------------------------------------------------------
// Server Events
// ---------------------------------------------------------------------------

/// Fired when the server has finished starting.
#[derive(Debug, Clone)]
pub struct ServerStartEvent;

impl Event for ServerStartEvent {
    fn event_name(&self) -> &str {
        "ServerStartEvent"
    }
}

/// Fired when the server is about to stop.
#[derive(Debug, Clone)]
pub struct ServerStopEvent;

impl Event for ServerStopEvent {
    fn event_name(&self) -> &str {
        "ServerStopEvent"
    }
}

/// Fired when a data packet is received from a client.
#[derive(Debug, Clone)]
pub struct DataPacketReceiveEvent {
    /// The raw packet data (placeholder — will be replaced with a proper packet type).
    pub packet_id: u32,
}

impl Event for DataPacketReceiveEvent {
    fn event_name(&self) -> &str {
        "DataPacketReceiveEvent"
    }
}

/// Fired when a data packet is about to be sent to a client.
#[derive(Debug, Clone)]
pub struct DataPacketSendEvent {
    /// The raw packet data (placeholder — will be replaced with a proper packet type).
    pub packet_id: u32,
}

impl Event for DataPacketSendEvent {
    fn event_name(&self) -> &str {
        "DataPacketSendEvent"
    }
}

// ---------------------------------------------------------------------------
// Player Events
// ---------------------------------------------------------------------------

/// Fired when a player joins the server.
#[derive(Debug, Clone)]
pub struct PlayerJoinEvent {
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// The player's name.
    pub player_name: String,
    /// Optional join message.
    pub join_message: Option<String>,
}

impl Event for PlayerJoinEvent {
    fn event_name(&self) -> &str {
        "PlayerJoinEvent"
    }
}

/// Fired when a player leaves the server.
#[derive(Debug, Clone)]
pub struct PlayerQuitEvent {
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// The player's name.
    pub player_name: String,
    /// Optional quit message.
    pub quit_message: Option<String>,
}

impl Event for PlayerQuitEvent {
    fn event_name(&self) -> &str {
        "PlayerQuitEvent"
    }
}

/// Fired when a player attempts to log in.
///
/// This event is cancellable — if cancelled, the player is denied login.
#[derive(Debug, Clone)]
pub struct PlayerLoginEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The player's name.
    pub player_name: String,
    /// Optional kick message shown when login is denied.
    pub kick_message: Option<String>,
}

impl Event for PlayerLoginEvent {
    fn event_name(&self) -> &str {
        "PlayerLoginEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// Fired when a player moves.
#[derive(Debug, Clone)]
pub struct PlayerMoveEvent {
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// Previous X position.
    pub from_x: f64,
    /// Previous Y position.
    pub from_y: f64,
    /// Previous Z position.
    pub from_z: f64,
    /// New X position.
    pub to_x: f64,
    /// New Y position.
    pub to_y: f64,
    /// New Z position.
    pub to_z: f64,
}

impl Event for PlayerMoveEvent {
    fn event_name(&self) -> &str {
        "PlayerMoveEvent"
    }
}

/// Fired when a player sends a chat message.
#[derive(Debug, Clone)]
pub struct PlayerChatEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// The player's name.
    pub player_name: String,
    /// The chat message.
    pub message: String,
}

impl Event for PlayerChatEvent {
    fn event_name(&self) -> &str {
        "PlayerChatEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// Fired when a player dies.
#[derive(Debug, Clone)]
pub struct PlayerDeathEvent {
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// The player's name.
    pub player_name: String,
    /// Death cause (optional).
    pub cause: Option<String>,
}

impl Event for PlayerDeathEvent {
    fn event_name(&self) -> &str {
        "PlayerDeathEvent"
    }
}

/// Fired when a player interacts with the world (right-click air/block).
#[derive(Debug, Clone)]
pub struct PlayerInteractEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// Interaction type.
    pub action: InteractAction,
}

impl Event for PlayerInteractEvent {
    fn event_name(&self) -> &str {
        "PlayerInteractEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// The type of interaction performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractAction {
    /// Left-click on a block (break/action).
    LeftClickBlock,
    /// Right-click on a block.
    RightClickBlock,
    /// Left-click in the air.
    LeftClickAir,
    /// Right-click in the air.
    RightClickAir,
}

/// Fired when a player breaks a block.
///
/// This event is cancellable.
#[derive(Debug, Clone)]
pub struct PlayerBreakBlockEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// Block X coordinate.
    pub x: i32,
    /// Block Y coordinate.
    pub y: i32,
    /// Block Z coordinate.
    pub z: i32,
}

impl Event for PlayerBreakBlockEvent {
    fn event_name(&self) -> &str {
        "PlayerBreakBlockEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// Fired when a player places a block.
///
/// This event is cancellable.
#[derive(Debug, Clone)]
pub struct PlayerPlaceBlockEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The player's runtime ID.
    pub runtime_id: u64,
    /// Block X coordinate.
    pub x: i32,
    /// Block Y coordinate.
    pub y: i32,
    /// Block Z coordinate.
    pub z: i32,
}

impl Event for PlayerPlaceBlockEvent {
    fn event_name(&self) -> &str {
        "PlayerPlaceBlockEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// Entity Events
// ---------------------------------------------------------------------------

/// Fired when an entity takes damage.
///
/// This event is cancellable.
#[derive(Debug, Clone)]
pub struct EntityDamageEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The entity's runtime ID.
    pub entity_runtime_id: u64,
    /// Amount of damage.
    pub damage: f32,
    /// Source of the damage.
    pub damage_source: DamageSource,
}

impl Event for EntityDamageEvent {
    fn event_name(&self) -> &str {
        "EntityDamageEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// Describes the source of damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageSource {
    /// Damage from another entity (mob or player).
    Entity,
    /// Fall damage.
    Fall,
    /// Fire damage.
    Fire,
    /// Lava damage.
    Lava,
    /// Drowning.
    Drowning,
    /// Suffocation (inside a block).
    Suffocation,
    /// Starvation.
    Starvation,
    /// Void (below the world).
    Void,
    /// Suicide/kill command.
    Suicide,
    /// Magic (e.g. potions, thorns).
    Magic,
    /// Custom / plugin-defined source.
    Custom,
}

/// Fired when an entity dies.
#[derive(Debug, Clone)]
pub struct EntityDeathEvent {
    /// The entity's runtime ID.
    pub entity_runtime_id: u64,
}

impl Event for EntityDeathEvent {
    fn event_name(&self) -> &str {
        "EntityDeathEvent"
    }
}

/// Fired when an entity spawns into the world.
#[derive(Debug, Clone)]
pub struct EntitySpawnEvent {
    /// The entity's runtime ID.
    pub entity_runtime_id: u64,
    /// Entity type identifier.
    pub entity_type: String,
}

impl Event for EntitySpawnEvent {
    fn event_name(&self) -> &str {
        "EntitySpawnEvent"
    }
}

/// Fired when an entity is despawned / removed from the world.
#[derive(Debug, Clone)]
pub struct EntityDespawnEvent {
    /// The entity's runtime ID.
    pub entity_runtime_id: u64,
}

impl Event for EntityDespawnEvent {
    fn event_name(&self) -> &str {
        "EntityDespawnEvent"
    }
}

// ---------------------------------------------------------------------------
// Block Events
// ---------------------------------------------------------------------------

/// Fired when a block is broken.
///
/// This event is cancellable.
#[derive(Debug, Clone)]
pub struct BlockBreakEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// Block X coordinate.
    pub x: i32,
    /// Block Y coordinate.
    pub y: i32,
    /// Block Z coordinate.
    pub z: i32,
    /// The runtime ID of the entity that broke the block, if any.
    pub breaker_runtime_id: Option<u64>,
}

impl Event for BlockBreakEvent {
    fn event_name(&self) -> &str {
        "BlockBreakEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// Fired when a block is placed.
///
/// This event is cancellable.
#[derive(Debug, Clone)]
pub struct BlockPlaceEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// Block X coordinate.
    pub x: i32,
    /// Block Y coordinate.
    pub y: i32,
    /// Block Z coordinate.
    pub z: i32,
    /// The runtime ID of the entity that placed the block, if any.
    pub placer_runtime_id: Option<u64>,
}

impl Event for BlockPlaceEvent {
    fn event_name(&self) -> &str {
        "BlockPlaceEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// Fired when a block is updated (neighbour update, redstone, etc.).
#[derive(Debug, Clone)]
pub struct BlockUpdateEvent {
    /// Block X coordinate.
    pub x: i32,
    /// Block Y coordinate.
    pub y: i32,
    /// Block Z coordinate.
    pub z: i32,
}

impl Event for BlockUpdateEvent {
    fn event_name(&self) -> &str {
        "BlockUpdateEvent"
    }
}

// ---------------------------------------------------------------------------
// Level Events
// ---------------------------------------------------------------------------

/// Fired when a chunk is loaded.
#[derive(Debug, Clone)]
pub struct ChunkLoadEvent {
    /// Chunk X coordinate.
    pub chunk_x: i32,
    /// Chunk Z coordinate.
    pub chunk_z: i32,
    /// The level / dimension name.
    pub level_name: String,
}

impl Event for ChunkLoadEvent {
    fn event_name(&self) -> &str {
        "ChunkLoadEvent"
    }
}

/// Fired when a chunk is unloaded.
#[derive(Debug, Clone)]
pub struct ChunkUnloadEvent {
    /// Chunk X coordinate.
    pub chunk_x: i32,
    /// Chunk Z coordinate.
    pub chunk_z: i32,
    /// The level / dimension name.
    pub level_name: String,
}

impl Event for ChunkUnloadEvent {
    fn event_name(&self) -> &str {
        "ChunkUnloadEvent"
    }
}

/// Fired when a level (dimension) is loaded.
#[derive(Debug, Clone)]
pub struct LevelLoadEvent {
    /// The level / dimension name.
    pub level_name: String,
}

impl Event for LevelLoadEvent {
    fn event_name(&self) -> &str {
        "LevelLoadEvent"
    }
}

// ---------------------------------------------------------------------------
// Inventory Events
// ---------------------------------------------------------------------------

/// Fired when a player opens an inventory.
#[derive(Debug, Clone)]
pub struct InventoryOpenEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The player's runtime ID.
    pub player_runtime_id: u64,
    /// The inventory ID.
    pub inventory_id: u64,
}

impl Event for InventoryOpenEvent {
    fn event_name(&self) -> &str {
        "InventoryOpenEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}

/// Fired when a player closes an inventory.
#[derive(Debug, Clone)]
pub struct InventoryCloseEvent {
    /// The player's runtime ID.
    pub player_runtime_id: u64,
    /// The inventory ID.
    pub inventory_id: u64,
}

impl Event for InventoryCloseEvent {
    fn event_name(&self) -> &str {
        "InventoryCloseEvent"
    }
}

/// Fired when an inventory transaction occurs.
#[derive(Debug, Clone)]
pub struct InventoryTransactionEvent {
    /// Cancellation state.
    pub cancel: CancellableEvent,
    /// The player's runtime ID.
    pub player_runtime_id: u64,
    /// The inventory ID.
    pub inventory_id: u64,
    /// The slot index involved.
    pub slot: u32,
}

impl Event for InventoryTransactionEvent {
    fn event_name(&self) -> &str {
        "InventoryTransactionEvent"
    }

    fn is_cancellable(&self) -> bool {
        true
    }
}
