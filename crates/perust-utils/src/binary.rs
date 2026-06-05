//! Binary encoding/decoding utilities for the Minecraft Bedrock protocol.
//!
//! This module provides:
//! - **VarInt** / **VarLong**: Variable-length integer encoding compatible with the
//!   Minecraft Bedrock Edition protocol (little-endian format).
//! - **BinaryReader**: A cursor-based reader over `&[u8]` that supports reading all
//!   primitive types, VarInt/VarLong, strings, and raw byte slices.
//! - **BinaryWriter**: A writer backed by `Vec<u8>` that supports writing all
//!   primitive types, VarInt/VarLong, strings, and raw byte slices.
//!
//! # Minecraft Bedrock Protocol Compatibility
//!
//! The VarInt/VarLong encoding follows the standard Minecraft protocol format:
//! each byte stores 7 bits of payload and 1 continuation bit (MSB). Bytes are
//! written in **little-endian** order (least-significant group first).
//!
//! Signed integers use **ZigZag encoding** to map signed values to unsigned:
//! - Encode: `(n << 1) ^ (n >> 31)` for i32, `(n << 1) ^ (n >> 63)` for i64
//! - Decode: `(n >> 1) ^ -(n & 1)`

use bytes::{Bytes, BytesMut};
use std::fmt;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur during binary reading/writing operations.
#[derive(Error, Debug)]
pub enum BinaryError {
    /// Attempted to read past the end of the buffer.
    #[error("unexpected end of buffer: requested {requested} bytes but only {remaining} available")]
    UnexpectedEof {
        /// Number of bytes requested.
        requested: usize,
        /// Number of bytes remaining in the buffer.
        remaining: usize,
    },

    /// A VarInt/VarLong encoding exceeded the maximum number of bytes.
    #[error("{ty} encoding exceeded maximum byte count of {max_bytes}")]
    VarOverflow {
        /// The type that overflowed ("VarInt" or "VarLong").
        ty: &'static str,
        /// Maximum number of bytes allowed.
        max_bytes: usize,
    },

    /// A string was invalid UTF-8.
    #[error("invalid UTF-8 in string: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),

    /// A string length was negative.
    #[error("string length is negative: {0}")]
    NegativeStringLength(i32),
}

// ---------------------------------------------------------------------------
// VarInt encoding / decoding (i32, unsigned u32)
// ---------------------------------------------------------------------------

/// Maximum number of bytes a VarInt can occupy on the wire.
pub const VAR_INT_MAX_BYTES: usize = 5;

/// Maximum number of bytes a VarLong can occupy on the wire.
pub const VAR_LONG_MAX_BYTES: usize = 10;

/// Reads a VarInt from a byte slice, returning the decoded value and the number of bytes consumed.
///
/// This reads the unsigned (raw) VarInt. For signed i32 values with ZigZag encoding,
/// use [`read_zigzag_var_int`].
///
/// # Errors
///
/// Returns [`BinaryError::VarOverflow`] if the encoding exceeds 5 bytes, or
/// [`BinaryError::UnexpectedEof`] if the slice ends prematurely.
pub fn read_var_int(buf: &[u8]) -> Result<(i32, usize), BinaryError> {
    let mut result: i32 = 0;
    let mut shift: u32 = 0;

    for (i, &byte) in buf.iter().enumerate() {
        if i >= VAR_INT_MAX_BYTES {
            return Err(BinaryError::VarOverflow {
                ty: "VarInt",
                max_bytes: VAR_INT_MAX_BYTES,
            });
        }

        result |= ((byte & 0x7F) as i32) << shift;

        if byte & 0x80 == 0 {
            return Ok((result, i + 1));
        }

        shift += 7;
    }

    Err(BinaryError::UnexpectedEof {
        requested: 1,
        remaining: 0,
    })
}

/// Writes a VarInt into a byte vector, returning the number of bytes written.
///
/// This writes the unsigned (raw) VarInt. For signed i32 values with ZigZag encoding,
/// use [`write_zigzag_var_int`].
pub fn write_var_int(buf: &mut Vec<u8>, mut value: i32) -> usize {
    let start = buf.len();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value != 0 {
            byte |= 0x80;
        }

        buf.push(byte);

        if value == 0 {
            break;
        }
    }
    buf.len() - start
}

