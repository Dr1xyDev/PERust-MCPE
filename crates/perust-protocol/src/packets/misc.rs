use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::{CommandData, ItemInstance};
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// InteractPacket
// ============================================================================

/// Player interaction with an entity.
#[derive(Debug, Clone)]
pub struct InteractPacket {
    pub action: u8,
    pub entity_runtime_id: u64,
}

impl Packet for InteractPacket {
    const PACKET_ID: u8 = protocol_info::INTERACT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u8(self.action);
        writer.write_u64(self.entity_runtime_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let action = reader.read_u8()?;
        let entity_runtime_id = reader.read_u64()?;
        Ok(Self { action, entity_runtime_id })
    }

    fn packet_name(&self) -> &'static str {
        "InteractPacket"
    }
}

// ============================================================================
// BlockPickRequestPacket
// ============================================================================

/// Client requests to pick a block.
#[derive(Debug, Clone)]
pub struct BlockPickRequestPacket {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub add_user_data: bool,
}

impl Packet for BlockPickRequestPacket {
    const PACKET_ID: u8 = protocol_info::BLOCK_PICK_REQUEST_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_bool(self.add_user_data);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let add_user_data = reader.read_bool()?;
        Ok(Self { x, y, z, add_user_data })
    }

    fn packet_name(&self) -> &'static str {
        "BlockPickRequestPacket"
    }
}

// ============================================================================
// UseItemPacket
// ============================================================================

/// Client uses an item.
#[derive(Debug, Clone)]
pub struct UseItemPacket {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub face: i32,
    pub item: ItemInstance,
    pub player_x: f32,
    pub player_y: f32,
    pub player_z: f32,
}

impl Packet for UseItemPacket {
    const PACKET_ID: u8 = protocol_info::USE_ITEM_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_i32(self.face);
        self.item.encode(writer);
        writer.write_f32(self.player_x);
        writer.write_f32(self.player_y);
        writer.write_f32(self.player_z);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let face = reader.read_i32()?;
        let item = ItemInstance::decode(reader)?;
        let player_x = reader.read_f32()?;
        let player_y = reader.read_f32()?;
        let player_z = reader.read_f32()?;
        Ok(Self {
            x,
            y,
            z,
            face,
            item,
            player_x,
            player_y,
            player_z,
        })
    }

    fn packet_name(&self) -> &'static str {
        "UseItemPacket"
    }
}

// ============================================================================
// RemoveBlockPacket
// ============================================================================

/// Client removes a block.
#[derive(Debug, Clone)]
pub struct RemoveBlockPacket {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Packet for RemoveBlockPacket {
    const PACKET_ID: u8 = protocol_info::REMOVE_BLOCK_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        Ok(Self { x, y, z })
    }

    fn packet_name(&self) -> &'static str {
        "RemoveBlockPacket"
    }
}

// ============================================================================
// SetHealthPacket
// ============================================================================

/// Sets the player's health.
#[derive(Debug, Clone)]
pub struct SetHealthPacket {
    pub health: i32,
}

impl Packet for SetHealthPacket {
    const PACKET_ID: u8 = protocol_info::SET_HEALTH_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.health);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            health: reader.read_i32()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SetHealthPacket"
    }
}

// ============================================================================
// RespawnPacket
// ============================================================================

/// Respawns the player.
#[derive(Debug, Clone)]
pub struct RespawnPacket {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Packet for RespawnPacket {
    const PACKET_ID: u8 = protocol_info::RESPAWN_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        Ok(Self { x, y, z })
    }

    fn packet_name(&self) -> &'static str {
        "RespawnPacket"
    }
}

// ============================================================================
// SetCommandsEnabledPacket
// ============================================================================

/// Enables/disables commands for the player.
#[derive(Debug, Clone)]
pub struct SetCommandsEnabledPacket {
    pub enabled: bool,
}

impl Packet for SetCommandsEnabledPacket {
    const PACKET_ID: u8 = protocol_info::SET_COMMANDS_ENABLED_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bool(self.enabled);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            enabled: reader.read_bool()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SetCommandsEnabledPacket"
    }
}

