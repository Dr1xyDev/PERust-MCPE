use crate::error::ProtocolError;
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// GameMode
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl GameMode {
    pub fn from_i32(val: i32) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(GameMode::Survival),
            1 => Ok(GameMode::Creative),
            2 => Ok(GameMode::Adventure),
            3 => Ok(GameMode::Spectator),
            _ => Err(ProtocolError::DecodeError(format!("Invalid GameMode: {}", val))),
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_i32(self.as_i32());
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Self::from_i32(reader.read_i32()?)
    }
}

// ============================================================================
// Dimension
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    Overworld = 0,
    Nether = 1,
    End = 2,
}

impl Dimension {
    pub fn from_i32(val: i32) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(Dimension::Overworld),
            1 => Ok(Dimension::Nether),
            2 => Ok(Dimension::End),
            _ => Err(ProtocolError::DecodeError(format!("Invalid Dimension: {}", val))),
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_i32(self.as_i32());
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Self::from_i32(reader.read_i32()?)
    }
}

// ============================================================================
// Difficulty
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

impl Difficulty {
    pub fn from_i32(val: i32) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err(ProtocolError::DecodeError(format!("Invalid Difficulty: {}", val))),
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_i32(self.as_i32());
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Self::from_i32(reader.read_i32()?)
    }
}

// ============================================================================
// TextPacketType
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextPacketType {
    Raw = 0,
    Chat = 1,
    Translation = 2,
    Popup = 3,
    Tip = 4,
    System = 5,
    Whisper = 6,
}

impl TextPacketType {
    pub fn from_u8(val: u8) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(TextPacketType::Raw),
            1 => Ok(TextPacketType::Chat),
            2 => Ok(TextPacketType::Translation),
            3 => Ok(TextPacketType::Popup),
            4 => Ok(TextPacketType::Tip),
            5 => Ok(TextPacketType::System),
            6 => Ok(TextPacketType::Whisper),
            _ => Err(ProtocolError::DecodeError(format!("Invalid TextPacketType: {}", val))),
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_u8(self.as_u8());
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Self::from_u8(reader.read_u8()?)
    }
}

// ============================================================================
// PlayStatus
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayStatus {
    LoginSuccess = 0,
    LoginFailedClient = 1,
    LoginFailedServer = 2,
    PlayerSpawn = 3,
    LoginFailedInvalidTenant = 4,
    LoginFailedVanillaEdu = 5,
    LoginFailedEduVanilla = 6,
}

impl PlayStatus {
    pub fn from_i32(val: i32) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(PlayStatus::LoginSuccess),
            1 => Ok(PlayStatus::LoginFailedClient),
            2 => Ok(PlayStatus::LoginFailedServer),
            3 => Ok(PlayStatus::PlayerSpawn),
            4 => Ok(PlayStatus::LoginFailedInvalidTenant),
            5 => Ok(PlayStatus::LoginFailedVanillaEdu),
            6 => Ok(PlayStatus::LoginFailedEduVanilla),
            _ => Err(ProtocolError::DecodeError(format!("Invalid PlayStatus: {}", val))),
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_i32(self.as_i32());
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Self::from_i32(reader.read_i32()?)
    }
}

// ============================================================================
// MovePlayerMode
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovePlayerMode {
    Normal = 0,
    Reset = 1,
    Teleport = 2,
    Pitch = 3,
}

impl MovePlayerMode {
    pub fn from_u8(val: u8) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(MovePlayerMode::Normal),
            1 => Ok(MovePlayerMode::Reset),
            2 => Ok(MovePlayerMode::Teleport),
            3 => Ok(MovePlayerMode::Pitch),
            _ => Err(ProtocolError::DecodeError(format!("Invalid MovePlayerMode: {}", val))),
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_u8(self.as_u8());
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Self::from_u8(reader.read_u8()?)
    }
}

// ============================================================================
// PlayerAction
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    StartBreak = 0,
    AbortBreak = 1,
    StopBreak = 2,
    ReleaseItem = 5,
    StopSleeping = 6,
    SpawnSameDimension = 7,
    Jump = 8,
    StartSprint = 9,
    StopSprint = 10,
    StartSneak = 11,
    StopSneak = 12,
    SpawnOverworld = 13,
    SpawnNether = 14,
    StartGlide = 15,
    StopGlide = 16,
    BuildDenied = 17,
    ContinueBreak = 18,
}

