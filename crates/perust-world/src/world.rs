//! World management for Minecraft Bedrock Edition.
//!
//! This module provides [`World`], the top-level world management structure
//! that handles chunk loading, generation, ticking, and persistence.

use std::path::PathBuf;
use std::sync::Arc;
use perust_utils::math::BlockPos;
use perust_protocol::types::Difficulty;
use crate::chunk::{Chunk, CHUNK_SIZE};
use crate::chunk_manager::ChunkManager;
use crate::generator::Generator;
use crate::region::{RegionFile, REGION_SIZE};
use crate::error::WorldError;

// ---------------------------------------------------------------------------
// World
// ---------------------------------------------------------------------------

/// Top-level world management structure.
///
/// A `World` owns a chunk manager, a terrain generator, and world-level
/// properties such as seed, time, spawn position, and difficulty.
pub struct World {
    /// The world name (used for folder naming).
    pub name: String,
    /// Path to the world directory on disk.
    pub folder: PathBuf,
    /// Chunk storage.
    chunk_manager: ChunkManager,
    /// Terrain generator.
    generator: Box<dyn Generator>,
    /// World seed for deterministic generation.
    pub seed: i64,
    /// Current world time (day/night cycle, in ticks).
    pub time: i32,
    /// Spawn position for new players.
    pub spawn_position: BlockPos,
    /// World difficulty.
    pub difficulty: Difficulty,
    /// Tick counter.
    pub tick_counter: u64,
}

impl World {
    /// Creates a new world with the given name, folder, and generator.
    pub fn new(
        name: String,
        folder: PathBuf,
        generator: Box<dyn Generator>,
        seed: i64,
        difficulty: Difficulty,
    ) -> Self {
        Self {
            name,
            folder,
            chunk_manager: ChunkManager::new(8),
            generator,
            seed,
            time: 0,
            spawn_position: BlockPos::new(0, 64, 0),
            difficulty,
            tick_counter: 0,
        }
    }

    /// Creates a default flat world.
    pub fn flat(name: String, folder: PathBuf) -> Self {
        use crate::generator::FlatGenerator;
        Self::new(
            name,
            folder,
            Box::new(FlatGenerator::new()),
            0,
            Difficulty::Normal,
        )
    }

    /// Creates a default void world.
    pub fn void(name: String, folder: PathBuf) -> Self {
        use crate::generator::VoidGenerator;
        Self::new(
            name,
            folder,
            Box::new(VoidGenerator::new()),
            0,
            Difficulty::Normal,
        )
    }

    /// Creates a default normal world.
    pub fn normal(name: String, folder: PathBuf, seed: i64) -> Self {
        use crate::generator::NormalGenerator;
        Self::new(
            name,
            folder,
            Box::new(NormalGenerator::new(seed)),
            seed,
            Difficulty::Normal,
        )
    }

    /// Loads the world from disk.
    ///
    /// This reads the world directory structure and loads chunk data
    /// from region files.
    pub fn load(&mut self) -> Result<(), WorldError> {
        // Ensure world directory exists
        std::fs::create_dir_all(&self.folder)?;

        let region_dir = self.folder.join("region");
        std::fs::create_dir_all(&region_dir)?;

        // Load level.dat equivalent (simplified: just read region files)
        if region_dir.exists() {
            self.load_regions(&region_dir)?;
        }

        log::info!("World '{}' loaded with {} chunks", self.name, self.chunk_manager.chunk_count());
        Ok(())
    }

