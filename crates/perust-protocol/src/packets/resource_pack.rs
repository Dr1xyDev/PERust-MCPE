use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::ResourcePackInfo;
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// ResourcePacksInfoPacket
// ============================================================================

/// Sent by the server to inform the client about available resource packs.
#[derive(Debug, Clone)]
pub struct ResourcePacksInfoPacket {
    pub must_accept: bool,
    pub has_scripts: bool,
    pub behavior_pack_infos: Vec<ResourcePackInfo>,
    pub resource_pack_infos: Vec<ResourcePackInfo>,
}

impl Packet for ResourcePacksInfoPacket {
    const PACKET_ID: u8 = protocol_info::RESOURCE_PACKS_INFO_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bool(self.must_accept);
        writer.write_u16_le(self.behavior_pack_infos.len() as u16);
        for pack in &self.behavior_pack_infos {
            pack.encode(writer);
        }
        writer.write_u16_le(self.resource_pack_infos.len() as u16);
        for pack in &self.resource_pack_infos {
            pack.encode(writer);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let must_accept = reader.read_bool()?;
        let behavior_count = reader.read_u16_le()? as usize;
        let mut behavior_pack_infos = Vec::with_capacity(behavior_count.min(256));
        for _ in 0..behavior_count {
            behavior_pack_infos.push(ResourcePackInfo::decode(reader)?);
        }
        let resource_count = reader.read_u16_le()? as usize;
        let mut resource_pack_infos = Vec::with_capacity(resource_count.min(256));
        for _ in 0..resource_count {
            resource_pack_infos.push(ResourcePackInfo::decode(reader)?);
        }
        Ok(Self {
            must_accept,
            has_scripts: false,
            behavior_pack_infos,
            resource_pack_infos,
        })
    }

    fn packet_name(&self) -> &'static str {
        "ResourcePacksInfoPacket"
    }
}

// ============================================================================
// ResourcePackStackPacket
// ============================================================================

/// Sent by the server to specify the order of resource packs.
#[derive(Debug, Clone)]
pub struct ResourcePackStackPacket {
    pub must_accept: bool,
    pub behavior_pack_stack: Vec<ResourcePackStackEntry>,
    pub resource_pack_stack: Vec<ResourcePackStackEntry>,
}

/// Entry in a resource pack stack.
#[derive(Debug, Clone)]
pub struct ResourcePackStackEntry {
    pub id: String,
    pub version: String,
    pub sub_pack_name: String,
}

impl ResourcePackStackEntry {
    pub fn new(id: String, version: String) -> Self {
        Self {
            id,
            version,
            sub_pack_name: String::new(),
        }
    }

    pub fn encode(&self, writer: &mut BinaryWriter) {
        writer.write_string(&self.id);
        writer.write_string(&self.version);
        writer.write_string(&self.sub_pack_name);
    }

    pub fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let id = reader.read_string_owned()?;
        let version = reader.read_string_owned()?;
        let sub_pack_name = reader.read_string_owned()?;
        Ok(Self { id, version, sub_pack_name })
    }
}

impl Packet for ResourcePackStackPacket {
    const PACKET_ID: u8 = protocol_info::RESOURCE_PACK_STACK_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bool(self.must_accept);
        writer.write_u32_le(self.behavior_pack_stack.len() as u32);
        for entry in &self.behavior_pack_stack {
            entry.encode(writer);
        }
        writer.write_u32_le(self.resource_pack_stack.len() as u32);
        for entry in &self.resource_pack_stack {
            entry.encode(writer);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let must_accept = reader.read_bool()?;
        let behavior_count = reader.read_u32_le()? as usize;
        let mut behavior_pack_stack = Vec::with_capacity(behavior_count.min(256));
        for _ in 0..behavior_count {
            behavior_pack_stack.push(ResourcePackStackEntry::decode(reader)?);
        }
        let resource_count = reader.read_u32_le()? as usize;
        let mut resource_pack_stack = Vec::with_capacity(resource_count.min(256));
        for _ in 0..resource_count {
            resource_pack_stack.push(ResourcePackStackEntry::decode(reader)?);
        }
        Ok(Self {
            must_accept,
            behavior_pack_stack,
            resource_pack_stack,
        })
    }

    fn packet_name(&self) -> &'static str {
        "ResourcePackStackPacket"
    }
}

// ============================================================================
// ResourcePackClientResponsePacket
// ============================================================================

