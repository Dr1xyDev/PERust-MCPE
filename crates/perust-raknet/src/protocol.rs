//! RakNet protocol packet structures and encode/decode implementations.
//!
//! This module defines all RakNet-level packets used during connection setup,
//! maintenance, and teardown. Each packet provides `encode` and `decode` methods
//! for binary serialization.

use crate::error::RakNetError;

/// RakNet protocol version used by Minecraft Bedrock Edition.
pub const RAKNET_PROTOCOL_VERSION: u8 = 6;

/// The RakNet "magic" bytes that identify valid RakNet packets.
pub const RAKNET_MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
    0x78,
];

/// Minimum MTU size allowed during negotiation.
pub const MIN_MTU_SIZE: u16 = 400;

/// Default maximum MTU size.
pub const DEFAULT_MTU_SIZE: u16 = 1492;

// ---------------------------------------------------------------------------
// RakNet Message IDs
// ---------------------------------------------------------------------------

/// ID_CONNECTED_PING — Used to measure latency on an established connection.
pub const ID_CONNECTED_PING: u8 = 0x00;
/// ID_UNCONNECTED_PING — Ping from an unconnected client (server discovery).
pub const ID_UNCONNECTED_PING: u8 = 0x01;
/// ID_UNCONNECTED_PING_OPEN_CONNECTIONS — Like UnconnectedPing but only if open.
pub const ID_UNCONNECTED_PING_OPEN_CONNECTIONS: u8 = 0x02;
/// ID_CONNECTED_PONG — Response to a ConnectedPing.
pub const ID_CONNECTED_PONG: u8 = 0x03;
/// ID_DETECT_LOST_CONNECTIONS — Probe to detect lost connections.
pub const ID_DETECT_LOST_CONNECTIONS: u8 = 0x04;
/// ID_OPEN_CONNECTION_REQUEST_1 — First step of the connection handshake.
pub const ID_OPEN_CONNECTION_REQUEST_1: u8 = 0x05;
/// ID_OPEN_CONNECTION_REPLY_1 — Server response to OpenConnectionRequest1.
pub const ID_OPEN_CONNECTION_REPLY_1: u8 = 0x06;
/// ID_OPEN_CONNECTION_REQUEST_2 — Second step of the connection handshake.
pub const ID_OPEN_CONNECTION_REQUEST_2: u8 = 0x07;
/// ID_OPEN_CONNECTION_REPLY_2 — Server response to OpenConnectionRequest2.
pub const ID_OPEN_CONNECTION_REPLY_2: u8 = 0x08;
/// ID_CONNECTION_REQUEST — Request to finalize the connection.
pub const ID_CONNECTION_REQUEST: u8 = 0x09;
/// ID_CONNECTION_REQUEST_ACCEPTED — Server accepts the connection request.
pub const ID_CONNECTION_REQUEST_ACCEPTED: u8 = 0x10;
/// ID_NEW_INCOMING_CONNECTION — Client confirms the accepted connection.
pub const ID_NEW_INCOMING_CONNECTION: u8 = 0x13;
/// ID_DISCONNECTION_NOTIFICATION — Client disconnected gracefully.
pub const ID_DISCONNECTION_NOTIFICATION: u8 = 0x15;
/// ID_CONNECTION_LOST — Connection lost unexpectedly.
pub const ID_CONNECTION_LOST: u8 = 0x16;
/// ID_INCOMPATIBLE_PROTOCOL_VERSION — Protocol version mismatch.
pub const ID_INCOMPATIBLE_PROTOCOL_VERSION: u8 = 0x19;
/// ID_UNCONNECTED_PONG — Response to UnconnectedPing (server discovery).
pub const ID_UNCONNECTED_PONG: u8 = 0x1c;
/// ID_ADVERTISE_SYSTEM — Server advertisement broadcast.
pub const ID_ADVERTISE_SYSTEM: u8 = 0x1d;

// Connected (encapsulated) packet frame IDs
/// A datagram carrying encapsulated packets.
pub const ID_DATAGRAM: u8 = 0x84;
/// An ACK (acknowledgment) for received reliable packets.
pub const ID_ACK: u8 = 0xc0;
/// A NACK (negative acknowledgment) for missing packets.
pub const ID_NACK: u8 = 0xa0;

// ---------------------------------------------------------------------------
// SocketAddress
// ---------------------------------------------------------------------------

/// A network address supporting both IPv4 and IPv6, as encoded in RakNet packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketAddress {
    /// An IPv4 address (4 bytes) + port.
    V4 {
        /// The 4 bytes of the IPv4 address.
        ip: [u8; 4],
        /// The port number.
        port: u16,
    },
    /// An IPv6 address (16 bytes) + port.
    V6 {
        /// The 16 bytes of the IPv6 address.
        ip: [u8; 16],
        /// The port number.
        port: u16,
    },
}

