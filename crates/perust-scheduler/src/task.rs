//! Task trait and handler definitions.
//!
//! A [`Task`] is a unit of work that can be scheduled on the server's main
//! tick loop. The [`TaskHandler`] wraps a task with scheduling metadata such
//! as delay, period, and cancellation state.

/// The core trait for schedulable tasks.
///
/// Implementors provide the `run` method which contains the work to be
/// performed. Tasks run on the server's main thread unless marked as async
/// (in which case they are dispatched to the [`crate::async_pool::AsyncPool`]).
pub trait Task: Send + Sync {
    /// Executes the task.
    fn run(&mut self);

    /// Returns a human-readable name for this task (useful for debugging).
    fn task_name(&self) -> &str {
        "UnnamedTask"
    }
}

/// A closure-based task that wraps a simple function.
pub struct FnTask {
    /// The task name.
    pub name: String,
    /// The closure to execute.
    pub func: Box<dyn FnMut() + Send + Sync>,
}

impl FnTask {
    /// Creates a new `FnTask` from a closure.
    pub fn new<F: FnMut() + Send + Sync + 'static>(func: F) -> Self {
        Self {
            name: "FnTask".to_string(),
            func: Box::new(func),
        }
    }

    /// Sets the task name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

impl Task for FnTask {
    fn run(&mut self) {
        (self.func)();
    }

    fn task_name(&self) -> &str {
        &self.name
    }
}

/// Wraps a [`Task`] with scheduling metadata.
///
/// The scheduler stores `TaskHandler`s and uses the metadata to determine
/// when a task should run and whether it should repeat.
pub struct TaskHandler {
    /// Unique identifier for this task.
    pub id: u64,
    /// The actual task to execute.
    pub task: Box<dyn Task>,
    /// Repeat period in ticks. `None` means the task is one-shot.
    pub period: Option<u64>,
    /// Delay in ticks before the first run.
    pub delay: u64,
    /// Ticks remaining before this task fires next.
    pub ticks_remaining: u64,
    /// Whether this task has been cancelled.
    pub is_cancelled: bool,
    /// Whether this task should run on the async thread pool.
    pub is_async: bool,
    /// The tick at which this task was last run, if ever.
    pub last_run: Option<u64>,
}

impl TaskHandler {
    /// Creates a new one-shot task handler with no delay.
    pub fn new(id: u64, task: Box<dyn Task>) -> Self {
        Self {
            id,
            task,
            period: None,
            delay: 0,
            ticks_remaining: 0,
            is_cancelled: false,
            is_async: false,
            last_run: None,
        }
    }

    /// Returns `true` if this is a repeating task.
    pub fn is_repeating(&self) -> bool {
        self.period.is_some()
    }

    /// Cancels this task.
    pub fn cancel(&mut self) {
        self.is_cancelled = true;
    }
}