/// Reads a ZigZag-encoded VarInt (signed i32) from a byte slice.
///
/// ZigZag encoding maps signed integers to unsigned so that small-magnitude
/// negative values use fewer bytes.
///
/// # Errors
///
/// Same as [`read_var_int`].
pub fn read_zigzag_var_int(buf: &[u8]) -> Result<(i32, usize), BinaryError> {
    let (raw, len) = read_var_int(buf)?;
    Ok((zigzag_decode_i32(raw as u32), len))
}

/// Writes a ZigZag-encoded VarInt (signed i32) into a byte vector.
///
/// Returns the number of bytes written.
pub fn write_zigzag_var_int(buf: &mut Vec<u8>, value: i32) -> usize {
    write_var_int(buf, zigzag_encode_i32(value) as i32)
}

// ---------------------------------------------------------------------------
// VarLong encoding / decoding (i64, unsigned u64)
// ---------------------------------------------------------------------------

/// Reads a VarLong from a byte slice, returning the decoded value and the number of bytes consumed.
///
/// This reads the unsigned (raw) VarLong. For signed i64 values with ZigZag encoding,
/// use [`read_zigzag_var_long`].
///
/// # Errors
///
/// Returns [`BinaryError::VarOverflow`] if the encoding exceeds 10 bytes, or
/// [`BinaryError::UnexpectedEof`] if the slice ends prematurely.
pub fn read_var_long(buf: &[u8]) -> Result<(i64, usize), BinaryError> {
    let mut result: i64 = 0;
    let mut shift: u32 = 0;

    for (i, &byte) in buf.iter().enumerate() {
        if i >= VAR_LONG_MAX_BYTES {
            return Err(BinaryError::VarOverflow {
                ty: "VarLong",
                max_bytes: VAR_LONG_MAX_BYTES,
            });
        }

        result |= ((byte & 0x7F) as i64) << shift;

        if byte & 0x80 == 0 {
            return Ok((result, i + 1));
        }

        shift += 7;
    }

    Err(BinaryError::UnexpectedEof {
        requested: 1,
        remaining: 0,
    })
}

/// Writes a VarLong into a byte vector, returning the number of bytes written.
pub fn write_var_long(buf: &mut Vec<u8>, mut value: i64) -> usize {
    let start = buf.len();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value != 0 {
            byte |= 0x80;
        }

        buf.push(byte);

        if value == 0 {
            break;
        }
    }
    buf.len() - start
}

/// Reads a ZigZag-encoded VarLong (signed i64) from a byte slice.
///
/// # Errors
///
/// Same as [`read_var_long`].
pub fn read_zigzag_var_long(buf: &[u8]) -> Result<(i64, usize), BinaryError> {
    let (raw, len) = read_var_long(buf)?;
    Ok((zigzag_decode_i64(raw as u64), len))
}

/// Writes a ZigZag-encoded VarLong (signed i64) into a byte vector.
///
/// Returns the number of bytes written.
pub fn write_zigzag_var_long(buf: &mut Vec<u8>, value: i64) -> usize {
    write_var_long(buf, zigzag_encode_i64(value) as i64)
}

// ---------------------------------------------------------------------------
// ZigZag helpers
// ---------------------------------------------------------------------------

/// ZigZag-encodes a signed 32-bit integer into an unsigned 32-bit integer.
///
/// Maps `0 → 0, -1 → 1, 1 → 2, -2 → 3, ...`
#[inline]
pub fn zigzag_encode_i32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

/// ZigZag-decodes an unsigned 32-bit integer into a signed 32-bit integer.
#[inline]
pub fn zigzag_decode_i32(n: u32) -> i32 {
    ((n >> 1) as i32) ^ -((n & 1) as i32)
}

/// ZigZag-encodes a signed 64-bit integer into an unsigned 64-bit integer.
#[inline]
pub fn zigzag_encode_i64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

/// ZigZag-decodes an unsigned 64-bit integer into a signed 64-bit integer.
#[inline]
pub fn zigzag_decode_i64(n: u64) -> i64 {
    ((n >> 1) as i64) ^ -((n & 1) as i64)
}

// ---------------------------------------------------------------------------
// BinaryReader
// ---------------------------------------------------------------------------