impl SocketAddress {
    /// The version byte for IPv4 in RakNet's address encoding.
    pub const IPV4: u8 = 4;
    /// The version byte for IPv6 in RakNet's address encoding.
    pub const IPV6: u8 = 6;

    /// Creates an IPv4 socket address from bytes and port.
    pub fn ipv4(a: u8, b: u8, c: u8, d: u8, port: u16) -> Self {
        SocketAddress::V4 {
            ip: [a, b, c, d],
            port,
        }
    }

    /// Returns the port number.
    pub fn port(&self) -> u16 {
        match self {
            SocketAddress::V4 { port, .. } => *port,
            SocketAddress::V6 { port, .. } => *port,
        }
    }

    /// Returns the address version byte (4 for IPv4, 6 for IPv6).
    pub fn version(&self) -> u8 {
        match self {
            SocketAddress::V4 { .. } => Self::IPV4,
            SocketAddress::V6 { .. } => Self::IPV6,
        }
    }

    /// Encodes this address into a byte vector.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(19);
        match self {
            SocketAddress::V4 { ip, port } => {
                buf.push(Self::IPV4);
                buf.extend_from_slice(ip);
                buf.extend_from_slice(&port.to_be_bytes());
            }
            SocketAddress::V6 { ip, port } => {
                buf.push(Self::IPV6);
                buf.extend_from_slice(ip);
                buf.extend_from_slice(&port.to_be_bytes());
            }
        }
        buf
    }

    /// Decodes a socket address from a byte slice starting at `offset`.
    ///
    /// Returns the address and the number of bytes consumed.
    pub fn decode(data: &[u8], offset: usize) -> Result<(Self, usize), RakNetError> {
        if data.len() <= offset {
            return Err(RakNetError::DecodeError(
                "not enough data for socket address version byte".to_string(),
            ));
        }
        let version = data[offset];
        match version {
            Self::IPV4 => {
                if data.len() < offset + 1 + 4 + 2 {
                    return Err(RakNetError::DecodeError(
                        "not enough data for IPv4 socket address".to_string(),
                    ));
                }
                let mut ip = [0u8; 4];
                ip.copy_from_slice(&data[offset + 1..offset + 5]);
                let port = u16::from_be_bytes([data[offset + 5], data[offset + 6]]);
                Ok((SocketAddress::V4 { ip, port }, 1 + 4 + 2))
            }
            Self::IPV6 => {
                if data.len() < offset + 1 + 16 + 2 {
                    return Err(RakNetError::DecodeError(
                        "not enough data for IPv6 socket address".to_string(),
                    ));
                }
                let mut ip = [0u8; 16];
                ip.copy_from_slice(&data[offset + 1..offset + 17]);
                let port = u16::from_be_bytes([data[offset + 17], data[offset + 18]]);
                Ok((SocketAddress::V6 { ip, port }, 1 + 16 + 2))
            }
            _ => Err(RakNetError::DecodeError(format!(
                "unknown socket address version: {}",
                version
            ))),
        }
    }
}

impl std::fmt::Display for SocketAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketAddress::V4 { ip, port } => {
                write!(f, "{}.{}.{}.{}:{}", ip[0], ip[1], ip[2], ip[3], port)
            }
            SocketAddress::V6 { ip, port } => {
                // Format as standard IPv6 hex groups
                let segments: Vec<String> = (0..8)
                    .map(|i| {
                        let val = u16::from_be_bytes([ip[i * 2], ip[i * 2 + 1]]);
                        format!("{:x}", val)
                    })
                    .collect();
                write!(f, "[{}]:{}", segments.join(":"), port)
            }
        }
    }
}

impl From<std::net::SocketAddr> for SocketAddress {
    fn from(addr: std::net::SocketAddr) -> Self {
        match addr {
            std::net::SocketAddr::V4(v4) => {
                let octets = v4.ip().octets();
                SocketAddress::V4 {
                    ip: octets,
                    port: v4.port(),
                }
            }
            std::net::SocketAddr::V6(v6) => {
                let octets = v6.ip().octets();
                SocketAddress::V6 {
                    ip: octets,
                    port: v6.port(),
                }
            }
        }
    }
}