impl PlayerAction {
    pub fn from_i32(val: i32) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(PlayerAction::StartBreak),
            1 => Ok(PlayerAction::AbortBreak),
            2 => Ok(PlayerAction::StopBreak),
            5 => Ok(PlayerAction::ReleaseItem),
            6 => Ok(PlayerAction::StopSleeping),
            7 => Ok(PlayerAction::SpawnSameDimension),
            8 => Ok(PlayerAction::Jump),
            9 => Ok(PlayerAction::StartSprint),
            10 => Ok(PlayerAction::StopSprint),
            11 => Ok(PlayerAction::StartSneak),
            12 => Ok(PlayerAction::StopSneak),
            13 => Ok(PlayerAction::SpawnOverworld),
            14 => Ok(PlayerAction::SpawnNether),
            15 => Ok(PlayerAction::StartGlide),
            16 => Ok(PlayerAction::StopGlide),
            17 => Ok(PlayerAction::BuildDenied),
            18 => Ok(PlayerAction::ContinueBreak),
            _ => Err(ProtocolError::DecodeError(format!("Invalid PlayerAction: {}", val))),
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

// ============================================================================
// EntityMetadata
// ============================================================================

/// Metadata types used in entity data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataType {
    Byte = 0,
    Short = 1,
    Int = 2,
    Float = 3,
    String = 4,
    CompoundTag = 5,
    BlockPos = 6,
    Long = 7,
    Vec3 = 8,
}

impl MetadataType {
    pub fn from_u8(val: u8) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(MetadataType::Byte),
            1 => Ok(MetadataType::Short),
            2 => Ok(MetadataType::Int),
            3 => Ok(MetadataType::Float),
            4 => Ok(MetadataType::String),
            5 => Ok(MetadataType::CompoundTag),
            6 => Ok(MetadataType::BlockPos),
            7 => Ok(MetadataType::Long),
            8 => Ok(MetadataType::Vec3),
            _ => Err(ProtocolError::DecodeError(format!("Invalid MetadataType: {}", val))),
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

/// A single metadata entry
#[derive(Debug, Clone)]
pub enum MetadataValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    CompoundTag(Vec<u8>),
    BlockPos(i32, i32, i32),
    Long(i64),
    Vec3(f32, f32, f32),
}

impl MetadataValue {
    pub fn metadata_type(&self) -> MetadataType {
        match self {
            MetadataValue::Byte(_) => MetadataType::Byte,
            MetadataValue::Short(_) => MetadataType::Short,
            MetadataValue::Int(_) => MetadataType::Int,
            MetadataValue::Float(_) => MetadataType::Float,
            MetadataValue::String(_) => MetadataType::String,
            MetadataValue::CompoundTag(_) => MetadataType::CompoundTag,
            MetadataValue::BlockPos(_, _, _) => MetadataType::BlockPos,
            MetadataValue::Long(_) => MetadataType::Long,
            MetadataValue::Vec3(_, _, _) => MetadataType::Vec3,
        }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        match self {
            MetadataValue::Byte(v) => writer.write_i8(*v),
            MetadataValue::Short(v) => writer.write_i16(*v),
            MetadataValue::Int(v) => writer.write_i32(*v),
            MetadataValue::Float(v) => writer.write_f32(*v),
            MetadataValue::String(v) => writer.write_string(v),
            MetadataValue::CompoundTag(v) => {
                writer.write_u16_le(v.len() as u16);
                writer.write_bytes(v);
            }
            MetadataValue::BlockPos(x, y, z) => {
                writer.write_i32(*x);
                writer.write_i32(*y);
                writer.write_i32(*z);
            }
            MetadataValue::Long(v) => writer.write_i64(*v),
            MetadataValue::Vec3(x, y, z) => {
                writer.write_f32(*x);
                writer.write_f32(*y);
                writer.write_f32(*z);
            }
        }
    }

