use std::fs;
use std::io::{Read as IoRead, Write as IoWrite};
use std::path::Path;

use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use perust_nbt::{Endianness, NamedTag, NbtReader, NbtWriter};

use crate::error::StorageError;

/// NBT file storage utility.
///
/// Provides static methods for reading and writing NBT files,
/// both in plain and compressed (gzip/zlib) formats.
pub struct NbtStorage;

impl NbtStorage {
    /// Reads an uncompressed NBT file (big-endian format).
    pub fn read(path: &Path) -> Result<NamedTag, StorageError> {
        if !path.exists() {
            return Err(StorageError::FileNotFound(path.to_path_buf()));
        }
        let data = fs::read(path)?;
        let mut reader = NbtReader::new(&data, Endianness::BigEndian);
        let tag = reader.read_compound()?;
        Ok(tag)
    }

    /// Writes an uncompressed NBT file (big-endian format).
    pub fn write(path: &Path, tag: &NamedTag) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut writer = NbtWriter::new(Endianness::BigEndian);
        writer.write_named_tag(tag);
        let data = writer.into_bytes();
        fs::write(path, data)?;
        Ok(())
    }

    /// Reads a zlib-compressed NBT file (big-endian format).
    pub fn read_compressed(path: &Path) -> Result<NamedTag, StorageError> {
        if !path.exists() {
            return Err(StorageError::FileNotFound(path.to_path_buf()));
        }
        let data = fs::read(path)?;
        let tag = NbtReader::read_from_compressed(&data)?;
        Ok(tag)
    }

    /// Writes a zlib-compressed NBT file (big-endian format).
    pub fn write_compressed(path: &Path, tag: &NamedTag) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let compressed = NbtWriter::write_compressed(&tag.name, &tag.tag);
        fs::write(path, compressed)?;
        Ok(())
    }

    /// Reads an NBT file with gzip compression.
    pub fn read_gzip(path: &Path) -> Result<NamedTag, StorageError> {
        if !path.exists() {
            return Err(StorageError::FileNotFound(path.to_path_buf()));
        }
        let data = fs::read(path)?;
        let mut decoder = GzDecoder::new(&data[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| StorageError::InvalidFormat(format!("Gzip decompression failed: {}", e)))?;
        let mut reader = NbtReader::new(&decompressed, Endianness::BigEndian);
        let tag = reader.read_compound()?;
        Ok(tag)
    }

    /// Writes an NBT file with gzip compression.
    pub fn write_gzip(path: &Path, tag: &NamedTag) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut writer = NbtWriter::new(Endianness::BigEndian);
        writer.write_named_tag(tag);
        let data = writer.into_bytes();

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&data)
            .map_err(|e| StorageError::InvalidFormat(format!("Gzip compression failed: {}", e)))?;
        let compressed = encoder
            .finish()
            .map_err(|e| StorageError::InvalidFormat(format!("Gzip finish failed: {}", e)))?;
        fs::write(path, compressed)?;
        Ok(())
    }

    /// Reads chunk data from compressed bytes, detecting compression type.
    ///
    /// Compression type: 1 = gzip, 2 = zlib
    pub fn read_chunk_nbt(data: &[u8], compression_type: u8) -> Result<NamedTag, StorageError> {
        match compression_type {
            1 => {
                let mut decoder = GzDecoder::new(data);
                let mut decompressed = Vec::new();
                decoder
                    .read_to_end(&mut decompressed)
                    .map_err(|e| StorageError::InvalidFormat(format!("Gzip decompression failed: {}", e)))?;
                let mut reader = NbtReader::new(&decompressed, Endianness::BigEndian);
                Ok(reader.read_compound()?)
            }
            2 => {
                let mut decoder = ZlibDecoder::new(data);
                let mut decompressed = Vec::new();
                decoder
                    .read_to_end(&mut decompressed)
                    .map_err(|e| StorageError::InvalidFormat(format!("Zlib decompression failed: {}", e)))?;
                let mut reader = NbtReader::new(&decompressed, Endianness::BigEndian);
                Ok(reader.read_compound()?)
            }
            _ => Err(StorageError::InvalidFormat(format!(
                "Unknown compression type: {}",
                compression_type
            ))),
        }
    }

    /// Compresses NBT data using zlib (compression type 2), returning the
    /// compression type byte and compressed bytes.
    pub fn write_chunk_nbt(tag: &NamedTag) -> Result<(u8, Vec<u8>), StorageError> {
        let mut writer = NbtWriter::new(Endianness::BigEndian);
        writer.write_named_tag(tag);
        let data = writer.into_bytes();

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&data)
            .map_err(|e| StorageError::InvalidFormat(format!("Zlib compression failed: {}", e)))?;
        let compressed = encoder
            .finish()
            .map_err(|e| StorageError::InvalidFormat(format!("Zlib finish failed: {}", e)))?;
        Ok((2, compressed))
    }
}
