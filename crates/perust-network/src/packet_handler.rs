//! Packet handler that processes incoming MCPE packets.
//!
//! Provides callback-based processing for all incoming packet types,
//! allowing the server layer to register handlers for specific events.

use std::net::SocketAddr;

use perust_protocol::packet::{create_reader, Packet};
use perust_protocol::packets::{
    LoginPacket, MovePlayerPacket, PlayerActionPacket, TextPacket,
    ContainerClosePacket, ResourcePackClientResponsePacket,
};
use perust_protocol::types::PlayerAction;

use crate::error::NetworkError;
use crate::network_session::PacketType;

/// The result of processing a login packet.
#[derive(Debug, Clone)]
pub struct LoginResult {
    /// The protocol version from the login packet.
    pub protocol: u32,
    /// The username from the login packet.
    pub username: String,
    /// The UUID from the login packet.
    pub uuid: Option<uuid::Uuid>,
    /// The XUID from the login packet, if available.
    pub xuid: Option<String>,
}

/// Callback type for when a player logs in.
pub type LoginCallback = Box<dyn Fn(SocketAddr, LoginResult) + Send + Sync>;

/// Callback type for when a player moves.
pub type MoveCallback = Box<dyn Fn(SocketAddr, f32, f32, f32, f32, f32, f32, bool) + Send + Sync>;

/// Callback type for when a player performs an action.
pub type PlayerActionCallback = Box<dyn Fn(SocketAddr, PlayerAction) + Send + Sync>;

/// Callback type for when a chat message is received.
pub type TextCallback = Box<dyn Fn(SocketAddr, String, String) + Send + Sync>;

/// Callback type for when an inventory transaction occurs.
pub type InventoryCallback = Box<dyn Fn(SocketAddr, Vec<u8>) + Send + Sync>;

/// Callback type for when a chunk radius request is received.
pub type ChunkRadiusCallback = Box<dyn Fn(SocketAddr, u32) + Send + Sync>;

/// Callback type for when a resource pack response is received.
pub type ResourcePackCallback = Box<dyn Fn(SocketAddr, u8) + Send + Sync>;

/// Callback type for when a container is closed.
pub type ContainerCloseCallback = Box<dyn Fn(SocketAddr, i8) + Send + Sync>;

/// Callback type for generic packet forwarding.
pub type RawPacketCallback = Box<dyn Fn(SocketAddr, PacketType, &[u8]) + Send + Sync>;

/// Packet handler that processes incoming MCPE packets and dispatches
/// to registered callbacks.
pub struct PacketHandler {
    login_callback: Option<LoginCallback>,
    move_callback: Option<MoveCallback>,
    player_action_callback: Option<PlayerActionCallback>,
    text_callback: Option<TextCallback>,
    inventory_callback: Option<InventoryCallback>,
    chunk_radius_callback: Option<ChunkRadiusCallback>,
    resource_pack_callback: Option<ResourcePackCallback>,
    container_close_callback: Option<ContainerCloseCallback>,
    raw_packet_callback: Option<RawPacketCallback>,
}

impl PacketHandler {
    /// Creates a new packet handler with no callbacks registered.
    pub fn new() -> Self {
        Self {
            login_callback: None,
            move_callback: None,
            player_action_callback: None,
            text_callback: None,
            inventory_callback: None,
            chunk_radius_callback: None,
            resource_pack_callback: None,
            container_close_callback: None,
            raw_packet_callback: None,
        }
    }

    /// Sets the login callback.
    pub fn on_login(&mut self, callback: LoginCallback) {
        self.login_callback = Some(callback);
    }

    /// Sets the movement callback.
    pub fn on_move(&mut self, callback: MoveCallback) {
        self.move_callback = Some(callback);
    }

    /// Sets the player action callback.
    pub fn on_player_action(&mut self, callback: PlayerActionCallback) {
        self.player_action_callback = Some(callback);
    }

    /// Sets the text (chat) callback.
    pub fn on_text(&mut self, callback: TextCallback) {
        self.text_callback = Some(callback);
    }

    /// Sets the inventory transaction callback.
    pub fn on_inventory_transaction(&mut self, callback: InventoryCallback) {
        self.inventory_callback = Some(callback);
    }

    /// Sets the chunk radius request callback.
    pub fn on_chunk_radius_request(&mut self, callback: ChunkRadiusCallback) {
        self.chunk_radius_callback = Some(callback);
    }

    /// Sets the resource pack response callback.
    pub fn on_resource_pack_response(&mut self, callback: ResourcePackCallback) {
        self.resource_pack_callback = Some(callback);
    }

    /// Sets the container close callback.
    pub fn on_container_close(&mut self, callback: ContainerCloseCallback) {
        self.container_close_callback = Some(callback);
    }

    /// Sets a raw packet callback for all packet types.
    pub fn on_raw_packet(&mut self, callback: RawPacketCallback) {
        self.raw_packet_callback = Some(callback);
    }

