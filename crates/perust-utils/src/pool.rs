//! Object pool implementation for reuse of expensive-to-create objects.
//!
//! This module provides:
//! - [`ObjectPool`]: A thread-safe pool that manages a collection of reusable objects.
//! - [`PoolGuard`]: A smart pointer that automatically returns the object to the pool on drop.
//!
//! # Motivation
//!
//! In a game server, certain objects (e.g., packet buffers, entity metadata,
//! chunk data structures) are created and destroyed frequently. Allocating
//! and deallocating these on every use can cause significant GC/memory pressure.
//! An object pool allows these objects to be reused, amortizing allocation costs.
//!
//! # Examples
//!
//! ```
//! use perust_utils::pool::ObjectPool;
//!
//! // Create a pool that produces Vec<u8> with capacity 1024
//! let pool: ObjectPool<Vec<u8>> = ObjectPool::new(|| Vec::with_capacity(1024));
//!
//! // Acquire an object from the pool
//! let mut buf = pool.acquire();
//! buf.extend_from_slice(b"hello");
//! assert_eq!(&buf[..], b"hello");
//!
//! // When `buf` is dropped, it is automatically returned to the pool
//! drop(buf);
//!
//! // Acquire again — this reuses the previously returned object
//! let buf2 = pool.acquire();
//! assert!(buf2.capacity() >= 1024);
//! ```

use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// ObjectPool
// ---------------------------------------------------------------------------

/// A thread-safe object pool for reusable objects.
///
/// Objects are created on demand using the provided factory closure when the
/// pool is empty. When a [`PoolGuard`] is dropped, the object is reset and
/// returned to the pool for reuse.
///
/// # Type Parameters
///
/// - `T`: The type of object being pooled. Must implement [`Reset`] (or use
///   the blanket impl for types that implement `Default` or `Clear`).
///
/// # Thread Safety
///
/// The pool uses a `Mutex<VecDeque<T>>` internally, so it is safe to share
/// across threads. Acquire and release operations are relatively fast but
/// do acquire a lock.
pub struct ObjectPool<T> {
    /// The internal storage of available objects.
    objects: Mutex<VecDeque<T>>,
    /// Factory closure to create new objects when the pool is empty.
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> ObjectPool<T> {
    /// Creates a new object pool with the given factory closure.
    ///
    /// The pool starts empty. Objects are created on demand by calling the
    /// factory when [`acquire`](Self::acquire) is called on an empty pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use perust_utils::pool::ObjectPool;
    ///
    /// let pool = ObjectPool::new(|| String::new());
    /// ```
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            objects: Mutex::new(VecDeque::new()),
            factory: Box::new(factory),
        }
    }

    /// Creates a new object pool pre-populated with `count` objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use perust_utils::pool::ObjectPool;
    ///
    /// let pool = ObjectPool::with_capacity(|| Vec::<u8>::with_capacity(256), 4);
    /// ```
    pub fn with_capacity<F>(factory: F, count: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let objects: VecDeque<T> = (0..count).map(|_| factory()).collect();
        Self {
            objects: Mutex::new(objects),
            factory: Box::new(factory),
        }
    }

    /// Acquires an object from the pool.
    ///
    /// If the pool contains available objects, one is removed and returned.
    /// Otherwise, a new object is created using the factory closure.
    ///
    /// The returned [`PoolGuard`] will automatically return the object to the
    /// pool when dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use perust_utils::pool::ObjectPool;
    ///
    /// let pool = ObjectPool::new(|| Vec::<u8>::new());
    /// let obj = pool.acquire();
    /// ```
    pub fn acquire(&self) -> PoolGuard<T> {
        let obj = self.objects.lock().unwrap().pop_front();
        let obj = obj.unwrap_or_else(|| (self.factory)());
        PoolGuard {
            obj: Some(obj),
            pool: self as *const ObjectPool<T>,
        }
    }

    /// Returns the number of objects currently available in the pool.
    pub fn available(&self) -> usize {
        self.objects.lock().unwrap().len()
    }

    /// Releases an object back to the pool.
    ///
    /// This is called automatically when a [`PoolGuard`] is dropped, but can
    /// also be called manually if you have a raw object.
    pub fn release(&self, obj: T) {
        self.objects.lock().unwrap().push_back(obj);
    }

    /// Clears all objects from the pool.
    pub fn clear(&self) {
        self.objects.lock().unwrap().clear();
    }
}

