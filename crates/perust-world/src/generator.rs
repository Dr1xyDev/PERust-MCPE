//! Terrain generation for Minecraft Bedrock Edition.
//!
//! This module provides:
//! - [`Generator`]: A trait for chunk generation.
//! - [`FlatGenerator`]: Flat world with configurable layers.
//! - [`VoidGenerator`]: Empty world (all air).
//! - [`NormalGenerator`]: Basic noise-based terrain with hills and valleys.

use crate::chunk::{Chunk, CHUNK_SIZE, MAX_SUBCHUNKS, BIOME_SIZE};
use crate::biome::Biome;

// ---------------------------------------------------------------------------
// Generator trait
// ---------------------------------------------------------------------------

/// A terrain generator that produces chunks.
pub trait Generator: Send + Sync {
    /// Generates a chunk at the given coordinates.
    fn generate_chunk(&self, x: i32, z: i32) -> Chunk;

    /// Returns the name of this generator (e.g., "flat", "void", "normal").
    fn name(&self) -> &str;
}

// ---------------------------------------------------------------------------
// FlatGenerator
// ---------------------------------------------------------------------------

/// A flat world generator with configurable layers.
///
/// Default layers: 1 bedrock + 2 dirt + 1 grass (total height 4).
pub struct FlatGenerator {
    /// Layer definitions: (block_id, count) from bottom to top.
    pub layers: Vec<(u8, u8)>,
}

impl FlatGenerator {
    /// Creates a new flat generator with default layers.
    ///
    /// Default: bedrock(7) × 1, dirt(3) × 2, grass(2) × 1.
    pub fn new() -> Self {
        Self {
            layers: vec![
                (7, 1),  // bedrock
                (3, 2),  // dirt
                (2, 1),  // grass
            ],
        }
    }

    /// Creates a flat generator with custom layers.
    pub fn with_layers(layers: Vec<(u8, u8)>) -> Self {
        Self { layers }
    }
}

impl Default for FlatGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator for FlatGenerator {
    fn generate_chunk(&self, x: i32, z: i32) -> Chunk {
        let mut chunk = Chunk::new(x, z);

        // Calculate total height
        let total_height: usize = self.layers.iter().map(|(_, count)| *count as usize).sum();

        // Fill layers from bottom up
        let mut y = 0usize;
        for &(block_id, count) in &self.layers {
            for _ in 0..count {
                if y < MAX_SUBCHUNKS * CHUNK_SIZE {
                    chunk.set_block(0, y, 0, block_id, 0);
                    // Actually fill the entire layer
                    for lz in 0..CHUNK_SIZE {
                        for lx in 0..CHUNK_SIZE {
                            chunk.set_block(lx, y, lz, block_id, 0);
                        }
                    }
                }
                y += 1;
            }
        }

        // Set height map
        let height = total_height.min(MAX_SUBCHUNKS * CHUNK_SIZE) as u16;
        for i in 0..BIOME_SIZE {
            chunk.height_map[i] = height;
        }

        chunk.is_generated = true;
        chunk.is_populated = true;
        chunk
    }

    fn name(&self) -> &str {
        "flat"
    }
}

// ---------------------------------------------------------------------------
// VoidGenerator
// ---------------------------------------------------------------------------

/// A void world generator that produces entirely empty chunks.
pub struct VoidGenerator;

impl VoidGenerator {
    /// Creates a new void generator.
    pub fn new() -> Self {
        Self
    }
}

impl Default for VoidGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator for VoidGenerator {
    fn generate_chunk(&self, x: i32, z: i32) -> Chunk {
        let mut chunk = Chunk::new(x, z);
        chunk.fill_biome(Biome::Void);
        chunk.is_generated = true;
        chunk.is_populated = true;
        chunk
    }

    fn name(&self) -> &str {
        "void"
    }
}

// ---------------------------------------------------------------------------
// NormalGenerator
// ---------------------------------------------------------------------------

/// A basic noise-based terrain generator.
///
/// Produces rolling hills and valleys using a simple pseudo-noise algorithm.
/// This is a simplified implementation and does not include full biome support
/// or advanced features like caves, trees, or ores.
pub struct NormalGenerator {
    /// World seed for deterministic generation.
    pub seed: i64,
    /// Base terrain height (sea level).
    pub base_height: f64,
    /// Amplitude of height variation.
    pub amplitude: f64,
    /// Frequency of the noise.
    pub frequency: f64,
    /// Number of noise octaves.
    pub octaves: u32,
}

impl NormalGenerator {
    /// Creates a new normal generator with the given seed.
    pub fn new(seed: i64) -> Self {
        Self {
            seed,
            base_height: 64.0,
            amplitude: 20.0,
            frequency: 0.01,
            octaves: 4,
        }
    }

