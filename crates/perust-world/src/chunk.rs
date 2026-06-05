//! Chunk data structures for Minecraft Bedrock Edition.
//!
//! This module provides:
//! - [`SubChunk`]: A 16×16×16 section of blocks with block IDs, data, and lighting.
//! - [`Chunk`]: A full chunk column containing up to 16 sub-chunks, biomes, and height map.
//!
//! # Network Serialization
//!
//! Chunks can be serialized to the MCPE network format via [`Chunk::serialize_network`].

use std::collections::HashMap;
use perust_utils::BinaryWriter;
use crate::biome::Biome;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Width/depth of a chunk in blocks.
pub const CHUNK_SIZE: usize = 16;
/// Maximum number of sub-chunks in a chunk column.
pub const MAX_SUBCHUNKS: usize = 16;
/// Number of blocks in a sub-chunk (16×16×16).
pub const SUBCHUNK_SIZE: usize = 4096;
/// Number of biome entries per chunk (16×16).
pub const BIOME_SIZE: usize = 256;

// ---------------------------------------------------------------------------
// Nibble Array Helpers
// ---------------------------------------------------------------------------

/// Index into a nibble array for the given sub-chunk coordinates.
/// Nibble arrays pack two 4-bit values per byte, so index = coord / 2.
#[inline]
fn nibble_index(x: usize, y: usize, z: usize) -> usize {
    let coord = (x << 8) | (z << 4) | y;
    coord >> 1
}

/// Whether the given coordinate is in the high nibble (true) or low nibble (false).
#[inline]
fn is_high_nibble(x: usize, y: usize, z: usize) -> bool {
    let coord = (x << 8) | (z << 4) | y;
    (coord & 1) == 1
}

// ---------------------------------------------------------------------------
// SubChunk
// ---------------------------------------------------------------------------

/// A 16×16×16 sub-chunk section of a chunk.
///
/// Block IDs are stored in XZY ordering: `(x << 8) | (z << 4) | y`.
/// Block data, sky light, and block light use nibble (4-bit) packing.
#[derive(Clone)]
pub struct SubChunk {
    /// Block IDs (one byte per block).
    pub block_ids: [u8; SUBCHUNK_SIZE],
    /// Block data (nibble array: two 4-bit values per byte).
    pub block_data: [u8; SUBCHUNK_SIZE / 2],
    /// Sky light levels (nibble array, default 0xFF = 15 per block).
    pub sky_light: [u8; SUBCHUNK_SIZE / 2],
    /// Block light levels (nibble array, default 0x00 = 0 per block).
    pub block_light: [u8; SUBCHUNK_SIZE / 2],
}

impl SubChunk {
    /// Creates a new empty sub-chunk filled with air (block ID 0).
    ///
    /// Sky light defaults to full brightness (0xFF = 15 per entry),
    /// and block light defaults to 0.
    pub fn new() -> Self {
        Self {
            block_ids: [0u8; SUBCHUNK_SIZE],
            block_data: [0u8; SUBCHUNK_SIZE / 2],
            sky_light: [0xFFu8; SUBCHUNK_SIZE / 2],
            block_light: [0u8; SUBCHUNK_SIZE / 2],
        }
    }

    /// Returns the block index in XZY ordering: `(x << 8) | (z << 4) | y`.
    #[inline]
    fn block_index(x: usize, y: usize, z: usize) -> usize {
        (x << 8) | (z << 4) | y
    }

    /// Gets the block ID and block data at the given sub-chunk local coordinates.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is >= 16.
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> (u8, u8) {
        assert!(x < 16 && y < 16 && z < 16, "sub-chunk coordinates out of range");
        let idx = Self::block_index(x, y, z);
        let block_id = self.block_ids[idx];
        let block_data = if is_high_nibble(x, y, z) {
            self.block_data[nibble_index(x, y, z)] >> 4
        } else {
            self.block_data[nibble_index(x, y, z)] & 0x0F
        };
        (block_id, block_data)
    }