    pub fn decode(reader: &mut BinaryReader, ty: MetadataType) -> Result<Self, ProtocolError> {
        Ok(match ty {
            MetadataType::Byte => MetadataValue::Byte(reader.read_i8()?),
            MetadataType::Short => MetadataValue::Short(reader.read_i16()?),
            MetadataType::Int => MetadataValue::Int(reader.read_i32()?),
            MetadataType::Float => MetadataValue::Float(reader.read_f32()?),
            MetadataType::String => MetadataValue::String(reader.read_string_owned()?),
            MetadataType::CompoundTag => {
                let len = reader.read_u16_le()? as usize;
                let data = reader.read_vec(len)?;
                MetadataValue::CompoundTag(data)
            }
            MetadataType::BlockPos => {
                let x = reader.read_i32()?;
                let y = reader.read_i32()?;
                let z = reader.read_i32()?;
                MetadataValue::BlockPos(x, y, z)
            }
            MetadataType::Long => MetadataValue::Long(reader.read_i64()?),
            MetadataType::Vec3 => {
                let x = reader.read_f32()?;
                let y = reader.read_f32()?;
                let z = reader.read_f32()?;
                MetadataValue::Vec3(x, y, z)
            }
        })
    }
}

/// Entity metadata: a map of (key -> (type, value))
pub type EntityMetadata = Vec<(u32, MetadataValue)>;

/// Encode entity metadata
pub fn encode_entity_metadata(writer: &mut BinaryWriter, metadata: &EntityMetadata) {
    writer.write_var_uint(metadata.len() as u32);
    for (key, value) in metadata {
        writer.write_var_uint(*key);
        writer.write_u8(value.metadata_type().as_u8());
        value.encode(writer);
    }
}

/// Decode entity metadata
pub fn decode_entity_metadata(reader: &mut BinaryReader) -> Result<EntityMetadata, ProtocolError> {
    let count = reader.read_var_uint()? as usize;
    let mut metadata = Vec::with_capacity(count.min(65536));
    for _ in 0..count {
        let key = reader.read_var_uint()?;
        let ty = MetadataType::from_u8(reader.read_u8()?)?;
        let value = MetadataValue::decode(reader, ty)?;
        metadata.push((key, value));
    }
    Ok(metadata)
}

// ============================================================================
// ItemInstance
// ============================================================================

/// Represents an item instance in the MCPE protocol.
#[derive(Debug, Clone)]
pub struct ItemInstance {
    pub network_id: i32,
    pub aux_value: i16,
    pub nbt_len: u16,
    pub nbt: Option<Vec<u8>>,
}

impl ItemInstance {
    pub fn empty() -> Self {
        Self {
            network_id: 0,
            aux_value: 0,
            nbt_len: 0,
            nbt: None,
        }
    }

    pub fn new(network_id: i32, aux_value: i16, nbt: Option<Vec<u8>>) -> Self {
        let nbt_len = nbt.as_ref().map_or(0, |n| n.len() as u16);
        Self {
            network_id,
            aux_value,
            nbt_len,
            nbt,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.network_id == 0
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_i32(self.network_id);
        if self.network_id != 0 {
            writer.write_i16(self.aux_value);
            if let Some(ref nbt) = self.nbt {
                writer.write_u16_le(nbt.len() as u16);
                writer.write_bytes(nbt);
            } else {
                writer.write_u16_le(0);
            }
        }
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let network_id = reader.read_i32()?;
        if network_id == 0 {
            return Ok(Self::empty());
        }
        let aux_value = reader.read_i16()?;
        let nbt_len = reader.read_u16_le()?;
        let nbt = if nbt_len > 0 {
            let data = reader.read_vec(nbt_len as usize)?;
            Some(data)
        } else {
            None
        };
        Ok(Self {
            network_id,
            aux_value,
            nbt_len,
            nbt,
        })
    }
}

// ============================================================================
// Attribute
// ============================================================================

/// Represents an entity attribute.
#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub default: f32,
}

impl Attribute {
    pub fn new(name: String, min: f32, max: f32, value: f32, default: f32) -> Self {
        Self { name, min, max, value, default }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_f32(self.min);
        writer.write_f32(self.max);
        writer.write_f32(self.value);
        writer.write_f32(self.default);
        writer.write_string(&self.name);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let min = reader.read_f32()?;
        let max = reader.read_f32()?;
        let value = reader.read_f32()?;
        let default = reader.read_f32()?;
        let name = reader.read_string_owned()?;
        Ok(Self { name, min, max, value, default })
    }
}

// ============================================================================
// EntityLink
// ============================================================================

/// Represents a link between two entities (e.g., rider/ridden).
#[derive(Debug, Clone)]
pub struct EntityLink {
    pub ridden_entity_id: u64,
    pub rider_entity_id: u64,
    pub link_type: u8,
}

