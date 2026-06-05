//! # perust-utils
//!
//! Core utility crates for PeRust, a Minecraft Bedrock Edition server written in Rust.
//!
//! This crate provides fundamental utilities used across the entire server codebase:
//!
//! - **binary**: VarInt/VarLong encoding (Minecraft protocol compatible), `BinaryReader` and
//!   `BinaryWriter` for reading/writing network packets
//! - **varint**: VarInt/VarLong type wrappers for NBT network format
//! - **color**: Terminal/console ANSI color formatting and styled string building
//! - **math**: Vector types (`Vector2`, `Vector3`), `BlockPos`, `BoundingBox`, facing directions
//! - **singleton**: Thread-safe lazy singleton pattern
//! - **pool**: Generic object pool for reuse of expensive-to-create objects
//! - **identity**: Atomic `RuntimeId` allocator for unique runtime identifiers

pub mod binary;
pub mod varint;
pub mod color;
pub mod identity;
pub mod math;
pub mod pool;
pub mod singleton;

// Re-export commonly used types at the crate root for convenience.
pub use binary::{BinaryReader, BinaryWriter};
