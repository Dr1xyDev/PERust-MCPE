use crate::error::ProtocolError;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use perust_utils::BinaryReader;
use std::io::{Read, Write};

/// Decode a batch packet (0xfe) into individual raw packets.
///
/// The batch packet contains zlib-compressed data, which when decompressed
/// contains a series of sub-packets, each prefixed with a VarInt length.
pub fn decode_batch(data: &[u8]) -> Result<Vec<Vec<u8>>, ProtocolError> {
    // Decompress with zlib
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|_| ProtocolError::CompressionError)?;

    // Parse sub-packets
    let mut reader = BinaryReader::new(&decompressed);
    let mut packets = Vec::new();

    while reader.remaining() > 0 {
        let length = reader.read_var_uint()?;
        if length as usize > reader.remaining() {
            return Err(ProtocolError::DecodeError(format!(
                "Packet length {} exceeds remaining data {}",
                length,
                reader.remaining()
            )));
        }
        let packet_data = reader.read_vec(length as usize)?;
        packets.push(packet_data);
    }

    Ok(packets)
}

/// Encode multiple raw packets into a batch packet (0xfe).
///
/// Each packet is prefixed with a VarInt length, then the whole thing
/// is zlib-compressed.
pub fn encode_batch(packets: &[Vec<u8>]) -> Result<Vec<u8>, ProtocolError> {
    use perust_utils::BinaryWriter;

    // Build the payload: each packet with VarInt length prefix
    let mut payload_writer = BinaryWriter::new();
    for packet in packets {
        payload_writer.write_var_uint(packet.len() as u32);
        payload_writer.write_bytes(packet);
    }
    let payload = payload_writer.into_vec();

    // Compress with zlib
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&payload)
        .map_err(|_| ProtocolError::CompressionError)?;
    encoder
        .finish()
        .map_err(|_| ProtocolError::CompressionError)
}
