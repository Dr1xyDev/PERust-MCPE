//! Encapsulated packet structure for RakNet connected sessions.
//!
//! Encapsulated packets are the payload units carried inside RakNet datagrams.
//! Each encapsulated packet has a reliability type that determines delivery
//! guarantees, optional ordering/sequencing indices, and optional split
//! information for large payloads.

use crate::error::RakNetError;
use crate::protocol::{decode_triple_byte, encode_triple_byte};
use crate::reliability::Reliability;

/// An encapsulated packet within a RakNet datagram.
///
/// Encapsulated packets carry the actual game data and are subject to RakNet's
/// reliability system. Depending on the reliability type, they may include
/// message indices (for ACK tracking), sequence indices (for sequencing),
/// and order indices (for ordering within channels).
#[derive(Debug, Clone)]
pub struct EncapsulatedPacket {
    /// The reliability type for this packet.
    pub reliability: Reliability,
    /// Whether this packet is part of a split payload.
    pub has_split: bool,
    /// The payload data.
    pub buffer: Vec<u8>,
    /// Message index (only for reliable packets).
    pub message_index: u32,
    /// Sequence index (only for sequenced packets).
    pub sequence_index: u32,
    /// Order index (for ordered/sequenced packets).
    pub order_index: u32,
    /// Order channel (0-31, for ordered/sequenced packets).
    pub order_channel: u8,
    /// Split packet count (only if has_split).
    pub split_count: u32,
    /// Split packet ID (only if has_split).
    pub split_id: u16,
    /// Split packet index (only if has_split).
    pub split_index: u32,
}

impl EncapsulatedPacket {
    /// Creates a new encapsulated packet with the given reliability and payload.
    pub fn new(reliability: Reliability, buffer: Vec<u8>) -> Self {
        EncapsulatedPacket {
            reliability,
            has_split: false,
            buffer,
            message_index: 0,
            sequence_index: 0,
            order_index: 0,
            order_channel: 0,
            split_count: 0,
            split_id: 0,
            split_index: 0,
        }
    }

    /// Creates a reliable ordered encapsulated packet on the given channel.
    pub fn reliable_ordered(buffer: Vec<u8>, channel: u8) -> Self {
        EncapsulatedPacket {
            reliability: Reliability::ReliableOrdered,
            has_split: false,
            buffer,
            message_index: 0,
            sequence_index: 0,
            order_index: 0,
            order_channel: channel,
            split_count: 0,
            split_id: 0,
            split_index: 0,
        }
    }

    /// Creates an unreliable encapsulated packet.
    pub fn unreliable(buffer: Vec<u8>) -> Self {
        EncapsulatedPacket {
            reliability: Reliability::Unreliable,
            has_split: false,
            buffer,
            message_index: 0,
            sequence_index: 0,
            order_index: 0,
            order_channel: 0,
            split_count: 0,
            split_id: 0,
            split_index: 0,
        }
    }

    /// Returns the total encoded size of this encapsulated packet in bytes.
    pub fn encoded_size(&self) -> usize {
        let mut size = 3; // flags (1) + buffer length (2)
        if self.reliability.needs_message_index() {
            size += 3; // message_index as triple byte
        }
        if self.reliability.needs_sequence_index() {
            size += 3; // sequence_index as triple byte
        }
        if self.reliability.needs_ordering() {
            size += 4; // order_index (3) + order_channel (1)
        }
        if self.has_split {
            size += 10; // split_count (4) + split_id (2) + split_index (4)
        }
        size += self.buffer.len();
        size
    }

