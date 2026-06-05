//! Thread-safe chunk storage and retrieval.
//!
//! This module provides [`ChunkManager`], which stores chunks in a concurrent
//! hash map and supports radius-based queries.

use std::sync::Arc;
use dashmap::DashMap;
use crate::chunk::Chunk;

// ---------------------------------------------------------------------------
// ChunkManager
// ---------------------------------------------------------------------------

/// Thread-safe manager for chunk storage and retrieval.
///
/// Chunks are stored as `Arc<Chunk>` in a `DashMap`, keyed by `(x, z)` coordinates.
pub struct ChunkManager {
    /// The chunk storage, keyed by (chunk_x, chunk_z).
    chunks: DashMap<(i32, i32), Arc<Chunk>>,
    /// The default chunk loading radius.
    radius: u32,
}

impl ChunkManager {
    /// Creates a new chunk manager with the given loading radius.
    pub fn new(radius: u32) -> Self {
        Self {
            chunks: DashMap::new(),
            radius,
        }
    }

    /// Gets a reference-counted chunk at the given coordinates.
    ///
    /// Returns `None` if the chunk is not loaded.
    pub fn get_chunk(&self, x: i32, z: i32) -> Option<Arc<Chunk>> {
        self.chunks.get(&(x, z)).map(|r| Arc::clone(r.value()))
    }

    /// Stores a chunk at the given coordinates.
    pub fn set_chunk(&self, chunk: Chunk) {
        let key = (chunk.x, chunk.z);
        self.chunks.insert(key, Arc::new(chunk));
    }

    /// Inserts an already-arc'd chunk.
    pub fn set_arc_chunk(&self, x: i32, z: i32, chunk: Arc<Chunk>) {
        self.chunks.insert((x, z), chunk);
    }

    /// Removes and returns the chunk at the given coordinates.
    pub fn remove_chunk(&self, x: i32, z: i32) -> Option<Arc<Chunk>> {
        self.chunks.remove(&(x, z)).map(|(_, v)| v)
    }

    /// Returns `true` if a chunk is loaded at the given coordinates.
    pub fn has_chunk(&self, x: i32, z: i32) -> bool {
        self.chunks.contains_key(&(x, z))
    }

    /// Returns the number of loaded chunks.
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Returns the loading radius.
    pub fn radius(&self) -> u32 {
        self.radius
    }

    /// Sets the loading radius.
    pub fn set_radius(&mut self, radius: u32) {
        self.radius = radius;
    }

    /// Returns all chunks within the given radius of the center chunk.
    ///
    /// The radius is measured in chunks (Chebyshev distance).
    pub fn get_chunks_in_radius(&self, center_x: i32, center_z: i32, radius: u32) -> Vec<Arc<Chunk>> {
        let mut result = Vec::new();
        let r = radius as i32;
        for dx in -r..=r {
            for dz in -r..=r {
                let cx = center_x + dx;
                let cz = center_z + dz;
                if let Some(chunk) = self.get_chunk(cx, cz) {
                    result.push(chunk);
                }
            }
        }
        result
    }

    /// Returns all loaded chunk coordinates.
    pub fn loaded_chunk_coords(&self) -> Vec<(i32, i32)> {
        self.chunks.iter().map(|r| *r.key()).collect()
    }

    /// Removes all chunks outside the given radius from the center.
    ///
    /// Returns the number of chunks unloaded.
    pub fn unload_outside_radius(&self, center_x: i32, center_z: i32, radius: u32) -> usize {
        let r = radius as i32;
        let to_remove: Vec<(i32, i32)> = self
            .chunks
            .iter()
            .filter(|entry| {
                let (cx, cz) = entry.key();
                (cx - center_x).unsigned_abs() > r as u32
                    || (cz - center_z).unsigned_abs() > r as u32
            })
            .map(|entry| *entry.key())
            .collect();

        let count = to_remove.len();
        for key in to_remove {
            self.chunks.remove(&key);
        }
        count
    }
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self::new(8)
    }
}
