use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::{GameRule, Vector3f};
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// SetTimePacket
// ============================================================================

/// Sets the world time.
#[derive(Debug, Clone)]
pub struct SetTimePacket {
    pub time: i32,
}

impl Packet for SetTimePacket {
    const PACKET_ID: u8 = protocol_info::SET_TIME_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.time);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            time: reader.read_i32()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SetTimePacket"
    }
}

// ============================================================================
// StartGamePacket
// ============================================================================

/// Critical packet sent by the server to start the game for a client.
#[derive(Debug, Clone)]
pub struct StartGamePacket {
    pub entity_unique_id: i64,
    pub entity_runtime_id: u64,
    pub player_gamemode: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub seed: i64,
    pub dimension: i32,
    pub generator: i32,
    pub world_gamemode: i32,
    pub difficulty: i32,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub has_achievements_disabled: bool,
    pub day_cycle_stop_time: i32,
    pub edu_mode: bool,
    pub rain_level: f32,
    pub lightning_level: f32,
    pub commands_enabled: bool,
    pub is_texture_packs_required: bool,
    pub gamerules: Vec<GameRule>,
    pub level_id: String,
    pub world_name: String,
    pub premium_world_template_id: String,
}

impl Packet for StartGamePacket {
    const PACKET_ID: u8 = protocol_info::START_GAME_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.entity_unique_id);
        writer.write_u64(self.entity_runtime_id);
        writer.write_i32(self.player_gamemode);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_f32(self.pitch);
        writer.write_f32(self.yaw);
        writer.write_i64(self.seed);
        writer.write_i32(self.dimension);
        writer.write_i32(self.generator);
        writer.write_i32(self.world_gamemode);
        writer.write_i32(self.difficulty);
        writer.write_i32(self.spawn_x);
        writer.write_i32(self.spawn_y);
        writer.write_i32(self.spawn_z);
        writer.write_bool(self.has_achievements_disabled);
        writer.write_i32(self.day_cycle_stop_time);
        writer.write_bool(self.edu_mode);
        writer.write_f32(self.rain_level);
        writer.write_f32(self.lightning_level);
        writer.write_bool(self.commands_enabled);
        writer.write_bool(self.is_texture_packs_required);

        // Game rules: count + pairs
        writer.write_var_uint(self.gamerules.len() as u32);
        for rule in &self.gamerules {
            rule.encode(writer);
        }

        writer.write_string(&self.level_id);
        writer.write_string(&self.world_name);
        writer.write_string(&self.premium_world_template_id);

        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_unique_id = reader.read_i64()?;
        let entity_runtime_id = reader.read_u64()?;
        let player_gamemode = reader.read_i32()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let pitch = reader.read_f32()?;
        let yaw = reader.read_f32()?;
        let seed = reader.read_i64()?;
        let dimension = reader.read_i32()?;
        let generator = reader.read_i32()?;
        let world_gamemode = reader.read_i32()?;
        let difficulty = reader.read_i32()?;
        let spawn_x = reader.read_i32()?;
        let spawn_y = reader.read_i32()?;
        let spawn_z = reader.read_i32()?;
        let has_achievements_disabled = reader.read_bool()?;
        let day_cycle_stop_time = reader.read_i32()?;
        let edu_mode = reader.read_bool()?;
        let rain_level = reader.read_f32()?;
        let lightning_level = reader.read_f32()?;
        let commands_enabled = reader.read_bool()?;
        let is_texture_packs_required = reader.read_bool()?;

        let gamerule_count = reader.read_var_uint()? as usize;
        let mut gamerules = Vec::with_capacity(gamerule_count.min(256));
        for _ in 0..gamerule_count {
            gamerules.push(GameRule::decode(reader)?);
        }

        let level_id = reader.read_string_owned()?;
        let world_name = reader.read_string_owned()?;
        let premium_world_template_id = reader.read_string_owned()?;

        Ok(Self {
            entity_unique_id,
            entity_runtime_id,
            player_gamemode,
            x,
            y,
            z,
            pitch,
            yaw,
            seed,
            dimension,
            generator,
            world_gamemode,
            difficulty,
            spawn_x,
            spawn_y,
            spawn_z,
            has_achievements_disabled,
            day_cycle_stop_time,
            edu_mode,
            rain_level,
            lightning_level,
            commands_enabled,
            is_texture_packs_required,
            gamerules,
            level_id,
            world_name,
            premium_world_template_id,
        })
    }

    fn packet_name(&self) -> &'static str {
        "StartGamePacket"
    }
}

