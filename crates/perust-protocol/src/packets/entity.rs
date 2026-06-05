use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::{Attribute, EntityLink};
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// AddEntityPacket
// ============================================================================

/// Adds an entity to the client's world.
#[derive(Debug, Clone)]
pub struct AddEntityPacket {
    pub entity_unique_id: i64,
    pub entity_runtime_id: u64,
    pub entity_type: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub metadata: Vec<(u32, crate::types::MetadataValue)>,
    pub links: Vec<EntityLink>,
}

impl Packet for AddEntityPacket {
    const PACKET_ID: u8 = protocol_info::ADD_ENTITY_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.entity_unique_id);
        writer.write_u64(self.entity_runtime_id);
        writer.write_string(&self.entity_type);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_f32(self.pitch);
        writer.write_f32(self.yaw);
        crate::types::encode_entity_metadata(writer, &self.metadata);
        writer.write_var_uint(self.links.len() as u32);
        for link in &self.links {
            link.encode(writer);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_unique_id = reader.read_i64()?;
        let entity_runtime_id = reader.read_u64()?;
        let entity_type = reader.read_string_owned()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let pitch = reader.read_f32()?;
        let yaw = reader.read_f32()?;
        let metadata = crate::types::decode_entity_metadata(reader)?;
        let link_count = reader.read_var_uint()? as usize;
        let mut links = Vec::with_capacity(link_count.min(256));
        for _ in 0..link_count {
            links.push(EntityLink::decode(reader)?);
        }
        Ok(Self {
            entity_unique_id,
            entity_runtime_id,
            entity_type,
            x,
            y,
            z,
            pitch,
            yaw,
            metadata,
            links,
        })
    }

    fn packet_name(&self) -> &'static str {
        "AddEntityPacket"
    }
}

// ============================================================================
// RemoveEntityPacket
// ============================================================================

/// Removes an entity from the client's world.
#[derive(Debug, Clone)]
pub struct RemoveEntityPacket {
    pub entity_unique_id: i64,
}

impl Packet for RemoveEntityPacket {
    const PACKET_ID: u8 = protocol_info::REMOVE_ENTITY_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.entity_unique_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            entity_unique_id: reader.read_i64()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "RemoveEntityPacket"
    }
}

// ============================================================================
// AddItemEntityPacket
// ============================================================================

/// Adds an item entity (dropped item) to the world.
#[derive(Debug, Clone)]
pub struct AddItemEntityPacket {
    pub entity_unique_id: i64,
    pub entity_runtime_id: u64,
    pub item: crate::types::ItemInstance,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub motion_x: f32,
    pub motion_y: f32,
    pub motion_z: f32,
    pub metadata: Vec<(u32, crate::types::MetadataValue)>,
}

impl Packet for AddItemEntityPacket {
    const PACKET_ID: u8 = protocol_info::ADD_ITEM_ENTITY_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.entity_unique_id);
        writer.write_u64(self.entity_runtime_id);
        self.item.encode(writer);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_f32(self.motion_x);
        writer.write_f32(self.motion_y);
        writer.write_f32(self.motion_z);
        crate::types::encode_entity_metadata(writer, &self.metadata);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_unique_id = reader.read_i64()?;
        let entity_runtime_id = reader.read_u64()?;
        let item = crate::types::ItemInstance::decode(reader)?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let motion_x = reader.read_f32()?;
        let motion_y = reader.read_f32()?;
        let motion_z = reader.read_f32()?;
        let metadata = crate::types::decode_entity_metadata(reader)?;
        Ok(Self {
            entity_unique_id,
            entity_runtime_id,
            item,
            x,
            y,
            z,
            motion_x,
            motion_y,
            motion_z,
            metadata,
        })
    }

    fn packet_name(&self) -> &'static str {
        "AddItemEntityPacket"
    }
}

// ============================================================================
// SetEntityDataPacket
// ============================================================================

/// Updates entity metadata.
#[derive(Debug, Clone)]
pub struct SetEntityDataPacket {
    pub entity_runtime_id: u64,
    pub metadata: Vec<(u32, crate::types::MetadataValue)>,
}

impl Packet for SetEntityDataPacket {
    const PACKET_ID: u8 = protocol_info::SET_ENTITY_DATA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        crate::types::encode_entity_metadata(writer, &self.metadata);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let metadata = crate::types::decode_entity_metadata(reader)?;
        Ok(Self { entity_runtime_id, metadata })
    }

    fn packet_name(&self) -> &'static str {
        "SetEntityDataPacket"
    }
}

// ============================================================================
// SetEntityMotionPacket
// ============================================================================

/// Sets entity motion/velocity.
#[derive(Debug, Clone)]
pub struct SetEntityMotionPacket {
    pub entity_runtime_id: u64,
    pub motion_x: f32,
    pub motion_y: f32,
    pub motion_z: f32,
}