/// The default socket address used for the 20 internal/system addresses
/// in connection handshake packets.
pub fn default_system_addresses() -> [SocketAddress; 20] {
    let default = SocketAddress::ipv4(0, 0, 0, 0, 0);
    [
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default.clone(),
        default,
    ]
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Reads exactly N bytes from the data at the given offset, advancing the offset.
fn read_bytes(data: &[u8], offset: &mut usize, n: usize) -> Result<Vec<u8>, RakNetError> {
    if data.len() < *offset + n {
        return Err(RakNetError::DecodeError(format!(
            "not enough data: need {} bytes at offset {}, have {}",
            n,
            *offset,
            data.len()
        )));
    }
    let result = data[*offset..*offset + n].to_vec();
    *offset += n;
    Ok(result)
}

/// Reads a single u8.
fn read_u8(data: &[u8], offset: &mut usize) -> Result<u8, RakNetError> {
    if data.len() <= *offset {
        return Err(RakNetError::DecodeError("not enough data for u8".to_string()));
    }
    let val = data[*offset];
    *offset += 1;
    Ok(val)
}

/// Reads a big-endian u16.
fn read_u16_be(data: &[u8], offset: &mut usize) -> Result<u16, RakNetError> {
    if data.len() < *offset + 2 {
        return Err(RakNetError::DecodeError(
            "not enough data for u16".to_string(),
        ));
    }
    let val = u16::from_be_bytes([data[*offset], data[*offset + 1]]);
    *offset += 2;
    Ok(val)
}

/// Reads a big-endian i64.
fn read_i64_be(data: &[u8], offset: &mut usize) -> Result<i64, RakNetError> {
    if data.len() < *offset + 8 {
        return Err(RakNetError::DecodeError(
            "not enough data for i64".to_string(),
        ));
    }
    let val = i64::from_be_bytes([
        data[*offset],
        data[*offset + 1],
        data[*offset + 2],
        data[*offset + 3],
        data[*offset + 4],
        data[*offset + 5],
        data[*offset + 6],
        data[*offset + 7],
    ]);
    *offset += 8;
    Ok(val)
}

/// Validates that the magic bytes at the given offset are correct.
fn validate_magic(data: &[u8], offset: usize) -> Result<(), RakNetError> {
    if data.len() < offset + 16 {
        return Err(RakNetError::DecodeError(
            "not enough data for magic bytes".to_string(),
        ));
    }
    if &data[offset..offset + 16] != RAKNET_MAGIC {
        return Err(RakNetError::InvalidPacket(
            "invalid magic bytes".to_string(),
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Packet Structures
// ---------------------------------------------------------------------------

/// UnconnectedPing — sent by a client to discover a RakNet server.
#[derive(Debug, Clone)]
pub struct UnconnectedPing {
    /// Client timestamp (used for round-trip time calculation).
    pub time: i64,
    /// RakNet magic bytes.
    pub magic: [u8; 16],
    /// The client's unique GUID.
    pub client_guid: i64,
}

impl UnconnectedPing {
    /// Decodes an UnconnectedPing from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        let time = read_i64_be(data, &mut offset)?;
        validate_magic(data, offset)?;
        let mut magic = [0u8; 16];
        magic.copy_from_slice(&data[offset..offset + 16]);
        offset += 16;
        let client_guid = read_i64_be(data, &mut offset)?;
        Ok(UnconnectedPing {
            time,
            magic,
            client_guid,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 8 + 16 + 8);
        buf.push(ID_UNCONNECTED_PING);
        buf.extend_from_slice(&self.time.to_be_bytes());
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.client_guid.to_be_bytes());
        buf
    }
}

/// UnconnectedPong — server response to UnconnectedPing, containing the MOTD.
#[derive(Debug, Clone)]
pub struct UnconnectedPong {
    /// Echoed client timestamp from the ping.
    pub time: i64,
    /// The server's unique GUID.
    pub server_guid: i64,
    /// RakNet magic bytes.
    pub magic: [u8; 16],
    /// The server's Message of the Day (MOTD).
    pub motd: String,
}

impl UnconnectedPong {
    /// Creates a new UnconnectedPong.
    pub fn new(time: i64, server_guid: i64, motd: String) -> Self {
        UnconnectedPong {
            time,
            server_guid,
            magic: RAKNET_MAGIC,
            motd,
        }
    }

    /// Decodes an UnconnectedPong from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        let time = read_i64_be(data, &mut offset)?;
        let server_guid = read_i64_be(data, &mut offset)?;
        validate_magic(data, offset)?;
        let mut magic = [0u8; 16];
        magic.copy_from_slice(&data[offset..offset + 16]);
        offset += 16;
        // MOTD is a length-prefixed string: 2-byte big-endian length + UTF-8 data
        let motd_len = read_u16_be(data, &mut offset)? as usize;
        let motd_bytes = read_bytes(data, &mut offset, motd_len)?;
        let motd = String::from_utf8(motd_bytes)
            .map_err(|e| RakNetError::DecodeError(format!("invalid MOTD UTF-8: {}", e)))?;
        Ok(UnconnectedPong {
            time,
            server_guid,
            magic,
            motd,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let motd_bytes = self.motd.as_bytes();
        let mut buf = Vec::with_capacity(1 + 8 + 8 + 16 + 2 + motd_bytes.len());
        buf.push(ID_UNCONNECTED_PONG);
        buf.extend_from_slice(&self.time.to_be_bytes());
        buf.extend_from_slice(&self.server_guid.to_be_bytes());
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&(motd_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(motd_bytes);
        buf
    }
}

/// OpenConnectionRequest1 — first step of the RakNet connection handshake.
///
/// Wire format: `[PacketID] [Magic 16 bytes] [Protocol 1 byte] [Padding...]`
#[derive(Debug, Clone)]
pub struct OpenConnectionRequest1 {
    /// RakNet magic bytes.
    pub magic: [u8; 16],
    /// The RakNet protocol version.
    pub protocol: u8,
    /// The MTU size proposed by the client (padding is used to probe path MTU).
    pub mtu_size: u16,
}

impl OpenConnectionRequest1 {
    /// Decodes an OpenConnectionRequest1 from a byte slice (excluding the packet ID byte).
    ///
    /// Wire order after packet ID: Magic (16 bytes) → Protocol (1 byte) → Padding
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        // FIX: Magic comes FIRST, then protocol
        validate_magic(data, offset)?;
        let mut magic = [0u8; 16];
        magic.copy_from_slice(&data[offset..offset + 16]);
        offset += 16;
        let protocol = read_u8(data, &mut offset)?;
        // MTU size is inferred from the total packet length (the rest is padding)
        let mtu_size = (data.len() + 1) as u16; // +1 for the packet ID byte
        Ok(OpenConnectionRequest1 {
            magic,
            protocol,
            mtu_size,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    ///
    /// The packet is padded to `mtu_size` bytes to probe the path MTU.
    ///
    /// Wire order: `[0x05] [Magic 16 bytes] [Protocol 1 byte] [Padding...]`
    pub fn encode(&self) -> Vec<u8> {
        // FIX: Header is PacketID(1) + Magic(16) + Protocol(1) = 18 bytes
        let header_size = 1 + 16 + 1;
        let padding_size = self.mtu_size as usize - header_size;
        let mut buf = Vec::with_capacity(self.mtu_size as usize);
        buf.push(ID_OPEN_CONNECTION_REQUEST_1);
        // FIX: Magic FIRST, then protocol
        buf.extend_from_slice(&self.magic);
        buf.push(self.protocol);
        // Pad with zeros to reach the MTU size
        buf.extend(std::iter::repeat(0u8).take(padding_size));
        buf
    }
}

/// OpenConnectionReply1 — server response to OpenConnectionRequest1.
#[derive(Debug, Clone)]
pub struct OpenConnectionReply1 {
    /// The server's unique GUID.
    pub server_guid: i64,
    /// RakNet magic bytes.
    pub magic: [u8; 16],
    /// Whether security/encryption is used (typically false for Bedrock).
    pub use_security: bool,
    /// The negotiated MTU size.
    pub mtu_size: u16,
}

impl OpenConnectionReply1 {
    /// Creates a new OpenConnectionReply1.
    pub fn new(server_guid: i64, mtu_size: u16) -> Self {
        OpenConnectionReply1 {
            server_guid,
            magic: RAKNET_MAGIC,
            use_security: false,
            mtu_size,
        }
    }

    /// Decodes an OpenConnectionReply1 from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        validate_magic(data, offset)?;
        let mut magic = [0u8; 16];
        magic.copy_from_slice(&data[offset..offset + 16]);
        offset += 16;
        let server_guid = read_i64_be(data, &mut offset)?;
        let use_security = read_u8(data, &mut offset)? != 0;
        let mtu_size = read_u16_be(data, &mut offset)?;
        Ok(OpenConnectionReply1 {
            server_guid,
            magic,
            use_security,
            mtu_size,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 16 + 8 + 1 + 2);
        buf.push(ID_OPEN_CONNECTION_REPLY_1);
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.server_guid.to_be_bytes());
        buf.push(if self.use_security { 1 } else { 0 });
        buf.extend_from_slice(&self.mtu_size.to_be_bytes());
        buf
    }
}

/// OpenConnectionRequest2 — second step of the RakNet connection handshake.
#[derive(Debug, Clone)]
pub struct OpenConnectionRequest2 {
    /// RakNet magic bytes.
    pub magic: [u8; 16],
    /// The server address the client is connecting to.
    pub server_address: SocketAddress,
    /// The negotiated MTU size.
    pub mtu_size: u16,
    /// The client's unique GUID.
    pub client_guid: i64,
}

impl OpenConnectionRequest2 {
    /// Decodes an OpenConnectionRequest2 from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        validate_magic(data, offset)?;
        let mut magic = [0u8; 16];
        magic.copy_from_slice(&data[offset..offset + 16]);
        offset += 16;
        let (server_address, addr_len) = SocketAddress::decode(data, offset)?;
        offset += addr_len;
        let mtu_size = read_u16_be(data, &mut offset)?;
        let client_guid = read_i64_be(data, &mut offset)?;
        Ok(OpenConnectionRequest2 {
            magic,
            server_address,
            mtu_size,
            client_guid,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 16 + 19 + 2 + 8);
        buf.push(ID_OPEN_CONNECTION_REQUEST_2);
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.server_address.encode());
        buf.extend_from_slice(&self.mtu_size.to_be_bytes());
        buf.extend_from_slice(&self.client_guid.to_be_bytes());
        buf
    }
}

/// OpenConnectionReply2 — server response to OpenConnectionRequest2.
#[derive(Debug, Clone)]
pub struct OpenConnectionReply2 {
    /// The server's unique GUID.
    pub server_guid: i64,
    /// RakNet magic bytes.
    pub magic: [u8; 16],
    /// The client's address as seen by the server.
    pub client_address: SocketAddress,
    /// The negotiated MTU size.
    pub mtu_size: u16,
    /// Whether security/encryption is used.
    pub use_security: bool,
}

impl OpenConnectionReply2 {
    /// Creates a new OpenConnectionReply2.
    pub fn new(server_guid: i64, client_address: SocketAddress, mtu_size: u16) -> Self {
        OpenConnectionReply2 {
            server_guid,
            magic: RAKNET_MAGIC,
            client_address,
            mtu_size,
            use_security: false,
        }
    }

    /// Decodes an OpenConnectionReply2 from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        validate_magic(data, offset)?;
        let mut magic = [0u8; 16];
        magic.copy_from_slice(&data[offset..offset + 16]);
        offset += 16;
        let server_guid = read_i64_be(data, &mut offset)?;
        let (client_address, addr_len) = SocketAddress::decode(data, offset)?;
        offset += addr_len;
        let mtu_size = read_u16_be(data, &mut offset)?;
        let use_security = read_u8(data, &mut offset)? != 0;
        Ok(OpenConnectionReply2 {
            server_guid,
            magic,
            client_address,
            mtu_size,
            use_security,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 16 + 8 + 19 + 2 + 1);
        buf.push(ID_OPEN_CONNECTION_REPLY_2);
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.server_guid.to_be_bytes());
        buf.extend_from_slice(&self.client_address.encode());
        buf.extend_from_slice(&self.mtu_size.to_be_bytes());
        buf.push(if self.use_security { 1 } else { 0 });
        buf
    }
}

/// ConnectionRequest — sent by the client to finalize the connection.
#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    /// The client's unique GUID.
    pub client_guid: i64,
    /// The timestamp when the request was sent.
    pub request_time: i64,
    /// Whether security/encryption is used.
    pub use_security: bool,
}

impl ConnectionRequest {
    /// Decodes a ConnectionRequest from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        let client_guid = read_i64_be(data, &mut offset)?;
        let request_time = read_i64_be(data, &mut offset)?;
        let use_security = read_u8(data, &mut offset)? != 0;
        Ok(ConnectionRequest {
            client_guid,
            request_time,
            use_security,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 8 + 8 + 1);
        buf.push(ID_CONNECTION_REQUEST);
        buf.extend_from_slice(&self.client_guid.to_be_bytes());
        buf.extend_from_slice(&self.request_time.to_be_bytes());
        buf.push(if self.use_security { 1 } else { 0 });
        buf
    }
}

/// ConnectionRequestAccepted — server accepts the client's connection request.
#[derive(Debug, Clone)]
pub struct ConnectionRequestAccepted {
    /// The client's address as seen by the server.
    pub client_address: SocketAddress,
    /// System index (typically 0).
    pub system_index: u16,
    /// Up to 20 internal/system addresses.
    pub internal_ids: [SocketAddress; 20],
    /// The timestamp from the client's ConnectionRequest.
    pub request_time: i64,
    /// The server's timestamp when the request was accepted.
    pub accepted_time: i64,
}

impl ConnectionRequestAccepted {
    /// Creates a new ConnectionRequestAccepted with default internal addresses.
    pub fn new(client_address: SocketAddress, request_time: i64, accepted_time: i64) -> Self {
        ConnectionRequestAccepted {
            client_address,
            system_index: 0,
            internal_ids: default_system_addresses(),
            request_time,
            accepted_time,
        }
    }

    /// Decodes a ConnectionRequestAccepted from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        let (client_address, addr_len) = SocketAddress::decode(data, offset)?;
        offset += addr_len;
        let system_index = read_u16_be(data, &mut offset)?;
        let mut internal_ids = default_system_addresses();
        for i in 0..20 {
            let (addr, len) = SocketAddress::decode(data, offset)?;
            offset += len;
            internal_ids[i] = addr;
        }
        let request_time = read_i64_be(data, &mut offset)?;
        let accepted_time = read_i64_be(data, &mut offset)?;
        Ok(ConnectionRequestAccepted {
            client_address,
            system_index,
            internal_ids,
            request_time,
            accepted_time,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 19 + 2 + 20 * 19 + 8 + 8);
        buf.push(ID_CONNECTION_REQUEST_ACCEPTED);
        buf.extend_from_slice(&self.client_address.encode());
        buf.extend_from_slice(&self.system_index.to_be_bytes());
        for addr in &self.internal_ids {
            buf.extend_from_slice(&addr.encode());
        }
        buf.extend_from_slice(&self.request_time.to_be_bytes());
        buf.extend_from_slice(&self.accepted_time.to_be_bytes());
        buf
    }
}

/// NewIncomingConnection — sent by the client to confirm the accepted connection.
#[derive(Debug, Clone)]
pub struct NewIncomingConnection {
    /// The server address the client connected to.
    pub address: SocketAddress,
    /// Up to 20 system addresses.
    pub system_addresses: [SocketAddress; 20],
    /// The timestamp from the ConnectionRequestAccepted.
    pub request_time: i64,
    /// The client's accepted timestamp.
    pub accepted_time: i64,
}

impl NewIncomingConnection {
    /// Decodes a NewIncomingConnection from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        let (address, addr_len) = SocketAddress::decode(data, offset)?;
        offset += addr_len;
        let mut system_addresses = default_system_addresses();
        for i in 0..20 {
            let (addr, len) = SocketAddress::decode(data, offset)?;
            offset += len;
            system_addresses[i] = addr;
        }
        let request_time = read_i64_be(data, &mut offset)?;
        let accepted_time = read_i64_be(data, &mut offset)?;
        Ok(NewIncomingConnection {
            address,
            system_addresses,
            request_time,
            accepted_time,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 19 + 20 * 19 + 8 + 8);
        buf.push(ID_NEW_INCOMING_CONNECTION);
        buf.extend_from_slice(&self.address.encode());
        for addr in &self.system_addresses {
            buf.extend_from_slice(&addr.encode());
        }
        buf.extend_from_slice(&self.request_time.to_be_bytes());
        buf.extend_from_slice(&self.accepted_time.to_be_bytes());
        buf
    }
}

/// DisconnectionNotification — sent when a client disconnects gracefully.
#[derive(Debug, Clone)]
pub struct DisconnectionNotification;

impl DisconnectionNotification {
    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        vec![ID_DISCONNECTION_NOTIFICATION]
    }
}

/// ConnectionLost — indicates the connection was lost unexpectedly.
#[derive(Debug, Clone)]
pub struct ConnectionLost;

impl ConnectionLost {
    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        vec![ID_CONNECTION_LOST]
    }
}

/// IncompatibleProtocolVersion — sent when the client's protocol version doesn't match.
///
/// Wire format: `[PacketID] [Protocol 1 byte] [Magic 16 bytes] [ServerGUID 8 bytes]`
///
/// Note: This is the one unconnected packet where protocol comes BEFORE magic,
/// unlike OpenConnectionRequest1 where magic comes first.
#[derive(Debug, Clone)]
pub struct IncompatibleProtocolVersion {
    /// The server's protocol version.
    pub protocol: u8,
    /// RakNet magic bytes.
    pub magic: [u8; 16],
    /// The server's unique GUID.
    pub server_guid: i64,
}

impl IncompatibleProtocolVersion {
    /// Creates a new IncompatibleProtocolVersion packet.
    pub fn new(protocol: u8, server_guid: i64) -> Self {
        IncompatibleProtocolVersion {
            protocol,
            magic: RAKNET_MAGIC,
            server_guid,
        }
    }

    /// Decodes an IncompatibleProtocolVersion from a byte slice (excluding the packet ID byte).
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        let mut offset = 0;
        let protocol = read_u8(data, &mut offset)?;
        validate_magic(data, offset)?;
        let mut magic = [0u8; 16];
        magic.copy_from_slice(&data[offset..offset + 16]);
        offset += 16;
        let server_guid = read_i64_be(data, &mut offset)?;
        Ok(IncompatibleProtocolVersion {
            protocol,
            magic,
            server_guid,
        })
    }

    /// Encodes this packet into bytes, including the packet ID.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 1 + 16 + 8);
        buf.push(ID_INCOMPATIBLE_PROTOCOL_VERSION);
        buf.push(self.protocol);
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.server_guid.to_be_bytes());
        buf
    }
}

