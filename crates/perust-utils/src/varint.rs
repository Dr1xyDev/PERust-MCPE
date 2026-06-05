//! VarInt/VarLong type wrappers for NBT network format compatibility.
//!
//! This module provides [`VarInt`] and [`VarLong`] types that wrap the
//! variable-length integer encoding/decoding functions from the [`binary`]
//! module, providing a convenient interface for the NBT crate.

use crate::binary::{
    read_var_int, read_var_long, write_var_int, write_var_long, BinaryError,
};

/// A wrapper for VarInt (variable-length i32) encoding.
///
/// This type provides convenience methods for reading and writing VarInt
/// values in the format used by the Minecraft Bedrock protocol and NBT
/// network format.
pub struct VarInt;

impl VarInt {
    /// Reads a VarInt from a byte slice, returning the decoded value and
    /// the number of bytes consumed.
    ///
    /// # Errors
    ///
    /// Returns [`BinaryError`] if the encoding is invalid or the slice is
    /// too short.
    pub fn read_from_slice(buf: &[u8]) -> Result<(i32, usize), BinaryError> {
        read_var_int(buf)
    }

    /// Writes a VarInt into a byte vector.
    ///
    /// Returns the number of bytes written.
    pub fn write_to_vec(value: i32, buf: &mut Vec<u8>) -> usize {
        write_var_int(buf, value)
    }
}

/// A wrapper for VarLong (variable-length i64) encoding.
pub struct VarLong;

impl VarLong {
    /// Reads a VarLong from a byte slice, returning the decoded value and
    /// the number of bytes consumed.
    pub fn read_from_slice(buf: &[u8]) -> Result<(i64, usize), BinaryError> {
        read_var_long(buf)
    }

    /// Writes a VarLong into a byte vector.
    ///
    /// Returns the number of bytes written.
    pub fn write_to_vec(value: i64, buf: &mut Vec<u8>) -> usize {
        write_var_long(buf, value)
    }
}
