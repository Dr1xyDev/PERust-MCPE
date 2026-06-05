//! Error types for the perust-network crate.

use thiserror::Error;

/// Errors that can occur during network operations.
#[derive(Debug, Error)]
pub enum NetworkError {
    /// Failed to establish a connection.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// No session found for the given address.
    #[error("session not found: {0}")]
    SessionNotFound(String),

    /// An error occurred while processing a packet.
    #[error("packet error: {0}")]
    PacketError(String),

    /// A timeout occurred during a network operation.
    #[error("timeout: {0}")]
    Timeout(String),

    /// A RakNet protocol error occurred.
    #[error("raknet error: {0}")]
    RakNetError(#[from] perust_raknet::RakNetError),

    /// A MCPE protocol error occurred.
    #[error("protocol error: {0}")]
    ProtocolError(#[from] perust_protocol::error::ProtocolError),

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
