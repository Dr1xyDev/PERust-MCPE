//! Network session wrapping a RakNet session with MCPE protocol handling.

use std::net::SocketAddr;

use perust_protocol::codec;
use perust_protocol::packet::{decode_packet_id, encode_packet, Packet};
use perust_protocol::protocol_info;
use perust_raknet::Reliability;

use crate::error::NetworkError;

/// The state of a MCPE network session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// The session is handshaking (RakNet connected, MCPE login not yet started).
    Handshaking,
    /// The session is in the MCPE login process.
    LoggingIn,
    /// The session is fully connected and in-game.
    Playing,
    /// The session has been disconnected.
    Disconnected,
}

impl std::fmt::Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionState::Handshaking => write!(f, "Handshaking"),
            SessionState::LoggingIn => write!(f, "LoggingIn"),
            SessionState::Playing => write!(f, "Playing"),
            SessionState::Disconnected => write!(f, "Disconnected"),
        }
    }
}

/// A network session that wraps a RakNet connection with MCPE protocol state.
///
/// Each connected player has a NetworkSession that tracks their MCPE-level
/// protocol state, buffers outgoing packets, and handles batch encoding.
pub struct NetworkSession {
    /// The remote address of this session.
    pub address: SocketAddr,
    /// A unique session ID.
    pub session_id: u64,
    /// The current MCPE protocol state.
    pub state: SessionState,
    /// The protocol version the client is using.
    pub protocol: u32,
    /// The username of the connected player, if known.
    pub username: Option<String>,
    /// Buffered packets waiting to be sent.
    send_buffer: Vec<Vec<u8>>,
    /// The compression level for batch packets (0-9, default 6).
    compression_level: u32,
}

impl NetworkSession {
    /// Creates a new network session.
    pub fn new(address: SocketAddr, session_id: u64) -> Self {
        Self {
            address,
            session_id,
            state: SessionState::Handshaking,
            protocol: 0,
            username: None,
            send_buffer: Vec::new(),
            compression_level: 6,
        }
    }

    /// Sets the compression level for batch packets.
    pub fn set_compression_level(&mut self, level: u32) {
        self.compression_level = level.clamp(0, 9);
    }

    /// Handles an incoming raw packet from the RakNet layer.
    ///
    /// The data may be a batch packet (0xfe) containing multiple sub-packets,
    /// or a single packet. Returns the identified packet type if recognized.
    pub fn handle_incoming(&mut self, data: &[u8]) -> Result<Option<PacketType>, NetworkError> {
        if data.is_empty() {
            return Ok(None);
        }

        // Check for batch packet
        if data[0] == protocol_info::BATCH_PACKET {
            let sub_packets = codec::decode_batch(&data[1..])?;
            let mut last_packet_type = None;

            for sub_packet in &sub_packets {
                if let Some(pt) = self.process_single_packet(sub_packet)? {
                    last_packet_type = Some(pt);
                }
            }

            return Ok(last_packet_type);
        }

        // Single packet
        self.process_single_packet(data)
    }

    /// Processes a single (non-batch) packet.
    fn process_single_packet(&mut self, data: &[u8]) -> Result<Option<PacketType>, NetworkError> {
        if data.is_empty() {
            return Ok(None);
        }

        let packet_id = decode_packet_id(data)?;
        let packet_type = match packet_id {
            protocol_info::LOGIN_PACKET => Some(PacketType::Login),
            protocol_info::MOVE_PLAYER_PACKET => Some(PacketType::MovePlayer),
            protocol_info::PLAYER_ACTION_PACKET => Some(PacketType::PlayerAction),
            protocol_info::TEXT_PACKET => Some(PacketType::Text),
            protocol_info::INVENTORY_ACTION_PACKET => Some(PacketType::InventoryTransaction),
            protocol_info::REQUEST_CHUNK_RADIUS_PACKET => Some(PacketType::RequestChunkRadius),
            protocol_info::RESOURCE_PACK_CLIENT_RESPONSE_PACKET => Some(PacketType::ResourcePackResponse),
            protocol_info::CLIENT_TO_SERVER_HANDSHAKE_PACKET => Some(PacketType::Handshake),
            protocol_info::INTERACT_PACKET => Some(PacketType::Interact),
            protocol_info::MOB_EQUIPMENT_PACKET => Some(PacketType::MobEquipment),
            protocol_info::CONTAINER_CLOSE_PACKET => Some(PacketType::ContainerClose),
            protocol_info::DROP_ITEM_PACKET => Some(PacketType::DropItem),
            _ => {
                log::trace!(
                    "Unhandled packet ID 0x{:02x} from {}",
                    packet_id,
                    self.address
                );
                None
            }
        };

        Ok(packet_type)
    }

