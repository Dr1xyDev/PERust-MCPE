//! # perust-player
//!
//! Player management crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **Player**: The main player struct with identity, connection, world state, inventory, and stats
//! - **LoginState**: Login flow tracking enum
//! - **PlayerList**: Thread-safe player list manager (tab list)
//! - **PlayerPermissions**: Permission system with attachments and operator status
//! - **PlayerError**: Player-specific error types

pub mod error;
pub mod login_state;
pub mod player;
pub mod player_list;
pub mod permission;

// Re-export commonly used types at crate root
pub use error::PlayerError;
pub use login_state::LoginState;
pub use player::Player;
pub use player_list::PlayerList;
pub use permission::{PermissionAttachment, PlayerPermissions};
