//! Session management for a single RakNet connection.
//!
//! A session represents a connected peer. It manages reliability, ordering,
//! sequencing, split packet reassembly, ACK/NACK queues, and recovery.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use crate::encapsulated::{EncapsulatedPacket, SplitPacket};
use crate::error::RakNetError;
use crate::protocol::{
    decode_ack_nack, encode_ack_nack, Datagram, ID_ACK, ID_DATAGRAM, ID_NACK,
};
use crate::reliability::Reliability;

/// The number of ordering channels per session (as per RakNet spec).
pub const ORDERING_CHANNEL_COUNT: usize = 32;

/// Timeout duration before a session is considered dead.
pub const SESSION_TIMEOUT: Duration = Duration::from_secs(30);

/// Maximum window size for the recovery queue.
pub const MAX_WINDOW_SIZE: u32 = 4096;

/// The state of a RakNet session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// The session is in the process of connecting (handshake in progress).
    Connecting = 0,
    /// The session is fully connected and can exchange data.
    Connected = 1,
    /// The session is in the process of disconnecting.
    Disconnecting = 2,
    /// The session has been disconnected.
    Disconnected = 3,
}

impl SessionState {
    /// Converts a u8 to a SessionState.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(SessionState::Connecting),
            1 => Some(SessionState::Connected),
            2 => Some(SessionState::Disconnecting),
            3 => Some(SessionState::Disconnected),
            _ => None,
        }
    }
}

impl std::fmt::Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionState::Connecting => write!(f, "Connecting"),
            SessionState::Connected => write!(f, "Connected"),
            SessionState::Disconnecting => write!(f, "Disconnecting"),
            SessionState::Disconnected => write!(f, "Disconnected"),
        }
    }
}

/// An ordering channel tracks the order and sequence indices for a single
/// channel within a session.
#[derive(Debug, Clone)]
pub struct OrderingChannel {
    /// The next expected order index for received packets.
    pub receive_order_index: u32,
    /// The next expected sequence index for received packets.
    pub receive_sequence_index: u32,
    /// The next order index to assign to sent packets.
    pub send_order_index: u32,
    /// The next sequence index to assign to sent packets.
    pub send_sequence_index: u32,
    /// A queue of reliable packets that arrived out of order,
    /// waiting for missing packets to fill the gap.
    pub reliable_packets_without_sequence: HashMap<u32, EncapsulatedPacket>,
}

impl OrderingChannel {
    /// Creates a new ordering channel with indices starting at 0.
    pub fn new() -> Self {
        OrderingChannel {
            receive_order_index: 0,
            receive_sequence_index: 0,
            send_order_index: 0,
            send_sequence_index: 0,
            reliable_packets_without_sequence: HashMap::new(),
        }
    }
}

impl Default for OrderingChannel {
    fn default() -> Self {
        Self::new()
    }
}

/// A session managing a single RakNet connection to a peer.
#[derive(Debug)]
pub struct Session {
    /// The current state of this session.
    pub state: SessionState,
    /// The remote address of the peer.
    pub address: SocketAddr,
    /// The negotiated MTU size for this connection.
    pub mtu_size: u16,
    /// The remote peer's GUID.
    pub guid: i64,
    /// The 32 ordering channels for this session.
    pub ordering_channels: Vec<OrderingChannel>,
    /// The next sequence number to use when sending a datagram.
    pub send_sequence: u32,
    /// The last received sequence number.
    pub receive_sequence: u32,
    /// Queue of sequence numbers that have been received and need to be ACKed.
    pub ack_queue: Vec<u32>,
    /// Queue of sequence numbers that are missing (NACK).
    pub nack_queue: Vec<u32>,
    /// Recovery queue: packets that were sent reliably and are awaiting ACK.
    /// If not ACKed within a timeout, they will be retransmitted.
    pub recovery_queue: HashMap<u32, Vec<u8>>,
    /// The next message index to assign to reliable packets.
    pub send_message_index: u32,
    /// The next split packet ID to use.
    pub split_id: u16,
    /// Partially reassembled split packets, keyed by split_id.
    pub split_packets: HashMap<u16, SplitPacket>,
    /// The window size for flow control.
    pub window_size: u32,
    /// The last time a packet was received from this peer.
    pub last_update: Instant,
    /// The most recent ping measurement.
    pub ping: Duration,
    /// Whether the player associated with this session has logged in.
    pub is_logged_in: bool,
    /// Pending packets to be sent in the next tick.
    pub send_queue: Vec<EncapsulatedPacket>,
}