impl Packet for SetEntityMotionPacket {
    const PACKET_ID: u8 = protocol_info::SET_ENTITY_MOTION_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_f32(self.motion_x);
        writer.write_f32(self.motion_y);
        writer.write_f32(self.motion_z);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let motion_x = reader.read_f32()?;
        let motion_y = reader.read_f32()?;
        let motion_z = reader.read_f32()?;
        Ok(Self { entity_runtime_id, motion_x, motion_y, motion_z })
    }

    fn packet_name(&self) -> &'static str {
        "SetEntityMotionPacket"
    }
}

// ============================================================================
// SetEntityLinkPacket
// ============================================================================

/// Sets a link between two entities (riding, etc).
#[derive(Debug, Clone)]
pub struct SetEntityLinkPacket {
    pub link: EntityLink,
}

impl Packet for SetEntityLinkPacket {
    const PACKET_ID: u8 = protocol_info::SET_ENTITY_LINK_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.link.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            link: EntityLink::decode(reader)?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "SetEntityLinkPacket"
    }
}

// ============================================================================
// EntityEventPacket
// ============================================================================

/// Entity event (hurt animation, death, etc).
#[derive(Debug, Clone)]
pub struct EntityEventPacket {
    pub entity_runtime_id: u64,
    pub event: u8,
    pub data: i32,
}

impl Packet for EntityEventPacket {
    const PACKET_ID: u8 = protocol_info::ENTITY_EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_u8(self.event);
        writer.write_var_int(self.data);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let event = reader.read_u8()?;
        let data = reader.read_var_int()?;
        Ok(Self { entity_runtime_id, event, data })
    }

    fn packet_name(&self) -> &'static str {
        "EntityEventPacket"
    }
}

// ============================================================================
// AnimatePacket
// ============================================================================

/// Entity animation packet.
#[derive(Debug, Clone)]
pub struct AnimatePacket {
    pub action: u32,
    pub entity_runtime_id: u64,
}

impl Packet for AnimatePacket {
    const PACKET_ID: u8 = protocol_info::ANIMATE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u32_le(self.action);
        writer.write_u64(self.entity_runtime_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let action = reader.read_u32_le()?;
        let entity_runtime_id = reader.read_u64()?;
        Ok(Self { action, entity_runtime_id })
    }

    fn packet_name(&self) -> &'static str {
        "AnimatePacket"
    }
}

// ============================================================================
// HurtArmorPacket
// ============================================================================

/// Notifies client that their armor was damaged.
#[derive(Debug, Clone)]
pub struct HurtArmorPacket {
    pub health: i32,
}

impl Packet for HurtArmorPacket {
    const PACKET_ID: u8 = protocol_info::HURT_ARMOR_PACKET;

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
        "HurtArmorPacket"
    }
}

// ============================================================================
// MobEffectPacket
// ============================================================================

/// Applies or removes a mob effect.
#[derive(Debug, Clone)]
pub struct MobEffectPacket {
    pub entity_runtime_id: u64,
    pub event: u8,
    pub effect_id: i8,
    pub amplifier: i8,
    pub particles: bool,
    pub duration: i32,
}

impl Packet for MobEffectPacket {
    const PACKET_ID: u8 = protocol_info::MOB_EFFECT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_u8(self.event);
        writer.write_i8(self.effect_id);
        writer.write_i8(self.amplifier);
        writer.write_bool(self.particles);
        writer.write_var_int(self.duration);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let event = reader.read_u8()?;
        let effect_id = reader.read_i8()?;
        let amplifier = reader.read_i8()?;
        let particles = reader.read_bool()?;
        let duration = reader.read_var_int()?;
        Ok(Self {
            entity_runtime_id,
            event,
            effect_id,
            amplifier,
            particles,
            duration,
        })
    }

    fn packet_name(&self) -> &'static str {
        "MobEffectPacket"
    }
}

// ============================================================================
// UpdateAttributesPacket
// ============================================================================

/// Updates entity attributes (health, movement speed, etc).
#[derive(Debug, Clone)]
pub struct UpdateAttributesPacket {
    pub entity_runtime_id: u64,
    pub attributes: Vec<Attribute>,
}

impl Packet for UpdateAttributesPacket {
    const PACKET_ID: u8 = protocol_info::UPDATE_ATTRIBUTES_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_u32_le(self.attributes.len() as u32);
        for attr in &self.attributes {
            attr.encode(writer);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let count = reader.read_u32_le()? as usize;
        let mut attributes = Vec::with_capacity(count.min(256));
        for _ in 0..count {
            attributes.push(Attribute::decode(reader)?);
        }
        Ok(Self { entity_runtime_id, attributes })
    }

