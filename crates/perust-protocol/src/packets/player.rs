use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::{MovePlayerMode, PlayerAction, PlayerListEntry};
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// AddPlayerPacket
// ============================================================================

/// Adds a player to the client's world.
#[derive(Debug, Clone)]
pub struct AddPlayerPacket {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub entity_unique_id: i64,
    pub entity_runtime_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub metadata: Vec<(u32, crate::types::MetadataValue)>,
}

impl Packet for AddPlayerPacket {
    const PACKET_ID: u8 = protocol_info::ADD_PLAYER_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_uuid(&self.uuid);
        writer.write_string(&self.username);
        writer.write_i64(self.entity_unique_id);
        writer.write_u64(self.entity_runtime_id);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_f32(self.pitch);
        writer.write_f32(self.yaw);
        crate::types::encode_entity_metadata(writer, &self.metadata);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let uuid = reader.read_uuid()?;
        let username = reader.read_string_owned()?;
        let entity_unique_id = reader.read_i64()?;
        let entity_runtime_id = reader.read_u64()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let pitch = reader.read_f32()?;
        let yaw = reader.read_f32()?;
        let metadata = crate::types::decode_entity_metadata(reader)?;
        Ok(Self {
            uuid,
            username,
            entity_unique_id,
            entity_runtime_id,
            x,
            y,
            z,
            pitch,
            yaw,
            metadata,
        })
    }

    fn packet_name(&self) -> &'static str {
        "AddPlayerPacket"
    }
}

// ============================================================================
// MovePlayerPacket
// ============================================================================

/// Moves a player in the world.
#[derive(Debug, Clone)]
pub struct MovePlayerPacket {
    pub entity_runtime_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub body_yaw: f32,
    pub mode: MovePlayerMode,
    pub on_ground: bool,
    pub teleport_cause: Option<i32>,
    pub teleport_item: Option<i32>,
}

impl Packet for MovePlayerPacket {
    const PACKET_ID: u8 = protocol_info::MOVE_PLAYER_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_f32(self.pitch);
        writer.write_f32(self.yaw);
        writer.write_f32(self.body_yaw);
        self.mode.encode(writer);
        writer.write_bool(self.on_ground);
        if self.mode == MovePlayerMode::Teleport {
            if let (Some(cause), Some(item)) = (self.teleport_cause, self.teleport_item) {
                writer.write_i32(cause);
                writer.write_i32(item);
            }
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let pitch = reader.read_f32()?;
        let yaw = reader.read_f32()?;
        let body_yaw = reader.read_f32()?;
        let mode = MovePlayerMode::decode(reader)?;
        let on_ground = reader.read_bool()?;
        let mut teleport_cause = None;
        let mut teleport_item = None;
        if mode == MovePlayerMode::Teleport {
            teleport_cause = Some(reader.read_i32()?);
            teleport_item = Some(reader.read_i32()?);
        }
        Ok(Self {
            entity_runtime_id,
            x,
            y,
            z,
            pitch,
            yaw,
            body_yaw,
            mode,
            on_ground,
            teleport_cause,
            teleport_item,
        })
    }

    fn packet_name(&self) -> &'static str {
        "MovePlayerPacket"
    }
}

// ============================================================================
// PlayerActionPacket
// ============================================================================

/// Player action packet (breaking, sprinting, sneaking, etc).
#[derive(Debug, Clone)]
pub struct PlayerActionPacket {
    pub entity_runtime_id: u64,
    pub action: PlayerAction,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub face: i32,
}

impl Packet for PlayerActionPacket {
    const PACKET_ID: u8 = protocol_info::PLAYER_ACTION_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_i32(self.action.as_i32());
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_i32(self.face);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let action = PlayerAction::from_i32(reader.read_i32()?)?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let face = reader.read_i32()?;
        Ok(Self {
            entity_runtime_id,
            action,
            x,
            y,
            z,
            face,
        })
    }

    fn packet_name(&self) -> &'static str {
        "PlayerActionPacket"
    }
}

// ============================================================================
// PlayerListPacket
// ============================================================================

