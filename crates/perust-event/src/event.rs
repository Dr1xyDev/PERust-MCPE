//! Core event types and traits.
//!
//! Defines the [`Event`] trait, [`EventPriority`] enum, [`CancellableEvent`] helper,
//! and the [`EventHandler`] type alias used throughout the event system.

use std::any::TypeId;

/// Priority levels for event handlers.
///
/// Handlers are invoked in order from `Lowest` to `Monitor`. Lower-priority handlers
/// run first, allowing higher-priority handlers to override or observe the final state.
/// `Monitor` handlers should never modify the event — they exist only to observe the
/// outcome after all other handlers have run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    /// Runs first; useful for setting defaults.
    Lowest = 0,
    /// Runs after `Lowest`.
    Low = 1,
    /// Default priority.
    Normal = 2,
    /// Runs after `Normal`; useful for overriding default behaviour.
    High = 3,
    /// Runs after `High`; useful for final modifications before monitoring.
    Highest = 4,
    /// Runs last; should **not** modify the event.
    Monitor = 5,
}

/// The core trait that all events must implement.
///
/// Every event type in the server must implement this trait so that the
/// [`crate::dispatcher::EventDispatcher`] can route events to the correct handlers.
pub trait Event: Send + Sync + 'static {
    /// Returns a human-readable name for this event (e.g. `"PlayerJoinEvent"`).
    fn event_name(&self) -> &str;

    /// Returns `true` if this event can be cancelled.
    ///
    /// The default implementation returns `false`. Cancellable events should
    /// override this to return `true` and embed a [`CancellableEvent`] field.
    fn is_cancellable(&self) -> bool {
        false
    }

    /// Returns the [`TypeId`] of this event, used for handler lookup.
    fn type_id(&self) -> TypeId
    where
        Self: Sized,
    {
        TypeId::of::<Self>()
    }
}

/// Helper struct for events that can be cancelled.
///
/// Cancellable events should include this struct as a field and delegate
/// `is_cancellable` to return `true`.
///
/// # Example
///
/// ```rust,ignore
/// struct MyEvent {
///     cancel: CancellableEvent,
///     // other fields…
/// }
///
/// impl Event for MyEvent {
///     fn event_name(&self) -> &str { "MyEvent" }
///     fn is_cancellable(&self) -> bool { true }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CancellableEvent {
    /// Whether the event has been cancelled.
    pub cancelled: bool,
}

impl CancellableEvent {
    /// Creates a new `CancellableEvent` that is not cancelled by default.
    pub fn new() -> Self {
        Self { cancelled: false }
    }

    /// Returns `true` if the event has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Sets the cancelled state of the event.
    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}

impl Default for CancellableEvent {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for an event handler callback.
///
/// Handlers receive a mutable reference to the event so that cancellable events
/// can be modified in-place.
pub type EventHandler = Box<dyn Fn(&mut dyn Event) + Send + Sync>;
