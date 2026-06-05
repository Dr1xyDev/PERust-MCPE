//! Tick-based task scheduler.
//!
//! The [`Scheduler`] runs on the server's main tick loop (≈ 20 TPS). Tasks
//! are scheduled with optional delay and repeat period, and the scheduler
//! decides which tasks should fire on each tick.

use std::collections::HashMap;

use crate::task::{Task, TaskHandler};

/// Tick-based task scheduler.
///
/// Each call to [`tick`](Self::tick) advances the internal clock by one tick
/// and returns the tasks that should be executed this tick. The caller is
/// responsible for running the tasks (typically on the main thread, unless
/// the task is async).
///
/// For one-shot tasks the scheduler removes the handler after returning it.
/// For repeating tasks the handler stays in the scheduler with its counter
/// reset — the returned `Box<dyn Task>` is the *same* allocation that lives
/// in the handler, so the caller must **not** store it beyond execution.
pub struct Scheduler {
    /// Active tasks keyed by ID.
    tasks: HashMap<u64, TaskHandler>,
    /// Tasks that have been scheduled but not yet inserted.
    pending: Vec<TaskHandler>,
    /// The next task ID to allocate.
    next_id: u64,
    /// The current server tick.
    current_tick: u64,
}

impl Scheduler {
    /// Creates a new, empty scheduler.
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            pending: Vec::new(),
            next_id: 1,
            current_tick: 0,
        }
    }

    /// Schedules a task to run on the next tick.
    ///
    /// Returns the task's unique ID.
    pub fn schedule_task(&mut self, task: Box<dyn Task>) -> u64 {
        let id = self.allocate_id();
        let handler = TaskHandler::new(id, task);
        self.tasks.insert(id, handler);
        id
    }

    /// Schedules a task to run after the specified delay (in ticks).
    ///
    /// Returns the task's unique ID.
    pub fn schedule_delayed_task(&mut self, task: Box<dyn Task>, delay: u64) -> u64 {
        let id = self.allocate_id();
        let mut handler = TaskHandler::new(id, task);
        handler.delay = delay;
        handler.ticks_remaining = delay;
        self.tasks.insert(id, handler);
        id
    }

    /// Schedules a repeating task that fires every `period` ticks,
    /// starting on the next tick.
    ///
    /// Returns the task's unique ID.
    pub fn schedule_repeating_task(&mut self, task: Box<dyn Task>, period: u64) -> u64 {
        let id = self.allocate_id();
        let mut handler = TaskHandler::new(id, task);
        handler.period = Some(period);
        handler.ticks_remaining = 0;
        self.tasks.insert(id, handler);
        id
    }

    /// Schedules a delayed repeating task.
    ///
    /// The task will first fire after `delay` ticks, then repeat every
    /// `period` ticks.
    ///
    /// Returns the task's unique ID.
    pub fn schedule_delayed_repeating_task(
        &mut self,
        task: Box<dyn Task>,
        delay: u64,
        period: u64,
    ) -> u64 {
        let id = self.allocate_id();
        let mut handler = TaskHandler::new(id, task);
        handler.period = Some(period);
        handler.delay = delay;
        handler.ticks_remaining = delay;
        self.tasks.insert(id, handler);
        id
    }

    /// Cancels a task by ID.
    ///
    /// If the task exists, it is marked as cancelled and will not run again.
    /// One-shot tasks that have already run are simply removed.
    pub fn cancel_task(&mut self, id: u64) {
        if let Some(handler) = self.tasks.get_mut(&id) {
            handler.cancel();
        }
    }

    /// Advances the scheduler by one tick and returns the tasks that should
    /// be executed.
    ///
    /// **Important**: For one-shot tasks the returned `Box<dyn Task>` is taken
    /// out of the scheduler permanently. For repeating tasks the `Box` is
    /// *temporarily* removed — the caller must call [`reinsert_task`](Self::reinsert_task)
    /// after execution so the task can fire again on its next period.
    pub fn tick(&mut self) -> Vec<Box<dyn Task>> {
        self.current_tick += 1;

        // Insert any pending tasks.
        for handler in self.pending.drain(..) {
            self.tasks.insert(handler.id, handler);
        }

        // Identify which task IDs need to run this tick.
        let mut ids_to_run: Vec<u64> = Vec::new();
        let mut ids_to_remove: Vec<u64> = Vec::new();

        for (&id, handler) in self.tasks.iter_mut() {
            if handler.is_cancelled {
                ids_to_remove.push(id);
                continue;
            }

            if handler.ticks_remaining == 0 {
                handler.last_run = Some(self.current_tick);
                ids_to_run.push(id);

                if handler.period.is_none() {
                    // One-shot — mark for removal after extraction.
                    ids_to_remove.push(id);
                }
            } else {
                handler.ticks_remaining -= 1;
            }
        }

        // Remove cancelled and completed one-shot tasks.
        for id in &ids_to_remove {
            self.tasks.remove(id);
        }

        // Extract the tasks that need to run.
        // For one-shot tasks they've already been removed above.
        // For repeating tasks we take the task out temporarily.
        let mut result: Vec<Box<dyn Task>> = Vec::new();
        for id in ids_to_run {
            if let Some(handler) = self.tasks.get_mut(&id) {
                // Repeating task — take the task temporarily.
                // We replace it with a placeholder and the caller must
                // reinsert via `reinsert_task`.
                let taken = std::mem::replace(
                    &mut handler.task,
                    Box::new(PlaceholderTask),
                );
                result.push(taken);
            }
        }

        result
    }

    /// Reinserts a task back into its repeating handler after execution.
    ///
    /// This must be called for each repeating task returned by [`tick`](Self::tick)
    /// after the task has been run. One-shot tasks should **not** be reinserted.
    pub fn reinsert_task(&mut self, id: u64, task: Box<dyn Task>) {
        if let Some(handler) = self.tasks.get_mut(&id) {
            handler.task = task;
            if let Some(period) = handler.period {
                handler.ticks_remaining = period;
            }
        }
    }

    /// Returns `true` if a task with the given ID is currently queued.
    pub fn is_queued(&self, id: u64) -> bool {
        self.tasks.contains_key(&id)
    }

    /// Returns the number of pending (not yet started) tasks.
    pub fn get_pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Returns the current server tick.
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Returns the total number of active tasks.
    pub fn active_task_count(&self) -> usize {
        self.tasks.len()
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn allocate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

/// Internal placeholder task used when temporarily extracting a repeating
/// task from its handler. It does nothing when run.
struct PlaceholderTask;

impl Task for PlaceholderTask {
    fn run(&mut self) {
        // No-op placeholder.
    }

    fn task_name(&self) -> &str {
        "PlaceholderTask"
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