/// A cursor-based binary reader that reads primitives, VarInt/VarLong, and strings
/// from a byte slice.
///
/// # Examples
///
/// ```
/// use perust_utils::binary::BinaryReader;
///
/// let data: &[u8] = &[0x05, 0x00, 0x00, 0x00, 0x00]; // varint 5
/// let mut reader = BinaryReader::new(data);
/// let value = reader.read_var_int().unwrap();
/// assert_eq!(value, 5);
/// ```
pub struct BinaryReader<'a> {
    /// The underlying byte slice.
    buf: &'a [u8],
    /// Current read position within the buffer.
    pos: usize,
}

impl<'a> BinaryReader<'a> {
    /// Creates a new `BinaryReader` over the given byte slice.
    #[inline]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Returns the current read position.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the total length of the underlying buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if the underlying buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Returns the number of bytes remaining to be read.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    /// Sets the read position.
    ///
    /// # Panics
    ///
    /// Panics if `pos` exceeds the buffer length.
    #[inline]
    pub fn set_pos(&mut self, pos: usize) {
        assert!(
            pos <= self.buf.len(),
            "position {} exceeds buffer length {}",
            pos,
            self.buf.len()
        );
        self.pos = pos;
    }

    /// Reads a single unsigned byte.
    ///
    /// # Errors
    ///
    /// Returns [`BinaryError::UnexpectedEof`] if no bytes remain.
    #[inline]
    pub fn read_u8(&mut self) -> Result<u8, BinaryError> {
        if self.remaining() < 1 {
            return Err(BinaryError::UnexpectedEof {
                requested: 1,
                remaining: 0,
            });
        }
        let val = self.buf[self.pos];
        self.pos += 1;
        Ok(val)
    }

    /// Reads a single signed byte.
    #[inline]
    pub fn read_i8(&mut self) -> Result<i8, BinaryError> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads a unsigned 16-bit integer in little-endian order.
    #[inline]
    pub fn read_u16_le(&mut self) -> Result<u16, BinaryError> {
        let needed = std::mem::size_of::<u16>();
        if self.remaining() < needed {
            return Err(BinaryError::UnexpectedEof {
                requested: needed,
                remaining: self.remaining(),
            });
        }
        let val = u16::from_le_bytes([self.buf[self.pos], self.buf[self.pos + 1]]);
        self.pos += needed;
        Ok(val)
    }

    /// Reads a signed 16-bit integer in little-endian order.
    #[inline]
    pub fn read_i16_le(&mut self) -> Result<i16, BinaryError> {
        Ok(self.read_u16_le()? as i16)
    }

    /// Reads an unsigned 32-bit integer in little-endian order.
    #[inline]
    pub fn read_u32_le(&mut self) -> Result<u32, BinaryError> {
        let needed = std::mem::size_of::<u32>();
        if self.remaining() < needed {
            return Err(BinaryError::UnexpectedEof {
                requested: needed,
                remaining: self.remaining(),
            });
        }
        let val = u32::from_le_bytes([
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
        ]);
        self.pos += needed;
        Ok(val)
    }

    /// Reads a signed 32-bit integer in little-endian order.
    #[inline]
    pub fn read_i32_le(&mut self) -> Result<i32, BinaryError> {
        Ok(self.read_u32_le()? as i32)
    }

    /// Reads an unsigned 64-bit integer in little-endian order.
    #[inline]
    pub fn read_u64_le(&mut self) -> Result<u64, BinaryError> {
        let needed = std::mem::size_of::<u64>();
        if self.remaining() < needed {
            return Err(BinaryError::UnexpectedEof {
                requested: needed,
                remaining: self.remaining(),
            });
        }
        let val = u64::from_le_bytes([
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
            self.buf[self.pos + 4],
            self.buf[self.pos + 5],
            self.buf[self.pos + 6],
            self.buf[self.pos + 7],
        ]);
        self.pos += needed;
        Ok(val)
    }

    /// Reads a signed 64-bit integer in little-endian order.
    #[inline]
    pub fn read_i64_le(&mut self) -> Result<i64, BinaryError> {
        Ok(self.read_u64_le()? as i64)
    }

