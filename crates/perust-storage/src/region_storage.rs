use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read as IoRead, Seek, SeekFrom, Write as IoWrite};
use std::path::{Path, PathBuf};

use log::debug;

use crate::error::StorageError;
use crate::nbt_storage::NbtStorage;

/// Size of a sector in bytes (4 KiB).
pub const SECTOR_SIZE: usize = 4096;

/// Number of chunks per region file (32 × 32).
pub const REGION_CHUNKS: usize = 1024;

/// Manages region files for a world directory.
///
/// Region files follow the Anvil format, storing up to 32×32 chunks each.
/// Chunks are stored in sector-aligned offsets within the file.
pub struct RegionStorage {
    /// Cache of opened region files, keyed by (region_x, region_z).
    cache: HashMap<(i32, i32), RegionFile>,
    /// Path to the world directory containing region files.
    world_dir: PathBuf,
}

impl RegionStorage {
    /// Creates a new `RegionStorage` for the given world directory.
    pub fn new(world_dir: &Path) -> Self {
        RegionStorage {
            cache: HashMap::new(),
            world_dir: world_dir.to_path_buf(),
        }
    }

    /// Reads chunk data at the given chunk coordinates.
    ///
    /// Returns `Ok(None)` if the chunk does not exist in the region file.
    pub fn read_chunk(&mut self, x: i32, z: i32) -> Result<Option<perust_nbt::NamedTag>, StorageError> {
        let (rx, rz, lx, lz) = Self::chunk_to_region(x, z);
        let region_file = self.get_or_open_region(rx, rz)?;
        let data = region_file.read_chunk_data(lx, lz)?;
        match data {
            Some(raw) => {
                if raw.is_empty() {
                    return Ok(None);
                }
                let compression_type = raw[0];
                let nbt_data = &raw[1..];
                if nbt_data.is_empty() {
                    return Ok(None);
                }
                let tag = NbtStorage::read_chunk_nbt(nbt_data, compression_type)?;
                Ok(Some(tag))
            }
            None => Ok(None),
        }
    }

    /// Writes chunk data at the given chunk coordinates.
    pub fn write_chunk(&mut self, x: i32, z: i32, tag: &perust_nbt::NamedTag) -> Result<(), StorageError> {
        let (rx, rz, lx, lz) = Self::chunk_to_region(x, z);
        let (compression_type, compressed) = NbtStorage::write_chunk_nbt(tag)?;

        // Build chunk data: compression_type byte + compressed NBT
        let mut chunk_data = Vec::with_capacity(1 + compressed.len());
        chunk_data.push(compression_type);
        chunk_data.extend_from_slice(&compressed);

        let region_file = self.get_or_open_region(rx, rz)?;
        region_file.write_chunk_data(lx, lz, &chunk_data)
    }

    /// Checks whether a chunk exists at the given coordinates.
    pub fn has_chunk(&mut self, x: i32, z: i32) -> Result<bool, StorageError> {
        let (rx, rz, lx, lz) = Self::chunk_to_region(x, z);
        let region_file = self.get_or_open_region(rx, rz)?;
        Ok(region_file.has_chunk(lx, lz))
    }

    /// Converts global chunk coordinates to region coordinates and local offsets.
    fn chunk_to_region(x: i32, z: i32) -> (i32, i32, i32, i32) {
        let rx = x >> 5; // floor division by 32
        let rz = z >> 5;
        let lx = x & 31; // modulo 32
        let lz = z & 31;
        (rx, rz, lx, lz)
    }

    /// Gets or opens the region file for the given region coordinates.
    fn get_or_open_region(&mut self, rx: i32, rz: i32) -> Result<&mut RegionFile, StorageError> {
        if !self.cache.contains_key(&(rx, rz)) {
            let region_dir = self.world_dir.join("region");
            let path = region_dir.join(format!("r.{}.{}.mca", rx, rz));
            let region = RegionFile::open(&path)?;
            self.cache.insert((rx, rz), region);
        }
        Ok(self.cache.get_mut(&(rx, rz)).unwrap())
    }
}

/// Represents an open Anvil-format region file.
///
/// The file layout is:
/// - Bytes 0..4096: 1024 u32 offsets (each encodes sector_start << 8 | sector_count)
/// - Bytes 4096..8192: 1024 u32 timestamps
/// - Bytes 8192..: sector-aligned chunk data
///
/// All integers are big-endian.
pub struct RegionFile {
    /// The underlying file handle.
    file: File,
    /// Offset and size table for each of the 1024 chunks.
    /// Each entry encodes `(sector_start << 8) | sector_count`.
    offsets: [u32; REGION_CHUNKS],
    /// Modification timestamps for each chunk.
    timestamps: [u32; REGION_CHUNKS],
    /// Tracks which sectors are free (true = free, false = used).
    free_sectors: Vec<bool>,
}