    /// Encodes this encapsulated packet into a byte vector.
    ///
    /// Binary format:
    /// ```text
    /// 1 byte: (reliability << 5) | (has_split << 4)
    /// 2 bytes: buffer length << 3 (big-endian)
    /// [3 bytes: message_index] if reliable
    /// [3 bytes: sequence_index] if sequenced
    /// [3 bytes: order_index + 1 byte: order_channel] if sequenced or ordered
    /// [4+2+4 bytes: split fields] if has_split
    /// N bytes: payload
    /// ```
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.encoded_size());

        // Flags byte: reliability (upper 3 bits) | has_split (bit 4)
        let flags = (self.reliability.as_u8() << 5) | ((self.has_split as u8) << 4);
        buf.push(flags);

        // Buffer length, shifted left by 3 bits (2 bytes, big-endian)
        let length_encoded = (self.buffer.len() as u16) << 3;
        buf.extend_from_slice(&length_encoded.to_be_bytes());

        // Message index (if reliable)
        if self.reliability.needs_message_index() {
            buf.extend_from_slice(&encode_triple_byte(self.message_index));
        }

        // Sequence index (if sequenced)
        if self.reliability.needs_sequence_index() {
            buf.extend_from_slice(&encode_triple_byte(self.sequence_index));
        }

        // Order index + channel (if sequenced or ordered)
        if self.reliability.needs_ordering() {
            buf.extend_from_slice(&encode_triple_byte(self.order_index));
            buf.push(self.order_channel);
        }

        // Split fields (if has_split)
        if self.has_split {
            buf.extend_from_slice(&self.split_count.to_be_bytes());
            buf.extend_from_slice(&self.split_id.to_be_bytes());
            buf.extend_from_slice(&self.split_index.to_be_bytes());
        }

        // Payload
        buf.extend_from_slice(&self.buffer);

        buf
    }

    /// Decodes an encapsulated packet from a byte slice.
    ///
    /// Returns the decoded packet and the number of bytes consumed.
    pub fn decode(data: &[u8]) -> Result<(Self, usize), RakNetError> {
        let mut offset = 0;

        if data.is_empty() {
            return Err(RakNetError::DecodeError(
                "empty data for encapsulated packet".to_string(),
            ));
        }

        // Flags byte
        let flags = data[offset];
        offset += 1;

        let reliability_value = (flags >> 5) & 0x07;
        let has_split = (flags & 0x10) != 0;

        let reliability = Reliability::from_u8(reliability_value).ok_or_else(|| {
            RakNetError::DecodeError(format!(
                "invalid reliability value: {}",
                reliability_value
            ))
        })?;

        // Buffer length (2 bytes, big-endian, shifted right by 3)
        if data.len() < offset + 2 {
            return Err(RakNetError::DecodeError(
                "not enough data for buffer length".to_string(),
            ));
        }
        let length_encoded = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let buffer_length = (length_encoded >> 3) as usize;
        offset += 2;

        // Message index (if reliable)
        let message_index = if reliability.needs_message_index() {
            decode_triple_byte(data, &mut offset)?
        } else {
            0
        };

        // Sequence index (if sequenced)
        let sequence_index = if reliability.needs_sequence_index() {
            decode_triple_byte(data, &mut offset)?
        } else {
            0
        };

        // Order index + channel (if sequenced or ordered)
        let (order_index, order_channel) = if reliability.needs_ordering() {
            let idx = decode_triple_byte(data, &mut offset)?;
            if offset >= data.len() {
                return Err(RakNetError::DecodeError(
                    "not enough data for order channel".to_string(),
                ));
            }
            let ch = data[offset];
            offset += 1;
            (idx, ch)
        } else {
            (0, 0)
        };

        // Split fields (if has_split)
        let (split_count, split_id, split_index) = if has_split {
            if data.len() < offset + 10 {
                return Err(RakNetError::DecodeError(
                    "not enough data for split fields".to_string(),
                ));
            }
            let count = u32::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]);
            offset += 4;
            let id = u16::from_be_bytes([data[offset], data[offset + 1]]);
            offset += 2;
            let index = u32::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]);
            offset += 4;
            (count, id, index)
        } else {
            (0, 0, 0)
        };

        // Payload
        if data.len() < offset + buffer_length {
            return Err(RakNetError::DecodeError(format!(
                "not enough data for payload: need {} bytes at offset {}, have {}",
                buffer_length,
                offset,
                data.len()
            )));
        }
        let buffer = data[offset..offset + buffer_length].to_vec();
        offset += buffer_length;

        Ok((
            EncapsulatedPacket {
                reliability,
                has_split,
                buffer,
                message_index,
                sequence_index,
                order_index,
                order_channel,
                split_count,
                split_id,
                split_index,
            },
            offset,
        ))
    }
}

/// A partially reassembled split packet.
///
/// When a large payload is split across multiple encapsulated packets,
/// this structure tracks the reassembly progress.
#[derive(Debug, Clone)]
pub struct SplitPacket {
    /// The total number of split parts.
    pub split_count: u32,
    /// The unique ID of this split packet group.
    pub split_id: u16,
    /// The received parts, indexed by their split_index.
    pub parts: Vec<Option<Vec<u8>>>,
    /// The number of parts received so far.
    pub received_count: u32,
}

impl SplitPacket {
    /// Creates a new SplitPacket tracker.
    pub fn new(split_count: u32, split_id: u16) -> Self {
        let mut parts = Vec::with_capacity(split_count as usize);
        for _ in 0..split_count {
            parts.push(None);
        }
        SplitPacket {
            split_count,
            split_id,
            parts,
            received_count: 0,
        }
    }

    /// Adds a part to this split packet.
    ///
    /// Returns `true` if this was a new part (not a duplicate).
    pub fn add_part(&mut self, split_index: u32, data: Vec<u8>) -> bool {
        if split_index as usize >= self.parts.len() {
            return false;
        }
        if self.parts[split_index as usize].is_some() {
            return false; // duplicate
        }
        self.parts[split_index as usize] = Some(data);
        self.received_count += 1;
        true
    }

    /// Returns `true` if all parts have been received.
    pub fn is_complete(&self) -> bool {
        self.received_count == self.split_count
    }

    /// Reassembles the complete payload from all parts.
    ///
    /// Returns `None` if not all parts have been received.
    pub fn reassemble(&self) -> Option<Vec<u8>> {
        if !self.is_complete() {
            return None;
        }
        let mut result = Vec::new();
        for part in &self.parts {
            if let Some(data) = part {
                result.extend_from_slice(data);
            } else {
                return None;
            }
        }
        Some(result)
    }
}