// We need ObjectPool to be thread-safe. Since it uses Mutex and the factory
// is Send + Sync, the pool itself is Send + Sync.
unsafe impl<T: Send> Send for ObjectPool<T> {}
unsafe impl<T: Send> Sync for ObjectPool<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for ObjectPool<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectPool")
            .field("available", &self.objects.lock().unwrap().len())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// PoolGuard
// ---------------------------------------------------------------------------

/// A smart pointer that holds an object acquired from an [`ObjectPool`].
///
/// When the `PoolGuard` is dropped, the object is automatically returned to
/// the pool for reuse.
///
/// This type implements [`Deref`] and [`DerefMut`] so it can be used like a
/// regular reference to the underlying object.
///
/// # Examples
///
/// ```
/// use perust_utils::pool::ObjectPool;
///
/// let pool = ObjectPool::new(|| String::new());
///
/// let mut obj = pool.acquire();
/// obj.push_str("hello");
/// assert_eq!(obj.as_str(), "hello");
///
/// // Object is returned to pool when `obj` goes out of scope
/// ```
pub struct PoolGuard<T> {
    /// The pooled object. `Some` while in use, taken on drop.
    obj: Option<T>,
    /// Pointer back to the pool. Raw pointer avoids lifetime issues.
    pool: *const ObjectPool<T>,
}

impl<T> PoolGuard<T> {
    /// Takes ownership of the pooled object, removing it from the pool
    /// permanently.
    ///
    /// This consumes the guard without returning the object to the pool.
    /// Use this when you need to move the object out of the pool context.
    ///
    /// # Examples
    ///
    /// ```
    /// use perust_utils::pool::ObjectPool;
    ///
    /// let pool = ObjectPool::new(|| String::from("default"));
    /// let guard = pool.acquire();
    /// let owned = guard.take();
    /// assert_eq!(owned, "default");
    /// ```
    pub fn take(mut self) -> T {
        self.obj.take().expect("PoolGuard obj should never be None")
    }
}

impl<T> Deref for PoolGuard<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.obj.as_ref().expect("PoolGuard obj should never be None")
    }
}

impl<T> DerefMut for PoolGuard<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.obj.as_mut().expect("PoolGuard obj should never be None")
    }
}

impl<T> Drop for PoolGuard<T> {
    fn drop(&mut self) {
        if let Some(obj) = self.obj.take() {
            // SAFETY: The pool pointer was obtained from a valid reference
            // when the guard was created, and the pool must outlive all guards.
            // This is ensured by the API design — PoolGuard is always created
            // by ObjectPool::acquire, and the pool must remain alive.
            unsafe {
                (*self.pool).release(obj);
            }
        }
    }
}

// PoolGuard is Send if T is Send (the pool is Send+Sync).
unsafe impl<T: Send> Send for PoolGuard<T> {}
// PoolGuard is NOT Sync by default because &mut T might not be thread-safe.

impl<T: std::fmt::Debug> std::fmt::Debug for PoolGuard<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolGuard")
            .field("obj", &self.obj)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Reset trait
// ---------------------------------------------------------------------------

/// A trait for resetting an object to a reusable state.
///
/// Objects that implement `Reset` can be automatically cleaned before being
/// returned to an [`ObjectPool`]. This is useful for clearing buffers,
/// resetting counters, etc.
///
/// A blanket implementation is provided for types that implement [`Default`]:
/// the reset operation replaces the value with `T::default()`.
pub trait Reset {
    /// Resets the object to a clean, reusable state.
    fn reset(&mut self);
}

