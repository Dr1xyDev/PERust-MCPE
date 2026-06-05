use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::PlayStatus;
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// JwtChain / ClientData
// ============================================================================

/// A JWT chain element (raw string).
#[derive(Debug, Clone)]
pub struct JwtChain {
    pub raw: String,
}

impl JwtChain {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    /// Parse the JWT header (first segment, base64-decoded JSON).
    pub fn header(&self) -> Result<serde_json::Value, ProtocolError> {
        let parts: Vec<&str> = self.raw.split('.').collect();
        if parts.is_empty() {
            return Err(ProtocolError::JwtError("Empty JWT".to_string()));
        }
        decode_base64_json(parts[0])
    }

    /// Parse the JWT payload (second segment, base64-decoded JSON).
    pub fn payload(&self) -> Result<serde_json::Value, ProtocolError> {
        let parts: Vec<&str> = self.raw.split('.').collect();
        if parts.len() < 2 {
            return Err(ProtocolError::JwtError("JWT missing payload".to_string()));
        }
        decode_base64_json(parts[1])
    }

    /// Extract the identity (Xbox Live display name) from the chain.
    /// Looks for "extraData" -> "displayName" in the chain payloads.
    pub fn extract_display_name(&self) -> Result<String, ProtocolError> {
        let payload = self.payload()?;
        if let Some(extra) = payload.get("extraData") {
            if let Some(name) = extra.get("displayName") {
                return name
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| ProtocolError::JwtError("displayName is not a string".to_string()));
            }
        }
        Err(ProtocolError::JwtError("displayName not found in chain".to_string()))
    }

    /// Extract the identity (UUID) from the chain.
    pub fn extract_identity(&self) -> Result<String, ProtocolError> {
        let payload = self.payload()?;
        if let Some(extra) = payload.get("extraData") {
            if let Some(identity) = extra.get("identity") {
                return identity
                    .as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| ProtocolError::JwtError("identity is not a string".to_string()));
            }
        }
        Err(ProtocolError::JwtError("identity not found in chain".to_string()))
    }
}

/// Client data sent in the last JWT of the login chain.
#[derive(Debug, Clone)]
pub struct ClientData {
    pub raw: String,
}

impl ClientData {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    /// Parse the client data from the raw JWT payload.
    pub fn parse(&self) -> Result<serde_json::Value, ProtocolError> {
        let parts: Vec<&str> = self.raw.split('.').collect();
        if parts.len() < 2 {
            return Err(ProtocolError::JwtError("Client data JWT missing payload".to_string()));
        }
        decode_base64_json(parts[1])
    }

    /// Extract the display name from client data.
    pub fn display_name(&self) -> Result<String, ProtocolError> {
        let payload = self.parse()?;
        payload
            .get("DisplayName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| ProtocolError::JwtError("DisplayName not found in client data".to_string()))
    }
}

/// Decode a base64url-encoded JSON value.
fn decode_base64_json(input: &str) -> Result<serde_json::Value, ProtocolError> {
    // Base64 URL-safe decoding (no padding)
    let padded = pad_base64(input);
    let decoded = base64url_decode(&padded)
        .map_err(|e| ProtocolError::JwtError(format!("Base64 decode error: {}", e)))?;
    let json_str = String::from_utf8(decoded)
        .map_err(|e| ProtocolError::JwtError(format!("UTF-8 decode error: {}", e)))?;
    serde_json::from_str(&json_str)
        .map_err(|e| ProtocolError::JwtError(format!("JSON parse error: {}", e)))
}

/// Add padding to base64 string.
fn pad_base64(input: &str) -> String {
    let mut s = input.to_string();
    while s.len() % 4 != 0 {
        s.push('=');
    }
    s
}

/// Decode base64url (URL-safe base64).
fn base64url_decode(input: &str) -> Result<Vec<u8>, String> {
    // Replace URL-safe characters with standard base64 characters
    let standard = input.replace('-', "+").replace('_', "/");
    let padded = pad_base64(&standard);

    // Manual base64 decode since we may not have the base64 crate
    const DECODE_TABLE: [i8; 128] = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63,
        52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1,
        -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
        -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
    ];

    let bytes = padded.as_bytes();
    let mut result = Vec::with_capacity(bytes.len() * 3 / 4);

    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;

    for &byte in bytes {
        if byte == b'=' {
            break;
        }
        let val = if (byte as usize) < DECODE_TABLE.len() {
            DECODE_TABLE[byte as usize]
        } else {
            -1
        };
        if val < 0 {
            return Err(format!("Invalid base64 character: {}", byte as char));
        }
        buffer = (buffer << 6) | (val as u32);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            result.push((buffer >> bits) as u8);
        }
    }

    Ok(result)
}

// ============================================================================
// LoginPacket
// ============================================================================

