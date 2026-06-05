use crate::tag::{NamedTag, Tag, TAG_COMPOUND, TAG_END};
use crate::Endianness;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use perust_utils::varint::VarInt;
use std::io::Write;

/// NBT data writer supporting multiple byte order formats.
pub struct NbtWriter {
    buffer: Vec<u8>,
    endian: Endianness,
}

impl NbtWriter {
    /// Creates a new NbtWriter with the specified endianness.
    pub fn new(endian: Endianness) -> Self {
        NbtWriter {
            buffer: Vec::new(),
            endian,
        }
    }

    // === Internal write helpers ===

    fn write_byte(&mut self, v: u8) {
        self.buffer.push(v);
    }

    fn write_i8(&mut self, v: i8) {
        self.buffer.push(v as u8);
    }

    fn write_i16(&mut self, v: i16) {
        match self.endian {
            Endianness::BigEndian => self.buffer.extend_from_slice(&v.to_be_bytes()),
            Endianness::LittleEndian | Endianness::Network => {
                self.buffer.extend_from_slice(&v.to_le_bytes())
            }
        }
    }

    fn write_u16(&mut self, v: u16) {
        match self.endian {
            Endianness::BigEndian => self.buffer.extend_from_slice(&v.to_be_bytes()),
            Endianness::LittleEndian | Endianness::Network => {
                self.buffer.extend_from_slice(&v.to_le_bytes())
            }
        }
    }

    fn write_i32(&mut self, v: i32) {
        match self.endian {
            Endianness::BigEndian => self.buffer.extend_from_slice(&v.to_be_bytes()),
            Endianness::LittleEndian | Endianness::Network => {
                self.buffer.extend_from_slice(&v.to_le_bytes())
            }
        }
    }

    fn write_i64(&mut self, v: i64) {
        match self.endian {
            Endianness::BigEndian => self.buffer.extend_from_slice(&v.to_be_bytes()),
            Endianness::LittleEndian | Endianness::Network => {
                self.buffer.extend_from_slice(&v.to_le_bytes())
            }
        }
    }

    fn write_f32(&mut self, v: f32) {
        match self.endian {
            Endianness::BigEndian => self.buffer.extend_from_slice(&v.to_be_bytes()),
            Endianness::LittleEndian | Endianness::Network => {
                self.buffer.extend_from_slice(&v.to_le_bytes())
            }
        }
    }

    fn write_f64(&mut self, v: f64) {
        match self.endian {
            Endianness::BigEndian => self.buffer.extend_from_slice(&v.to_be_bytes()),
            Endianness::LittleEndian | Endianness::Network => {
                self.buffer.extend_from_slice(&v.to_le_bytes())
            }
        }
    }

    /// Writes a length value. For Network format, writes a VarInt; otherwise writes an i32.
    fn write_length(&mut self, len: i32) {
        match self.endian {
            Endianness::Network => {
                VarInt::write_to_vec(len, &mut self.buffer);
            }
            _ => {
                self.write_i32(len);
            }
        }
    }

    /// Writes a string length prefix.
    /// For Network format, uses VarInt; for LittleEndian, uses u16;
    /// for BigEndian, uses u16 (modified UTF-8 length prefix).
    fn write_string_length(&mut self, len: i32) {
        match self.endian {
            Endianness::Network => {
                VarInt::write_to_vec(len, &mut self.buffer);
            }
            Endianness::LittleEndian => {
                self.write_u16(len as u16);
            }
            Endianness::BigEndian => {
                self.write_u16(len as u16);
            }
        }
    }

    // === Public API ===

    /// Writes a complete named compound tag.
    pub fn write_compound(&mut self, name: &str, tag: &Tag) {
        self.write_byte(TAG_COMPOUND);
        self.write_string_raw(name);
        self.write_tag(tag);
    }

