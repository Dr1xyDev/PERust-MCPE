//! Event dispatcher — registry and invocation of event handlers.
//!
//! The [`EventDispatcher`] maintains a mapping from event types to ordered lists of
//! handlers. When an event is dispatched, handlers are invoked in priority order
//! (Lowest → Monitor).

use std::any::TypeId;
use std::collections::HashMap;

use crate::event::{Event, EventHandler, EventPriority};

/// Central registry and dispatcher for event handlers.
///
/// # Ordering
///
/// Handlers for the same event type are stored sorted by priority. When
/// [`dispatch`](Self::dispatch) is called, handlers fire from `Lowest` to `Monitor`.
pub struct EventDispatcher {
    handlers: HashMap<TypeId, Vec<(EventPriority, EventHandler)>>,
}

impl EventDispatcher {
    /// Creates a new, empty `EventDispatcher`.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a handler for event type `E` with the given priority.
    ///
    /// The handler list for `E` is re-sorted by priority after insertion so that
    /// dispatch order is always correct regardless of registration order.
    pub fn register<E: Event>(&mut self, priority: EventPriority, handler: EventHandler) {
        let type_id = TypeId::of::<E>();
        let list = self.handlers.entry(type_id).or_default();
        list.push((priority, handler));
        list.sort_by_key(|(p, _)| *p);
    }

    /// Dispatches an event, invoking all registered handlers in priority order.
    ///
    /// Handlers receive a `&mut` reference to the event, allowing them to modify
    /// cancellable events.
    pub fn dispatch<E: Event>(&self, event: &mut E) {
        let type_id = TypeId::of::<E>();
        if let Some(handlers) = self.handlers.get(&type_id) {
            for (_priority, handler) in handlers {
                handler(event);
            }
        }
    }

    /// Removes all registered handlers.
    pub fn clear(&mut self) {
        self.handlers.clear();
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