    /// Sets the block ID and block data at the given sub-chunk local coordinates.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is >= 16.
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u8, block_data: u8) {
        assert!(x < 16 && y < 16 && z < 16, "sub-chunk coordinates out of range");
        let idx = Self::block_index(x, y, z);
        self.block_ids[idx] = block_id;
        let ni = nibble_index(x, y, z);
        if is_high_nibble(x, y, z) {
            self.block_data[ni] = (self.block_data[ni] & 0x0F) | ((block_data & 0x0F) << 4);
        } else {
            self.block_data[ni] = (self.block_data[ni] & 0xF0) | (block_data & 0x0F);
        }
    }

    /// Gets the block light level at the given coordinates.
    pub fn get_block_light(&self, x: usize, y: usize, z: usize) -> u8 {
        let ni = nibble_index(x, y, z);
        if is_high_nibble(x, y, z) {
            self.block_light[ni] >> 4
        } else {
            self.block_light[ni] & 0x0F
        }
    }

    /// Sets the block light level at the given coordinates.
    pub fn set_block_light(&mut self, x: usize, y: usize, z: usize, level: u8) {
        let ni = nibble_index(x, y, z);
        if is_high_nibble(x, y, z) {
            self.block_light[ni] = (self.block_light[ni] & 0x0F) | ((level & 0x0F) << 4);
        } else {
            self.block_light[ni] = (self.block_light[ni] & 0xF0) | (level & 0x0F);
        }
    }

    /// Gets the sky light level at the given coordinates.
    pub fn get_sky_light(&self, x: usize, y: usize, z: usize) -> u8 {
        let ni = nibble_index(x, y, z);
        if is_high_nibble(x, y, z) {
            self.sky_light[ni] >> 4
        } else {
            self.sky_light[ni] & 0x0F
        }
    }

    /// Sets the sky light level at the given coordinates.
    pub fn set_sky_light(&mut self, x: usize, y: usize, z: usize, level: u8) {
        let ni = nibble_index(x, y, z);
        if is_high_nibble(x, y, z) {
            self.sky_light[ni] = (self.sky_light[ni] & 0x0F) | ((level & 0x0F) << 4);
        } else {
            self.sky_light[ni] = (self.sky_light[ni] & 0xF0) | (level & 0x0F);
        }
    }

    /// Serializes this sub-chunk to the MCPE network format.
    ///
    /// Format:
    /// ```text
    /// 1 byte: version (0x00)
    /// 4096 bytes: block IDs
    /// 2048 bytes: block data (nibble)
    /// 2048 bytes: sky light (nibble)
    /// 2048 bytes: block light (nibble)
    /// ```
    pub fn serialize_network(&self, writer: &mut BinaryWriter) {
        writer.write_u8(0x00); // version
        writer.write_bytes(&self.block_ids);
        writer.write_bytes(&self.block_data);
        writer.write_bytes(&self.sky_light);
        writer.write_bytes(&self.block_light);
    }
}