// ---------------------------------------------------------------------------
// Datagram / ACK / NACK structures
// ---------------------------------------------------------------------------

/// A RakNet datagram carrying encapsulated packets.
///
/// Datagrams are the basic transmission unit for connected sessions.
/// Each datagram contains a sequence number (3-byte unsigned triad) and one or
/// more encapsulated packets.
#[derive(Debug, Clone)]
pub struct Datagram {
    /// The sequence number of this datagram (stored as u32, wire format is 3 bytes).
    pub sequence_number: u32,
    /// The encapsulated packets contained in this datagram.
    pub packets: Vec<Vec<u8>>,
}

impl Datagram {
    /// Decodes a datagram from raw bytes (after the ID byte has been consumed).
    ///
    /// The sequence number is a 3-byte unsigned triad (u24), consistent with
    /// RakLib's `Binary::readUnsignedTriad` / `writeUnsignedTriad`.
    pub fn decode(data: &[u8]) -> Result<Self, RakNetError> {
        if data.len() < 3 {
            return Err(RakNetError::DecodeError(
                "datagram too short for sequence number".to_string(),
            ));
        }
        // FIX: Sequence number is a 3-byte (u24) triad, not a 4-byte u32
        let sequence_number = ((data[0] as u32) << 16)
            | ((data[1] as u32) << 8)
            | (data[2] as u32);
        // The rest of the datagram contains encapsulated packet data
        // (parsing individual encapsulated packets is handled by the session)
        Ok(Datagram {
            sequence_number,
            packets: vec![data[3..].to_vec()],
        })
    }

