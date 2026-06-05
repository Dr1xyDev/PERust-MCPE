//! Error types for the perust-player crate.

use thiserror::Error;

/// Errors that can occur during player operations.
#[derive(Debug, Error)]
pub enum PlayerError {
    /// The player is not connected to the server.
    #[error("player is not connected")]
    NotConnected,

    /// The player is in an invalid state for the requested operation.
    #[error("invalid player state: {0}")]
    InvalidState(String),

    /// The player's login attempt failed.
    #[error("login failed: {0}")]
    LoginFailed(String),

    /// The player does not have permission to perform the action.
    #[error("permission denied: {0}")]
    PermissionDenied(String),

    /// The player was kicked from the server.
    #[error("kicked: {0}")]
    Kick(String),

    /// A timeout occurred during a player operation.
    #[error("timeout: {0}")]
    Timeout(String),

    /// A protocol error occurred.
    #[error("protocol error: {0}")]
    ProtocolError(#[from] perust_protocol::error::ProtocolError),

    /// An inventory error occurred.
    #[error("inventory error: {0}")]
    InventoryError(#[from] perust_inventory::InventoryError),
}