// ============================================================================
// FullChunkDataPacket
// ============================================================================

/// Sends a full chunk's data to the client.
#[derive(Debug, Clone)]
pub struct FullChunkDataPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: Vec<u8>,
}

impl Packet for FullChunkDataPacket {
    const PACKET_ID: u8 = protocol_info::FULL_CHUNK_DATA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.chunk_x);
        writer.write_i32(self.chunk_z);
        writer.write_u32_le(self.data.len() as u32);
        writer.write_bytes(&self.data);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let chunk_x = reader.read_i32()?;
        let chunk_z = reader.read_i32()?;
        let data_len = reader.read_u32_le()? as usize;
        let data = reader.read_vec(data_len)?;
        Ok(Self { chunk_x, chunk_z, data })
    }

    fn packet_name(&self) -> &'static str {
        "FullChunkDataPacket"
    }
}

// ============================================================================
// UpdateBlockPacket
// ============================================================================

/// Updates a single block in the world.
#[derive(Debug, Clone)]
pub struct UpdateBlockPacket {
    pub x: i32,
    pub z: i32,
    pub y: u8,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub data_layer: u32,
}

impl Packet for UpdateBlockPacket {
    const PACKET_ID: u8 = protocol_info::UPDATE_BLOCK_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.x);
        writer.write_i32(self.z);
        writer.write_u8(self.y);
        writer.write_var_uint(self.block_runtime_id);
        writer.write_var_uint(self.flags);
        writer.write_var_uint(self.data_layer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let z = reader.read_i32()?;
        let y = reader.read_u8()?;
        let block_runtime_id = reader.read_var_uint()?;
        let flags = reader.read_var_uint()?;
        let data_layer = reader.read_var_uint()?;
        Ok(Self {
            x,
            z,
            y,
            block_runtime_id,
            flags,
            data_layer,
        })
    }

    fn packet_name(&self) -> &'static str {
        "UpdateBlockPacket"
    }
}

// ============================================================================
// ChangeDimensionPacket
// ============================================================================

/// Sent to change the player's dimension.
#[derive(Debug, Clone)]
pub struct ChangeDimensionPacket {
    pub dimension: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub respawn: bool,
}

impl Packet for ChangeDimensionPacket {
    const PACKET_ID: u8 = protocol_info::CHANGE_DIMENSION_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.dimension);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_bool(self.respawn);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let dimension = reader.read_i32()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let respawn = reader.read_bool()?;
        Ok(Self { dimension, x, y, z, respawn })
    }

    fn packet_name(&self) -> &'static str {
        "ChangeDimensionPacket"
    }
}

// ============================================================================
// SetSpawnPositionPacket
// ============================================================================

/// Sets the spawn position.
#[derive(Debug, Clone)]
pub struct SetSpawnPositionPacket {
    pub spawn_type: u8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Packet for SetSpawnPositionPacket {
    const PACKET_ID: u8 = protocol_info::SET_SPAWN_POSITION_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u8(self.spawn_type);
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let spawn_type = reader.read_u8()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        Ok(Self { spawn_type, x, y, z })
    }

    fn packet_name(&self) -> &'static str {
        "SetSpawnPositionPacket"
    }
}

// ============================================================================
// ExplodePacket
// ============================================================================

/// Explosion effect in the world.
#[derive(Debug, Clone)]
pub struct ExplodePacket {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub records: Vec<Vector3f>,
}

