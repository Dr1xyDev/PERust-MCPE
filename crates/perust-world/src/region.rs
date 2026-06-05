//! Region file storage for the Anvil-like format.
//!
//! This module provides [`RegionFile`] for reading and writing chunks
//! stored in region files. Each region file covers a 32×32 chunk area
//! and uses a simplified Anvil-like format with a header table.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use crate::error::WorldError;

/// Size of a region in chunks (32×32).
pub const REGION_SIZE: usize = 32;
/// Size of a sector in bytes (4 KiB).
pub const SECTOR_BYTES: usize = 4096;

// ---------------------------------------------------------------------------
// RegionFile
// ---------------------------------------------------------------------------

/// A region file containing up to 32×32 chunks in Anvil-like format.
///
/// The file layout is:
/// - 4 KiB: offset table (1024 × 3-byte offsets + padding)
/// - 4 KiB: timestamp table (1024 × 4-byte timestamps + padding)
/// - Chunk data sectors
pub struct RegionFile {
    /// The underlying file handle.
    file: File,
    /// Offset table: 32×32 entries. Each offset is in sectors from the
    /// start of the file. 0 means the chunk is not present.
    offsets: [u32; 1024],
    /// Timestamp table: last modification time for each chunk.
    timestamps: [u32; 1024],
}

impl RegionFile {
    /// Opens an existing region file or creates a new one.
    ///
    /// The path should point to a file named like `r.{rx}.{rz}.mca`.
    pub fn new(path: &PathBuf) -> Result<Self, WorldError> {
        let exists = path.exists();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let mut offsets = [0u32; 1024];
        let mut timestamps = [0u32; 1024];

        if exists && file.metadata()?.len() >= SECTOR_BYTES as u64 * 2 {
            // Read existing header
            file.seek(SeekFrom::Start(0))?;

            // Read offset table
            let mut header = [0u8; SECTOR_BYTES];
            file.read_exact(&mut header)?;
            for i in 0..1024 {
                offsets[i] = ((header[i * 4] as u32) << 16)
                    | ((header[i * 4 + 1] as u32) << 8)
                    | (header[i * 4 + 2] as u32);
            }

            // Read timestamp table
            let mut ts_header = [0u8; SECTOR_BYTES];
            file.read_exact(&mut ts_header)?;
            for i in 0..1024 {
                timestamps[i] = ((ts_header[i * 4] as u32) << 24)
                    | ((ts_header[i * 4 + 1] as u32) << 16)
                    | ((ts_header[i * 4 + 2] as u32) << 8)
                    | (ts_header[i * 4 + 3] as u32);
            }
        } else {
            // Write empty header
            let zero_sector = [0u8; SECTOR_BYTES];
            file.write_all(&zero_sector)?;
            file.write_all(&zero_sector)?;
        }

        Ok(Self {
            file,
            offsets,
            timestamps,
        })
    }

    /// Returns the index into the offset/timestamp tables for chunk (x, z).
    ///
    /// x and z are local to the region (0..32).
    #[inline]
    fn chunk_index(x: usize, z: usize) -> usize {
        (z % REGION_SIZE) * REGION_SIZE + (x % REGION_SIZE)
    }

    /// Returns `true` if this region file contains a chunk at the given
    /// local coordinates.
    pub fn has_chunk(&self, x: usize, z: usize) -> bool {
        let idx = Self::chunk_index(x, z);
        self.offsets[idx] != 0
    }