    /// Reads a 32-bit floating-point number in little-endian order.
    #[inline]
    pub fn read_f32_le(&mut self) -> Result<f32, BinaryError> {
        let bits = self.read_u32_le()?;
        Ok(f32::from_bits(bits))
    }

    /// Reads a 64-bit floating-point number in little-endian order.
    #[inline]
    pub fn read_f64_le(&mut self) -> Result<f64, BinaryError> {
        let bits = self.read_u64_le()?;
        Ok(f64::from_bits(bits))
    }

    /// Reads a VarInt (variable-length 32-bit integer).
    ///
    /// This reads the raw unsigned VarInt. Use [`read_zigzag_var_int`](Self::read_zigzag_var_int)
    /// for signed ZigZag-encoded values.
    #[inline]
    pub fn read_var_int(&mut self) -> Result<i32, BinaryError> {
        let (val, consumed) = read_var_int(&self.buf[self.pos..])?;
        self.pos += consumed;
        Ok(val)
    }

    /// Reads a ZigZag-encoded VarInt (signed 32-bit integer).
    #[inline]
    pub fn read_zigzag_var_int(&mut self) -> Result<i32, BinaryError> {
        let (val, consumed) = read_zigzag_var_int(&self.buf[self.pos..])?;
        self.pos += consumed;
        Ok(val)
    }

    /// Reads a VarLong (variable-length 64-bit integer).
    #[inline]
    pub fn read_var_long(&mut self) -> Result<i64, BinaryError> {
        let (val, consumed) = read_var_long(&self.buf[self.pos..])?;
        self.pos += consumed;
        Ok(val)
    }

    /// Reads a ZigZag-encoded VarLong (signed 64-bit integer).
    #[inline]
    pub fn read_zigzag_var_long(&mut self) -> Result<i64, BinaryError> {
        let (val, consumed) = read_zigzag_var_long(&self.buf[self.pos..])?;
        self.pos += consumed;
        Ok(val)
    }

