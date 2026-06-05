//! Session reference type for safe access to session data.

use dashmap::mapref::one::Ref;

use crate::session::Session;
use std::net::SocketAddr;

/// A reference to a session in the session manager.
///
/// This wraps a DashMap reference to provide safe access to session data
/// without holding a lock on the entire session map.
pub struct SessionRef<'a> {
    inner: Ref<'a, SocketAddr, Session>,
}

impl<'a> SessionRef<'a> {
    /// Creates a new session reference from a DashMap ref.
    pub fn new(refm: Ref<'a, SocketAddr, Session>) -> Self {
        SessionRef { inner: refm }
    }

    /// Returns a reference to the session.
    pub fn session(&self) -> &Session {
        self.inner.value()
    }

    /// Returns the session's remote address.
    pub fn address(&self) -> SocketAddr {
        *self.inner.key()
    }
}