impl Packet for ExplodePacket {
    const PACKET_ID: u8 = protocol_info::EXPLODE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_f32(self.radius);
        writer.write_u32_le(self.records.len() as u32);
        for record in &self.records {
            let rx = (record.x - self.x) as i8;
            let ry = (record.y - self.y) as i8;
            let rz = (record.z - self.z) as i8;
            writer.write_i8(rx);
            writer.write_i8(ry);
            writer.write_i8(rz);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let radius = reader.read_f32()?;
        let count = reader.read_u32_le()? as usize;
        let mut records = Vec::with_capacity(count.min(65536));
        for _ in 0..count {
            let rx = reader.read_i8()? as f32 + x;
            let ry = reader.read_i8()? as f32 + y;
            let rz = reader.read_i8()? as f32 + z;
            records.push(Vector3f::new(rx, ry, rz));
        }
        Ok(Self { x, y, z, radius, records })
    }

    fn packet_name(&self) -> &'static str {
        "ExplodePacket"
    }
}

// ============================================================================
// LevelSoundEventPacket
// ============================================================================

/// Plays a sound event at a position in the level.
#[derive(Debug, Clone)]
pub struct LevelSoundEventPacket {
    pub sound_id: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub extra_data: i32,
    pub entity_type: String,
    pub is_baby_mob: bool,
    pub disable_relative_volume: bool,
}

impl Packet for LevelSoundEventPacket {
    const PACKET_ID: u8 = protocol_info::LEVEL_SOUND_EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u8(self.sound_id);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_i32(self.extra_data);
        writer.write_string(&self.entity_type);
        writer.write_bool(self.is_baby_mob);
        writer.write_bool(self.disable_relative_volume);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let sound_id = reader.read_u8()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let extra_data = reader.read_i32()?;
        let entity_type = reader.read_string_owned()?;
        let is_baby_mob = reader.read_bool()?;
        let disable_relative_volume = reader.read_bool()?;
        Ok(Self {
            sound_id,
            x,
            y,
            z,
            extra_data,
            entity_type,
            is_baby_mob,
            disable_relative_volume,
        })
    }

    fn packet_name(&self) -> &'static str {
        "LevelSoundEventPacket"
    }
}

// ============================================================================
// LevelEventPacket
// ============================================================================

/// Level-wide event (particles, weather, etc).
#[derive(Debug, Clone)]
pub struct LevelEventPacket {
    pub event_id: i16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub data: i32,
}

impl Packet for LevelEventPacket {
    const PACKET_ID: u8 = protocol_info::LEVEL_EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i16(self.event_id);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_var_int(self.data);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let event_id = reader.read_i16()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let data = reader.read_var_int()?;
        Ok(Self { event_id, x, y, z, data })
    }

    fn packet_name(&self) -> &'static str {
        "LevelEventPacket"
    }
}

// ============================================================================
// BlockEventPacket
// ============================================================================

/// Block-specific event (e.g., chest open, note block play).
#[derive(Debug, Clone)]
pub struct BlockEventPacket {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub case_1: i32,
    pub case_2: i32,
}

impl Packet for BlockEventPacket {
    const PACKET_ID: u8 = protocol_info::BLOCK_EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_i32(self.case_1);
        writer.write_i32(self.case_2);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let case_1 = reader.read_i32()?;
        let case_2 = reader.read_i32()?;
        Ok(Self { x, y, z, case_1, case_2 })
    }

    fn packet_name(&self) -> &'static str {
        "BlockEventPacket"
    }
}

// ============================================================================
// BlockEntityDataPacket
// ============================================================================

/// Block entity (tile entity) data update.
#[derive(Debug, Clone)]
pub struct BlockEntityDataPacket {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub named_tag: Vec<u8>,
}

impl Packet for BlockEntityDataPacket {
    const PACKET_ID: u8 = protocol_info::BLOCK_ENTITY_DATA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_bytes(&self.named_tag);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let named_tag = reader.read_remaining().to_vec();
        Ok(Self { x, y, z, named_tag })
    }

    fn packet_name(&self) -> &'static str {
        "BlockEntityDataPacket"
    }
}
