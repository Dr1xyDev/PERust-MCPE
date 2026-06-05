//! Error types for the RakNet protocol implementation.

use std::net::SocketAddr;

use thiserror::Error;

/// Errors that can occur during RakNet protocol operations.
#[derive(Error, Debug)]
pub enum RakNetError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The received packet is invalid or malformed.
    #[error("invalid packet: {0}")]
    InvalidPacket(String),

    /// No session found for the given address.
    #[error("session not found for {0}")]
    SessionNotFound(SocketAddr),

    /// A timeout occurred during a network operation.
    #[error("timeout: {0}")]
    Timeout(String),

    /// The connection was rejected by the server.
    #[error("connection rejected: {0}")]
    ConnectionRejected(String),

    /// The client protocol version does not match the server.
    #[error("protocol mismatch: expected {expected}, got {actual}")]
    ProtocolMismatch {
        /// The protocol version expected by the server.
        expected: u8,
        /// The protocol version sent by the client.
        actual: u8,
    },

    /// The server is full and cannot accept more connections.
    #[error("server full: {0}")]
    ServerFull(String),

    /// A packet exceeded the maximum MTU size.
    #[error("packet exceeds MTU size: {size} > {mtu}")]
    PacketTooLarge {
        /// The actual size of the packet.
        size: usize,
        /// The maximum MTU size.
        mtu: u16,
    },

    /// Failed to decode a packet from binary.
    #[error("decode error: {0}")]
    DecodeError(String),

    /// Failed to encode a packet to binary.
    #[error("encode error: {0}")]
    EncodeError(String),

    /// The session is in an invalid state for the requested operation.
    #[error("invalid session state: {0}")]
    InvalidState(String),

    /// Failed to reassemble a split packet.
    #[error("split packet reassembly failed: {0}")]
    SplitPacketError(String),
}
