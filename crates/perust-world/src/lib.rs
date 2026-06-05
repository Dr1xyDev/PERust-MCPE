//! # perust-world
//!
//! World management crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **Chunk**: Chunk and sub-chunk data structures with block storage
//! - **ChunkManager**: Thread-safe chunk storage with radius-based queries
//! - **Generator**: Terrain generation traits (Flat, Void, Normal)
//! - **Region**: Anvil-style region file reader/writer
//! - **World**: Top-level world management with tick loop and persistence
//! - **Biome**: Biome enum and biome selection utilities
//! - **Error**: World-specific error types

pub mod error;
pub mod biome;
pub mod chunk;
pub mod chunk_manager;
pub mod generator;
pub mod region;
pub mod world;

pub use error::WorldError;
pub use biome::Biome;
pub use chunk::{Chunk, SubChunk, CHUNK_SIZE, MAX_SUBCHUNKS, SUBCHUNK_SIZE, BIOME_SIZE};
pub use chunk_manager::ChunkManager;
pub use generator::{Generator, FlatGenerator, VoidGenerator, NormalGenerator};
pub use region::{RegionFile, REGION_SIZE};
pub use world::World;
