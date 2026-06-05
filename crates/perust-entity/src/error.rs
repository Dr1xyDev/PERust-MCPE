//! Error types for entity operations.

use thiserror::Error;

/// Errors that can occur during entity operations.
#[derive(Debug, Error)]
pub enum EntityError {
    /// The requested entity was not found.
    #[error("entity not found: {0}")]
    EntityNotFound(u64),

    /// An invalid entity type was specified.
    #[error("invalid entity type: {0}")]
    InvalidEntityType(String),

    /// The entity is already dead.
    #[error("entity {0} is already dead")]
    EntityAlreadyDead(u64),

    /// The effect could not be applied.
    #[error("effect error: {0}")]
    EffectError(String),

    /// An attribute error occurred.
    #[error("attribute error: {0}")]
    AttributeError(String),

    /// An NBT error occurred.
    #[error("NBT error: {0}")]
    NbtError(#[from] perust_nbt::NbtError),
}