impl EntityLink {
    pub fn new(ridden_entity_id: u64, rider_entity_id: u64, link_type: u8) -> Self {
        Self { ridden_entity_id, rider_entity_id, link_type }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_u64(self.ridden_entity_id);
        writer.write_u64(self.rider_entity_id);
        writer.write_u8(self.link_type);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let ridden_entity_id = reader.read_u64()?;
        let rider_entity_id = reader.read_u64()?;
        let link_type = reader.read_u8()?;
        Ok(Self { ridden_entity_id, rider_entity_id, link_type })
    }
}

// ============================================================================
// ResourcePackInfo
// ============================================================================

/// Information about a resource pack.
#[derive(Debug, Clone)]
pub struct ResourcePackInfo {
    pub id: String,
    pub version: String,
    pub size: u64,
    pub encryption_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
}

impl ResourcePackInfo {
    pub fn new(id: String, version: String, size: u64) -> Self {
        Self {
            id,
            version,
            size,
            encryption_key: String::new(),
            sub_pack_name: String::new(),
            content_identity: String::new(),
            has_scripts: false,
        }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_string(&self.id);
        writer.write_string(&self.version);
        writer.write_u64(self.size);
        writer.write_string(&self.encryption_key);
        writer.write_string(&self.sub_pack_name);
        writer.write_string(&self.content_identity);
        writer.write_bool(self.has_scripts);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let id = reader.read_string_owned()?;
        let version = reader.read_string_owned()?;
        let size = reader.read_u64()?;
        let encryption_key = reader.read_string_owned()?;
        let sub_pack_name = reader.read_string_owned()?;
        let content_identity = reader.read_string_owned()?;
        let has_scripts = reader.read_bool()?;
        Ok(Self { id, version, size, encryption_key, sub_pack_name, content_identity, has_scripts })
    }
}

// ============================================================================
// CommandData / CommandOverload / CommandParameter
// ============================================================================

/// A command parameter definition.
#[derive(Debug, Clone)]
pub struct CommandParameter {
    pub name: String,
    pub type_id: u32,
    pub optional: bool,
    pub flags: u8,
}

impl CommandParameter {
    pub fn new(name: String, type_id: u32, optional: bool, flags: u8) -> Self {
        Self { name, type_id, optional, flags }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_string(&self.name);
        writer.write_u32_le(self.type_id);
        writer.write_bool(self.optional);
        writer.write_u8(self.flags);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let name = reader.read_string_owned()?;
        let type_id = reader.read_u32_le()?;
        let optional = reader.read_bool()?;
        let flags = reader.read_u8()?;
        Ok(Self { name, type_id, optional, flags })
    }
}

/// A command overload (set of parameter overloads).
#[derive(Debug, Clone)]
pub struct CommandOverload {
    pub parameters: Vec<CommandParameter>,
}

impl CommandOverload {
    pub fn new(parameters: Vec<CommandParameter>) -> Self {
        Self { parameters }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_bool(!self.parameters.is_empty());
        if !self.parameters.is_empty() {
            writer.write_var_uint(self.parameters.len() as u32);
            for param in &self.parameters {
                param.encode(writer);
            }
        }
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let has_params = reader.read_bool()?;
        let parameters = if has_params {
            let count = reader.read_var_uint()? as usize;
            let mut params = Vec::with_capacity(count.min(256));
            for _ in 0..count {
                params.push(CommandParameter::decode(reader)?);
            }
            params
        } else {
            Vec::new()
        };
        Ok(Self { parameters })
    }
}

/// Command data definition.
#[derive(Debug, Clone)]
pub struct CommandData {
    pub name: String,
    pub description: String,
    pub flags: u32,
    pub permission: u32,
    pub aliases: Option<u64>,
    pub overloads: Vec<CommandOverload>,
}

impl CommandData {
    pub fn new(name: String, description: String, flags: u32, permission: u32, overloads: Vec<CommandOverload>) -> Self {
        Self { name, description, flags, permission, aliases: None, overloads }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_string(&self.name);
        writer.write_string(&self.description);
        writer.write_u8(self.flags as u8);
        writer.write_u8(self.permission as u8);
        writer.write_bool(self.aliases.is_some());
        if let Some(alias) = self.aliases {
            writer.write_u64(alias);
        }
        writer.write_var_uint(self.overloads.len() as u32);
        for overload in &self.overloads {
            overload.encode(writer);
        }
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let name = reader.read_string_owned()?;
        let description = reader.read_string_owned()?;
        let flags = reader.read_u8()? as u32;
        let permission = reader.read_u8()? as u32;
        let has_aliases = reader.read_bool()?;
        let aliases = if has_aliases {
            Some(reader.read_u64()?)
        } else {
            None
        };
        let overload_count = reader.read_var_uint()? as usize;
        let mut overloads = Vec::with_capacity(overload_count.min(256));
        for _ in 0..overload_count {
            overloads.push(CommandOverload::decode(reader)?);
        }
        Ok(Self { name, description, flags, permission, aliases, overloads })
    }
}