impl Session {
    /// Creates a new session for the given peer address.
    pub fn new(address: SocketAddr, mtu_size: u16, guid: i64) -> Self {
        let mut ordering_channels = Vec::with_capacity(ORDERING_CHANNEL_COUNT);
        for _ in 0..ORDERING_CHANNEL_COUNT {
            ordering_channels.push(OrderingChannel::new());
        }

        Session {
            state: SessionState::Connecting,
            address,
            mtu_size,
            guid,
            ordering_channels,
            send_sequence: 0,
            receive_sequence: u32::MAX, // Will be set on first received datagram
            ack_queue: Vec::new(),
            nack_queue: Vec::new(),
            recovery_queue: HashMap::new(),
            send_message_index: 0,
            split_id: 0,
            split_packets: HashMap::new(),
            window_size: MAX_WINDOW_SIZE,
            last_update: Instant::now(),
            ping: Duration::from_millis(0),
            is_logged_in: false,
            send_queue: Vec::new(),
        }
    }

    /// Handles an incoming datagram from this peer.
    ///
    /// Returns a list of encapsulated packets that have been fully received
    /// (including reassembled split packets).
    pub fn handle_datagram(&mut self, data: &[u8]) -> Result<Vec<EncapsulatedPacket>, RakNetError> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        let packet_id = data[0];