    /// Encodes a datagram with the given sequence number and payload.
    ///
    /// The sequence number is encoded as a 3-byte unsigned triad (u24),
    /// consistent with RakLib's `Binary::writeUnsignedTriad`.
    pub fn encode(sequence_number: u32, payload: &[u8]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 3 + payload.len());
        buf.push(ID_DATAGRAM);
        // FIX: Encode sequence number as 3-byte triad, not 4-byte u32
        buf.extend_from_slice(&encode_triple_byte(sequence_number));
        buf.extend_from_slice(payload);
        buf
    }
}

/// An ACK record acknowledging received datagrams.
#[derive(Debug, Clone)]
pub struct AckRecord {
    /// Whether this is a single entry (false) or a range (true).
    pub is_range: bool,
    /// For single: the sequence number. For range: the start.
    pub sequence: u32,
    /// For range: the end sequence number.
    pub range_end: u32,
}

/// Encodes an ACK or NACK packet from a list of sequence numbers.
///
/// Uses run-length encoding to efficiently represent consecutive sequences.
pub fn encode_ack_nack(packet_id: u8, sequences: &[u32]) -> Vec<u8> {
    if sequences.is_empty() {
        return vec![packet_id, 0, 0]; // 0 records
    }

    let mut sorted: Vec<u32> = sequences.to_vec();
    sorted.sort();
    sorted.dedup();

    let mut records: Vec<AckRecord> = Vec::new();
    let mut i = 0;

    while i < sorted.len() {
        let start = sorted[i];
        let mut end = start;

        while i + 1 < sorted.len() && sorted[i + 1] == end + 1 {
            i += 1;
            end = sorted[i];
        }

        if start == end {
            records.push(AckRecord {
                is_range: false,
                sequence: start,
                range_end: 0,
            });
        } else {
            records.push(AckRecord {
                is_range: true,
                sequence: start,
                range_end: end,
            });
        }

        i += 1;
    }

    let mut buf = Vec::new();
    buf.push(packet_id);
    // Number of records as u16 big-endian
    buf.extend_from_slice(&(records.len() as u16).to_be_bytes());

    for record in &records {
        if record.is_range {
            buf.push(1); // range flag
            buf.extend_from_slice(&encode_triple_byte(record.sequence));
            buf.extend_from_slice(&encode_triple_byte(record.range_end));
        } else {
            buf.push(0); // single flag
            buf.extend_from_slice(&encode_triple_byte(record.sequence));
        }
    }

    buf
}