    /// Reads the raw chunk data for chunk (x, z) within this region.
    ///
    /// Returns `None` if the chunk is not present.
    pub fn read_chunk_data(&mut self, x: usize, z: usize) -> Result<Option<Vec<u8>>, WorldError> {
        let idx = Self::chunk_index(x, z);
        let offset = self.offsets[idx];
        if offset == 0 {
            return Ok(None);
        }

        let sector_offset = (offset >> 8) as u64 * SECTOR_BYTES as u64;
        let _sector_count = (offset & 0xFF) as usize;

        self.file.seek(SeekFrom::Start(sector_offset))?;

        // First 4 bytes: chunk data length (big-endian)
        let mut len_buf = [0u8; 4];
        self.file.read_exact(&mut len_buf)?;
        let chunk_len =
            ((len_buf[0] as u32) << 24) | ((len_buf[1] as u32) << 16) | ((len_buf[2] as u32) << 8) | (len_buf[3] as u32);

        // Next byte: compression type (1 = gzip, 2 = zlib)
        let mut comp_buf = [0u8; 1];
        self.file.read_exact(&mut comp_buf)?;
        let _compression_type = comp_buf[0];

        // Read the remaining chunk data
        let data_len = chunk_len as usize - 1; // subtract compression type byte
        let mut data = vec![0u8; data_len];
        self.file.read_exact(&mut data)?;

        Ok(Some(data))
    }

    /// Writes chunk data for chunk (x, z) within this region.
    ///
    /// The data should be the uncompressed chunk NBT data; this method
    /// handles compression and sector allocation.
    pub fn write_chunk_data(&mut self, x: usize, z: usize, data: &[u8]) -> Result<(), WorldError> {
        let idx = Self::chunk_index(x, z);

        // Calculate needed sectors
        // 4 bytes length + 1 byte compression + data
        let total_len = 4 + 1 + data.len();
        let sectors_needed = (total_len + SECTOR_BYTES - 1) / SECTOR_BYTES;

        // Find the end of the file for new allocation
        let mut max_sector = 2u32; // header takes 2 sectors
        for &offset in &self.offsets {
            if offset != 0 {
                let end = (offset >> 8) + (offset & 0xFF);
                if end > max_sector {
                    max_sector = end;
                }
            }
        }

        let start_sector = max_sector;
        self.file.seek(SeekFrom::Start(start_sector as u64 * SECTOR_BYTES as u64))?;

        // Write length (4 bytes, big-endian): data.len() + 1 (for compression byte)
        let chunk_len = (data.len() + 1) as u32;
        self.file.write_all(&chunk_len.to_be_bytes())?;

        // Write compression type: 2 = zlib
        self.file.write_all(&[2])?;

        // Write chunk data
        self.file.write_all(data)?;

        // Pad to sector boundary
        let written = 4 + 1 + data.len();
        let padding = (sectors_needed * SECTOR_BYTES) - written;
        if padding > 0 {
            let zeros = vec![0u8; padding];
            self.file.write_all(&zeros)?;
        }

        // Update offset table
        self.offsets[idx] = (start_sector << 8) | (sectors_needed as u32);

        // Update timestamp
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as u32;
        self.timestamps[idx] = now;

        // Write header
        self.write_header()?;

        Ok(())
    }

    /// Writes the offset and timestamp tables to the file header.
    fn write_header(&mut self) -> Result<(), WorldError> {
        self.file.seek(SeekFrom::Start(0))?;

        // Write offset table (1024 × 3 bytes + padding)
        for i in 0..1024 {
            let offset = self.offsets[i];
            self.file.write_all(&[
                (offset >> 16) as u8,
                (offset >> 8) as u8,
                offset as u8,
            ])?;
        }
        // Pad to SECTOR_BYTES
        let written = 1024 * 3;
        let padding = SECTOR_BYTES - written;
        if padding > 0 {
            let zeros = vec![0u8; padding];
            self.file.write_all(&zeros)?;
        }

        // Write timestamp table (1024 × 4 bytes + padding)
        for i in 0..1024 {
            self.file.write_all(&self.timestamps[i].to_be_bytes())?;
        }
        let ts_written = 1024 * 4;
        let ts_padding = SECTOR_BYTES - ts_written;
        if ts_padding > 0 {
            let zeros = vec![0u8; ts_padding];
            self.file.write_all(&zeros)?;
        }

        self.file.flush()?;
        Ok(())
    }
}
