//! # perust-storage
//!
//! File storage abstractions for PeRust, providing read/write access to
//! JSON, YAML, NBT, and Region (Anvil) file formats.
//!
//! This crate provides:
//! - **JsonStorage**: Read/write JSON files with serde support
//! - **YamlStorage**: Read/write YAML files with serde support
//! - **NbtStorage**: Read/write NBT files (plain and compressed)
//! - **RegionStorage**: Read/write Anvil-format region files for chunk data

pub mod error;
pub mod json_storage;
pub mod yaml_storage;
pub mod nbt_storage;
pub mod region_storage;

pub use error::StorageError;
pub use json_storage::JsonStorage;
pub use yaml_storage::YamlStorage;
pub use nbt_storage::NbtStorage;
pub use region_storage::{RegionFile, RegionStorage, SECTOR_SIZE, REGION_CHUNKS};