/// Decodes ACK/NACK sequence numbers from the packet body (after the packet ID).
pub fn decode_ack_nack(data: &[u8]) -> Result<Vec<u32>, RakNetError> {
    if data.len() < 2 {
        return Err(RakNetError::DecodeError(
            "ACK/NACK packet too short".to_string(),
        ));
    }

    let record_count = u16::from_be_bytes([data[0], data[1]]) as usize;
    let mut offset = 2;
    let mut sequences = Vec::new();

    for _ in 0..record_count {
        if offset >= data.len() {
            return Err(RakNetError::DecodeError(
                "ACK/NACK record extends beyond data".to_string(),
            ));
        }
        let is_range = data[offset] != 0;
        offset += 1;

        let start = decode_triple_byte(data, &mut offset)?;
        if is_range {
            let end = decode_triple_byte(data, &mut offset)?;
            for seq in start..=end {
                sequences.push(seq);
            }
        } else {
            sequences.push(start);
        }
    }

    Ok(sequences)
}

/// Encodes a u24 value as 3 bytes (big-endian).
pub fn encode_triple_byte(value: u32) -> [u8; 3] {
    [
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ]
}

/// Decodes a u24 value from 3 bytes at the given offset.
pub fn decode_triple_byte(data: &[u8], offset: &mut usize) -> Result<u32, RakNetError> {
    if data.len() < *offset + 3 {
        return Err(RakNetError::DecodeError(
            "not enough data for triple byte".to_string(),
        ));
    }
    let value = ((data[*offset] as u32) << 16)
        | ((data[*offset + 1] as u32) << 8)
        | (data[*offset + 2] as u32);
    *offset += 3;
    Ok(value)
}