    /// Reads a Minecraft-protocol string (VarInt length prefix + UTF-8 bytes).
    ///
    /// # Errors
    ///
    /// Returns an error if the length prefix is negative, the buffer is too short,
    /// or the bytes are not valid UTF-8.
    pub fn read_string(&mut self) -> Result<&'a str, BinaryError> {
        let len = self.read_var_int()?;
        if len < 0 {
            return Err(BinaryError::NegativeStringLength(len));
        }
        let len = len as usize;
        if self.remaining() < len {
            return Err(BinaryError::UnexpectedEof {
                requested: len,
                remaining: self.remaining(),
            });
        }
        let s = std::str::from_utf8(&self.buf[self.pos..self.pos + len])?;
        self.pos += len;
        Ok(s)
    }

    /// Reads an owned `String` (VarInt length prefix + UTF-8 bytes).
    pub fn read_string_owned(&mut self) -> Result<String, BinaryError> {
        Ok(self.read_string()?.to_owned())
    }

    /// Reads a fixed-length byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer does not contain enough bytes.
    pub fn read_vec(&mut self, len: usize) -> Result<Vec<u8>, BinaryError> {
        if self.remaining() < len {
            return Err(BinaryError::UnexpectedEof {
                requested: len,
                remaining: self.remaining(),
            });
        }
        let data = self.buf[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Ok(data)
    }

    /// Reads a byte slice of the given length without copying.
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], BinaryError> {
        if self.remaining() < len {
            return Err(BinaryError::UnexpectedEof {
                requested: len,
                remaining: self.remaining(),
            });
        }
        let data = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        Ok(data)
    }

    /// Reads a boolean value (1 byte: 0 = false, non-zero = true).
    #[inline]
    pub fn read_bool(&mut self) -> Result<bool, BinaryError> {
        Ok(self.read_u8()? != 0)
    }

    /// Returns a reference to the remaining unread bytes.
    #[inline]
    pub fn remaining_bytes(&self) -> &'a [u8] {
        &self.buf[self.pos..]
    }

    /// Skips `n` bytes forward.
    ///
    /// # Errors
    ///
    /// Returns an error if there are fewer than `n` bytes remaining.
    pub fn skip(&mut self, n: usize) -> Result<(), BinaryError> {
        if self.remaining() < n {
            return Err(BinaryError::UnexpectedEof {
                requested: n,
                remaining: self.remaining(),
            });
        }
        self.pos += n;
        Ok(())
    }

    // === Big-endian read methods ===

    /// Reads an unsigned 16-bit integer in big-endian order.
    #[inline]
    pub fn read_u16(&mut self) -> Result<u16, BinaryError> {
        let needed = std::mem::size_of::<u16>();
        if self.remaining() < needed {
            return Err(BinaryError::UnexpectedEof {
                requested: needed,
                remaining: self.remaining(),
            });
        }
        let val = u16::from_be_bytes([self.buf[self.pos], self.buf[self.pos + 1]]);
        self.pos += needed;
        Ok(val)
    }

    /// Reads a signed 16-bit integer in big-endian order.
    #[inline]
    pub fn read_i16(&mut self) -> Result<i16, BinaryError> {
        Ok(self.read_u16()? as i16)
    }

    /// Reads an unsigned 32-bit integer in big-endian order.
    #[inline]
    pub fn read_u32(&mut self) -> Result<u32, BinaryError> {
        let needed = std::mem::size_of::<u32>();
        if self.remaining() < needed {
            return Err(BinaryError::UnexpectedEof {
                requested: needed,
                remaining: self.remaining(),
            });
        }
        let val = u32::from_be_bytes([
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
        ]);
        self.pos += needed;
        Ok(val)
    }

    /// Reads a signed 32-bit integer in big-endian order.
    #[inline]
    pub fn read_i32(&mut self) -> Result<i32, BinaryError> {
        Ok(self.read_u32()? as i32)
    }

    /// Reads an unsigned 64-bit integer in big-endian order.
    #[inline]
    pub fn read_u64(&mut self) -> Result<u64, BinaryError> {
        let needed = std::mem::size_of::<u64>();
        if self.remaining() < needed {
            return Err(BinaryError::UnexpectedEof {
                requested: needed,
                remaining: self.remaining(),
            });
        }
        let val = u64::from_be_bytes([
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
            self.buf[self.pos + 4],
            self.buf[self.pos + 5],
            self.buf[self.pos + 6],
            self.buf[self.pos + 7],
        ]);
        self.pos += needed;
        Ok(val)
    }

    /// Reads a signed 64-bit integer in big-endian order.
    #[inline]
    pub fn read_i64(&mut self) -> Result<i64, BinaryError> {
        Ok(self.read_u64()? as i64)
    }

    /// Reads a 32-bit floating-point number in big-endian order.
    #[inline]
    pub fn read_f32(&mut self) -> Result<f32, BinaryError> {
        let bits = self.read_u32()?;
        Ok(f32::from_bits(bits))
    }

    /// Reads a 64-bit floating-point number in big-endian order.
    #[inline]
    pub fn read_f64(&mut self) -> Result<f64, BinaryError> {
        let bits = self.read_u64()?;
        Ok(f64::from_bits(bits))
    }

    // === Additional convenience methods ===

    /// Reads an unsigned VarInt (convenience wrapper).
    #[inline]
    pub fn read_var_uint(&mut self) -> Result<u32, BinaryError> {
        let val = self.read_var_int()?;
        Ok(val as u32)
    }

    /// Reads a UUID (16 bytes big-endian).
    pub fn read_uuid(&mut self) -> Result<uuid::Uuid, BinaryError> {
        let bytes = self.read_bytes(16)?;
        Ok(uuid::Uuid::from_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15],
        ]))
    }

    /// Returns the remaining bytes as an owned vector.
    #[inline]
    pub fn read_remaining(&mut self) -> Vec<u8> {
        let data = self.buf[self.pos..].to_vec();
        self.pos = self.buf.len();
        data
    }
}

impl<'a> fmt::Debug for BinaryReader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BinaryReader")
            .field("len", &self.buf.len())
            .field("pos", &self.pos)
            .field("remaining", &self.remaining())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// BinaryWriter
// ---------------------------------------------------------------------------

/// A binary writer that appends primitives, VarInt/VarLong, and strings to a
/// `Vec<u8>` buffer.
///
/// # Examples
///
/// ```
/// use perust_utils::binary::BinaryWriter;
///
/// let mut writer = BinaryWriter::new();
/// writer.write_var_int(42);
/// writer.write_string("hello");
/// let bytes = writer.finish();
/// assert!(!bytes.is_empty());
/// ```
pub struct BinaryWriter {
    /// The underlying byte buffer.
    buf: Vec<u8>,
}

