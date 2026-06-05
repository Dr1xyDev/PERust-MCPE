//! Runtime ID allocator for assigning unique identifiers.
//!
//! This module provides [`RuntimeIdAllocator`], a thread-safe allocator that
//! hands out monotonically increasing `u64` IDs. These IDs are suitable for
//! identifying runtime entities such as players, entities, network sessions,
//! or any other object that needs a unique identifier within a single server
//! lifetime.
//!
//! # Examples
//!
//! ```
//! use perust_utils::identity::RuntimeIdAllocator;
//!
//! let allocator = RuntimeIdAllocator::new();
//! let id1 = allocator.next();
//! let id2 = allocator.next();
//! assert!(id2 > id1);
//! ```

use std::sync::atomic::{AtomicU64, Ordering};

// ---------------------------------------------------------------------------
// RuntimeIdAllocator
// ---------------------------------------------------------------------------

/// A thread-safe allocator for monotonically increasing `u64` runtime IDs.
///
/// Each call to [`next`](Self::next) returns the current counter value and
/// increments it by 1. The counter starts at `0` by default, or at a custom
/// starting value when using [`with_start`](Self::with_start).
///
/// # Thread Safety
///
/// The allocator uses [`AtomicU64`] with [`Ordering::Relaxed`] for maximum
/// performance. This is sufficient because:
/// - IDs only need to be unique, not ordered relative to other operations.
/// - The increment operation is atomic, so no two threads will receive the
///   same ID.
///
/// # Use Cases
///
/// - Entity runtime IDs (Minecraft Bedrock uses `u64` runtime IDs)
/// - Player session IDs
/// - Request/response correlation IDs
/// - Any scenario requiring lightweight unique identifiers
///
/// # Examples
///
/// ```
/// use perust_utils::identity::RuntimeIdAllocator;
///
/// let allocator = RuntimeIdAllocator::new();
///
/// let id1 = allocator.next(); // 0
/// let id2 = allocator.next(); // 1
/// let id3 = allocator.next(); // 2
///
/// assert_eq!(id1, 0);
/// assert_eq!(id2, 1);
/// assert_eq!(id3, 2);
/// ```
pub struct RuntimeIdAllocator {
    /// The internal counter.
    counter: AtomicU64,
}

impl RuntimeIdAllocator {
    /// Creates a new allocator starting at `0`.
    ///
    /// The first call to [`next`](Self::next) will return `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use perust_utils::identity::RuntimeIdAllocator;
    ///
    /// let allocator = RuntimeIdAllocator::new();
    /// assert_eq!(allocator.next(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    /// Creates a new allocator starting at the given value.
    ///
    /// The first call to [`next`](Self::next) will return `start`.
    ///
    /// # Examples
    ///
    /// ```
    /// use perust_utils::identity::RuntimeIdAllocator;
    ///
    /// let allocator = RuntimeIdAllocator::with_start(1000);
    /// assert_eq!(allocator.next(), 1000);
    /// ```
    pub fn with_start(start: u64) -> Self {
        Self {
            counter: AtomicU64::new(start),
        }
    }

    /// Allocates and returns the next runtime ID.
    ///
    /// Each call returns a unique value that is strictly greater than all
    /// previously returned values (within this allocator instance).
    ///
    /// # Atomicity
    ///
    /// This operation is atomic and lock-free. It is safe to call from
    /// multiple threads concurrently.
    ///
    /// # Overflow
    ///
    /// If the counter reaches `u64::MAX`, it will wrap around to `0`.
    /// In practice this is extremely unlikely (18 quintillion IDs).
    pub fn next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }

    /// Returns the current value of the counter **without** incrementing it.
    ///
    /// This is the value that will be returned by the next call to [`next`](Self::next).
    ///
    /// # Note
    ///
    /// This is a snapshot; other threads may have incremented the counter
    /// between this call and any subsequent operation.
    pub fn current(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }

    /// Returns the last ID that was allocated.
    ///
    /// This is equivalent to `current() - 1` (with wrapping for the edge
    /// case where no IDs have been allocated yet).
    pub fn last_allocated(&self) -> u64 {
        self.counter.load(Ordering::Relaxed).wrapping_sub(1)
    }

    /// Resets the allocator to the given starting value.
    ///
    /// **Warning**: Use with extreme caution. Resetting the allocator can
    /// cause ID collisions if previously allocated IDs are still in use.
    /// This is primarily intended for testing or server restart scenarios.
    pub fn reset(&self, value: u64) {
        self.counter.store(value, Ordering::Relaxed);
    }
}

