//! # perust-items
//!
//! Item definitions and registry for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **item_ids**: Constants for all item IDs matching MCPE v113
//! - **Item**: Item type definition with properties (stack size, durability, damage, etc.)
//! - **ToolType**: Tool type classification for items
//! - **ToolTier**: Tool tier classification (wooden, stone, iron, diamond, gold)
//! - **ItemRegistry**: Global registry of all item types with lookup by ID or name
//! - **EnchantmentType**: All enchantment types matching MCPE protocol
//! - **Enchantment**: Enchantment instance with type and level

pub mod item_ids;
pub mod item;
pub mod item_registry;
pub mod enchantment;

// Re-export commonly used types at crate root
pub use item_ids::*;
pub use item::{Item, ToolType, ToolTier};
pub use item_registry::{ItemRegistry, ITEM_REGISTRY};
pub use enchantment::{EnchantmentType, Enchantment};