/// Player list (tab list) update.
#[derive(Debug, Clone)]
pub struct PlayerListPacket {
    pub action: u8, // 0 = add, 1 = remove
    pub entries: Vec<PlayerListEntry>,
}

impl Packet for PlayerListPacket {
    const PACKET_ID: u8 = protocol_info::PLAYER_LIST_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u8(self.action);
        writer.write_u32_le(self.entries.len() as u32);
        for entry in &self.entries {
            if self.action == 0 {
                entry.encode(writer);
            } else {
                // For remove, only UUID is needed
                writer.write_uuid(&entry.uuid);
            }
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let action = reader.read_u8()?;
        let count = reader.read_u32_le()? as usize;
        let mut entries = Vec::with_capacity(count.min(256));
        for _ in 0..count {
            if action == 0 {
                entries.push(PlayerListEntry::decode(reader)?);
            } else {
                let uuid = reader.read_uuid()?;
                entries.push(PlayerListEntry {
                    uuid,
                    unique_entity_id: 0,
                    name: String::new(),
                    xbox_user_id: String::new(),
                    platform_chat_id: String::new(),
                    build_platform: 0,
                    skin_data: None,
                });
            }
        }
        Ok(Self { action, entries })
    }

    fn packet_name(&self) -> &'static str {
        "PlayerListPacket"
    }
}

// ============================================================================
// AdventureSettingsPacket
// ============================================================================

/// Adventure settings (game mode flags).
#[derive(Debug, Clone)]
pub struct AdventureSettingsPacket {
    pub flags: u32,
    pub command_permission: u32,
    pub action_permissions: u32,
    pub permission_level: u32,
    pub custom_flags: u32,
    pub entity_unique_id: i64,
}

impl Packet for AdventureSettingsPacket {
    const PACKET_ID: u8 = protocol_info::ADVENTURE_SETTINGS_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u32_le(self.flags);
        writer.write_u32_le(self.command_permission);
        writer.write_u32_le(self.action_permissions);
        writer.write_u32_le(self.permission_level);
        writer.write_u32_le(self.custom_flags);
        writer.write_i64(self.entity_unique_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let flags = reader.read_u32_le()?;
        let command_permission = reader.read_u32_le()?;
        let action_permissions = reader.read_u32_le()?;
        let permission_level = reader.read_u32_le()?;
        let custom_flags = reader.read_u32_le()?;
        let entity_unique_id = reader.read_i64()?;
        Ok(Self {
            flags,
            command_permission,
            action_permissions,
            permission_level,
            custom_flags,
            entity_unique_id,
        })
    }

    fn packet_name(&self) -> &'static str {
        "AdventureSettingsPacket"
    }
}

// ============================================================================
// SetPlayerGameTypePacket
// ============================================================================

/// Sets the player's game type (gamemode).
#[derive(Debug, Clone)]
pub struct SetPlayerGameTypePacket {
    pub gamemode: i32,
}

impl Packet for SetPlayerGameTypePacket {
    const PACKET_ID: u8 = protocol_info::SET_PLAYER_GAME_TYPE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.gamemode);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            gamemode: reader.read_i32()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SetPlayerGameTypePacket"
    }
}

// ============================================================================
// PlayerInputPacket
// ============================================================================

/// Player input (for minecart/boat steering).
#[derive(Debug, Clone)]
pub struct PlayerInputPacket {
    pub motion_x: f32,
    pub motion_z: f32,
    pub jumping: bool,
    pub sneaking: bool,
}

impl Packet for PlayerInputPacket {
    const PACKET_ID: u8 = protocol_info::PLAYER_INPUT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_f32(self.motion_x);
        writer.write_f32(self.motion_z);
        writer.write_bool(self.jumping);
        writer.write_bool(self.sneaking);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let motion_x = reader.read_f32()?;
        let motion_z = reader.read_f32()?;
        let jumping = reader.read_bool()?;
        let sneaking = reader.read_bool()?;
        Ok(Self { motion_x, motion_z, jumping, sneaking })
    }

    fn packet_name(&self) -> &'static str {
        "PlayerInputPacket"
    }
}