/// Login packet sent by the client.
#[derive(Debug, Clone)]
pub struct LoginPacket {
    pub protocol: u32,
    pub game_edition: u8,
    pub chain_data: Vec<JwtChain>,
    pub client_data: ClientData,
}

impl Packet for LoginPacket {
    const PACKET_ID: u8 = protocol_info::LOGIN_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.protocol as i32);

        // Encode the chain data as a JSON object with "chain" array
        let chain_strings: Vec<&str> = self.chain_data.iter().map(|c| c.raw.as_str()).collect();
        let chain_json = serde_json::json!({
            "chain": chain_strings
        });
        let chain_json_str = serde_json::to_string(&chain_json)
            .map_err(|e| ProtocolError::EncodeError(format!("Failed to serialize chain data: {}", e)))?;

        // Write the combined payload length (chain data + client data JWT)
        let chain_bytes = chain_json_str.as_bytes();
        let client_bytes = self.client_data.raw.as_bytes();
        let combined_len = (chain_bytes.len() + client_bytes.len() + 4) as u32; // +4 for chain length prefix

        writer.write_u32_le(combined_len);
        writer.write_u32_le(chain_bytes.len() as u32);
        writer.write_bytes(chain_bytes);
        writer.write_bytes(client_bytes);

        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let protocol = reader.read_i32()? as u32;

        // Read the combined payload
        let combined_len = reader.read_u32_le()? as usize;
        let combined_data = reader.read_vec(combined_len)?;

        // Parse chain data length
        let mut combined_reader = BinaryReader::new(&combined_data);
        let chain_len = combined_reader.read_u32_le()? as usize;
        let chain_json_bytes = combined_reader.read_vec(chain_len)?;
        let chain_json_str = String::from_utf8(chain_json_bytes)
            .map_err(|e| ProtocolError::DecodeError(format!("Invalid chain JSON: {}", e)))?;

        // Parse chain JSON
        let chain_json: serde_json::Value = serde_json::from_str(&chain_json_str)
            .map_err(|e| ProtocolError::DecodeError(format!("Failed to parse chain JSON: {}", e)))?;

        let chain_array = chain_json
            .get("chain")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ProtocolError::DecodeError("Missing 'chain' array in chain data".to_string()))?;

        let chain_data: Vec<JwtChain> = chain_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| JwtChain::new(s.to_string())))
            .collect();

        // Read client data JWT (remaining bytes in combined payload)
        let remaining = combined_reader.read_remaining();
        let client_data_raw = String::from_utf8(remaining.to_vec())
            .map_err(|e| ProtocolError::DecodeError(format!("Invalid client data: {}", e)))?;

        let client_data = ClientData::new(client_data_raw);

        Ok(Self {
            protocol,
            game_edition: 0, // Default, not separately encoded in v113
            chain_data,
            client_data,
        })
    }

    fn packet_name(&self) -> &'static str {
        "LoginPacket"
    }
}

// ============================================================================
// PlayStatusPacket
// ============================================================================

/// Play status packet sent by the server to indicate login result.
#[derive(Debug, Clone)]
pub struct PlayStatusPacket {
    pub status: PlayStatus,
}

impl Packet for PlayStatusPacket {
    const PACKET_ID: u8 = protocol_info::PLAY_STATUS_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.status.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            status: PlayStatus::decode(reader)?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "PlayStatusPacket"
    }
}

// ============================================================================
// ServerToClientHandshakePacket
// ============================================================================

/// Server-to-client handshake packet containing JWT token for encryption.
#[derive(Debug, Clone)]
pub struct ServerToClientHandshakePacket {
    pub jwt_token: String,
}

impl Packet for ServerToClientHandshakePacket {
    const PACKET_ID: u8 = protocol_info::SERVER_TO_CLIENT_HANDSHAKE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.jwt_token);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            jwt_token: reader.read_string_owned()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "ServerToClientHandshakePacket"
    }
}

// ============================================================================
// ClientToServerHandshakePacket
// ============================================================================

/// Client-to-server handshake packet (empty payload).
#[derive(Debug, Clone)]
pub struct ClientToServerHandshakePacket;

impl Packet for ClientToServerHandshakePacket {
    const PACKET_ID: u8 = protocol_info::CLIENT_TO_SERVER_HANDSHAKE_PACKET;

    fn encode(&self, _writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn decode(_reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self)
    }

    fn packet_name(&self) -> &'static str {
        "ClientToServerHandshakePacket"
    }
}

// ============================================================================
// DisconnectPacket
// ============================================================================

/// Disconnect packet with a reason message.
#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    pub message: String,
}

impl Packet for DisconnectPacket {
    const PACKET_ID: u8 = protocol_info::DISCONNECT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bool(false); // message is not hidden
        writer.write_string(&self.message);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let _hide_disconnection_screen = reader.read_bool()?;
        let message = reader.read_string_owned()?;
        Ok(Self { message })
    }

    fn packet_name(&self) -> &'static str {
        "DisconnectPacket"
    }
}
