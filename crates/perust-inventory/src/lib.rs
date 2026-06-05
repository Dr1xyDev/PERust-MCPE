//! # perust-inventory
//!
//! Inventory system for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **ItemStack**: Representation of a stack of items in an inventory slot
//! - **InventoryType**: Enum of all inventory types (chest, furnace, player, etc.)
//! - **Inventory**: Trait for generic inventory operations
//! - **BaseInventory**: Basic inventory implementation with fixed slots
//! - **PlayerInventory**: Player-specific inventory with hotbar, armor, and offhand
//! - **ContainerInventory**: Block-based container inventories (chests, furnaces, etc.)
//! - **Transaction**: Inventory transaction system for validating and applying changes

pub mod error;
pub mod item_stack;
pub mod inventory_type;
pub mod inventory;
pub mod player_inventory;
pub mod container_inventory;
pub mod transaction;

// Re-export commonly used types at crate root
pub use error::InventoryError;
pub use item_stack::ItemStack;
pub use inventory_type::InventoryType;
pub use inventory::{Inventory, BaseInventory};
pub use player_inventory::PlayerInventory;
pub use container_inventory::ContainerInventory;
pub use transaction::{Transaction, TransactionQueue};