/// Identifies a RakNet packet by its first byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketId {
    /// Connected ping within a session.
    ConnectedPing,
    /// Unconnected ping (server discovery).
    UnconnectedPing,
    /// Unconnected ping with open connections check.
    UnconnectedPingOpenConnections,
    /// Connected pong within a session.
    ConnectedPong,
    /// Detect lost connections probe.
    DetectLostConnections,
    /// Open Connection Request 1.
    OpenConnectionRequest1,
    /// Open Connection Reply 1.
    OpenConnectionReply1,
    /// Open Connection Request 2.
    OpenConnectionRequest2,
    /// Open Connection Reply 2.
    OpenConnectionReply2,
    /// Connection request (session handshake).
    ConnectionRequest,
    /// Connection request accepted.
    ConnectionRequestAccepted,
    /// New incoming connection confirmation.
    NewIncomingConnection,
    /// Disconnection notification.
    DisconnectionNotification,
    /// Connection lost.
    ConnectionLost,
    /// Incompatible protocol version.
    IncompatibleProtocolVersion,
    /// Unconnected pong (server discovery response).
    UnconnectedPong,
    /// Advertise system broadcast.
    AdvertiseSystem,
    /// Datagram carrying encapsulated packets.
    Datagram,
    /// Acknowledgment.
    Ack,
    /// Negative acknowledgment.
    Nack,
    /// Unknown packet ID.
    Unknown(u8),
}