    /// Writes a tag value.
    pub fn write_tag(&mut self, tag: &Tag) {
        match tag {
            Tag::End => {
                self.write_byte(TAG_END);
            }
            Tag::Byte(v) => {
                self.write_i8(*v);
            }
            Tag::Short(v) => {
                self.write_i16(*v);
            }
            Tag::Int(v) => {
                self.write_i32(*v);
            }
            Tag::Long(v) => {
                self.write_i64(*v);
            }
            Tag::Float(v) => {
                self.write_f32(*v);
            }
            Tag::Double(v) => {
                self.write_f64(*v);
            }
            Tag::ByteArray(arr) => {
                self.write_length(arr.len() as i32);
                for &v in arr {
                    self.write_i8(v);
                }
            }
            Tag::String(s) => {
                self.write_string_raw(s);
            }
            Tag::List(list) => {
                if list.is_empty() {
                    self.write_byte(TAG_END); // list element type for empty list
                    self.write_length(0);
                } else {
                    let list_type = list[0].tag_type();
                    self.write_byte(list_type);
                    self.write_length(list.len() as i32);
                    for item in list {
                        self.write_tag(item);
                    }
                }
            }
            Tag::Compound(map) => {
                for (key, value) in map {
                    self.write_byte(value.tag_type());
                    self.write_string_raw(key);
                    self.write_tag(value);
                }
                self.write_byte(TAG_END);
            }
            Tag::IntArray(arr) => {
                self.write_length(arr.len() as i32);
                for &v in arr {
                    self.write_i32(v);
                }
            }
            Tag::LongArray(arr) => {
                self.write_length(arr.len() as i32);
                for &v in arr {
                    self.write_i64(v);
                }
            }
        }
    }

    /// Writes a raw string with its length prefix.
    pub fn write_string_raw(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_string_length(bytes.len() as i32);
        self.buffer.extend_from_slice(bytes);
    }

    /// Consumes the writer and returns the written bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    /// Returns a reference to the written bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// Clears the buffer, allowing the writer to be reused.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Writes a complete named compound tag and returns the zlib-compressed result (big-endian format).
    pub fn write_compressed(name: &str, tag: &Tag) -> Vec<u8> {
        Self::write_compressed_with_endian(name, tag, Endianness::BigEndian)
    }

    /// Writes a complete named compound tag and returns the zlib-compressed result with specified endianness.
    pub fn write_compressed_with_endian(name: &str, tag: &Tag, endian: Endianness) -> Vec<u8> {
        let mut writer = NbtWriter::new(endian);
        writer.write_compound(name, tag);
        let data = writer.into_bytes();

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&data)
            .expect("Failed to compress NBT data");
        encoder
            .finish()
            .expect("Failed to finish compression")
    }

    /// Convenience method to write a NamedTag.
    pub fn write_named_tag(&mut self, named: &NamedTag) {
        self.write_compound(&named.name, &named.tag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tag::Tag;

    #[test]
    fn test_write_read_roundtrip_big_endian() {
        let mut compound = indexmap::IndexMap::new();
        compound.insert("hello".to_string(), Tag::String("world".to_string()));
        compound.insert("number".to_string(), Tag::Int(42));
        compound.insert("float_val".to_string(), Tag::Float(3.14));
        let tag = Tag::Compound(compound);

        let mut writer = NbtWriter::new(Endianness::BigEndian);
        writer.write_compound("test", &tag);
        let bytes = writer.into_bytes();

        let mut reader = crate::NbtReader::new(&bytes, Endianness::BigEndian);
        let result = reader.read_compound().unwrap();

        assert_eq!(result.name, "test");
        assert_eq!(result.tag, tag);
    }

    #[test]
    fn test_write_read_roundtrip_little_endian() {
        let mut compound = indexmap::IndexMap::new();
        compound.insert("name".to_string(), Tag::String("bedrock".to_string()));
        compound.insert("x".to_string(), Tag::Int(100));
        let tag = Tag::Compound(compound);

        let mut writer = NbtWriter::new(Endianness::LittleEndian);
        writer.write_compound("root", &tag);
        let bytes = writer.into_bytes();

        let mut reader = crate::NbtReader::new(&bytes, Endianness::LittleEndian);
        let result = reader.read_compound().unwrap();

        assert_eq!(result.name, "root");
        assert_eq!(result.tag, tag);
    }

    #[test]
    fn test_write_read_roundtrip_network() {
        let mut compound = indexmap::IndexMap::new();
        compound.insert("key".to_string(), Tag::String("value".to_string()));
        compound.insert("count".to_string(), Tag::Long(123456789));
        let tag = Tag::Compound(compound);

        let mut writer = NbtWriter::new(Endianness::Network);
        writer.write_compound("net", &tag);
        let bytes = writer.into_bytes();

        let mut reader = crate::NbtReader::new(&bytes, Endianness::Network);
        let result = reader.read_compound().unwrap();

        assert_eq!(result.name, "net");
        assert_eq!(result.tag, tag);
    }

    #[test]
    fn test_compressed_roundtrip() {
        let mut compound = indexmap::IndexMap::new();
        compound.insert("data".to_string(), Tag::String("compressed".to_string()));
        let tag = Tag::Compound(compound);

        let compressed = NbtWriter::write_compressed("compressed_test", &tag);
        let result = crate::NbtReader::read_from_compressed(&compressed).unwrap();

        assert_eq!(result.name, "compressed_test");
        assert_eq!(result.tag, tag);
    }
}