impl RegionFile {
    /// Opens an existing region file, or creates a new one if it doesn't exist.
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        if path.exists() {
            Self::open_existing(path)
        } else {
            Self::create_new(path)
        }
    }

    /// Opens an existing region file and reads its header.
    fn open_existing(path: &Path) -> Result<Self, StorageError> {
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;

        let file_size = file.metadata()?.len() as usize;
        let sectors = (file_size + SECTOR_SIZE - 1) / SECTOR_SIZE;

        let mut free_sectors = vec![true; sectors.max(2)]; // At least 2 sectors for header
        free_sectors[0] = false; // Offset table
        free_sectors[1] = false; // Timestamp table

        // Read offset table
        let mut offsets = [0u32; REGION_CHUNKS];
        let mut buf = [0u8; 4];
        for offset in &mut offsets {
            file.read_exact(&mut buf)
                .map_err(|e| StorageError::RegionError(format!("Failed to read offset table: {}", e)))?;
            *offset = u32::from_be_bytes(buf);
            if *offset != 0 {
                let sector_start = (*offset >> 8) as usize;
                let sector_count = (*offset & 0xFF) as usize;
                if sector_start < sectors {
                    for s in sector_start..sector_start + sector_count.min(sectors - sector_start) {
                        free_sectors[s] = false;
                    }
                }
            }
        }

        // Read timestamp table
        let mut timestamps = [0u32; REGION_CHUNKS];
        for ts in &mut timestamps {
            file.read_exact(&mut buf)
                .map_err(|e| StorageError::RegionError(format!("Failed to read timestamp table: {}", e)))?;
            *ts = u32::from_be_bytes(buf);
        }

        debug!("Opened region file: {:?}", path);

        Ok(RegionFile {
            file,
            offsets,
            timestamps,
            free_sectors,
        })
    }

    /// Creates a new empty region file with a blank header.
    fn create_new(path: &Path) -> Result<Self, StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut rf = RegionFile {
            file,
            offsets: [0u32; REGION_CHUNKS],
            timestamps: [0u32; REGION_CHUNKS],
            free_sectors: vec![false, false], // header uses 2 sectors
        };

        rf.write_header()?;
        debug!("Created new region file: {:?}", path);
        Ok(rf)
    }

    /// Writes the offset and timestamp tables to the file header.
    fn write_header(&mut self) -> Result<(), StorageError> {
        self.file.seek(SeekFrom::Start(0))?;

        // Write offset table (1024 × u32 big-endian)
        for &offset in &self.offsets {
            self.file
                .write_all(&offset.to_be_bytes())
                .map_err(|e| StorageError::RegionError(format!("Failed to write offset table: {}", e)))?;
        }

        // Write timestamp table (1024 × u32 big-endian)
        for &ts in &self.timestamps {
            self.file
                .write_all(&ts.to_be_bytes())
                .map_err(|e| StorageError::RegionError(format!("Failed to write timestamp table: {}", e)))?;
        }

        self.file.flush()?;
        Ok(())
    }

    /// Reads the raw chunk data for a chunk at local coordinates (x, z).
    ///
    /// Returns `Ok(None)` if the chunk is not present.
    /// The returned data includes the compression type byte and compressed NBT data.
    pub fn read_chunk_data(&mut self, x: i32, z: i32) -> Result<Option<Vec<u8>>, StorageError> {
        let index = Self::chunk_index(x, z)?;
        let offset = self.offsets[index];

        if offset == 0 {
            return Ok(None);
        }

        let sector_start = (offset >> 8) as u64 * SECTOR_SIZE as u64;
        let sector_count = (offset & 0xFF) as usize;

        self.file.seek(SeekFrom::Start(sector_start))?;

        // Read chunk length (4 bytes, big-endian)
        let mut len_buf = [0u8; 4];
        self.file
            .read_exact(&mut len_buf)
            .map_err(|e| StorageError::RegionError(format!("Failed to read chunk length: {}", e)))?;
        let chunk_length = u32::from_be_bytes(len_buf) as usize;

        if chunk_length == 0 || chunk_length > sector_count * SECTOR_SIZE - 4 {
            return Err(StorageError::RegionError(format!(
                "Invalid chunk length {} at offset {}",
                chunk_length, sector_start
            )));
        }

        // Read chunk data (compression_type + compressed NBT)
        let mut data = vec![0u8; chunk_length];
        self.file
            .read_exact(&mut data)
            .map_err(|e| StorageError::RegionError(format!("Failed to read chunk data: {}", e)))?;

        Ok(Some(data))
    }

    /// Writes raw chunk data for a chunk at local coordinates (x, z).
    ///
    /// The `data` slice should contain the compression type byte followed by
    /// the compressed NBT data. The method will allocate sectors and update
    /// the header.
    pub fn write_chunk_data(&mut self, x: i32, z: i32, data: &[u8]) -> Result<(), StorageError> {
        let index = Self::chunk_index(x, z)?;

        // Compute total data size: 4 bytes length + data
        let total_size = 4 + data.len();
        let sectors_needed = (total_size + SECTOR_SIZE - 1) / SECTOR_SIZE;

        // Free old sectors if this chunk was already written
        let old_offset = self.offsets[index];
        if old_offset != 0 {
            let old_start = (old_offset >> 8) as usize;
            let old_count = (old_offset & 0xFF) as usize;
            for s in old_start..old_start + old_count {
                if s < self.free_sectors.len() {
                    self.free_sectors[s] = true;
                }
            }
        }

        // Find contiguous free sectors
        let sector_start = self.find_free_sectors(sectors_needed)?;

        // Mark sectors as used
        for s in sector_start..sector_start + sectors_needed {
            self.free_sectors[s] = false;
        }

        // Write chunk data
        let file_offset = sector_start as u64 * SECTOR_SIZE as u64;
        self.file.seek(SeekFrom::Start(file_offset))?;

        // Write length (big-endian u32)
        let length = data.len() as u32;
        self.file
            .write_all(&length.to_be_bytes())
            .map_err(|e| StorageError::RegionError(format!("Failed to write chunk length: {}", e)))?;

        // Write chunk data
        self.file
            .write_all(data)
            .map_err(|e| StorageError::RegionError(format!("Failed to write chunk data: {}", e)))?;

        // Pad to sector boundary
        let written = 4 + data.len();
        let padding = (sector_start + sectors_needed) * SECTOR_SIZE - (sector_start * SECTOR_SIZE + written);
        if padding > 0 {
            let zeros = vec![0u8; padding];
            self.file
                .write_all(&zeros)
                .map_err(|e| StorageError::RegionError(format!("Failed to write padding: {}", e)))?;
        }

        self.file.flush()?;

        // Update offset table
        let new_offset = ((sector_start as u32) << 8) | (sectors_needed as u32);
        self.offsets[index] = new_offset;

        // Update timestamp
        self.timestamps[index] = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as u32;

        // Rewrite header
        self.write_header()?;

        Ok(())
    }

    /// Returns whether a chunk exists at the given local coordinates.
    pub fn has_chunk(&self, x: i32, z: i32) -> bool {
        if let Ok(index) = Self::chunk_index(x, z) {
            self.offsets[index] != 0
        } else {
            false
        }
    }

    /// Computes the index into the offset/timestamp arrays for local chunk coords.
    fn chunk_index(x: i32, z: i32) -> Result<usize, StorageError> {
        if x < 0 || x >= 32 || z < 0 || z >= 32 {
            return Err(StorageError::RegionError(format!(
                "Chunk local coordinates out of range: ({}, {})",
                x, z
            )));
        }
        Ok((x + z * 32) as usize)
    }

    /// Finds a contiguous run of `count` free sectors.
    ///
    /// If not enough free sectors exist, the file is extended.
    fn find_free_sectors(&mut self, count: usize) -> Result<usize, StorageError> {
        let mut run_start = None;
        let mut run_length = 0;

        for (i, &free) in self.free_sectors.iter().enumerate() {
            if free {
                if run_start.is_none() {
                    run_start = Some(i);
                }
                run_length += 1;
                if run_length >= count {
                    return Ok(run_start.unwrap());
                }
            } else {
                run_start = None;
                run_length = 0;
            }
        }

        // Not enough contiguous free sectors; extend the file
        let current_sectors = self.free_sectors.len();
        let new_start = current_sectors;
        let needed = count;
        let new_total = current_sectors + needed;

        // Extend the file
        let new_size = new_total * SECTOR_SIZE;
        self.file
            .set_len(new_size as u64)
            .map_err(|e| StorageError::RegionError(format!("Failed to extend region file: {}", e)))?;

        // Mark new sectors: first `needed` are used, rest are free
        self.free_sectors.resize(new_total, true);
        for s in new_start..new_start + needed {
            self.free_sectors[s] = false;
        }

        Ok(new_start)
    }
}