// ============================================================================
// SetDifficultyPacket
// ============================================================================

/// Sets the world difficulty.
#[derive(Debug, Clone)]
pub struct SetDifficultyPacket {
    pub difficulty: u32,
}

impl Packet for SetDifficultyPacket {
    const PACKET_ID: u8 = protocol_info::SET_DIFFICULTY_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u32(self.difficulty);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            difficulty: reader.read_u32()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SetDifficultyPacket"
    }
}

// ============================================================================
// SimpleEventPacket
// ============================================================================

/// Simple event packet.
#[derive(Debug, Clone)]
pub struct SimpleEventPacket {
    pub event_type: u16,
}

impl Packet for SimpleEventPacket {
    const PACKET_ID: u8 = protocol_info::SIMPLE_EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u16(self.event_type);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            event_type: reader.read_u16()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SimpleEventPacket"
    }
}

// ============================================================================
// EventPacket
// ============================================================================

/// Event packet.
#[derive(Debug, Clone)]
pub struct EventPacket {
    pub event_data: Vec<u8>,
}

impl Packet for EventPacket {
    const PACKET_ID: u8 = protocol_info::EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bytes(&self.event_data);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            event_data: reader.read_remaining().to_vec(),
        })
    }

    fn packet_name(&self) -> &'static str {
        "EventPacket"
    }
}

// ============================================================================
// SpawnExperienceOrbPacket
// ============================================================================

/// Spawns experience orbs.
#[derive(Debug, Clone)]
pub struct SpawnExperienceOrbPacket {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub experience: i32,
}

impl Packet for SpawnExperienceOrbPacket {
    const PACKET_ID: u8 = protocol_info::SPAWN_EXPERIENCE_ORB_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_var_int(self.experience);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let experience = reader.read_var_int()?;
        Ok(Self { x, y, z, experience })
    }

    fn packet_name(&self) -> &'static str {
        "SpawnExperienceOrbPacket"
    }
}

// ============================================================================
// ClientboundMapItemDataPacket
// ============================================================================

/// Map item data packet.
#[derive(Debug, Clone)]
pub struct ClientboundMapItemDataPacket {
    pub map_id: i64,
    pub update: u32,
    pub scale: u8,
    pub data: Vec<u8>,
}

impl Packet for ClientboundMapItemDataPacket {
    const PACKET_ID: u8 = protocol_info::CLIENTBOUND_MAP_ITEM_DATA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.map_id);
        writer.write_u32(self.update);
        writer.write_u8(self.scale);
        if !self.data.is_empty() {
            writer.write_bytes(&self.data);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let map_id = reader.read_i64()?;
        let update = reader.read_u32()?;
        let scale = reader.read_u8()?;
        let data = reader.read_remaining().to_vec();
        Ok(Self { map_id, update, scale, data })
    }

    fn packet_name(&self) -> &'static str {
        "ClientboundMapItemDataPacket"
    }
}

// ============================================================================
// MapInfoRequestPacket
// ============================================================================

/// Client requests map info.
#[derive(Debug, Clone)]
pub struct MapInfoRequestPacket {
    pub map_id: i64,
}

impl Packet for MapInfoRequestPacket {
    const PACKET_ID: u8 = protocol_info::MAP_INFO_REQUEST_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.map_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            map_id: reader.read_i64()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "MapInfoRequestPacket"
    }
}

// ============================================================================
// ItemFrameDropItemPacket
// ============================================================================

/// Item frame drop item packet.
#[derive(Debug, Clone)]
pub struct ItemFrameDropItemPacket {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Packet for ItemFrameDropItemPacket {
    const PACKET_ID: u8 = protocol_info::ITEM_FRAME_DROP_ITEM_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        Ok(Self { x, y, z })
    }

    fn packet_name(&self) -> &'static str {
        "ItemFrameDropItemPacket"
    }
}

// ============================================================================
// ReplaceItemInSlotPacket
// ============================================================================

/// Replace item in a specific slot.
#[derive(Debug, Clone)]
pub struct ReplaceItemInSlotPacket {
    pub slot: u8,
    pub item: ItemInstance,
}