/// Sent by the client in response to resource pack info/stack packets.
#[derive(Debug, Clone)]
pub struct ResourcePackClientResponsePacket {
    pub response_status: u8,
    pub pack_ids: Vec<String>,
}

/// Resource pack response status codes.
pub const RESOURCE_PACK_RESPONSE_REFUSED: u8 = 1;
pub const RESOURCE_PACK_RESPONSE_SEND_PACKS: u8 = 2;
pub const RESOURCE_PACK_RESPONSE_HAVE_ALL_PACKS: u8 = 3;
pub const RESOURCE_PACK_RESPONSE_COMPLETED: u8 = 4;

impl Packet for ResourcePackClientResponsePacket {
    const PACKET_ID: u8 = protocol_info::RESOURCE_PACK_CLIENT_RESPONSE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u8(self.response_status);
        writer.write_u16_le(self.pack_ids.len() as u16);
        for id in &self.pack_ids {
            writer.write_string(id);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let response_status = reader.read_u8()?;
        let count = reader.read_u16_le()? as usize;
        let mut pack_ids = Vec::with_capacity(count.min(256));
        for _ in 0..count {
            pack_ids.push(reader.read_string_owned()?);
        }
        Ok(Self { response_status, pack_ids })
    }

    fn packet_name(&self) -> &'static str {
        "ResourcePackClientResponsePacket"
    }
}

// ============================================================================
// ResourcePackDataInfoPacket
// ============================================================================

/// Sent by the server to provide info about a resource pack's data.
#[derive(Debug, Clone)]
pub struct ResourcePackDataInfoPacket {
    pub pack_id: String,
    pub max_chunk_size: u32,
    pub chunk_count: u32,
    pub compressed_pack_size: u64,
    pub hash: Vec<u8>,
}

impl Packet for ResourcePackDataInfoPacket {
    const PACKET_ID: u8 = protocol_info::RESOURCE_PACK_DATA_INFO_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.pack_id);
        writer.write_u32_le(self.max_chunk_size);
        writer.write_u32_le(self.chunk_count);
        writer.write_u64_le(self.compressed_pack_size);
        writer.write_u8(self.hash.len() as u8);
        writer.write_bytes(&self.hash);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let pack_id = reader.read_string_owned()?;
        let max_chunk_size = reader.read_u32_le()?;
        let chunk_count = reader.read_u32_le()?;
        let compressed_pack_size = reader.read_u64_le()?;
        let hash_len = reader.read_u8()? as usize;
        let hash = reader.read_vec(hash_len)?;
        Ok(Self {
            pack_id,
            max_chunk_size,
            chunk_count,
            compressed_pack_size,
            hash,
        })
    }

    fn packet_name(&self) -> &'static str {
        "ResourcePackDataInfoPacket"
    }
}

// ============================================================================
// ResourcePackChunkDataPacket
// ============================================================================

/// Sent by the server to send a chunk of resource pack data.
#[derive(Debug, Clone)]
pub struct ResourcePackChunkDataPacket {
    pub pack_id: String,
    pub chunk_index: u32,
    pub data: Vec<u8>,
}

impl Packet for ResourcePackChunkDataPacket {
    const PACKET_ID: u8 = protocol_info::RESOURCE_PACK_CHUNK_DATA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.pack_id);
        writer.write_u32_le(self.chunk_index);
        writer.write_u32_le(self.data.len() as u32);
        writer.write_bytes(&self.data);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let pack_id = reader.read_string_owned()?;
        let chunk_index = reader.read_u32_le()?;
        let data_len = reader.read_u32_le()? as usize;
        let data = reader.read_vec(data_len)?;
        Ok(Self { pack_id, chunk_index, data })
    }

    fn packet_name(&self) -> &'static str {
        "ResourcePackChunkDataPacket"
    }
}

// ============================================================================
// ResourcePackChunkRequestPacket
// ============================================================================

/// Sent by the client to request a chunk of resource pack data.
#[derive(Debug, Clone)]
pub struct ResourcePackChunkRequestPacket {
    pub pack_id: String,
    pub chunk_index: u32,
}

impl Packet for ResourcePackChunkRequestPacket {
    const PACKET_ID: u8 = protocol_info::RESOURCE_PACK_CHUNK_REQUEST_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_string(&self.pack_id);
        writer.write_u32_le(self.chunk_index);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let pack_id = reader.read_string_owned()?;
        let chunk_index = reader.read_u32_le()?;
        Ok(Self { pack_id, chunk_index })
    }

    fn packet_name(&self) -> &'static str {
        "ResourcePackChunkRequestPacket"
    }
}
