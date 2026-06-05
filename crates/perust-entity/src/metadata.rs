//! Entity metadata system matching the MCPE protocol.
//!
//! This module provides the metadata types, data flag constants, and
//! flag bit positions used in entity data packets.

use std::collections::HashMap;
use perust_protocol::types::ItemInstance;

// ---------------------------------------------------------------------------
// MetadataType
// ---------------------------------------------------------------------------

/// Metadata value types matching the MCPE protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MetadataType {
    Byte = 0,
    Short = 1,
    Int = 2,
    Float = 3,
    String = 4,
    Slot = 5,
    Pos = 6,
    Long = 7,
    Vector3f = 8,
}

impl MetadataType {
    /// Converts a type ID byte to a [`MetadataType`].
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(MetadataType::Byte),
            1 => Some(MetadataType::Short),
            2 => Some(MetadataType::Int),
            3 => Some(MetadataType::Float),
            4 => Some(MetadataType::String),
            5 => Some(MetadataType::Slot),
            6 => Some(MetadataType::Pos),
            7 => Some(MetadataType::Long),
            8 => Some(MetadataType::Vector3f),
            _ => None,
        }
    }

    /// Returns the protocol ID for this type.
    pub fn as_id(self) -> u8 {
        self as u8
    }
}

// ---------------------------------------------------------------------------
// MetadataValue
// ---------------------------------------------------------------------------

/// A metadata value that can be one of several types.
#[derive(Debug, Clone)]
pub enum MetadataValue {
    Byte(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    Slot(ItemInstance),
    Pos(i32, i32, i32),
    Long(i64),
    Vector3f(f32, f32, f32),
}

impl MetadataValue {
    /// Returns the metadata type of this value.
    pub fn metadata_type(&self) -> MetadataType {
        match self {
            MetadataValue::Byte(_) => MetadataType::Byte,
            MetadataValue::Short(_) => MetadataType::Short,
            MetadataValue::Int(_) => MetadataType::Int,
            MetadataValue::Float(_) => MetadataType::Float,
            MetadataValue::String(_) => MetadataType::String,
            MetadataValue::Slot(_) => MetadataType::Slot,
            MetadataValue::Pos(_, _, _) => MetadataType::Pos,
            MetadataValue::Long(_) => MetadataType::Long,
            MetadataValue::Vector3f(_, _, _) => MetadataType::Vector3f,
        }
    }
}

// ---------------------------------------------------------------------------
// EntityMetadata
// ---------------------------------------------------------------------------

/// Entity metadata: a map of data key → (type, value).
///
/// Each entry is identified by a key (u32) and stores both the type
/// and the value. This maps directly to the MCPE entity metadata format.
#[derive(Debug, Clone)]
pub struct EntityMetadata {
    data: HashMap<u32, (MetadataType, MetadataValue)>,
}

impl EntityMetadata {
    /// Creates an empty metadata map.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Sets a metadata entry.
    pub fn set(&mut self, key: u32, value: MetadataValue) {
        let ty = value.metadata_type();
        self.data.insert(key, (ty, value));
    }

    /// Gets a metadata entry by key.
    pub fn get(&self, key: u32) -> Option<&MetadataValue> {
        self.data.get(&key).map(|(_, v)| v)
    }

    /// Removes a metadata entry by key.
    pub fn remove(&mut self, key: u32) -> Option<MetadataValue> {
        self.data.remove(&key).map(|(_, v)| v)
    }

    /// Returns `true` if the given key exists.
    pub fn contains(&self, key: u32) -> bool {
        self.data.contains_key(&key)
    }

    /// Returns the number of metadata entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if there are no metadata entries.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over all entries.
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &MetadataValue)> {
        self.data.iter().map(|(k, (_, v))| (k, v))
    }

    /// Converts the metadata to the protocol's Vec format.
    pub fn to_protocol_vec(&self) -> Vec<(u32, perust_protocol::types::MetadataValue)> {
        self.data
            .iter()
            .map(|(key, (_, value))| {
                let proto_value = match value {
                    MetadataValue::Byte(v) => perust_protocol::types::MetadataValue::Byte(*v as i8),
                    MetadataValue::Short(v) => perust_protocol::types::MetadataValue::Short(*v),
                    MetadataValue::Int(v) => perust_protocol::types::MetadataValue::Int(*v),
                    MetadataValue::Float(v) => perust_protocol::types::MetadataValue::Float(*v),
                    MetadataValue::String(v) => perust_protocol::types::MetadataValue::String(v.clone()),
                    MetadataValue::Slot(_v) => perust_protocol::types::MetadataValue::CompoundTag(Vec::new()), // simplified
                    MetadataValue::Pos(x, y, z) => perust_protocol::types::MetadataValue::BlockPos(*x, *y, *z),
                    MetadataValue::Long(v) => perust_protocol::types::MetadataValue::Long(*v),
                    MetadataValue::Vector3f(x, y, z) => perust_protocol::types::MetadataValue::Vec3(*x, *y, *z),
                };
                (*key, proto_value)
            })
            .collect()
    }

