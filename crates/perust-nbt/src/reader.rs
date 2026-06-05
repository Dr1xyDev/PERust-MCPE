use crate::error::NbtError;
use crate::tag::{NamedTag, Tag, TAG_COMPOUND, TAG_END};
use crate::Endianness;
use flate2::read::ZlibDecoder;
use perust_utils::varint::VarInt;
use std::io::Read;

/// NBT data reader supporting multiple byte order formats.
pub struct NbtReader<'a> {
    data: &'a [u8],
    pos: usize,
    endian: Endianness,
}

impl<'a> NbtReader<'a> {
    /// Creates a new NbtReader from a byte slice with the specified endianness.
    pub fn new(data: &'a [u8], endian: Endianness) -> Self {
        NbtReader {
            data,
            pos: 0,
            endian,
        }
    }

    /// Returns the current read position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the remaining bytes.
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    // === Internal read helpers ===

    fn read_byte(&mut self) -> Result<u8, NbtError> {
        if self.pos >= self.data.len() {
            return Err(NbtError::UnexpectedEof);
        }
        let byte = self.data[self.pos];
        self.pos += 1;
        Ok(byte)
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], NbtError> {
        if self.pos + len > self.data.len() {
            return Err(NbtError::UnexpectedEof);
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    fn read_i16(&mut self) -> Result<i16, NbtError> {
        let bytes = self.read_bytes(2)?;
        match self.endian {
            Endianness::BigEndian => Ok(i16::from_be_bytes([bytes[0], bytes[1]])),
            Endianness::LittleEndian | Endianness::Network => {
                Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
            }
        }
    }

    fn read_u16(&mut self) -> Result<u16, NbtError> {
        let bytes = self.read_bytes(2)?;
        match self.endian {
            Endianness::BigEndian => Ok(u16::from_be_bytes([bytes[0], bytes[1]])),
            Endianness::LittleEndian | Endianness::Network => {
                Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
            }
        }
    }

    fn read_i32(&mut self) -> Result<i32, NbtError> {
        let bytes = self.read_bytes(4)?;
        match self.endian {
            Endianness::BigEndian => Ok(i32::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ])),
            Endianness::LittleEndian | Endianness::Network => Ok(i32::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ])),
        }
    }

    fn read_i64(&mut self) -> Result<i64, NbtError> {
        let bytes = self.read_bytes(8)?;
        match self.endian {
            Endianness::BigEndian => Ok(i64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
            Endianness::LittleEndian | Endianness::Network => Ok(i64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
        }
    }

    fn read_f32(&mut self) -> Result<f32, NbtError> {
        let bytes = self.read_bytes(4)?;
        match self.endian {
            Endianness::BigEndian => Ok(f32::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ])),
            Endianness::LittleEndian | Endianness::Network => Ok(f32::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ])),
        }
    }

    fn read_f64(&mut self) -> Result<f64, NbtError> {
        let bytes = self.read_bytes(8)?;
        match self.endian {
            Endianness::BigEndian => Ok(f64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
            Endianness::LittleEndian | Endianness::Network => Ok(f64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
        }
    }

    /// Reads a length value. For Network format, reads a VarInt; for BigEndian/LittleEndian, reads an i32.
    fn read_length(&mut self) -> Result<i32, NbtError> {
        match self.endian {
            Endianness::Network => self.read_varint(),
            _ => self.read_i32(),
        }
    }

    /// Reads a string length. For Network format, uses VarInt; for LittleEndian, uses u16 (short);
    /// for BigEndian, uses u16 (modified UTF-8 length prefix).
    fn read_string_length(&mut self) -> Result<i32, NbtError> {
        match self.endian {
            Endianness::Network => self.read_varint(),
            Endianness::LittleEndian => Ok(self.read_u16()? as i32),
            Endianness::BigEndian => Ok(self.read_u16()? as i32),
        }
    }

    fn read_varint(&mut self) -> Result<i32, NbtError> {
        let remaining = &self.data[self.pos..];
        let (value, bytes_read) =
            VarInt::read_from_slice(remaining).map_err(NbtError::BinaryError)?;
        self.pos += bytes_read;
        Ok(value)
    }

    // === Public API ===

    /// Reads a complete named compound tag from the data.
    pub fn read_compound(&mut self) -> Result<NamedTag, NbtError> {
        let tag_type = self.read_byte()?;
        if tag_type != TAG_COMPOUND {
            return Err(NbtError::InvalidTagType(tag_type));
        }
        let name = self.read_string_raw()?;
        let tag = self.read_tag(TAG_COMPOUND)?;
        Ok(NamedTag::new(name, tag))
    }

    /// Reads a tag value given its type ID.
    pub fn read_tag(&mut self, tag_type: u8) -> Result<Tag, NbtError> {
        match tag_type {
            TAG_END => Ok(Tag::End),
            1 => {
                let byte = self.read_byte()?;
                Ok(Tag::Byte(byte as i8))
            }
            2 => Ok(Tag::Short(self.read_i16()?)),
            3 => Ok(Tag::Int(self.read_i32()?)),
            4 => Ok(Tag::Long(self.read_i64()?)),
            5 => Ok(Tag::Float(self.read_f32()?)),
            6 => Ok(Tag::Double(self.read_f64()?)),
            7 => {
                // ByteArray
                let len = self.read_length()?;
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(self.read_byte()? as i8);
                }
                Ok(Tag::ByteArray(arr))
            }
            8 => {
                // String
                let s = self.read_string_raw()?;
                Ok(Tag::String(s))
            }
            9 => {
                // List
                let list_type = self.read_byte()?;
                let len = self.read_length()?;
                let mut list = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    list.push(self.read_tag(list_type)?);
                }
                Ok(Tag::List(list))
            }
            10 => {
                // Compound
                let mut map = indexmap::IndexMap::new();
                loop {
                    let child_type = self.read_byte()?;
                    if child_type == TAG_END {
                        break;
                    }
                    let key = self.read_string_raw()?;
                    let value = self.read_tag(child_type)?;
                    map.insert(key, value);
                }
                Ok(Tag::Compound(map))
            }
            11 => {
                // IntArray
                let len = self.read_length()?;
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(self.read_i32()?);
                }
                Ok(Tag::IntArray(arr))
            }
            12 => {
                // LongArray
                let len = self.read_length()?;
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(self.read_i64()?);
                }
                Ok(Tag::LongArray(arr))
            }
            _ => Err(NbtError::InvalidTagType(tag_type)),
        }
    }

    /// Reads a raw string from the data, handling the length prefix based on endianness.
    pub fn read_string_raw(&mut self) -> Result<String, NbtError> {
        let len = self.read_string_length()?;
        if len < 0 {
            return Err(NbtError::Custom(format!(
                "Negative string length: {}",
                len
            )));
        }
        let bytes = self.read_bytes(len as usize)?;
        String::from_utf8(bytes.to_vec()).map_err(NbtError::Utf8Error)
    }

    /// Reads NBT data from zlib-compressed input (big-endian format).
    pub fn read_from_compressed(data: &[u8]) -> Result<NamedTag, NbtError> {
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| NbtError::Custom(format!("Decompression failed: {}", e)))?;
        let mut reader = NbtReader::new(&decompressed, Endianness::BigEndian);
        reader.read_compound()
    }

    /// Reads NBT data from zlib-compressed input with specified endianness.
    pub fn read_from_compressed_with_endian(
        data: &[u8],
        endian: Endianness,
    ) -> Result<NamedTag, NbtError> {
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| NbtError::Custom(format!("Decompression failed: {}", e)))?;
        let mut reader = NbtReader::new(&decompressed, endian);
        reader.read_compound()
    }
}