// ============================================================================
// BlockPosition
// ============================================================================

/// A block position in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPosition {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_i32(self.x);
        writer.write_i32(self.y);
        writer.write_i32(self.z);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let z = reader.read_i32()?;
        Ok(Self { x, y, z })
    }
}

// ============================================================================
// Vector3f
// ============================================================================

/// A 3D floating-point vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_f32(self.x);
        writer.write_f32(self.y);
        writer.write_f32(self.z);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;
        Ok(Self { x, y, z })
    }
}

// ============================================================================
// GameRule
// ============================================================================

/// A game rule entry.
#[derive(Debug, Clone)]
pub struct GameRule {
    pub name: String,
    pub editable: bool,
    pub value: GameRuleValue,
}

/// Game rule value types.
#[derive(Debug, Clone)]
pub enum GameRuleValue {
    Bool(bool),
    Int(i32),
    Float(f32),
}

impl GameRule {
    pub fn bool_rule(name: &str, editable: bool, value: bool) -> Self {
        Self { name: name.to_string(), editable, value: GameRuleValue::Bool(value) }
    }

    pub fn int_rule(name: &str, editable: bool, value: i32) -> Self {
        Self { name: name.to_string(), editable, value: GameRuleValue::Int(value) }
    }

    pub fn float_rule(name: &str, editable: bool, value: f32) -> Self {
        Self { name: name.to_string(), editable, value: GameRuleValue::Float(value) }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_bool(self.editable);
        match &self.value {
            GameRuleValue::Bool(v) => {
                writer.write_u8(1); // bool type
                writer.write_bool(*v);
            }
            GameRuleValue::Int(v) => {
                writer.write_u8(2); // int type
                writer.write_i32(*v);
            }
            GameRuleValue::Float(v) => {
                writer.write_u8(3); // float type
                writer.write_f32(*v);
            }
        }
        writer.write_string(&self.name);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let editable = reader.read_bool()?;
        let value_type = reader.read_u8()?;
        let value = match value_type {
            1 => GameRuleValue::Bool(reader.read_bool()?),
            2 => GameRuleValue::Int(reader.read_i32()?),
            3 => GameRuleValue::Float(reader.read_f32()?),
            _ => return Err(ProtocolError::DecodeError(format!("Invalid GameRule type: {}", value_type))),
        };
        let name = reader.read_string_owned()?;
        Ok(Self { name, editable, value })
    }
}

// ============================================================================
// PlayerListEntry
// ============================================================================

/// Entry in the player list.
#[derive(Debug, Clone)]
pub struct PlayerListEntry {
    pub uuid: uuid::Uuid,
    pub unique_entity_id: i64,
    pub name: String,
    pub xbox_user_id: String,
    pub platform_chat_id: String,
    pub build_platform: i32,
    pub skin_data: Option<SkinData>,
}

/// Skin data for a player.
#[derive(Debug, Clone)]
pub struct SkinData {
    pub skin_id: String,
    pub skin_resource_patch: String,
    pub skin_data: Vec<u8>,
    pub animation_data: Vec<u8>,
    pub cape_data: Vec<u8>,
    pub geometry_data: Vec<u8>,
    pub geometry_name: String,
    pub animated_image_data: Vec<u8>,
}

