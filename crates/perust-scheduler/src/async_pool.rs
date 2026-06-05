//! Async thread pool for off-thread task execution.
//!
//! The [`AsyncPool`] wraps a Tokio multi-threaded runtime and provides a
//! simple interface for submitting futures to be executed asynchronously.
//! This is used by the scheduler for tasks marked as `is_async`.

use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

/// A Tokio-based thread pool for async task execution.
///
/// Tasks submitted via [`submit`](Self::submit) are spawned on the Tokio
/// runtime and run on the thread pool's worker threads.
pub struct AsyncPool {
    pool: Runtime,
    max_workers: usize,
}

impl AsyncPool {
    /// Creates a new async pool with the given maximum number of worker threads.
    pub fn new(max_workers: usize) -> Self {
        let pool = Runtime::new()
            .expect("Failed to create Tokio runtime for AsyncPool");

        Self { pool, max_workers }
    }

    /// Submits a future to be executed on the thread pool.
    ///
    /// Returns a [`JoinHandle`] that can be used to await the result.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let pool = AsyncPool::new(4);
    /// let handle = pool.submit(async {
    ///     // expensive computation
    ///     42
    /// });
    /// // ... later ...
    /// let result = handle.await.unwrap();
    /// ```
    pub fn submit<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.pool.spawn(future)
    }

    /// Returns the maximum number of worker threads.
    pub fn max_workers(&self) -> usize {
        self.max_workers
    }
}
