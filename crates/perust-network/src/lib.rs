//! # perust-network
//!
//! Network management crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate bridges the RakNet transport layer with the MCPE protocol layer,
//! providing:
//! - **NetworkSession**: Per-connection MCPE session state wrapping a RakNet session
//! - **NetworkManager**: Top-level network coordinator managing RakNet + MCPE sessions
//! - **PacketHandler**: Incoming packet processor with callbacks to the server
//! - **NetworkError**: Network-specific error types

pub mod error;
pub mod network_session;
pub mod network_manager;
pub mod packet_handler;

// Re-export commonly used types at crate root
pub use error::NetworkError;
pub use network_session::{NetworkSession, SessionState, PacketType};
pub use network_manager::NetworkManager;
pub use packet_handler::{PacketHandler, LoginResult};