impl PlayerListEntry {
    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_uuid(&self.uuid);
        writer.write_i64(self.unique_entity_id);
        writer.write_string(&self.name);
        writer.write_string(&self.xbox_user_id);
        writer.write_string(&self.platform_chat_id);
        writer.write_i32(self.build_platform);
        if let Some(ref skin) = self.skin_data {
            writer.write_bool(true);
            writer.write_string(&skin.skin_id);
            writer.write_string(&skin.skin_resource_patch);
            writer.write_var_uint(skin.skin_data.len() as u32);
            writer.write_bytes(&skin.skin_data);
            writer.write_var_uint(skin.animation_data.len() as u32);
            writer.write_bytes(&skin.animation_data);
            writer.write_var_uint(skin.cape_data.len() as u32);
            writer.write_bytes(&skin.cape_data);
            writer.write_string(&skin.geometry_name);
            writer.write_var_uint(skin.geometry_data.len() as u32);
            writer.write_bytes(&skin.geometry_data);
            writer.write_var_uint(skin.animated_image_data.len() as u32);
            writer.write_bytes(&skin.animated_image_data);
        } else {
            writer.write_bool(false);
        }
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let uuid = reader.read_uuid()?;
        let unique_entity_id = reader.read_i64()?;
        let name = reader.read_string_owned()?;
        let xbox_user_id = reader.read_string_owned()?;
        let platform_chat_id = reader.read_string_owned()?;
        let build_platform = reader.read_i32()?;
        let has_skin = reader.read_bool()?;
        let skin_data = if has_skin {
            let skin_id = reader.read_string_owned()?;
            let skin_resource_patch = reader.read_string_owned()?;
            let skin_data_len = reader.read_var_uint()? as usize;
            let skin_data_bytes = reader.read_vec(skin_data_len)?;
            let animation_data_len = reader.read_var_uint()? as usize;
            let animation_data = reader.read_vec(animation_data_len)?;
            let cape_data_len = reader.read_var_uint()? as usize;
            let cape_data = reader.read_vec(cape_data_len)?;
            let geometry_name = reader.read_string_owned()?;
            let geometry_data_len = reader.read_var_uint()? as usize;
            let geometry_data = reader.read_vec(geometry_data_len)?;
            let animated_image_data_len = reader.read_var_uint()? as usize;
            let animated_image_data = reader.read_vec(animated_image_data_len)?;
            Some(SkinData {
                skin_id,
                skin_resource_patch,
                skin_data: skin_data_bytes,
                animation_data,
                cape_data,
                geometry_data,
                geometry_name,
                animated_image_data,
            })
        } else {
            None
        };
        Ok(Self {
            uuid,
            unique_entity_id,
            name,
            xbox_user_id,
            platform_chat_id,
            build_platform,
            skin_data,
        })
    }
}

// ============================================================================
// UpdateBlockFlag
// ============================================================================

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UpdateBlockFlag: u32 {
        const NONE = 0;
        const NEIGHBORS = 0b001;
        const NETWORK = 0b010;
        const NO_GRAPHIC = 0b100;
    }
}

// ============================================================================
// ContainerType
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerType {
    Inventory = 0,
    Container = 1,
    Workbench = 2,
    Furnace = 3,
    Enchantment = 4,
    Brewing = 5,
    Anvil = 6,
    Dispenser = 7,
    Hopper = 8,
    Cauldron = 9,
    MinecartHopper = 10,
}

impl ContainerType {
    pub fn from_i8(val: i8) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(ContainerType::Inventory),
            1 => Ok(ContainerType::Container),
            2 => Ok(ContainerType::Workbench),
            3 => Ok(ContainerType::Furnace),
            4 => Ok(ContainerType::Enchantment),
            5 => Ok(ContainerType::Brewing),
            6 => Ok(ContainerType::Anvil),
            7 => Ok(ContainerType::Dispenser),
            8 => Ok(ContainerType::Hopper),
            9 => Ok(ContainerType::Cauldron),
            10 => Ok(ContainerType::MinecartHopper),
            _ => Err(ProtocolError::DecodeError(format!("Invalid ContainerType: {}", val))),
        }
    }

    pub fn as_i8(&self) -> i8 {
        *self as i8
    }
}

// ============================================================================
// InteractAction
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractAction {
    LeaveVehicle = 0,
    MouseOver = 1,
    OpenInventory = 2,
}

impl InteractAction {
    pub fn from_u8(val: u8) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(InteractAction::LeaveVehicle),
            1 => Ok(InteractAction::MouseOver),
            2 => Ok(InteractAction::OpenInventory),
            _ => Err(ProtocolError::DecodeError(format!("Invalid InteractAction: {}", val))),
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

// ============================================================================
// AdventureSettingsFlags
// ============================================================================

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AdventureSettingsFlags: u32 {
        const WORLD_IMMUTABLE = 0x01;
        const NO_PVP = 0x02;
        const NO_PVM = 0x04;
        const NO_MVP = 0x08;
        const AUTO_JUMP = 0x10;
        const ALLOW_FLIGHT = 0x20;
        const NO_CLIP = 0x40;
        const WORLD_BUILDER = 0x80;
        const FLYING = 0x100;
        const MUTED = 0x200;
    }
}

