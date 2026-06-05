// Example custom world generator for PeRust.
//
// This demonstrates how to implement the Generator trait to create
// custom terrain generation algorithms. Custom generators can be
// registered with a World to produce chunks with any desired terrain.
//
// In a real scenario, this could be part of a plugin or a built-in
// generator module.

use perust_world::generator::Generator;
use perust_world::chunk::Chunk;

/// A skylands-style generator that creates floating islands.
///
/// This is a conceptual example showing the API pattern. A full
/// implementation would use 3D noise for island shapes, vary the
/// terrain based on position, and add features like trees and ore.
struct SkylandsGenerator {
    /// World seed for deterministic generation.
    seed: i64,
}

impl SkylandsGenerator {
    /// Creates a new skylands generator with the given seed.
    pub fn new(seed: i64) -> Self {
        Self { seed }
    }
}

impl Generator for SkylandsGenerator {
    /// Generates a chunk with floating island terrain.
    ///
    /// A real implementation would:
    /// 1. Use 3D Perlin/Simplex noise to determine island shapes
    /// 2. Carve out the space between islands (all air)
    /// 3. Add surface blocks (grass on top, dirt underneath, stone core)
    /// 4. Optionally place trees, ores, and other features
    /// 5. Set biome data for each column
    fn generate_chunk(&self, x: i32, z: i32) -> Chunk {
        let mut chunk = Chunk::new(x, z);

        // Simple placeholder: create a floating platform at y=64
        // A real implementation would use noise-based island detection
        let center_x = 8;
        let center_z = 8;
        let radius = 6;

        for lz in 0..16 {
            for lx in 0..16 {
                let dx = (lx as i32) - center_x;
                let dz = (lz as i32) - center_z;
                let dist = ((dx * dx + dz * dz) as f64).sqrt();

                if dist < radius as f64 {
                    // Island surface
                    chunk.set_block(lx, 68, lz, 2, 0); // grass
                    chunk.set_block(lx, 67, lz, 3, 0); // dirt
                    chunk.set_block(lx, 66, lz, 3, 0); // dirt
                    chunk.set_block(lx, 65, lz, 1, 0); // stone
                    chunk.set_block(lx, 64, lz, 7, 0); // bedrock

                    // Taper the bottom
                    if dist < (radius - 2) as f64 {
                        chunk.set_block(lx, 63, lz, 1, 0); // stone
                        chunk.set_block(lx, 62, lz, 1, 0); // stone
                    }
                }
            }
        }

        chunk.is_generated = true;
        chunk.is_populated = true;
        chunk
    }

    /// Returns the generator name for identification.
    fn name(&self) -> &str {
        "skylands"
    }
}

// To use this generator with a World:
//
// ```rust,ignore
// let generator = SkylandsGenerator::new(12345);
// let world = World::with_generator("skylands_world", path, Box::new(generator));
// ```
