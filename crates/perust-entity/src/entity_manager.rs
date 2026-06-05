//! Entity manager for thread-safe entity storage and ID allocation.
//!
//! This module provides [`EntityManager`], which manages entity lifecycle
//! and provides efficient spatial queries.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use dashmap::DashMap;
use perust_utils::math::Vector3f;
use crate::entity::Entity;
use crate::error::EntityError;

// ---------------------------------------------------------------------------
// EntityManager
// ---------------------------------------------------------------------------

/// Thread-safe manager for entity storage, ID allocation, and queries.
///
/// Entities are stored as `Arc<Entity>` in a `DashMap`, keyed by runtime ID.
pub struct EntityManager {
    /// Entity storage, keyed by runtime entity ID.
    entities: DashMap<u64, Arc<Entity>>,
    /// Next runtime entity ID.
    next_id: AtomicU64,
}

impl EntityManager {
    /// Creates a new entity manager.
    pub fn new() -> Self {
        Self {
            entities: DashMap::new(),
            next_id: AtomicU64::new(1),
        }
    }

    /// Allocates a new unique runtime entity ID.
    pub fn allocate_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Spawns a new entity, assigning it a runtime ID.
    ///
    /// Returns the runtime ID of the spawned entity.
    pub fn spawn_entity(&self, mut entity: Entity) -> u64 {
        let id = self.allocate_id();
        entity.id = id;
        if entity.unique_id == 0 {
            entity.unique_id = id as i64;
        }
        self.entities.insert(id, Arc::new(entity));
        id
    }

    /// Removes an entity by runtime ID.
    ///
    /// Returns the removed entity, or an error if not found.
    pub fn remove_entity(&self, id: u64) -> Result<Arc<Entity>, EntityError> {
        self.entities
            .remove(&id)
            .map(|(_, v)| v)
            .ok_or(EntityError::EntityNotFound(id))
    }

    /// Gets an entity by runtime ID.
    pub fn get_entity(&self, id: u64) -> Option<Arc<Entity>> {
        self.entities.get(&id).map(|r| Arc::clone(r.value()))
    }

    /// Returns `true` if an entity with the given ID exists.
    pub fn has_entity(&self, id: u64) -> bool {
        self.entities.contains_key(&id)
    }

    /// Returns the number of entities.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Ticks all entities.
    ///
    /// Removes dead entities whose death animation has completed.
    pub fn tick_all(&self) {
        // In a real implementation, we'd use Arc<RwLock<Entity>> or similar
        // for mutable ticking. For now, just track entity removal.
        // TODO: Implement proper mutable entity ticking.
    }

    /// Returns all entities within the given range of a position.
    ///
    /// Uses squared distance for efficiency.
    pub fn get_entities_in_range(&self, position: Vector3f, range: f32) -> Vec<Arc<Entity>> {
        let range_sq = range * range;
        self.entities
            .iter()
            .filter(|entry| {
                let entity = entry.value();
                let dx = entity.position.x - position.x;
                let dy = entity.position.y - position.y;
                let dz = entity.position.z - position.z;
                (dx * dx + dy * dy + dz * dz) <= range_sq
            })
            .map(|entry| Arc::clone(entry.value()))
            .collect()
    }

    /// Returns all entity IDs.
    pub fn entity_ids(&self) -> Vec<u64> {
        self.entities.iter().map(|entry| *entry.key()).collect()
    }

    /// Returns all entities.
    pub fn all_entities(&self) -> Vec<Arc<Entity>> {
        self.entities.iter().map(|entry| Arc::clone(entry.value())).collect()
    }

    /// Removes all entities.
    pub fn clear(&self) {
        self.entities.clear();
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}