    /// Sets a data flag bit.
    pub fn set_flag(&mut self, key: u32, flag_bit: u32, value: bool) {
        let current = match self.get(key) {
            Some(MetadataValue::Long(v)) => *v as u64,
            Some(MetadataValue::Int(v)) => *v as u64,
            _ => 0u64,
        };
        let new_val = if value {
            current | (1u64 << flag_bit)
        } else {
            current & !(1u64 << flag_bit)
        };
        self.set(key, MetadataValue::Long(new_val as i64));
    }

    /// Gets a data flag bit.
    pub fn get_flag(&self, key: u32, flag_bit: u32) -> bool {
        match self.get(key) {
            Some(MetadataValue::Long(v)) => (*v as u64) & (1u64 << flag_bit) != 0,
            Some(MetadataValue::Int(v)) => (*v as u64) & (1u64 << flag_bit) != 0,
            _ => false,
        }
    }
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Data flag constants (entity metadata keys)
// ---------------------------------------------------------------------------

/// Entity data flag constants matching MCPE protocol.
pub mod data_keys {
    /// Data flags (bitfield).
    pub const DATA_FLAGS: u32 = 0;
    /// Entity health (int).
    pub const DATA_HEALTH: u32 = 1;
    /// Entity variant (int).
    pub const DATA_VARIANT: u32 = 2;
    /// Block color (byte).
    pub const DATA_COLOR: u32 = 3;
    /// Entity name tag (string).
    pub const DATA_NAMETAG: u32 = 4;
    /// Entity owner runtime ID (long).
    pub const DATA_OWNER_EID: u32 = 5;
    /// Entity target runtime ID (long).
    pub const DATA_TARGET_EID: u32 = 6;
    /// Entity air supply (short).
    pub const DATA_AIR: u32 = 7;
    /// Potion color (int, ARGB).
    pub const DATA_POTION_COLOR: u32 = 8;
    /// Potion ambient (byte).
    pub const DATA_POTION_AMBIENT: u32 = 9;
    /// Hurt direction (byte).
    pub const DATA_HURT_DIRECTION: u32 = 10;
    /// Hurt time (int).
    pub const DATA_HURT_TIME: u32 = 11;
    /// Knockback (float).
    pub const DATA_KNOCKBACK: u32 = 12;
    /// Entity max health (int).
    pub const DATA_MAX_HEALTH: u32 = 13;
    /// Entity fall distance (float).
    pub const DATA_FALL_DISTANCE: u32 = 14;
    /// Entity age (int, ticks).
    pub const DATA_AGE: u32 = 15;
    /// Entity scale (float).
    pub const DATA_SCALE: u32 = 16;
    /// Entity bounding box width (float).
    pub const DATA_WIDTH: u32 = 17;
    /// Entity bounding box height (float).
    pub const DATA_HEIGHT: u32 = 18;
    /// Entity leash holder EID (long).
    pub const DATA_LEASH_HOLDER: u32 = 24;
    /// Entity score (int).
    pub const DATA_SCORE: u32 = 25;
    /// Entity color 2 (byte).
    pub const DATA_COLOR_2: u32 = 26;
    /// Entity lead (long).
    pub const DATA_LEAD: u32 = 38;
    /// Entity base runtime ID (long).
    pub const DATA_BASE_RUNTIME_ID: u32 = 41;
    /// Entity collision box (Vector3f).
    pub const DATA_COLLISION_BOX: u32 = 48;
    /// Entity seat offset (Vector3f).
    pub const DATA_SEAT_OFFSET: u32 = 50;
    /// Entity maximum air supply (short).
    pub const DATA_MAX_AIR: u32 = 54;
    /// Entity mark variant (int).
    pub const DATA_MARK_VARIANT: u32 = 55;
}

// ---------------------------------------------------------------------------
// Flag bit positions (for DATA_FLAGS key)
// ---------------------------------------------------------------------------

/// Flag bit positions for the DATA_FLAGS metadata entry.
pub mod flag_bits {
    /// Entity is on fire.
    pub const ON_FIRE: u32 = 0;
    /// Entity is sneaking.
    pub const SNEAKING: u32 = 1;
    /// Entity is riding.
    pub const RIDING: u32 = 2;
    /// Entity is sprinting.
    pub const SPRINTING: u32 = 3;
    /// Entity is using an item.
    pub const USING_ITEM: u32 = 4;
    /// Entity is invisible.
    pub const INVISIBLE: u32 = 5;
    /// Entity has a glowing effect.
    pub const GLOWING: u32 = 6;
    /// Entity is gliding (elytra).
    pub const GLIDING: u32 = 7;
    /// Entity is breathing (underwater).
    pub const BREATHING: u32 = 8;
    /// Entity is wet.
    pub const WET: u32 = 9;
    /// Entity is in water.
    pub const IN_WATER: u32 = 10;
    /// Entity is immobile.
    pub const IMMOBILE: u32 = 16;
    /// Entity is silent.
    pub const SILENT: u32 = 17;
    /// Entity is wall climbing.
    pub const WALL_CLIMBING: u32 = 18;
    /// Entity is sleeping.
    pub const SLEEPING: u32 = 19;
    /// Entity has a collision box.
    pub const HAS_COLLISION: u32 = 20;
    /// Entity is affected by gravity.
    pub const AFFECTED_BY_GRAVITY: u32 = 21;
    /// Entity is swimming.
    pub const SWIMMING: u32 = 22;
    /// Entity is panicking.
    pub const PANICKING: u32 = 23;
}