    /// Handles a packet identified by its type.
    ///
    /// Dispatches to the appropriate handler based on the packet type.
    pub fn handle_packet(&self, address: SocketAddr, packet_type: PacketType, data: &[u8]) {
        // Always call the raw packet callback first
        if let Some(ref callback) = self.raw_packet_callback {
            callback(address, packet_type, data);
        }

        // Dispatch to specific handler
        match packet_type {
            PacketType::Login => {
                if let Err(e) = self.handle_login_packet(address, data) {
                    log::warn!("Failed to handle login packet from {}: {:?}", address, e);
                }
            }
            PacketType::MovePlayer => {
                if let Err(e) = self.handle_move_player(address, data) {
                    log::debug!("Failed to handle move packet from {}: {:?}", address, e);
                }
            }
            PacketType::PlayerAction => {
                if let Err(e) = self.handle_player_action(address, data) {
                    log::debug!("Failed to handle player action from {}: {:?}", address, e);
                }
            }
            PacketType::Text => {
                if let Err(e) = self.handle_text_packet(address, data) {
                    log::debug!("Failed to handle text packet from {}: {:?}", address, e);
                }
            }
            PacketType::InventoryTransaction => {
                self.handle_inventory_transaction(address, data);
            }
            PacketType::RequestChunkRadius => {
                if let Err(e) = self.handle_request_chunk_radius(address, data) {
                    log::debug!(
                        "Failed to handle chunk radius request from {}: {:?}",
                        address,
                        e
                    );
                }
            }
            PacketType::ResourcePackResponse => {
                self.handle_resource_pack_response(address, data);
            }
            PacketType::ContainerClose => {
                self.handle_container_close(address, data);
            }
            PacketType::Handshake => {
                log::debug!("Received handshake from {}", address);
            }
            _ => {
                log::trace!("Unhandled packet type {:?} from {}", packet_type, address);
            }
        }
    }

    /// Handles a LoginPacket.
    fn handle_login_packet(&self, address: SocketAddr, data: &[u8]) -> Result<(), NetworkError> {
        let mut reader = create_reader(data)?;
        let login_packet = LoginPacket::decode(&mut reader)?;

        let mut username = String::new();
        let mut uuid = None;
        let mut xuid = None;

        // Extract player info from JWT chain
        for chain in &login_packet.chain_data {
            if let Ok(name) = chain.extract_display_name() {
                username = name;
            }
            let identity_result = chain.extract_identity();
            if let Ok(identity_str) = identity_result {
                if let Ok(parsed_uuid) = uuid::Uuid::parse_str(&identity_str) {
                    uuid = Some(parsed_uuid);
                }
            }
            // Try to extract XUID from extra data
            if let Ok(payload) = chain.payload() {
                if let Some(extra) = payload.get("extraData") {
                    if let Some(xuid_val) = extra.get("XUID").and_then(|v| v.as_str()) {
                        xuid = Some(xuid_val.to_string());
                    }
                }
            }
        }

        // Try client data for display name
        if let Ok(name) = login_packet.client_data.display_name() {
            if !name.is_empty() && username.is_empty() {
                username = name;
            }
        }

        let result = LoginResult {
            protocol: login_packet.protocol,
            username,
            uuid,
            xuid,
        };

        log::info!(
            "Player {} logging in from {} with protocol {}",
            result.username,
            address,
            result.protocol
        );

        if let Some(ref callback) = self.login_callback {
            callback(address, result);
        }

        Ok(())
    }

    /// Handles a MovePlayerPacket.
    fn handle_move_player(&self, address: SocketAddr, data: &[u8]) -> Result<(), NetworkError> {
        let mut reader = create_reader(data)?;
        let move_packet = MovePlayerPacket::decode(&mut reader)?;

        if let Some(ref callback) = self.move_callback {
            callback(
                address,
                move_packet.x,
                move_packet.y,
                move_packet.z,
                move_packet.yaw,
                move_packet.pitch,
                move_packet.body_yaw,
                move_packet.on_ground,
            );
        }

        Ok(())
    }

    /// Handles a PlayerActionPacket.
    fn handle_player_action(&self, address: SocketAddr, data: &[u8]) -> Result<(), NetworkError> {
        let mut reader = create_reader(data)?;
        let action_packet = PlayerActionPacket::decode(&mut reader)?;

        if let Some(ref callback) = self.player_action_callback {
            callback(address, action_packet.action);
        }

        Ok(())
    }

    /// Handles a TextPacket.
    fn handle_text_packet(&self, address: SocketAddr, data: &[u8]) -> Result<(), NetworkError> {
        let mut reader = create_reader(data)?;
        let text_packet = TextPacket::decode(&mut reader)?;

        if let Some(ref callback) = self.text_callback {
            callback(address, text_packet.source, text_packet.message);
        }

        Ok(())
    }

    /// Handles an InventoryActionPacket.
    fn handle_inventory_transaction(&self, address: SocketAddr, data: &[u8]) {
        if let Some(ref callback) = self.inventory_callback {
            // Forward raw data for server-side processing
            callback(address, data.to_vec());
        }
    }

    /// Handles a RequestChunkRadiusPacket.
    fn handle_request_chunk_radius(
        &self,
        address: SocketAddr,
        data: &[u8],
    ) -> Result<(), NetworkError> {
        // The RequestChunkRadiusPacket is simple: just a VarInt for the radius
        let mut reader = create_reader(data)?;
        let radius = reader.read_var_uint().unwrap_or(8) as u32;

        if let Some(ref callback) = self.chunk_radius_callback {
            callback(address, radius);
        }

        Ok(())
    }

    /// Handles a ResourcePackClientResponsePacket.
    fn handle_resource_pack_response(&self, address: SocketAddr, data: &[u8]) {
        if let Ok(mut reader) = create_reader(data) {
            if let Ok(response) = ResourcePackClientResponsePacket::decode(&mut reader) {
                if let Some(ref callback) = self.resource_pack_callback {
                    callback(address, response.response_status);
                }
            }
        }
    }

    /// Handles a ContainerClosePacket.
    fn handle_container_close(&self, address: SocketAddr, data: &[u8]) {
        if let Ok(mut reader) = create_reader(data) {
            if let Ok(packet) = ContainerClosePacket::decode(&mut reader) {
                if let Some(ref callback) = self.container_close_callback {
                    callback(address, packet.window_id);
                }
            }
        }
    }
}

impl Default for PacketHandler {
    fn default() -> Self {
        Self::new()
    }
}
