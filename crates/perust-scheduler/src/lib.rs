//! # perust-scheduler
//!
//! Task scheduler crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **Task**: Trait for schedulable units of work
//! - **TaskHandler**: Metadata wrapper for scheduled tasks (delay, period, cancellation)
//! - **Scheduler**: Tick-based task scheduler (main-thread and delayed/repeating tasks)
//! - **AsyncPool**: Tokio-based thread pool for async (off-thread) tasks

pub mod task;
pub mod scheduler;
pub mod async_pool;

pub use task::{Task, TaskHandler};
pub use scheduler::Scheduler;
pub use async_pool::AsyncPool;