    /// Queues a packet to be sent to this session.
    ///
    /// Packets are buffered and sent in a batch during the next flush.
    pub fn queue_packet(&mut self, data: Vec<u8>) {
        self.send_buffer.push(data);
    }

    /// Queues a typed packet to be sent to this session.
    pub fn queue_typed_packet<P: Packet>(&mut self, packet: &P) -> Result<(), NetworkError> {
        let data = encode_packet(packet)?;
        self.send_buffer.push(data);
        Ok(())
    }

    /// Flushes all buffered packets, encoding them into a batch packet.
    ///
    /// Returns the encoded batch packet data ready to be sent over RakNet,
    /// or `None` if there are no buffered packets.
    pub fn flush(&mut self) -> Option<Vec<u8>> {
        if self.send_buffer.is_empty() {
            return None;
        }

        // Encode the batch
        match codec::encode_batch(&self.send_buffer) {
            Ok(mut batch_data) => {
                self.send_buffer.clear();

                // Prepend the batch packet ID
                let mut result = Vec::with_capacity(1 + batch_data.len());
                result.push(protocol_info::BATCH_PACKET);
                result.append(&mut batch_data);

                Some(result)
            }
            Err(e) => {
                log::error!(
                    "Failed to encode batch for {}: {:?}",
                    self.address,
                    e
                );
                self.send_buffer.clear();
                None
            }
        }
    }

    /// Disconnects this session with the given reason.
    pub fn disconnect(&mut self, reason: &str) {
        log::info!("Disconnecting session {}: {}", self.address, reason);

        // Try to send a disconnect packet
        let disconnect_packet = perust_protocol::packets::DisconnectPacket {
            message: reason.to_string(),
        };

        if let Ok(data) = encode_packet(&disconnect_packet) {
            self.send_buffer.push(data);
        }

        self.state = SessionState::Disconnected;
    }

    /// Returns the number of buffered packets waiting to be sent.
    pub fn buffered_packet_count(&self) -> usize {
        self.send_buffer.len()
    }

    /// Returns `true` if this session is in the Playing state.
    pub fn is_playing(&self) -> bool {
        self.state == SessionState::Playing
    }

    /// Returns `true` if this session is still connected.
    pub fn is_connected(&self) -> bool {
        self.state != SessionState::Disconnected
    }
}

/// Identifies the type of an incoming MCPE packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Login,
    Handshake,
    MovePlayer,
    PlayerAction,
    Text,
    InventoryTransaction,
    RequestChunkRadius,
    ResourcePackResponse,
    Interact,
    MobEquipment,
    ContainerClose,
    DropItem,
}

/// Returns the RakNet reliability level appropriate for a given packet type.
pub fn reliability_for_packet(packet_id: u8) -> Reliability {
    match packet_id {
        // Critical packets that must arrive and be in order
        protocol_info::LOGIN_PACKET
        | protocol_info::PLAY_STATUS_PACKET
        | protocol_info::START_GAME_PACKET
        | protocol_info::DISCONNECT_PACKET
        | protocol_info::RESOURCE_PACKS_INFO_PACKET
        | protocol_info::RESOURCE_PACK_STACK_PACKET => Reliability::ReliableOrdered,

        // Position/movement packets - should arrive but order matters less
        protocol_info::MOVE_PLAYER_PACKET
        | protocol_info::MOVE_ENTITY_PACKET
        | protocol_info::PLAYER_ACTION_PACKET => Reliability::ReliableSequenced,

        // Chat and other gameplay packets
        protocol_info::TEXT_PACKET
        | protocol_info::INVENTORY_ACTION_PACKET
        | protocol_info::CONTAINER_SET_SLOT_PACKET
        | protocol_info::CONTAINER_SET_CONTENT_PACKET => Reliability::ReliableOrdered,

        // Chunk data - reliable but can be sequenced
        protocol_info::FULL_CHUNK_DATA_PACKET => Reliability::ReliableOrdered,

        // Less critical packets
        protocol_info::LEVEL_EVENT_PACKET
        | protocol_info::ENTITY_EVENT_PACKET
        | protocol_info::ANIMATE_PACKET => Reliability::ReliableSequenced,

        // Default to reliable ordered for unknown packets
        _ => Reliability::ReliableOrdered,
    }
}
