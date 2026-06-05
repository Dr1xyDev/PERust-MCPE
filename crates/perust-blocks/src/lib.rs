//! # perust-blocks
//!
//! Block definitions and registry for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **block_ids**: Constants for all block IDs matching MCPE v113
//! - **Block**: Block type definition with properties (hardness, resistance, light, etc.)
//! - **ToolType**: Tool type classification for blocks
//! - **BlockRegistry**: Global registry of all block types with lookup by ID or name
//! - **BlockState**: Block state representation for runtime ID mapping
//! - **BlockStateRegistry**: Registry mapping block states to runtime IDs

pub mod block_ids;
pub mod block;
pub mod block_state;

// Re-export commonly used types at crate root
pub use block_ids::*;
pub use block::{Block, ToolType, BlockRegistry, BLOCK_REGISTRY};
pub use block_state::{BlockState, BlockStateRegistry, BLOCK_STATE_REGISTRY};
