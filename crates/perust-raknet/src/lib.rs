//! # perust-raknet
//!
//! RakNet protocol implementation for Minecraft Bedrock Edition.
//!
//! This crate provides a complete RakNet networking layer for the PeRust
//! Minecraft Bedrock Edition server, including:
//!
//! - **Protocol packets**: All RakNet packet types with binary encode/decode
//! - **Reliability system**: 8 reliability types with ACK/NACK tracking
//! - **Session management**: Per-connection state, ordering, and sequencing
//! - **Async UDP server**: Tokio-based server with 100 TPS tick loop
//!
//! # Quick Start
//!
//! ```no_run
//! use perust_raknet::server::RakNetServer;
//! use perust_raknet::reliability::Reliability;
//!
//! #[tokio::main]
//! async fn main() {
//!     let server = RakNetServer::new("0.0.0.0", 19132)
//!         .await
//!         .expect("Failed to create RakNet server");
//!
//!     server.set_motd("PeRust Server").await;
//!     server.set_on_packet(Box::new(|addr, data| {
//!         println!("Packet from {}: {} bytes", addr, data.len());
//!     })).await;
//!
//!     let handle = server.start();
//!     // ...
//!     server.stop();
//!     let _ = handle.await;
//! }
//! ```

pub mod encapsulated;
pub mod error;
pub mod protocol;
pub mod reliability;
pub mod server;
pub mod session;
pub mod session_manager;
pub mod session_ref;

// Re-export commonly used types at the crate root.
pub use error::RakNetError;
pub use protocol::{
    RAKNET_MAGIC, RAKNET_PROTOCOL_VERSION, MIN_MTU_SIZE, DEFAULT_MTU_SIZE,
    SocketAddress, PacketId,
};
pub use reliability::Reliability;
pub use encapsulated::{EncapsulatedPacket, SplitPacket};
pub use session::{Session, SessionState, OrderingChannel, ORDERING_CHANNEL_COUNT};
pub use session_manager::SessionManager;
pub use server::RakNetServer;