    /// Creates a normal generator with custom parameters.
    pub fn with_params(seed: i64, base_height: f64, amplitude: f64, frequency: f64, octaves: u32) -> Self {
        Self {
            seed,
            base_height,
            amplitude,
            frequency,
            octaves,
        }
    }

    /// Simple value noise function based on a hash.
    fn hash(&self, x: i64, z: i64) -> f64 {
        // Simple hash combining seed and coordinates
        let mut h = (self.seed as u64).wrapping_add(
            (x as u64).wrapping_mul(374761393).wrapping_add(
                (z as u64).wrapping_mul(668265263)
            )
        );
        h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        h = h ^ (h >> 16);
        // Map to [-1, 1]
        ((h as i64) as f64) / (i64::MAX as f64)
    }

    /// Smoothed noise with bilinear interpolation.
    fn smooth_noise(&self, x: f64, z: f64) -> f64 {
        let ix = x.floor() as i64;
        let iz = z.floor() as i64;
        let fx = x - ix as f64;
        let fz = z - iz as f64;

        // Smooth interpolation
        let sx = fx * fx * (3.0 - 2.0 * fx);
        let sz = fz * fz * (3.0 - 2.0 * fz);

        let v00 = self.hash(ix, iz);
        let v10 = self.hash(ix + 1, iz);
        let v01 = self.hash(ix, iz + 1);
        let v11 = self.hash(ix + 1, iz + 1);

        let i0 = v00 + sx * (v10 - v00);
        let i1 = v01 + sx * (v11 - v01);

        i0 + sz * (i1 - i0)
    }

    /// Multi-octave noise.
    fn noise(&self, x: f64, z: f64) -> f64 {
        let mut value = 0.0;
        let mut amplitude = self.amplitude;
        let mut frequency = self.frequency;
        let mut max_value = 0.0;

        for _ in 0..self.octaves {
            value += self.smooth_noise(x * frequency, z * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value / max_value
    }

    /// Gets the terrain height at the given world coordinates.
    fn get_height(&self, world_x: i64, world_z: i64) -> i32 {
        let noise = self.noise(world_x as f64, world_z as f64);
        let height = (self.base_height + noise).round() as i32;
        height.clamp(1, (MAX_SUBCHUNKS * CHUNK_SIZE - 1) as i32)
    }

    /// Selects a biome based on noise at the given position.
    fn select_biome(&self, world_x: i64, world_z: i64) -> Biome {
        let temp = self.smooth_noise(world_x as f64 * 0.005, world_z as f64 * 0.005);
        let rain = self.smooth_noise(world_x as f64 * 0.005 + 100.0, world_z as f64 * 0.005 + 100.0);

        if temp < -0.3 {
            if rain > 0.2 {
                Biome::ColdTaiga
            } else {
                Biome::IcePlains
            }
        } else if temp > 0.3 {
            if rain < -0.2 {
                Biome::Desert
            } else if rain > 0.3 {
                Biome::Jungle
            } else {
                Biome::Savanna
            }
        } else {
            if rain < -0.1 {
                Biome::Plain
            } else if rain > 0.2 {
                Biome::Forest
            } else {
                Biome::BirchForest
            }
        }
    }
}

impl Generator for NormalGenerator {
    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        let mut chunk = Chunk::new(chunk_x, chunk_z);

        // Base world X/Z for this chunk
        let base_x = (chunk_x as i64) * (CHUNK_SIZE as i64);
        let base_z = (chunk_z as i64) * (CHUNK_SIZE as i64);

        // Generate terrain column by column
        for lz in 0..CHUNK_SIZE {
            for lx in 0..CHUNK_SIZE {
                let world_x = base_x + lx as i64;
                let world_z = base_z + lz as i64;
                let height = self.get_height(world_x, world_z);

                // Set biome
                let biome = self.select_biome(world_x, world_z);
                chunk.set_biome(lx, lz, biome.as_id());

                // Fill terrain
                for y in 0..height as usize {
                    let block_id = if y == 0 {
                        7  // bedrock
                    } else if y < (height as usize).saturating_sub(4) {
                        1  // stone
                    } else if y < (height as usize).saturating_sub(1) {
                        3  // dirt
                    } else {
                        // Surface block depends on biome
                        match biome {
                            Biome::Desert | Biome::DesertHills => 12, // sand
                            Biome::Ocean | Biome::DeepOcean | Biome::FrozenOcean => 13, // gravel
                            _ => 2, // grass
                        }
                    };
                    chunk.set_block(lx, y, lz, block_id, 0);
                }

                // Fill water up to sea level
                let sea_level = 62usize;
                for y in height as usize..sea_level {
                    chunk.set_block(lx, y, lz, 9, 0); // water
                }
            }
        }

        chunk.recalculate_height_map();
        chunk.is_generated = true;
        chunk.is_populated = true;
        chunk
    }

    fn name(&self) -> &str {
        "normal"
    }
}