impl Default for SubChunk {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Chunk
// ---------------------------------------------------------------------------

/// A full chunk column (16×256×16 blocks).
///
/// A chunk consists of up to [`MAX_SUBCHUNKS`] sub-chunks stacked vertically,
/// plus a biome array, height map, and extra data for tile entities/blocks
/// with metadata beyond the 4-bit limit.
pub struct Chunk {
    /// Chunk X coordinate.
    pub x: i32,
    /// Chunk Z coordinate.
    pub z: i32,
    /// Sub-chunks, indexed 0 (bottom) to MAX_SUBCHUNKS-1 (top).
    /// `None` means the sub-chunk is entirely air.
    pub sub_chunks: Vec<Option<SubChunk>>,
    /// Height map: the highest non-air block Y+1 for each column.
    pub height_map: [u16; BIOME_SIZE],
    /// Biome IDs for each column (16×16).
    pub biomes: [u8; BIOME_SIZE],
    /// Whether this chunk has been generated.
    pub is_generated: bool,
    /// Whether this chunk has been populated (decorated).
    pub is_populated: bool,
    /// Whether this chunk has unsaved changes.
    pub is_dirty: bool,
    /// Extra block data: maps `(x<<12)|(z<<8)|y` to `(meta<<8)|id`.
    pub extra_data: HashMap<u32, u16>,
}

impl Chunk {
    /// Creates a new empty chunk at the given coordinates.
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            sub_chunks: (0..MAX_SUBCHUNKS).map(|_| None).collect(),
            height_map: [0u16; BIOME_SIZE],
            biomes: [Biome::Plain.as_id(); BIOME_SIZE],
            is_generated: false,
            is_populated: false,
            is_dirty: false,
            extra_data: HashMap::new(),
        }
    }

    /// Gets the block ID and block data at the given world-relative coordinates.
    ///
    /// Coordinates are local to the chunk (0..16 for x/z, 0..256 for y).
    /// Returns `(0, 0)` (air) if the position is out of range or the
    /// sub-chunk is not present.
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> (u8, u8) {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE || y >= (MAX_SUBCHUNKS * CHUNK_SIZE) {
            return (0, 0);
        }
        let sub_y = y / CHUNK_SIZE;
        let local_y = y % CHUNK_SIZE;
        match &self.sub_chunks[sub_y] {
            Some(sub) => sub.get_block(x, local_y, z),
            None => (0, 0),
        }
    }

    /// Sets the block ID and block data at the given world-relative coordinates.
    ///
    /// Creates the sub-chunk if it doesn't exist yet.
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, id: u8, data: u8) {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE || y >= (MAX_SUBCHUNKS * CHUNK_SIZE) {
            return;
        }
        let sub_y = y / CHUNK_SIZE;
        let local_y = y % CHUNK_SIZE;
        if self.sub_chunks[sub_y].is_none() {
            self.sub_chunks[sub_y] = Some(SubChunk::new());
        }
        if let Some(ref mut sub) = self.sub_chunks[sub_y] {
            sub.set_block(x, local_y, z, id, data);
        }
        self.is_dirty = true;

        // Update height map: if placing a non-air block above the current height
        let idx = (z << 4) | x;
        if id != 0 && (y as u16 + 1) > self.height_map[idx] {
            self.height_map[idx] = y as u16 + 1;
        } else if id == 0 && self.height_map[idx] == y as u16 + 1 {
            // Block removed at height map level — recalculate
            self.recalculate_height_map_column(x, z);
        }
    }

    /// Gets the biome at the given column (x, z) within this chunk.
    pub fn get_biome(&self, x: usize, z: usize) -> u8 {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return Biome::Plain.as_id();
        }
        self.biomes[(z << 4) | x]
    }

    /// Sets the biome at the given column (x, z) within this chunk.
    pub fn set_biome(&mut self, x: usize, z: usize, biome: u8) {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return;
        }
        self.biomes[(z << 4) | x] = biome;
        self.is_dirty = true;
    }

    /// Gets the height map value at the given column (x, z).
    pub fn get_height(&self, x: usize, z: usize) -> u16 {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return 0;
        }
        self.height_map[(z << 4) | x]
    }

    /// Recalculates the entire height map by scanning from top to bottom
    /// for the first non-air block in each column.
    pub fn recalculate_height_map(&mut self) {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                self.recalculate_height_map_column(x, z);
            }
        }
    }

    /// Recalculates the height map for a single column.
    fn recalculate_height_map_column(&mut self, x: usize, z: usize) {
        let mut height = 0u16;
        for sub_y in (0..MAX_SUBCHUNKS).rev() {
            if let Some(ref sub) = self.sub_chunks[sub_y] {
                for local_y in (0..CHUNK_SIZE).rev() {
                    let (block_id, _) = sub.get_block(x, local_y, z);
                    if block_id != 0 {
                        let y = sub_y * CHUNK_SIZE + local_y;
                        height = (y as u16) + 1;
                        break;
                    }
                }
                if height > 0 {
                    break;
                }
            }
        }
        self.height_map[(z << 4) | x] = height;
    }

    /// Returns the number of non-empty sub-chunks (for network serialization).
    pub fn sub_chunk_count(&self) -> u8 {
        let mut count = 0u8;
        for sub in &self.sub_chunks {
            if sub.is_some() {
                count = (count + 1).min(MAX_SUBCHUNKS as u8);
            }
        }
        // Return the highest sub-chunk index + 1
        let mut highest = 0u8;
        for (i, sub) in self.sub_chunks.iter().enumerate() {
            if sub.is_some() {
                highest = (i + 1) as u8;
            }
        }
        highest
    }

    /// Serializes this chunk to the MCPE network format for FullChunkDataPacket.
    ///
    /// Format:
    /// ```text
    /// 1 byte: sub-chunk count
    /// [sub-chunk data × count]: serialized sub-chunks
    /// 512 bytes: height map (256 × u16 big-endian)
    /// 256 bytes: biome array
    /// 0 bytes: border blocks
    /// VarInt: extra data count (0 for now)
    /// ```
    pub fn serialize_network(&self) -> Vec<u8> {
        let sub_count = self.sub_chunk_count();
        // Estimate: 1 + sub_count * (1+4096+2048+2048+2048) + 512 + 256 + 0 + 1
        let estimated_size = 1 + (sub_count as usize) * 10241 + 512 + 256 + 8;
        let mut writer = BinaryWriter::with_capacity(estimated_size);

        // Sub-chunk count
        writer.write_u8(sub_count);

        // Serialize each present sub-chunk
        for i in 0..(sub_count as usize) {
            match &self.sub_chunks[i] {
                Some(sub) => sub.serialize_network(&mut writer),
                None => {
                    // Write an empty sub-chunk
                    let empty = SubChunk::new();
                    empty.serialize_network(&mut writer);
                }
            }
        }

        // Height map: 256 u16 values in big-endian
        for &h in &self.height_map {
            writer.write_u16(h);
        }

        // Biomes: 256 bytes
        writer.write_bytes(&self.biomes);

        // Border blocks: 0 bytes (deprecated)

        // Extra data count: 0 for now
        writer.write_var_int(0);

        writer.finish()
    }

    /// Sets the biome for all columns in this chunk.
    pub fn fill_biome(&mut self, biome: Biome) {
        self.biomes = [biome.as_id(); BIOME_SIZE];
        self.is_dirty = true;
    }

    /// Returns `true` if this chunk has no non-air blocks.
    pub fn is_empty(&self) -> bool {
        self.sub_chunks.iter().all(|s| s.is_none())
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new(0, 0)
    }
}