impl From<u8> for PacketId {
    fn from(value: u8) -> Self {
        match value {
            ID_CONNECTED_PING => PacketId::ConnectedPing,
            ID_UNCONNECTED_PING => PacketId::UnconnectedPing,
            ID_UNCONNECTED_PING_OPEN_CONNECTIONS => PacketId::UnconnectedPingOpenConnections,
            ID_CONNECTED_PONG => PacketId::ConnectedPong,
            ID_DETECT_LOST_CONNECTIONS => PacketId::DetectLostConnections,
            ID_OPEN_CONNECTION_REQUEST_1 => PacketId::OpenConnectionRequest1,
            ID_OPEN_CONNECTION_REPLY_1 => PacketId::OpenConnectionReply1,
            ID_OPEN_CONNECTION_REQUEST_2 => PacketId::OpenConnectionRequest2,
            ID_OPEN_CONNECTION_REPLY_2 => PacketId::OpenConnectionReply2,
            ID_CONNECTION_REQUEST => PacketId::ConnectionRequest,
            ID_CONNECTION_REQUEST_ACCEPTED => PacketId::ConnectionRequestAccepted,
            ID_NEW_INCOMING_CONNECTION => PacketId::NewIncomingConnection,
            ID_DISCONNECTION_NOTIFICATION => PacketId::DisconnectionNotification,
            ID_CONNECTION_LOST => PacketId::ConnectionLost,
            ID_INCOMPATIBLE_PROTOCOL_VERSION => PacketId::IncompatibleProtocolVersion,
            ID_UNCONNECTED_PONG => PacketId::UnconnectedPong,
            ID_ADVERTISE_SYSTEM => PacketId::AdvertiseSystem,
            ID_DATAGRAM => PacketId::Datagram,
            ID_ACK => PacketId::Ack,
            ID_NACK => PacketId::Nack,
            _ => PacketId::Unknown(value),
        }
    }
}