// ============================================================================
// BossEventType
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossEventType {
    Show = 0,
    Hide = 1,
    HealthPercent = 2,
    Title = 3,
}

impl BossEventType {
    pub fn from_u32(val: u32) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(BossEventType::Show),
            1 => Ok(BossEventType::Hide),
            2 => Ok(BossEventType::HealthPercent),
            3 => Ok(BossEventType::Title),
            _ => Err(ProtocolError::DecodeError(format!("Invalid BossEventType: {}", val))),
        }
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

// ============================================================================
// LevelEvent
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelEvent {
    SoundClick = 1000,
    SoundClickFail = 1001,
    SoundShoot = 1002,
    SoundDoor = 1003,
    SoundFizz = 1004,
    SoundTnt = 1005,
    SoundGhast = 1007,
    SoundGhastCharge = 1008,
    SoundBlazeFire = 1009,
    SoundBlazeCharge = 1010,
    SoundZombieWood = 1012,
    SoundZombieMetal = 1013,
    SoundZombieDoor = 1016,
    SoundEndermanTeleport = 1018,
    ParticleDestroy = 2000,
    ParticleSplash = 2001,
    ParticleEyeDespawn = 2002,
    ParticleMobBlockSpawn = 2004,
    ParticleCropGrowth = 2005,
    ParticleSoundGuardianGhost = 2006,
    ParticleDeathSmoke = 2007,
    ParticleDenyBlock = 2008,
    ParticleHappyVillager = 2009,
    ParticlesHugeExplosion = 2010,
    ParticleDragonBreath = 2011,
    StartRain = 3001,
    StopRain = 3002,
    StartThunder = 3003,
    StopThunder = 3004,
    RedstoneTrigger = 3500,
    CauldronExplode = 3501,
    CauldronDyeArmor = 3502,
    CauldronCleanArmor = 3503,
    CauldronFillPotion = 3504,
    CauldronTakePotion = 3505,
    CauldronFillWater = 3506,
    CauldronTakeWater = 3507,
    CauldronAddDye = 3508,
    SetData = 4000,
    PlaySound = 4001,
}