impl Packet for ReplaceItemInSlotPacket {
    const PACKET_ID: u8 = protocol_info::REPLACE_ITEM_IN_SLOT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u8(self.slot);
        self.item.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let slot = reader.read_u8()?;
        let item = ItemInstance::decode(reader)?;
        Ok(Self { slot, item })
    }

    fn packet_name(&self) -> &'static str {
        "ReplaceItemInSlotPacket"
    }
}

// ============================================================================
// GameRulesChangedPacket
// ============================================================================

/// Game rules changed notification.
#[derive(Debug, Clone)]
pub struct GameRulesChangedPacket {
    pub gamerules: Vec<u8>,
}

impl Packet for GameRulesChangedPacket {
    const PACKET_ID: u8 = protocol_info::GAME_RULES_CHANGED_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bytes(&self.gamerules);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            gamerules: reader.read_remaining().to_vec(),
        })
    }

    fn packet_name(&self) -> &'static str {
        "GameRulesChangedPacket"
    }
}

// ============================================================================
// CameraPacket
// ============================================================================

/// Camera packet (sets camera target entity).
#[derive(Debug, Clone)]
pub struct CameraPacket {
    pub camera_entity_unique_id: i64,
    pub player_entity_unique_id: i64,
}

impl Packet for CameraPacket {
    const PACKET_ID: u8 = protocol_info::CAMERA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.camera_entity_unique_id);
        writer.write_i64(self.player_entity_unique_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let camera_entity_unique_id = reader.read_i64()?;
        let player_entity_unique_id = reader.read_i64()?;
        Ok(Self { camera_entity_unique_id, player_entity_unique_id })
    }

    fn packet_name(&self) -> &'static str {
        "CameraPacket"
    }
}

// ============================================================================
// BossEventPacket
// ============================================================================

/// Boss bar event packet.
#[derive(Debug, Clone)]
pub struct BossEventPacket {
    pub boss_unique_id: i64,
    pub event_type: u32,
    pub title: String,
    pub health_percent: f32,
}

impl Packet for BossEventPacket {
    const PACKET_ID: u8 = protocol_info::BOSS_EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.boss_unique_id as u64);
        writer.write_u32_le(self.event_type);
        match self.event_type {
            0 => {
                // Show
                writer.write_string(&self.title);
                writer.write_f32(self.health_percent);
            }
            2 => {
                // Health percent
                writer.write_f32(self.health_percent);
            }
            3 => {
                // Title
                writer.write_string(&self.title);
            }
            _ => {}
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let boss_unique_id = reader.read_u64()? as i64;
        let event_type = reader.read_u32_le()?;
        let mut title = String::new();
        let mut health_percent = 0.0f32;
        match event_type {
            0 => {
                title = reader.read_string_owned()?;
                health_percent = reader.read_f32()?;
            }
            2 => {
                health_percent = reader.read_f32()?;
            }
            3 => {
                title = reader.read_string_owned()?;
            }
            _ => {}
        }
        Ok(Self { boss_unique_id, event_type, title, health_percent })
    }

    fn packet_name(&self) -> &'static str {
        "BossEventPacket"
    }
}

// ============================================================================
// ShowCreditsPacket
// ============================================================================

/// Show credits (end game) packet.
#[derive(Debug, Clone)]
pub struct ShowCreditsPacket {
    pub entity_runtime_id: i64,
    pub status: i32,
}

impl Packet for ShowCreditsPacket {
    const PACKET_ID: u8 = protocol_info::SHOW_CREDITS_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.entity_runtime_id);
        writer.write_var_int(self.status);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_i64()?;
        let status = reader.read_var_int()?;
        Ok(Self { entity_runtime_id, status })
    }

    fn packet_name(&self) -> &'static str {
        "ShowCreditsPacket"
    }
}

// ============================================================================
// AvailableCommandsPacket
// ============================================================================

/// Available commands packet.
#[derive(Debug, Clone)]
pub struct AvailableCommandsPacket {
    pub commands: Vec<CommandData>,
}