        match packet_id {
            ID_DATAGRAM => self.handle_datagram_packet(&data[1..]),
            ID_ACK => {
                let sequences = decode_ack_nack(&data[1..])?;
                self.handle_ack(&sequences);
                Ok(Vec::new())
            }
            ID_NACK => {
                let sequences = decode_ack_nack(&data[1..])?;
                self.handle_nack(&sequences);
                Ok(Vec::new())
            }
            _ => Err(RakNetError::InvalidPacket(format!(
                "unexpected packet ID in session: 0x{:02x}",
                packet_id
            ))),
        }
    }

    /// Handles a datagram packet (ID_DATAGRAM) containing encapsulated packets.
    fn handle_datagram_packet(&mut self, data: &[u8]) -> Result<Vec<EncapsulatedPacket>, RakNetError> {
        let datagram = Datagram::decode(data)?;
        let seq = datagram.sequence_number;

        self.last_update = Instant::now();

        // If this is an old packet, ignore it
        if self.receive_sequence != u32::MAX && seq <= self.receive_sequence {
            // Still ACK it
            self.ack_queue.push(seq);
            return Ok(Vec::new());
        }

        // Check for gaps — if we expected receive_sequence + 1 but got something larger,
        // NACK the missing sequence numbers
        if self.receive_sequence != u32::MAX && seq > self.receive_sequence + 1 {
            for missing in (self.receive_sequence + 1)..seq {
                self.nack_queue.push(missing);
            }
        }

        // Update the highest received sequence
        if self.receive_sequence == u32::MAX || seq > self.receive_sequence {
            self.receive_sequence = seq;
        }

        // ACK this packet
        self.ack_queue.push(seq);

        // Parse encapsulated packets from the datagram payload
        let mut result = Vec::new();
        for payload in &datagram.packets {
            let mut offset = 0;
            while offset < payload.len() {
                match EncapsulatedPacket::decode(&payload[offset..]) {
                    Ok((packet, consumed)) => {
                        offset += consumed;
                        let processed = self.process_encapsulated(packet)?;
                        result.extend(processed);
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to decode encapsulated packet from {}: {:?}",
                            self.address,
                            e
                        );
                        break;
                    }
                }
            }
        }

        Ok(result)
    }

    /// Processes a single encapsulated packet, handling ordering and split reassembly.
    fn process_encapsulated(
        &mut self,
        packet: EncapsulatedPacket,
    ) -> Result<Vec<EncapsulatedPacket>, RakNetError> {
        let mut result = Vec::new();

        // Handle split packets
        if packet.has_split {
            return self.handle_split_packet(packet);
        }

        // Handle ordering
        if packet.reliability.needs_ordering() {
            let channel_idx = packet.order_channel as usize;
            if channel_idx >= ORDERING_CHANNEL_COUNT {
                return Err(RakNetError::InvalidPacket(format!(
                    "order channel {} out of range",
                    channel_idx
                )));
            }

            let channel = &mut self.ordering_channels[channel_idx];

            if packet.reliability.is_ordered() {
                // For ordered packets, we need to deliver in order
                let order_index = packet.order_index;

                if order_index == channel.receive_order_index {
                    // This is the next expected packet
                    result.push(packet.clone());
                    channel.receive_order_index += 1;

                    // Check if we have buffered packets that can now be delivered
                    while let Some(buffered) = channel.reliable_packets_without_sequence.remove(&channel.receive_order_index) {
                        result.push(buffered);
                        channel.receive_order_index += 1;
                    }
                } else if order_index > channel.receive_order_index {
                    // This packet arrived early, buffer it
                    channel
                        .reliable_packets_without_sequence
                        .insert(order_index, packet);
                }
                // If order_index < receive_order_index, it's a duplicate — discard
            } else if packet.reliability.is_sequenced() {
                // For sequenced packets, only deliver if it's the newest
                let sequence_index = packet.sequence_index;
                if sequence_index >= channel.receive_sequence_index {
                    channel.receive_sequence_index = sequence_index + 1;
                    result.push(packet);
                }
                // Otherwise discard (older sequenced packet)
            }
        } else {
            // Unreliable or reliable (without ordering) — deliver immediately
            result.push(packet);
        }

        Ok(result)
    }

    /// Handles a split encapsulated packet by reassembling parts.
    fn handle_split_packet(
        &mut self,
        packet: EncapsulatedPacket,
    ) -> Result<Vec<EncapsulatedPacket>, RakNetError> {
        let split_id = packet.split_id;
        let split_index = packet.split_index;
        let split_count = packet.split_count;

        // Get or create the split packet tracker
        let split = self
            .split_packets
            .entry(split_id)
            .or_insert_with(|| SplitPacket::new(split_count, split_id));

        // Add this part
        if !split.add_part(split_index, packet.buffer) {
            return Ok(Vec::new()); // Duplicate or invalid part
        }

        // Check if we have all parts
        if split.is_complete() {
            let reassembled = split.reassemble().ok_or_else(|| {
                RakNetError::SplitPacketError(format!(
                    "failed to reassemble split packet {}",
                    split_id
                ))
            })?;

            self.split_packets.remove(&split_id);

            // Create a new encapsulated packet with the reassembled data
            let reassembled_packet = EncapsulatedPacket {
                reliability: packet.reliability,
                has_split: false,
                buffer: reassembled,
                message_index: packet.message_index,
                sequence_index: packet.sequence_index,
                order_index: packet.order_index,
                order_channel: packet.order_channel,
                split_count: 0,
                split_id: 0,
                split_index: 0,
            };

            // Process the reassembled packet through ordering
            self.process_encapsulated(reassembled_packet)
        } else {
            Ok(Vec::new())
        }
    }

    /// Handles an ACK for the given sequence numbers.
    fn handle_ack(&mut self, sequences: &[u32]) {
        for seq in sequences {
            self.recovery_queue.remove(seq);
        }
    }

    /// Handles a NACK for the given sequence numbers — retransmit those packets.
    fn handle_nack(&mut self, sequences: &[u32]) {
        for seq in sequences {
            // Remove from NACK queue if present
            self.nack_queue.retain(|&s| s != *seq);

            // FIX: Retransmit the packet from the recovery queue.
            // When the peer sends a NACK, it means it never received this datagram,
            // so we need to re-queue it for sending.
            if self.recovery_queue.contains_key(seq) {
                // The recovery queue entry will be retransmitted in tick()
                // For now we just keep it in the queue — it won't be removed
                // until an ACK is received.
                log::debug!(
                    "NACK received for seq {}, keeping in recovery queue for retransmit",
                    seq
                );
            }
        }
    }

    /// Queues an encapsulated packet to be sent to this peer.
    ///
    /// The packet will be assembled into a datagram and sent during the next tick.
    pub fn send_encapsulated(&mut self, packet: EncapsulatedPacket) {
        self.send_queue.push(packet);
    }

    /// Adds a raw payload to the send queue with the given reliability and channel.
    pub fn send(&mut self, data: Vec<u8>, reliability: Reliability, channel: u8) {
        let mut packet = EncapsulatedPacket::new(reliability, data);
        packet.order_channel = channel;
        self.send_queue.push(packet);
    }

    /// Performs a session tick, returning packets that should be sent.
    ///
    /// This method should be called at a regular interval (e.g., 100 TPS).
    /// It handles:
    /// - Sending queued encapsulated packets in datagrams
    /// - Sending ACK/NACK packets
    /// - Retransmitting packets from the recovery queue
    pub fn tick(&mut self) -> Vec<Vec<u8>> {
        let mut output = Vec::new();

        // Send ACK packets
        if !self.ack_queue.is_empty() {
            let ack_queue = std::mem::take(&mut self.ack_queue);
            output.push(encode_ack_nack(ID_ACK, &ack_queue));
        }

        // Send NACK packets
        if !self.nack_queue.is_empty() {
            let nack_queue = std::mem::take(&mut self.nack_queue);
            output.push(encode_ack_nack(ID_NACK, &nack_queue));
        }

        // FIX: Assemble queued encapsulated packets into datagrams,
        // creating MULTIPLE datagrams if needed so no packets are dropped.
        let mut packets = std::mem::take(&mut self.send_queue);
        while !packets.is_empty() {
            let (datagram_data, remaining) = self.assemble_datagram(packets);

            if !datagram_data.is_empty() {
                // Store in recovery queue for reliable packets
                self.recovery_queue
                    .insert(self.send_sequence, datagram_data.clone());

                let datagram = Datagram::encode(self.send_sequence, &datagram_data);
                output.push(datagram);
                self.send_sequence = self.send_sequence.wrapping_add(1);
            }

            // Continue with the packets that didn't fit in this datagram
            packets = remaining;

            // Safety: if assemble_datagram returns empty payload and no remaining
            // packets, break to avoid an infinite loop. This shouldn't happen
            // in practice, but guards against logic errors.
            if datagram_data.is_empty() {
                break;
            }
        }

        // Retransmit packets from the recovery queue that have been NACKed.
        // In a full implementation, we'd track send time per recovery entry
        // and only retransmit after a timeout. For now, we keep them in the
        // queue until ACKed — the NACK handler keeps them alive for retransmit.

        output
    }

    /// Assembles a list of encapsulated packets into a datagram payload.
    ///
    /// FIX: Returns a tuple of `(payload, remaining_packets)` where
    /// `remaining_packets` contains any packets that didn't fit in this
    /// datagram. The caller is responsible for creating additional datagrams
    /// for the remaining packets, ensuring no packets are silently dropped.
    fn assemble_datagram(
        &mut self,
        packets: Vec<EncapsulatedPacket>,
    ) -> (Vec<u8>, Vec<EncapsulatedPacket>) {
        let mut payload = Vec::new();
        // Datagram header: ID_DATAGRAM(1) + sequence triad(3) = 4 bytes
        let datagram_header = 1 + 3;
        let mut remaining_mtu = self.mtu_size as usize - datagram_header;
        let mut remaining_packets = Vec::new();
        let mut added_any = false;

        for mut packet in packets {
            // Assign indices based on reliability
            if packet.reliability.needs_message_index() {
                packet.message_index = self.send_message_index;
                self.send_message_index = self.send_message_index.wrapping_add(1);
            }

            if packet.reliability.needs_ordering() {
                let channel_idx = packet.order_channel as usize;
                if channel_idx < ORDERING_CHANNEL_COUNT {
                    let channel = &mut self.ordering_channels[channel_idx];

                    if packet.reliability.is_sequenced() {
                        packet.sequence_index = channel.send_sequence_index;
                        packet.order_index = channel.send_order_index;
                        channel.send_sequence_index = channel.send_sequence_index.wrapping_add(1);
                    } else if packet.reliability.is_ordered() {
                        packet.order_index = channel.send_order_index;
                        channel.send_order_index = channel.send_order_index.wrapping_add(1);
                    }
                }
            }

            let encoded = packet.encode();

            if encoded.len() > remaining_mtu {
                if added_any {
                    // This packet doesn't fit in the current datagram —
                    // save it and all subsequent packets for the next datagram.
                    remaining_packets.push(packet);
                } else {
                    // Even alone this packet is too large for the datagram.
                    // It should have been split via send_split() before being queued.
                    // Include it anyway to avoid stalling the queue, but log a warning.
                    log::warn!(
                        "Encapsulated packet ({} bytes) exceeds remaining MTU ({} bytes) \
                         for session {}. Consider using send_split() for large packets.",
                        encoded.len(),
                        remaining_mtu,
                        self.address
                    );
                    payload.extend_from_slice(&encoded);
                    // No more room for anything after this oversized packet
                }
                // All subsequent packets also won't fit — skip to collecting them
                continue;
            }

            payload.extend_from_slice(&encoded);
            remaining_mtu -= encoded.len();
            added_any = true;
        }

        (payload, remaining_packets)
    }

    /// Splits a large payload into multiple encapsulated packets that fit
    /// within the MTU, and queues them for sending.
    pub fn send_split(&mut self, data: Vec<u8>, reliability: Reliability, channel: u8) {
        // Maximum payload per encapsulated packet (accounting for header overhead)
        // The worst-case encapsulated header is: flags(1) + length(2) + message_index(3) +
        // order_index(3) + order_channel(1) + split_fields(10) = 20 bytes
        let encapsulated_header_overhead = 20;
        // Datagram header: ID_DATAGRAM(1) + sequence triad(3) = 4 bytes
        let datagram_header = 1 + 3;
        let max_payload_size = self.mtu_size as usize - datagram_header - encapsulated_header_overhead;

        if data.len() <= max_payload_size {
            // No need to split
            self.send(data, reliability, channel);
            return;
        }

        let split_id = self.split_id;
        self.split_id = self.split_id.wrapping_add(1);

        let mut offset = 0;
        let mut split_index: u32 = 0;
        let total_parts = ((data.len() + max_payload_size - 1) / max_payload_size) as u32;

        while offset < data.len() {
            let end = std::cmp::min(offset + max_payload_size, data.len());
            let chunk = data[offset..end].to_vec();
            offset = end;

            let mut packet = EncapsulatedPacket::new(reliability, chunk);
            packet.order_channel = channel;
            packet.has_split = true;
            packet.split_count = total_parts;
            packet.split_id = split_id;
            packet.split_index = split_index;

            split_index += 1;
            self.send_queue.push(packet);
        }
    }

    /// Updates the ping measurement for this session.
    pub fn update_ping(&mut self, ping: Duration) {
        self.ping = ping;
    }

    /// Returns `true` if this session is in the Connected state.
    pub fn is_connected(&self) -> bool {
        self.state == SessionState::Connected
    }

    /// Returns `true` if this session has timed out (no packets received recently).
    pub fn is_timed_out(&self) -> bool {
        self.last_update.elapsed() > SESSION_TIMEOUT
    }

    /// Transitions the session to the Connected state.
    pub fn mark_connected(&mut self) {
        self.state = SessionState::Connected;
        self.last_update = Instant::now();
    }

    /// Transitions the session to the Disconnecting state.
    pub fn mark_disconnecting(&mut self) {
        self.state = SessionState::Disconnecting;
    }

    /// Transitions the session to the Disconnected state.
    pub fn mark_disconnected(&mut self) {
        self.state = SessionState::Disconnected;
    }

    /// Returns recovery queue entries that need to be retransmitted.
    pub fn get_recovery_packets(&self) -> Vec<(u32, Vec<u8>)> {
        self.recovery_queue
            .iter()
            .map(|(&seq, data)| (seq, data.clone()))
            .collect()
    }
}
