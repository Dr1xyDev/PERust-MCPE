use crate::error::ProtocolError;
use perust_utils::{BinaryReader, BinaryWriter};

/// Trait for all MCPE protocol packets.
pub trait Packet: Send + Sync {
    const PACKET_ID: u8;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError>;

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError>
    where
        Self: Sized;

    fn packet_name(&self) -> &'static str;
}

/// Header of a packet, containing the packet ID.
pub struct PacketHeader {
    pub packet_id: u8,
}

/// Decode the packet ID from raw packet data.
/// The packet ID is the first byte.
pub fn decode_packet_id(data: &[u8]) -> Result<u8, ProtocolError> {
    if data.is_empty() {
        return Err(ProtocolError::DecodeError("Empty packet data".to_string()));
    }
    Ok(data[0])
}

/// Encode a packet into raw bytes (packet ID + payload).
pub fn encode_packet<P: Packet>(packet: &P) -> Result<Vec<u8>, ProtocolError> {
    let mut writer = BinaryWriter::new();
    writer.write_u8(P::PACKET_ID);
    packet.encode(&mut writer)?;
    Ok(writer.into_vec())
}

/// Decode a packet header from raw data.
pub fn decode_packet_header(data: &[u8]) -> Result<PacketHeader, ProtocolError> {
    let packet_id = decode_packet_id(data)?;
    Ok(PacketHeader { packet_id })
}

/// Create a BinaryReader positioned after the packet ID byte.
pub fn create_reader(data: &[u8]) -> Result<BinaryReader<'_>, ProtocolError> {
    if data.is_empty() {
        return Err(ProtocolError::DecodeError("Empty packet data".to_string()));
    }
    let mut reader = BinaryReader::new(data);
    let _packet_id = reader.read_u8()?; // skip packet ID
    Ok(reader)
}