impl Packet for AvailableCommandsPacket {
    const PACKET_ID: u8 = protocol_info::AVAILABLE_COMMANDS_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_var_uint(self.commands.len() as u32);
        for cmd in &self.commands {
            cmd.encode(writer);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let count = reader.read_var_uint()? as usize;
        let mut commands = Vec::with_capacity(count.min(256));
        for _ in 0..count {
            commands.push(CommandData::decode(reader)?);
        }
        Ok(Self { commands })
    }

    fn packet_name(&self) -> &'static str {
        "AvailableCommandsPacket"
    }
}

// ============================================================================
// CommandStepPacket
// ============================================================================

/// Command step (execution) packet.
#[derive(Debug, Clone)]
pub struct CommandStepPacket {
    pub command_name: String,
    pub overload_name: String,
    pub current_step: i32,
    pub output: Vec<u8>,
}

impl Packet for CommandStepPacket {
    const PACKET_ID: u8 = protocol_info::COMMAND_STEP_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.command_name);
        writer.write_string(&self.overload_name);
        writer.write_i32(self.current_step);
        writer.write_bytes(&self.output);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let command_name = reader.read_string_owned()?;
        let overload_name = reader.read_string_owned()?;
        let current_step = reader.read_i32()?;
        let output = reader.read_remaining().to_vec();
        Ok(Self { command_name, overload_name, current_step, output })
    }

    fn packet_name(&self) -> &'static str {
        "CommandStepPacket"
    }
}

// ============================================================================
// CommandBlockUpdatePacket
// ============================================================================

/// Command block update packet.
#[derive(Debug, Clone)]
pub struct CommandBlockUpdatePacket {
    pub is_block: bool,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub command: String,
    pub mode: i32,
    pub is_conditional: bool,
    pub output_tracked: bool,
}

impl Packet for CommandBlockUpdatePacket {
    const PACKET_ID: u8 = protocol_info::COMMAND_BLOCK_UPDATE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bool(self.is_block);
        if self.is_block {
            writer.write_i32(self.x);
            writer.write_i32(self.y);
            writer.write_i32(self.z);
        }
        writer.write_string(&self.command);
        writer.write_i32(self.mode);
        writer.write_bool(self.is_conditional);
        writer.write_bool(self.output_tracked);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let is_block = reader.read_bool()?;
        let (x, y, z) = if is_block {
            (reader.read_i32()?, reader.read_i32()?, reader.read_i32()?)
        } else {
            (0, 0, 0)
        };
        let command = reader.read_string_owned()?;
        let mode = reader.read_i32()?;
        let is_conditional = reader.read_bool()?;
        let output_tracked = reader.read_bool()?;
        Ok(Self {
            is_block,
            x,
            y,
            z,
            command,
            mode,
            is_conditional,
            output_tracked,
        })
    }

    fn packet_name(&self) -> &'static str {
        "CommandBlockUpdatePacket"
    }
}

// ============================================================================
// UpdateTradePacket
// ============================================================================

/// Update trade (villager) packet.
#[derive(Debug, Clone)]
pub struct UpdateTradePacket {
    pub window_id: i8,
    pub trade_tier: i32,
    pub trader_unique_id: i64,
    pub entity_runtime_id: u64,
    pub name: String,
}

impl Packet for UpdateTradePacket {
    const PACKET_ID: u8 = protocol_info::UPDATE_TRADE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        writer.write_i32(self.trade_tier);
        writer.write_i64(self.trader_unique_id);
        writer.write_u64(self.entity_runtime_id);
        writer.write_string(&self.name);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let window_id = reader.read_i8()?;
        let trade_tier = reader.read_i32()?;
        let trader_unique_id = reader.read_i64()?;
        let entity_runtime_id = reader.read_u64()?;
        let name = reader.read_string_owned()?;
        Ok(Self {
            window_id,
            trade_tier,
            trader_unique_id,
            entity_runtime_id,
            name,
        })
    }

    fn packet_name(&self) -> &'static str {
        "UpdateTradePacket"
    }
}