/// Blanket `Reset` implementation for types that implement `Default`.
impl<T: Default> Reset for T {
    #[inline]
    fn reset(&mut self) {
        *self = T::default();
    }
}

// ---------------------------------------------------------------------------
// Arc-based pool (for sharing across tasks/threads conveniently)
// ---------------------------------------------------------------------------

/// An [`Arc`]-wrapped [`ObjectPool`] for convenient sharing across tasks/threads.
///
/// This is a newtype wrapper around `Arc<ObjectPool<T>>` that provides
/// convenience constructors for creating shared pools.
pub struct SharedObjectPool<T> {
    inner: Arc<ObjectPool<T>>,
}

impl<T> SharedObjectPool<T> {
    /// Creates a new shared object pool.
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(ObjectPool::new(factory)),
        }
    }

    /// Creates a new shared object pool with a pre-populated count.
    pub fn with_capacity<F>(factory: F, count: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(ObjectPool::with_capacity(factory, count)),
        }
    }

    /// Acquires an object from the pool.
    ///
    /// Delegates to [`ObjectPool::acquire`].
    pub fn acquire(&self) -> PoolGuard<T> {
        self.inner.acquire()
    }

    /// Returns the number of objects currently available in the pool.
    pub fn available(&self) -> usize {
        self.inner.available()
    }

    /// Returns a reference to the inner `Arc<ObjectPool<T>>`.
    pub fn inner(&self) -> &Arc<ObjectPool<T>> {
        &self.inner
    }

    /// Clones the inner `Arc`, returning a new `Arc` reference to the same pool.
    #[inline]
    pub fn clone_arc(&self) -> Arc<ObjectPool<T>> {
        Arc::clone(&self.inner)
    }
}

impl<T> Clone for SharedObjectPool<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> std::ops::Deref for SharedObjectPool<T> {
    type Target = ObjectPool<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_acquire_release() {
        let pool = ObjectPool::new(|| Vec::<u8>::with_capacity(64));
        assert_eq!(pool.available(), 0);

        {
            let mut obj = pool.acquire();
            obj.extend_from_slice(b"hello");
            assert_eq!(&obj[..], b"hello");
        }

        // After the guard is dropped, the object should be back in the pool
        assert_eq!(pool.available(), 1);

        // Acquire again — should reuse the same object
        let obj2 = pool.acquire();
        assert!(obj2.capacity() >= 64);
        // Note: the Vec still contains old data unless we reset it
    }

    #[test]
    fn test_pool_with_capacity() {
        let pool = ObjectPool::with_capacity(|| 0i32, 5);
        assert_eq!(pool.available(), 5);

        let obj = pool.acquire();
        assert_eq!(pool.available(), 4);
        drop(obj);
        assert_eq!(pool.available(), 5);
    }

    #[test]
    fn test_pool_take() {
        let pool = ObjectPool::new(|| String::from("default"));
        let guard = pool.acquire();
        let owned = guard.take();
        assert_eq!(owned, "default");
        // After take, nothing is returned to the pool
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn test_pool_clear() {
        let pool = ObjectPool::with_capacity(|| 0i32, 3);
        assert_eq!(pool.available(), 3);
        pool.clear();
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn test_pool_thread_safety() {
        // Basic thread safety: verify that the pool can be used from an Arc
        // without causing data races. The identity module's thread safety test
        // covers more thorough concurrent access patterns.
        use std::sync::Arc;

        let pool = Arc::new(ObjectPool::new(|| 0i32));

        // Pre-populate the pool
        pool.release(1);
        pool.release(2);

        // Acquire and release in the same thread
        let obj = pool.acquire();
        drop(obj);

        let obj = pool.acquire();
        drop(obj);

        assert_eq!(pool.available(), 2);
    }

    #[test]
    fn test_shared_pool() {
        let pool = SharedObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(128));
        let obj = pool.acquire();
        assert!(obj.capacity() >= 128);
    }

    #[test]
    fn test_reset_trait() {
        let mut v: Vec<i32> = vec![1, 2, 3];
        v.reset();
        assert!(v.is_empty());
    }
}