impl BinaryWriter {
    /// Creates a new `BinaryWriter` with an empty buffer.
    #[inline]
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Creates a new `BinaryWriter` with the given initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    /// Returns the current length of the written data.
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if no bytes have been written.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Returns a reference to the underlying byte buffer.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    /// Writes a single unsigned byte.
    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        self.buf.push(value);
    }

    /// Writes a single signed byte.
    #[inline]
    pub fn write_i8(&mut self, value: i8) {
        self.buf.push(value as u8);
    }

    /// Writes an unsigned 16-bit integer in little-endian order.
    #[inline]
    pub fn write_u16_le(&mut self, value: u16) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a signed 16-bit integer in little-endian order.
    #[inline]
    pub fn write_i16_le(&mut self, value: i16) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes an unsigned 32-bit integer in little-endian order.
    #[inline]
    pub fn write_u32_le(&mut self, value: u32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a signed 32-bit integer in little-endian order.
    #[inline]
    pub fn write_i32_le(&mut self, value: i32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes an unsigned 64-bit integer in little-endian order.
    #[inline]
    pub fn write_u64_le(&mut self, value: u64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a signed 64-bit integer in little-endian order.
    #[inline]
    pub fn write_i64_le(&mut self, value: i64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a 32-bit floating-point number in little-endian order.
    #[inline]
    pub fn write_f32_le(&mut self, value: f32) {
        self.buf.extend_from_slice(&value.to_bits().to_le_bytes());
    }

    /// Writes a 64-bit floating-point number in little-endian order.
    #[inline]
    pub fn write_f64_le(&mut self, value: f64) {
        self.buf.extend_from_slice(&value.to_bits().to_le_bytes());
    }

    /// Writes a VarInt (variable-length 32-bit integer).
    ///
    /// Returns the number of bytes written.
    #[inline]
    pub fn write_var_int(&mut self, value: i32) -> usize {
        write_var_int(&mut self.buf, value)
    }

    /// Writes a ZigZag-encoded VarInt (signed 32-bit integer).
    ///
    /// Returns the number of bytes written.
    #[inline]
    pub fn write_zigzag_var_int(&mut self, value: i32) -> usize {
        write_zigzag_var_int(&mut self.buf, value)
    }

    /// Writes a VarLong (variable-length 64-bit integer).
    ///
    /// Returns the number of bytes written.
    #[inline]
    pub fn write_var_long(&mut self, value: i64) -> usize {
        write_var_long(&mut self.buf, value)
    }

    /// Writes a ZigZag-encoded VarLong (signed 64-bit integer).
    ///
    /// Returns the number of bytes written.
    #[inline]
    pub fn write_zigzag_var_long(&mut self, value: i64) -> usize {
        write_zigzag_var_long(&mut self.buf, value)
    }

    /// Writes a Minecraft-protocol string (VarInt length prefix + UTF-8 bytes).
    ///
    /// # Panics
    ///
    /// Panics if the string length exceeds `i32::MAX` bytes.
    pub fn write_string(&mut self, value: &str) {
        let bytes = value.as_bytes();
        assert!(
            bytes.len() <= i32::MAX as usize,
            "string length exceeds i32::MAX"
        );
        self.write_var_int(bytes.len() as i32);
        self.buf.extend_from_slice(bytes);
    }

    /// Writes a raw byte slice.
    #[inline]
    pub fn write_bytes(&mut self, value: &[u8]) {
        self.buf.extend_from_slice(value);
    }

    /// Writes a boolean value (1 byte: 0 = false, 1 = true).
    #[inline]
    pub fn write_bool(&mut self, value: bool) {
        self.buf.push(if value { 1 } else { 0 });
    }

    /// Consumes the writer and returns the underlying byte buffer.
    #[inline]
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    /// Converts the written data into a `Bytes` object (zero-copy where possible).
    #[inline]
    pub fn to_bytes(self) -> Bytes {
        Bytes::from(self.buf)
    }

    /// Converts the written data into a `BytesMut` object.
    #[inline]
    pub fn to_bytes_mut(self) -> BytesMut {
        BytesMut::from(self.buf.as_slice())
    }

    /// Clears the buffer, keeping allocated memory for reuse.
    #[inline]
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    /// Reserves additional capacity for at least `additional` more bytes.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(additional);
    }

    // === Big-endian write methods ===

    /// Writes an unsigned 16-bit integer in big-endian order.
    #[inline]
    pub fn write_u16(&mut self, value: u16) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    /// Writes a signed 16-bit integer in big-endian order.
    #[inline]
    pub fn write_i16(&mut self, value: i16) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    /// Writes an unsigned 32-bit integer in big-endian order.
    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    /// Writes a signed 32-bit integer in big-endian order.
    #[inline]
    pub fn write_i32(&mut self, value: i32) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    /// Writes an unsigned 64-bit integer in big-endian order.
    #[inline]
    pub fn write_u64(&mut self, value: u64) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    /// Writes a signed 64-bit integer in big-endian order.
    #[inline]
    pub fn write_i64(&mut self, value: i64) {
        self.buf.extend_from_slice(&value.to_be_bytes());
    }

    /// Writes a 32-bit floating-point number in big-endian order.
    #[inline]
    pub fn write_f32(&mut self, value: f32) {
        self.buf.extend_from_slice(&value.to_bits().to_be_bytes());
    }

    /// Writes a 64-bit floating-point number in big-endian order.
    #[inline]
    pub fn write_f64(&mut self, value: f64) {
        self.buf.extend_from_slice(&value.to_bits().to_be_bytes());
    }

    // === Additional convenience methods ===

    /// Writes an unsigned VarInt (convenience wrapper).
    #[inline]
    pub fn write_var_uint(&mut self, value: u32) -> usize {
        self.write_var_int(value as i32)
    }

    /// Writes a UUID (16 bytes big-endian).
    #[inline]
    pub fn write_uuid(&mut self, value: &uuid::Uuid) {
        self.buf.extend_from_slice(value.as_bytes());
    }

    /// Consumes the writer and returns the underlying byte buffer.
    /// Alias for `finish()`.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }
}

impl Default for BinaryWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BinaryWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BinaryWriter")
            .field("len", &self.buf.len())
            .field("capacity", &self.buf.capacity())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_int_zero() {
        let mut buf = Vec::new();
        let written = write_var_int(&mut buf, 0);
        assert_eq!(written, 1);
        assert_eq!(buf, [0x00]);

        let (val, consumed) = read_var_int(&buf).unwrap();
        assert_eq!(val, 0);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_var_int_small() {
        let mut buf = Vec::new();
        let written = write_var_int(&mut buf, 1);
        assert_eq!(written, 1);
        assert_eq!(buf, [0x01]);

        let (val, consumed) = read_var_int(&buf).unwrap();
        assert_eq!(val, 1);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_var_int_127() {
        let mut buf = Vec::new();
        write_var_int(&mut buf, 127);
        assert_eq!(buf, [0x7F]);

        let (val, consumed) = read_var_int(&buf).unwrap();
        assert_eq!(val, 127);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_var_int_128() {
        let mut buf = Vec::new();
        write_var_int(&mut buf, 128);
        // 128 = 0x80 → [0x80, 0x01]
        assert_eq!(buf, [0x80, 0x01]);

        let (val, consumed) = read_var_int(&buf).unwrap();
        assert_eq!(val, 128);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn test_var_int_large() {
        let mut buf = Vec::new();
        write_var_int(&mut buf, 300);
        // 300 = 0x12C → [0xAC, 0x02]
        assert_eq!(buf, [0xAC, 0x02]);

        let (val, _) = read_var_int(&buf).unwrap();
        assert_eq!(val, 300);
    }

    #[test]
    fn test_var_int_negative() {
        // Negative values in raw VarInt are large unsigned values
        let mut buf = Vec::new();
        write_var_int(&mut buf, -1);
        // -1 in i32 = 0xFFFFFFFF, which takes 5 bytes in VarInt
        assert_eq!(buf.len(), 5);

        let (val, _) = read_var_int(&buf).unwrap();
        assert_eq!(val, -1);
    }

    #[test]
    fn test_zigzag_encode_decode_i32() {
        assert_eq!(zigzag_encode_i32(0), 0);
        assert_eq!(zigzag_encode_i32(-1), 1);
        assert_eq!(zigzag_encode_i32(1), 2);
        assert_eq!(zigzag_encode_i32(-2), 3);
        assert_eq!(zigzag_encode_i32(2), 4);

        assert_eq!(zigzag_decode_i32(0), 0);
        assert_eq!(zigzag_decode_i32(1), -1);
        assert_eq!(zigzag_decode_i32(2), 1);
        assert_eq!(zigzag_decode_i32(3), -2);
        assert_eq!(zigzag_decode_i32(4), 2);
    }

    #[test]
    fn test_zigzag_var_int_roundtrip() {
        for val in [0i32, 1, -1, 127, -128, 32767, -32768, i32::MIN / 2, i32::MAX / 2] {
            let mut buf = Vec::new();
            write_zigzag_var_int(&mut buf, val);
            let (decoded, _) = read_zigzag_var_int(&buf).unwrap();
            assert_eq!(decoded, val, "ZigZag VarInt roundtrip failed for {}", val);
        }
    }

    #[test]
    fn test_zigzag_var_long_roundtrip() {
        for val in [0i64, 1, -1, 127, -128, i64::MIN / 2, i64::MAX / 2] {
            let mut buf = Vec::new();
            write_zigzag_var_long(&mut buf, val);
            let (decoded, _) = read_zigzag_var_long(&buf).unwrap();
            assert_eq!(decoded, val, "ZigZag VarLong roundtrip failed for {}", val);
        }
    }

    #[test]
    fn test_binary_reader_writer_roundtrip() {
        let mut writer = BinaryWriter::new();
        writer.write_u8(0xFF);
        writer.write_i8(-1);
        writer.write_u16_le(0x1234);
        writer.write_i16_le(-1000);
        writer.write_u32_le(0xDEADBEEF);
        writer.write_i32_le(-42);
        writer.write_u64_le(0xCAFEBABEDEADBEEF);
        writer.write_i64_le(-123456789);
        writer.write_f32_le(3.14);
        writer.write_f64_le(2.718281828);
        writer.write_bool(true);
        writer.write_bool(false);
        writer.write_var_int(300);
        writer.write_string("hello world");

        let data = writer.finish();
        let mut reader = BinaryReader::new(&data);

        assert_eq!(reader.read_u8().unwrap(), 0xFF);
        assert_eq!(reader.read_i8().unwrap(), -1);
        assert_eq!(reader.read_u16_le().unwrap(), 0x1234);
        assert_eq!(reader.read_i16_le().unwrap(), -1000);
        assert_eq!(reader.read_u32_le().unwrap(), 0xDEADBEEF);
        assert_eq!(reader.read_i32_le().unwrap(), -42);
        assert_eq!(reader.read_u64_le().unwrap(), 0xCAFEBABEDEADBEEF);
        assert_eq!(reader.read_i64_le().unwrap(), -123456789);
        assert!((reader.read_f32_le().unwrap() - 3.14).abs() < 0.001);
        assert!((reader.read_f64_le().unwrap() - 2.718281828).abs() < 0.0001);
        assert_eq!(reader.read_bool().unwrap(), true);
        assert_eq!(reader.read_bool().unwrap(), false);
        assert_eq!(reader.read_var_int().unwrap(), 300);
        assert_eq!(reader.read_string().unwrap(), "hello world");
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_binary_reader_unexpected_eof() {
        let data = [0x01];
        let mut reader = BinaryReader::new(&data);
        assert!(reader.read_u8().is_ok());
        assert!(reader.read_u8().is_err());
    }

    #[test]
    fn test_binary_reader_skip() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut reader = BinaryReader::new(&data);
        reader.skip(2).unwrap();
        assert_eq!(reader.read_u8().unwrap(), 0x03);
    }

    #[test]
    fn test_reader_writer_vec() {
        let mut writer = BinaryWriter::new();
        writer.write_var_int(5); // length prefix
        writer.write_bytes(&[10, 20, 30, 40, 50]);

        let data = writer.finish();
        let mut reader = BinaryReader::new(&data);
        let len = reader.read_var_int().unwrap() as usize;
        let vec = reader.read_vec(len).unwrap();
        assert_eq!(vec, [10, 20, 30, 40, 50]);
    }
}