impl LevelEvent {
    pub fn from_i32(val: i32) -> Result<Self, ProtocolError> {
        match val {
            1000 => Ok(LevelEvent::SoundClick),
            1001 => Ok(LevelEvent::SoundClickFail),
            1002 => Ok(LevelEvent::SoundShoot),
            1003 => Ok(LevelEvent::SoundDoor),
            1004 => Ok(LevelEvent::SoundFizz),
            1005 => Ok(LevelEvent::SoundTnt),
            1007 => Ok(LevelEvent::SoundGhast),
            1008 => Ok(LevelEvent::SoundGhastCharge),
            1009 => Ok(LevelEvent::SoundBlazeFire),
            1010 => Ok(LevelEvent::SoundBlazeCharge),
            1012 => Ok(LevelEvent::SoundZombieWood),
            1013 => Ok(LevelEvent::SoundZombieMetal),
            1016 => Ok(LevelEvent::SoundZombieDoor),
            1018 => Ok(LevelEvent::SoundEndermanTeleport),
            2000 => Ok(LevelEvent::ParticleDestroy),
            2001 => Ok(LevelEvent::ParticleSplash),
            2002 => Ok(LevelEvent::ParticleEyeDespawn),
            2004 => Ok(LevelEvent::ParticleMobBlockSpawn),
            2005 => Ok(LevelEvent::ParticleCropGrowth),
            2006 => Ok(LevelEvent::ParticleSoundGuardianGhost),
            2007 => Ok(LevelEvent::ParticleDeathSmoke),
            2008 => Ok(LevelEvent::ParticleDenyBlock),
            2009 => Ok(LevelEvent::ParticleHappyVillager),
            2010 => Ok(LevelEvent::ParticlesHugeExplosion),
            2011 => Ok(LevelEvent::ParticleDragonBreath),
            3001 => Ok(LevelEvent::StartRain),
            3002 => Ok(LevelEvent::StopRain),
            3003 => Ok(LevelEvent::StartThunder),
            3004 => Ok(LevelEvent::StopThunder),
            3500 => Ok(LevelEvent::RedstoneTrigger),
            3501 => Ok(LevelEvent::CauldronExplode),
            3502 => Ok(LevelEvent::CauldronDyeArmor),
            3503 => Ok(LevelEvent::CauldronCleanArmor),
            3504 => Ok(LevelEvent::CauldronFillPotion),
            3505 => Ok(LevelEvent::CauldronTakePotion),
            3506 => Ok(LevelEvent::CauldronFillWater),
            3507 => Ok(LevelEvent::CauldronTakeWater),
            3508 => Ok(LevelEvent::CauldronAddDye),
            4000 => Ok(LevelEvent::SetData),
            4001 => Ok(LevelEvent::PlaySound),
            _ => Err(ProtocolError::DecodeError(format!("Invalid LevelEvent: {}", val))),
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

// ============================================================================
// EntityEvent
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityEvent {
    HurtAnimation = 0,
    HurtAnimation2 = 1,
    DeathAnimation = 2,
    TameFail = 6,
    TameSuccess = 7,
    ShakeWet = 8,
    UseItem = 9,
    EatGrassAnimation = 10,
    FishHookBubbles = 11,
    FishHookPosition = 12,
    FishHookHook = 13,
    FishHookTease = 14,
    SquidInkCloud = 15,
    ZombieVillagerCure = 16,
    ZombieVillagerCure2 = 17,
    Respawn = 18,
    IronGolemHoldFlower = 19,
    LoveParticles = 20,
    VillagerAngry = 21,
    VillagerHappy = 22,
    WitchSpellParticles = 23,
    FireworkParticles = 24,
    InLoveParticles = 25,
    SilverfishMergeWithStone = 26,
    GuardianAttack = 27,
    WitchHutParticles = 28,
}

impl EntityEvent {
    pub fn from_u8(val: u8) -> Result<Self, ProtocolError> {
        match val {
            0 => Ok(EntityEvent::HurtAnimation),
            1 => Ok(EntityEvent::HurtAnimation2),
            2 => Ok(EntityEvent::DeathAnimation),
            6 => Ok(EntityEvent::TameFail),
            7 => Ok(EntityEvent::TameSuccess),
            8 => Ok(EntityEvent::ShakeWet),
            9 => Ok(EntityEvent::UseItem),
            10 => Ok(EntityEvent::EatGrassAnimation),
            11 => Ok(EntityEvent::FishHookBubbles),
            12 => Ok(EntityEvent::FishHookPosition),
            13 => Ok(EntityEvent::FishHookHook),
            14 => Ok(EntityEvent::FishHookTease),
            15 => Ok(EntityEvent::SquidInkCloud),
            16 => Ok(EntityEvent::ZombieVillagerCure),
            17 => Ok(EntityEvent::ZombieVillagerCure2),
            18 => Ok(EntityEvent::Respawn),
            19 => Ok(EntityEvent::IronGolemHoldFlower),
            20 => Ok(EntityEvent::LoveParticles),
            21 => Ok(EntityEvent::VillagerAngry),
            22 => Ok(EntityEvent::VillagerHappy),
            23 => Ok(EntityEvent::WitchSpellParticles),
            24 => Ok(EntityEvent::FireworkParticles),
            25 => Ok(EntityEvent::InLoveParticles),
            26 => Ok(EntityEvent::SilverfishMergeWithStone),
            27 => Ok(EntityEvent::GuardianAttack),
            28 => Ok(EntityEvent::WitchHutParticles),
            _ => Err(ProtocolError::DecodeError(format!("Invalid EntityEvent: {}", val))),
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

// ============================================================================
// AnimateAction
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimateAction {
    SwingArm = 1,
    CriticalHit = 2,
    MagicCriticalHit = 3,
}

impl AnimateAction {
    pub fn from_u32(val: u32) -> Result<Self, ProtocolError> {
        match val {
            1 => Ok(AnimateAction::SwingArm),
            2 => Ok(AnimateAction::CriticalHit),
            3 => Ok(AnimateAction::MagicCriticalHit),
            _ => Err(ProtocolError::DecodeError(format!("Invalid AnimateAction: {}", val))),
        }
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

// ============================================================================
// MobEffectEvent
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobEffectEvent {
    Add = 1,
    Modify = 2,
    Remove = 3,
}

impl MobEffectEvent {
    pub fn from_u8(val: u8) -> Result<Self, ProtocolError> {
        match val {
            1 => Ok(MobEffectEvent::Add),
            2 => Ok(MobEffectEvent::Modify),
            3 => Ok(MobEffectEvent::Remove),
            _ => Err(ProtocolError::DecodeError(format!("Invalid MobEffectEvent: {}", val))),
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
