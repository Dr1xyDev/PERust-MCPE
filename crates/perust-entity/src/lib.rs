//! # perust-entity
//!
//! Entity management crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **Entity**: Base entity type with position, motion, health, and metadata
//! - **LivingEntity**: Living entity with effects, equipment, and attributes
//! - **PlayerEntity**: Player-specific entity with gamemode, food, and experience
//! - **EntityManager**: Thread-safe entity storage with ID allocation
//! - **Metadata**: Entity metadata system matching the MCPE protocol
//! - **Effect**: Status effects (Speed, Slowness, Haste, etc.)
//! - **Attribute**: Entity attribute system (health, movement speed, etc.)
//! - **Error**: Entity-specific error types

pub mod error;
pub mod metadata;
pub mod effect;
pub mod attribute;
pub mod entity;
pub mod living;
pub mod player_entity;
pub mod entity_manager;

pub use error::EntityError;
pub use metadata::{EntityMetadata, MetadataType, MetadataValue};
pub use effect::Effect;
pub use attribute::Attribute;
pub use entity::{Entity, EntityDataType};
pub use living::LivingEntity;
pub use player_entity::PlayerEntity;
pub use entity_manager::EntityManager;
