//! Thread-safe singleton pattern implementation.
//!
//! This module provides a [`Singleton`] type that ensures at most one instance
//! of a value exists at runtime. Initialization is lazy and thread-safe.
//!
//! # Examples
//!
//! ```
//! use perust_utils::singleton::Singleton;
//! use std::sync::Arc;
//!
//! static CONFIG: Singleton<String> = Singleton::new();
//!
//! // Initialize the singleton (only the first call succeeds)
//! CONFIG.set("server-config".to_string());
//!
//! // Access the value
//! let value = CONFIG.get().unwrap();
//! assert_eq!(value.as_str(), "server-config");
//! ```

use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Singleton
// ---------------------------------------------------------------------------

/// A thread-safe, lazily-initialized singleton container.
///
/// The value can be set once using [`set`](Singleton::set) and then accessed
/// via [`get`](Singleton::get) or [`get_or_init`](Singleton::get_or_init).
///
/// This is built on [`std::sync::OnceLock`] internally, which provides
/// the same guarantees as `std::sync::Once` but with a friendlier API.
///
/// # Thread Safety
///
/// - Initialization is safe to call from multiple threads concurrently.
/// - Only the first call to [`set`](Singleton::set) or
///   [`get_or_init`](Singleton::get_or_init) will succeed; subsequent calls
///   are ignored (for `set`) or return the existing value (for `get_or_init`).
/// - Access via [`get`](Singleton::get) is wait-free once initialized.
///
/// # Examples
///
/// ```
/// use perust_utils::singleton::Singleton;
///
/// static COUNTER: Singleton<u64> = Singleton::new();
///
/// COUNTER.set(42);
/// assert_eq!(*COUNTER.get().unwrap(), 42);
/// ```
pub struct Singleton<T> {
    /// The inner storage. `OnceLock` ensures one-time initialization.
    inner: OnceLock<T>,
    /// A mutex to support `get_or_init_with_mutex` for fallible initialization.
    init_mutex: Mutex<()>,
}

impl<T> Singleton<T> {
    /// Creates a new, uninitialized singleton.
    ///
    /// This is `const` so it can be used in `static` declarations.
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
            init_mutex: Mutex::new(()),
        }
    }

    /// Sets the value of this singleton.
    ///
    /// Returns `Ok(())` if this was the first initialization, or
    /// `Err(value)` if the singleton was already initialized (the
    /// provided value is returned back).
    #[inline]
    pub fn set(&self, value: T) -> Result<(), T> {
        self.inner.set(value)
    }

    /// Returns a reference to the value if it has been initialized.
    ///
    /// Returns `None` if the singleton has not yet been set.
    #[inline]
    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }

    /// Returns a reference to the value, initializing it with `f` if needed.
    ///
    /// If another thread is currently initializing the singleton, this will
    /// block until the initialization is complete.
    ///
    /// This is safe to call concurrently from multiple threads; the closure
    /// `f` will be invoked at most once.
    #[inline]
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.inner.get_or_init(f)
    }

    /// Returns a reference to the value, initializing it with `f` if needed.
    ///
    /// Unlike [`get_or_init`](Self::get_or_init), this method acquires a mutex
    /// before calling `f`, making it safe to perform non-atomic side effects
    /// inside the closure (e.g., checking other state before deciding what to
    /// initialize).
    ///
    /// If the singleton is already initialized, this returns immediately
    /// without calling `f`.
    pub fn get_or_init_with_mutex<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        if let Some(val) = self.inner.get() {
            return val;
        }

        let _guard = self.init_mutex.lock().unwrap();

        // Double-check after acquiring the lock
        self.inner.get_or_init(f)
    }

    /// Returns `true` if the singleton has been initialized.
    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.inner.get().is_some()
    }
}

impl<T: Default> Singleton<T> {
    /// Returns a reference to the value, initializing it with `T::default()`
    /// if needed.
    #[inline]
    pub fn get_or_default(&self) -> &T {
        self.inner.get_or_init(T::default)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Singleton<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Singleton")
            .field("value", &self.inner.get())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_singleton_set_get() {
        static S: Singleton<i32> = Singleton::new();
        assert!(S.get().is_none());
        assert!(S.set(42).is_ok());
        assert_eq!(*S.get().unwrap(), 42);
        // Setting again should fail
        assert!(S.set(99).is_err());
        // Original value should remain
        assert_eq!(*S.get().unwrap(), 42);
    }

    #[test]
    fn test_singleton_get_or_init() {
        static S: Singleton<String> = Singleton::new();
        let val = S.get_or_init(|| "hello".to_string());
        assert_eq!(val.as_str(), "hello");
        // Calling again should return the same value
        let val2 = S.get_or_init(|| "world".to_string());
        assert_eq!(val2.as_str(), "hello");
    }

    #[test]
    fn test_singleton_default() {
        static S: Singleton<Vec<i32>> = Singleton::new();
        let val = S.get_or_default();
        assert!(val.is_empty());
    }

    #[test]
    fn test_singleton_is_initialized() {
        static S: Singleton<i32> = Singleton::new();
        assert!(!S.is_initialized());
        S.set(1).unwrap();
        assert!(S.is_initialized());
    }

    #[test]
    fn test_singleton_thread_safety() {
        static S: Singleton<i32> = Singleton::new();

        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    S.get_or_init(|| i);
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        // Should be initialized exactly once
        assert!(S.get().is_some());
    }
}
