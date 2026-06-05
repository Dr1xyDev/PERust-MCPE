use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use perust_utils::{BinaryReader, BinaryWriter};

// Note: FullChunkDataPacket is in world.rs to avoid duplication.
// This module contains only chunk-radius related packets.

// ============================================================================
// RequestChunkRadiusPacket
// ============================================================================

/// Client requests a certain chunk radius for rendering.
#[derive(Debug, Clone)]
pub struct RequestChunkRadiusPacket {
    pub radius: i32,
}

impl Packet for RequestChunkRadiusPacket {
    const PACKET_ID: u8 = protocol_info::REQUEST_CHUNK_RADIUS_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.radius);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            radius: reader.read_i32()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "RequestChunkRadiusPacket"
    }
}

// ============================================================================
// ChunkRadiusUpdatedPacket
// ============================================================================

/// Server responds with the actual chunk radius.
#[derive(Debug, Clone)]
pub struct ChunkRadiusUpdatedPacket {
    pub radius: i32,
}

impl Packet for ChunkRadiusUpdatedPacket {
    const PACKET_ID: u8 = protocol_info::CHUNK_RADIUS_UPDATED_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i32(self.radius);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            radius: reader.read_i32()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "ChunkRadiusUpdatedPacket"
    }
}
