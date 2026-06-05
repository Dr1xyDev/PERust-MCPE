//! # perust-event
//!
//! Event system crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **Event**: Core trait for all server events
//! - **EventPriority**: Priority levels for event handlers (Lowest → Monitor)
//! - **CancellableEvent**: Helper struct for events that can be cancelled
//! - **EventDispatcher**: Registry and dispatcher for event handlers
//! - **events**: Concrete event types (Server, Player, Entity, Block, Level, Inventory)

pub mod event;
pub mod dispatcher;
pub mod events;

pub use event::{Event, EventPriority, CancellableEvent, EventHandler};
pub use dispatcher::EventDispatcher;