    /// Loads all region files from the given directory.
    fn load_regions(&mut self, region_dir: &PathBuf) -> Result<(), WorldError> {
        let entries = std::fs::read_dir(region_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("mca") {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some((rx, rz)) = Self::parse_region_filename(filename) {
                        if let Ok(mut region) = RegionFile::new(&path) {
                            for lz in 0..REGION_SIZE {
                                for lx in 0..REGION_SIZE {
                                    if region.has_chunk(lx, lz) {
                                        if let Ok(Some(_data)) = region.read_chunk_data(lx, lz) {
                                            // For now, just note that the chunk exists;
                                            // actual deserialization would go here.
                                            let cx = rx as i32 * REGION_SIZE as i32 + lx as i32;
                                            let cz = rz as i32 * REGION_SIZE as i32 + lz as i32;
                                            if !self.chunk_manager.has_chunk(cx, cz) {
                                                let chunk = self.generator.generate_chunk(cx, cz);
                                                self.chunk_manager.set_chunk(chunk);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Parses a region filename like `r.0.0.mca` into region coordinates.
    fn parse_region_filename(filename: &str) -> Option<(i32, i32)> {
        let parts: Vec<&str> = filename.split('.').collect();
        if parts.len() == 4 && parts[0] == "r" && parts[3] == "mca" {
            let rx = parts[1].parse::<i32>().ok()?;
            let rz = parts[2].parse::<i32>().ok()?;
            Some((rx, rz))
        } else {
            None
        }
    }

    /// Saves the world to disk.
    ///
    /// Writes all dirty chunks to their respective region files.
    pub fn save(&self) -> Result<(), WorldError> {
        let region_dir = self.folder.join("region");
        std::fs::create_dir_all(&region_dir)?;

        // Group chunks by region
        let mut region_chunks: std::collections::HashMap<(i32, i32), Vec<Arc<Chunk>>> =
            std::collections::HashMap::new();

        for (cx, cz) in self.chunk_manager.loaded_chunk_coords() {
            if let Some(chunk) = self.chunk_manager.get_chunk(cx, cz) {
                if chunk.is_dirty {
                    let rx = cx.div_euclid(REGION_SIZE as i32);
                    let rz = cz.div_euclid(REGION_SIZE as i32);
                    region_chunks.entry((rx, rz)).or_default().push(chunk);
                }
            }
        }

        for ((rx, rz), chunks) in region_chunks {
            let filename = format!("r.{}.{}.mca", rx, rz);
            let path = region_dir.join(&filename);

            let mut region = RegionFile::new(&path)?;
            for chunk in chunks {
                let local_x = chunk.x.rem_euclid(REGION_SIZE as i32) as usize;
                let local_z = chunk.z.rem_euclid(REGION_SIZE as i32) as usize;

                // Serialize chunk to raw bytes (simplified)
                let data = chunk.serialize_network();
                region.write_chunk_data(local_x, local_z, &data)?;
            }
        }

        log::info!("World '{}' saved", self.name);
        Ok(())
    }

    /// Performs one tick of the world simulation.
    ///
    /// This advances the time, processes scheduled tasks, etc.
    pub fn tick(&mut self) {
        self.tick_counter += 1;
        self.time = (self.time + 1) % 24000;
    }

    /// Gets the block at the given world block position.
    ///
    /// Returns `(0, 0)` (air) if the chunk is not loaded.
    pub fn get_block(&self, pos: &BlockPos) -> (u8, u8) {
        let chunk_x = pos.x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = pos.z.div_euclid(CHUNK_SIZE as i32);
        if let Some(chunk) = self.chunk_manager.get_chunk(chunk_x, chunk_z) {
            let local_x = pos.x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_z = pos.z.rem_euclid(CHUNK_SIZE as i32) as usize;
            let y = pos.y as usize;
            chunk.get_block(local_x, y, local_z)
        } else {
            (0, 0)
        }
    }

    /// Sets the block at the given world block position.
    ///
    /// Note: Currently a stub due to Arc<Chunk> being immutable.
    /// TODO: Use interior mutability for chunk write access.
    pub fn set_block(&self, pos: &BlockPos, _id: u8, _data: u8) {
        let chunk_x = pos.x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = pos.z.div_euclid(CHUNK_SIZE as i32);
        if self.chunk_manager.has_chunk(chunk_x, chunk_z) {
            // We need mutable access, but Arc<Chunk> is shared.
            // In a real implementation, we'd use interior mutability (RwLock on Chunk).
            log::warn!("set_block: chunk mutability not yet fully supported");
        }
    }

    /// Gets the chunk at the given coordinates, generating it if necessary.
    pub fn get_chunk(&self, x: i32, z: i32) -> Option<Arc<Chunk>> {
        self.chunk_manager.get_chunk(x, z)
    }

    /// Generates and loads a chunk at the given coordinates.
    pub fn generate_chunk(&self, x: i32, z: i32) -> Arc<Chunk> {
        if let Some(chunk) = self.chunk_manager.get_chunk(x, z) {
            return chunk;
        }
        let chunk = self.generator.generate_chunk(x, z);
        let arc = Arc::new(chunk);
        self.chunk_manager.set_arc_chunk(x, z, Arc::clone(&arc));
        arc
    }

    /// Returns the spawn position.
    pub fn get_spawn_position(&self) -> BlockPos {
        self.spawn_position
    }

    /// Sets the spawn position.
    pub fn set_spawn_position(&mut self, pos: BlockPos) {
        self.spawn_position = pos;
    }

    /// Returns a reference to the chunk manager.
    pub fn chunk_manager(&self) -> &ChunkManager {
        &self.chunk_manager
    }

    /// Returns a mutable reference to the chunk manager.
    pub fn chunk_manager_mut(&mut self) -> &mut ChunkManager {
        &mut self.chunk_manager
    }

    /// Returns the generator name.
    pub fn generator_name(&self) -> &str {
        self.generator.name()
    }

    /// Ensures chunks are loaded in the given radius around a center position.
    ///
    /// Returns the list of newly generated chunks.
    pub fn ensure_chunks_in_radius(&self, center_x: i32, center_z: i32, radius: u32) -> Vec<Arc<Chunk>> {
        let mut new_chunks = Vec::new();
        let r = radius as i32;
        for dx in -r..=r {
            for dz in -r..=r {
                let cx = center_x + dx;
                let cz = center_z + dz;
                if !self.chunk_manager.has_chunk(cx, cz) {
                    let chunk = self.generator.generate_chunk(cx, cz);
                    let arc = Arc::new(chunk);
                    self.chunk_manager.set_arc_chunk(cx, cz, Arc::clone(&arc));
                    new_chunks.push(arc);
                }
            }
        }
        new_chunks
    }

    /// Unloads chunks outside the given radius.
    ///
    /// Returns the number of chunks unloaded.
    pub fn unload_chunks_outside_radius(&self, center_x: i32, center_z: i32, radius: u32) -> usize {
        self.chunk_manager.unload_outside_radius(center_x, center_z, radius)
    }
}