// ============================================================================
// UpdateEquipPacket
// ============================================================================

/// Update equipment packet.
#[derive(Debug, Clone)]
pub struct UpdateEquipPacket {
    pub window_id: i8,
    pub window_type: i8,
    pub entity_runtime_id: u64,
}

impl Packet for UpdateEquipPacket {
    const PACKET_ID: u8 = protocol_info::UPDATE_EQUIP_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        writer.write_i8(self.window_type);
        writer.write_u64(self.entity_runtime_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let window_id = reader.read_i8()?;
        let window_type = reader.read_i8()?;
        let entity_runtime_id = reader.read_u64()?;
        Ok(Self { window_id, window_type, entity_runtime_id })
    }

    fn packet_name(&self) -> &'static str {
        "UpdateEquipPacket"
    }
}

// ============================================================================
// TransferPacket
// ============================================================================

/// Transfer player to another server.
#[derive(Debug, Clone)]
pub struct TransferPacket {
    pub address: String,
    pub port: u16,
}

impl Packet for TransferPacket {
    const PACKET_ID: u8 = protocol_info::TRANSFER_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.address);
        writer.write_u16(self.port);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let address = reader.read_string_owned()?;
        let port = reader.read_u16()?;
        Ok(Self { address, port })
    }

    fn packet_name(&self) -> &'static str {
        "TransferPacket"
    }
}

// ============================================================================
// PlaySoundPacket
// ============================================================================

/// Play a sound at a position.
#[derive(Debug, Clone)]
pub struct PlaySoundPacket {
    pub sound_name: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub volume: f32,
    pub pitch: f32,
}

impl Packet for PlaySoundPacket {
    const PACKET_ID: u8 = protocol_info::PLAY_SOUND_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.sound_name);
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_f32(self.volume);
        writer.write_f32(self.pitch);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let sound_name = reader.read_string_owned()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let volume = reader.read_f32()?;
        let pitch = reader.read_f32()?;
        Ok(Self { sound_name, x, y, z, volume, pitch })
    }

    fn packet_name(&self) -> &'static str {
        "PlaySoundPacket"
    }
}

// ============================================================================
// StopSoundPacket
// ============================================================================

/// Stop a playing sound.
#[derive(Debug, Clone)]
pub struct StopSoundPacket {
    pub sound_name: String,
}

impl Packet for StopSoundPacket {
    const PACKET_ID: u8 = protocol_info::STOP_SOUND_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.sound_name);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            sound_name: reader.read_string_owned()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "StopSoundPacket"
    }
}

// ============================================================================
// SetTitlePacket
// ============================================================================

/// Set title/subtitle text.
#[derive(Debug, Clone)]
pub struct SetTitlePacket {
    pub title_type: i32,
    pub text: String,
    pub fade_in_time: i32,
    pub stay_time: i32,
    pub fade_out_time: i32,
}

impl Packet for SetTitlePacket {
    const PACKET_ID: u8 = protocol_info::SET_TITLE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.title_type);
        writer.write_string(&self.text);
        writer.write_i32(self.fade_in_time);
        writer.write_i32(self.stay_time);
        writer.write_i32(self.fade_out_time);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let title_type = reader.read_i32()?;
        let text = reader.read_string_owned()?;
        let fade_in_time = reader.read_i32()?;
        let stay_time = reader.read_i32()?;
        let fade_out_time = reader.read_i32()?;
        Ok(Self {
            title_type,
            text,
            fade_in_time,
            stay_time,
            fade_out_time,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SetTitlePacket"
    }
}

// ============================================================================
// BatchPacket (0xfe)
// ============================================================================

/// Batch packet containing compressed sub-packets.
/// The actual encode/decode is handled by the codec module.
#[derive(Debug, Clone)]
pub struct BatchPacket {
    pub payload: Vec<u8>,
}

impl Packet for BatchPacket {
    const PACKET_ID: u8 = protocol_info::BATCH_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bytes(&self.payload);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            payload: reader.read_remaining().to_vec(),
        })
    }

    fn packet_name(&self) -> &'static str {
        "BatchPacket"
    }
}