    fn packet_name(&self) -> &'static str {
        "UpdateAttributesPacket"
    }
}

// ============================================================================
// MoveEntityPacket
// ============================================================================

/// Moves an entity (absolute position).
#[derive(Debug, Clone)]
pub struct MoveEntityPacket {
    pub entity_runtime_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
}

impl Packet for MoveEntityPacket {
    const PACKET_ID: u8 = protocol_info::MOVE_ENTITY_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
        writer.write_f32(self.pitch);
        writer.write_f32(self.yaw);
        writer.write_f32(self.head_yaw);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        let pitch = reader.read_f32()?;
        let yaw = reader.read_f32()?;
        let head_yaw = reader.read_f32()?;
        Ok(Self {
            entity_runtime_id,
            x,
            y,
            z,
            pitch,
            yaw,
            head_yaw,
        })
    }

    fn packet_name(&self) -> &'static str {
        "MoveEntityPacket"
    }
}

// ============================================================================
// EntityFallPacket
// ============================================================================

/// Entity fall distance packet.
#[derive(Debug, Clone)]
pub struct EntityFallPacket {
    pub entity_runtime_id: u64,
    pub fall_distance: f32,
}

impl Packet for EntityFallPacket {
    const PACKET_ID: u8 = protocol_info::ENTITY_FALL_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        writer.write_f32(self.fall_distance);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let fall_distance = reader.read_f32()?;
        Ok(Self { entity_runtime_id, fall_distance })
    }

    fn packet_name(&self) -> &'static str {
        "EntityFallPacket"
    }
}

// ============================================================================
// AddPaintingPacket
// ============================================================================

/// Adds a painting entity.
#[derive(Debug, Clone)]
pub struct AddPaintingPacket {
    pub entity_unique_id: i64,
    pub entity_runtime_id: u64,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub direction: i32,
    pub title: String,
}

impl Packet for AddPaintingPacket {
    const PACKET_ID: u8 = protocol_info::ADD_PAINTING_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.entity_unique_id);
        writer.write_u64(self.entity_runtime_id);
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_i32(self.direction);
        writer.write_string(&self.title);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_unique_id = reader.read_i64()?;
        let entity_runtime_id = reader.read_u64()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let direction = reader.read_i32()?;
        let title = reader.read_string_owned()?;
        Ok(Self {
            entity_unique_id,
            entity_runtime_id,
            x,
            y,
            z,
            direction,
            title,
        })
    }

    fn packet_name(&self) -> &'static str {
        "AddPaintingPacket"
    }
}

// ============================================================================
// AddHangingEntityPacket
// ============================================================================

/// Adds a hanging entity (item frame, painting).
#[derive(Debug, Clone)]
pub struct AddHangingEntityPacket {
    pub entity_unique_id: i64,
    pub entity_runtime_id: u64,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub direction: i32,
}

impl Packet for AddHangingEntityPacket {
    const PACKET_ID: u8 = protocol_info::ADD_HANGING_ENTITY_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i64(self.entity_unique_id);
        writer.write_u64(self.entity_runtime_id);
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
        writer.write_i32(self.direction);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_unique_id = reader.read_i64()?;
        let entity_runtime_id = reader.read_u64()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        let direction = reader.read_i32()?;
        Ok(Self {
            entity_unique_id,
            entity_runtime_id,
            x,
            y,
            z,
            direction,
        })
    }

    fn packet_name(&self) -> &'static str {
        "AddHangingEntityPacket"
    }
}

// ============================================================================
// TakeItemEntityPacket
// ============================================================================

/// Entity picks up an item.
#[derive(Debug, Clone)]
pub struct TakeItemEntityPacket {
    pub runtime_entity_id: u64,
    pub item_entity_runtime_id: u64,
}

impl Packet for TakeItemEntityPacket {
    const PACKET_ID: u8 = protocol_info::TAKE_ITEM_ENTITY_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.runtime_entity_id);
        writer.write_u64(self.item_entity_runtime_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let runtime_entity_id = reader.read_u64()?;
        let item_entity_runtime_id = reader.read_u64()?;
        Ok(Self { runtime_entity_id, item_entity_runtime_id })
    }

    fn packet_name(&self) -> &'static str {
        "TakeItemEntityPacket"
    }
}

// ============================================================================
// RiderJumpPacket
// ============================================================================

/// Rider jump strength packet.
#[derive(Debug, Clone)]
pub struct RiderJumpPacket {
    pub jump_strength: i32,
}

impl Packet for RiderJumpPacket {
    const PACKET_ID: u8 = protocol_info::RIDER_JUMP_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_var_int(self.jump_strength);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            jump_strength: reader.read_var_int()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "RiderJumpPacket"
    }
}
