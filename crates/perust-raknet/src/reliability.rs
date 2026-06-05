//! Reliability types for RakNet encapsulated packets.
//!
//! RakNet defines 8 reliability types that control delivery guarantees:
//! - **Unreliable**: No guarantee of delivery or ordering.
//! - **Sequenced**: Guaranteed to be the most recent, older packets are dropped.
//! - **Reliable**: Guaranteed delivery, but no ordering.
//! - **Reliable Ordered**: Guaranteed delivery and ordering within a channel.
//! - **Reliable Sequenced**: Guaranteed delivery of the most recent, ordered.
//! - **With Ack Receipt**: Variants that provide acknowledgment receipts.

/// RakNet reliability types.
///
/// Each reliability type determines the delivery guarantees for an encapsulated
/// packet. Higher reliability means more overhead (ACKs, ordering indices, etc.).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reliability {
    /// No guarantee of delivery or ordering.
    Unreliable = 0,
    /// Unreliable but sequenced — only the most recent packet is kept.
    UnreliableSequenced = 1,
    /// Guaranteed delivery, but no ordering guarantee.
    Reliable = 2,
    /// Guaranteed delivery and ordering within a channel.
    ReliableOrdered = 3,
    /// Reliable and sequenced — guaranteed delivery of the most recent.
    ReliableSequenced = 4,
    /// Unreliable with acknowledgment receipt.
    UnreliableWithAckReceipt = 5,
    /// Reliable with acknowledgment receipt.
    ReliableWithAckReceipt = 6,
    /// Reliable ordered with acknowledgment receipt.
    ReliableOrderedWithAckReceipt = 7,
}

impl Reliability {
    /// Creates a `Reliability` from a u8 value.
    ///
    /// Returns `None` if the value does not correspond to a valid reliability type.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Reliability::Unreliable),
            1 => Some(Reliability::UnreliableSequenced),
            2 => Some(Reliability::Reliable),
            3 => Some(Reliability::ReliableOrdered),
            4 => Some(Reliability::ReliableSequenced),
            5 => Some(Reliability::UnreliableWithAckReceipt),
            6 => Some(Reliability::ReliableWithAckReceipt),
            7 => Some(Reliability::ReliableOrderedWithAckReceipt),
            _ => None,
        }
    }

    /// Converts the reliability to its u8 value.
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns `true` if this reliability type guarantees delivery.
    ///
    /// Reliable types will resend packets until an ACK is received.
    pub fn is_reliable(self) -> bool {
        matches!(
            self,
            Reliability::Reliable
                | Reliability::ReliableOrdered
                | Reliability::ReliableSequenced
                | Reliability::ReliableWithAckReceipt
                | Reliability::ReliableOrderedWithAckReceipt
        )
    }

    /// Returns `true` if this reliability type guarantees ordering.
    ///
    /// Ordered types ensure packets arrive in the correct order within a channel.
    pub fn is_ordered(self) -> bool {
        matches!(
            self,
            Reliability::ReliableOrdered | Reliability::ReliableOrderedWithAckReceipt
        )
    }

    /// Returns `true` if this reliability type uses sequencing.
    ///
    /// Sequenced types only keep the most recent packet, dropping older ones.
    pub fn is_sequenced(self) -> bool {
        matches!(
            self,
            Reliability::UnreliableSequenced | Reliability::ReliableSequenced
        )
    }

    /// Returns `true` if this reliability type provides acknowledgment receipts.
    pub fn has_ack_receipt(self) -> bool {
        matches!(
            self,
            Reliability::UnreliableWithAckReceipt
                | Reliability::ReliableWithAckReceipt
                | Reliability::ReliableOrderedWithAckReceipt
        )
    }

    /// Returns `true` if this reliability type requires a message index.
    ///
    /// All reliable types need a message index for ACK tracking.
    pub fn needs_message_index(self) -> bool {
        self.is_reliable()
    }

    /// Returns `true` if this reliability type requires a sequence index.
    ///
    /// Sequenced types need a sequence index.
    pub fn needs_sequence_index(self) -> bool {
        self.is_sequenced()
    }

    /// Returns `true` if this reliability type requires an order index and channel.
    ///
    /// Both sequenced and ordered types need ordering information.
    pub fn needs_ordering(self) -> bool {
        self.is_sequenced() || self.is_ordered()
    }
}

impl std::fmt::Display for Reliability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reliability::Unreliable => write!(f, "Unreliable"),
            Reliability::UnreliableSequenced => write!(f, "UnreliableSequenced"),
            Reliability::Reliable => write!(f, "Reliable"),
            Reliability::ReliableOrdered => write!(f, "ReliableOrdered"),
            Reliability::ReliableSequenced => write!(f, "ReliableSequenced"),
            Reliability::UnreliableWithAckReceipt => write!(f, "UnreliableWithAckReceipt"),
            Reliability::ReliableWithAckReceipt => write!(f, "ReliableWithAckReceipt"),
            Reliability::ReliableOrderedWithAckReceipt => write!(f, "ReliableOrderedWithAckReceipt"),
        }
    }
}