impl Default for RuntimeIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RuntimeIdAllocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeIdAllocator")
            .field("current", &self.current())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// RuntimeId
// ---------------------------------------------------------------------------

/// A strongly-typed runtime ID.
///
/// This wraps a `u64` value but provides type safety so that IDs for
/// different entity types cannot be accidentally mixed up.
///
/// # Type Parameter
///
/// - `T`: A marker type that identifies what kind of entity this ID belongs to.
///   This is typically a zero-sized type (ZST) used purely as a type tag.
///
/// # Examples
///
/// ```
/// use perust_utils::identity::{RuntimeId, RuntimeIdAllocator};
///
/// // Define marker types for different entity kinds
/// struct Player;
/// struct Entity;
///
/// let player_allocator: RuntimeIdAllocator = RuntimeIdAllocator::new();
/// let entity_allocator: RuntimeIdAllocator = RuntimeIdAllocator::new();
///
/// let player_id: RuntimeId<Player> = RuntimeId::new(player_allocator.next());
/// let entity_id: RuntimeId<Entity> = RuntimeId::new(entity_allocator.next());
///
/// // These are different types and cannot be mixed:
/// // fn take_player(id: RuntimeId<Player>) { ... }
/// // take_player(entity_id); // compile error!
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuntimeId<T> {
    /// The raw ID value.
    value: u64,
    /// Phantom data to carry the type parameter.
    _marker: std::marker::PhantomData<T>,
}

impl<T> RuntimeId<T> {
    /// Creates a new `RuntimeId` from a raw `u64` value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the raw `u64` value of this ID.
    #[inline]
    pub const fn value(self) -> u64 {
        self.value
    }

    /// Returns the raw `u64` value of this ID by reference.
    #[inline]
    pub const fn value_ref(&self) -> u64 {
        self.value
    }
}

impl<T> std::fmt::Display for RuntimeId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_allocator_basic() {
        let allocator = RuntimeIdAllocator::new();
        assert_eq!(allocator.next(), 0);
        assert_eq!(allocator.next(), 1);
        assert_eq!(allocator.next(), 2);
    }

    #[test]
    fn test_allocator_with_start() {
        let allocator = RuntimeIdAllocator::with_start(1000);
        assert_eq!(allocator.next(), 1000);
        assert_eq!(allocator.next(), 1001);
    }

    #[test]
    fn test_allocator_current() {
        let allocator = RuntimeIdAllocator::new();
        assert_eq!(allocator.current(), 0);
        allocator.next();
        assert_eq!(allocator.current(), 1);
    }

    #[test]
    fn test_allocator_last_allocated() {
        let allocator = RuntimeIdAllocator::new();
        allocator.next();
        allocator.next();
        allocator.next();
        assert_eq!(allocator.last_allocated(), 2);
    }

    #[test]
    fn test_allocator_reset() {
        let allocator = RuntimeIdAllocator::new();
        allocator.next();
        allocator.next();
        allocator.reset(0);
        assert_eq!(allocator.next(), 0);
    }

    #[test]
    fn test_allocator_thread_safety() {
        let allocator = Arc::new(RuntimeIdAllocator::new());
        let num_threads = 4;
        let ids_per_thread = 100;

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let alloc = Arc::clone(&allocator);
                thread::spawn(move || {
                    let mut ids = Vec::with_capacity(ids_per_thread);
                    for _ in 0..ids_per_thread {
                        ids.push(alloc.next());
                    }
                    ids
                })
            })
            .collect();

        let mut all_ids: Vec<u64> = Vec::new();
        for h in handles {
            all_ids.extend(h.join().unwrap());
        }

        // All IDs should be unique
        all_ids.sort();
        all_ids.dedup();
        assert_eq!(all_ids.len(), num_threads * ids_per_thread);
    }

    #[test]
    fn test_runtime_id() {
        struct Player;
        let id: RuntimeId<Player> = RuntimeId::new(42);
        assert_eq!(id.value_ref(), 42);
        assert_eq!(format!("{}", id), "42");
    }

    #[test]
    fn test_runtime_id_type_safety() {
        struct Player;
        struct Entity;

        let player_id: RuntimeId<Player> = RuntimeId::new(1);
        let entity_id: RuntimeId<Entity> = RuntimeId::new(1);

        // These are different types even with the same value
        assert_eq!(player_id.value(), entity_id.value());
        // But the types are different — this wouldn't compile:
        // let _: RuntimeId<Player> = entity_id;
    }
}
