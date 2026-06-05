//! Session manager that manages all active RakNet sessions.
//!
//! The session manager is responsible for creating, tracking, and removing
//! sessions. It runs a tick loop to handle ACK/NACK processing, recovery,
//! and timeout detection.

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Instant;

use dashmap::DashMap;
use parking_lot::RwLock;

use crate::error::RakNetError;
use crate::session::{Session, SessionState};

/// Callback type invoked when a packet is received from a session.
pub type PacketCallback = Box<dyn Fn(SocketAddr, Vec<u8>) + Send + Sync>;

/// Callback type invoked when a new session is created.
pub type SessionCreateCallback = Box<dyn Fn(SocketAddr) + Send + Sync>;

/// Callback type invoked when a session is closed.
pub type SessionCloseCallback = Box<dyn Fn(SocketAddr, &str) + Send + Sync>;

/// Manages all active RakNet sessions.
pub struct SessionManager {
    /// Active sessions keyed by remote address.
    sessions: DashMap<SocketAddr, Session>,
    /// The server's unique GUID.
    server_guid: Arc<RwLock<i64>>,
    /// Maximum number of concurrent sessions.
    max_sessions: usize,
    /// Blocked IP addresses with their block timestamp.
    blocked_ips: DashMap<IpAddr, Instant>,
    /// The server's MOTD.
    motd: Arc<RwLock<String>>,
    /// Callback invoked when a raw packet is received from a session.
    on_packet_received: Arc<RwLock<Option<PacketCallback>>>,
    /// Callback invoked when a new session is created.
    on_session_created: Arc<RwLock<Option<SessionCreateCallback>>>,
    /// Callback invoked when a session is closed.
    on_session_closed: Arc<RwLock<Option<SessionCloseCallback>>>,
}

impl SessionManager {
    /// Creates a new session manager.
    pub fn new(server_guid: i64, max_sessions: usize) -> Self {
        SessionManager {
            sessions: DashMap::new(),
            server_guid: Arc::new(RwLock::new(server_guid)),
            max_sessions,
            blocked_ips: DashMap::new(),
            motd: Arc::new(RwLock::new(String::new())),
            on_packet_received: Arc::new(RwLock::new(None)),
            on_session_created: Arc::new(RwLock::new(None)),
            on_session_closed: Arc::new(RwLock::new(None)),
        }
    }

    /// Creates a new session manager with default settings (4096 max sessions).
    pub fn with_defaults(server_guid: i64) -> Self {
        Self::new(server_guid, 4096)
    }

    /// Returns the server GUID.
    pub fn server_guid(&self) -> i64 {
        *self.server_guid.read()
    }

    /// Sets the server GUID.
    pub fn set_server_guid(&self, guid: i64) {
        *self.server_guid.write() = guid;
    }

    /// Returns the current MOTD.
    pub fn motd(&self) -> String {
        self.motd.read().clone()
    }

    /// Sets the MOTD.
    pub fn set_motd(&self, motd: String) {
        *self.motd.write() = motd;
    }

    /// Returns the number of active sessions.
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Returns `true` if the server is at maximum capacity.
    pub fn is_full(&self) -> bool {
        self.sessions.len() >= self.max_sessions
    }

    /// Checks if an IP address is blocked.
    pub fn is_ip_blocked(&self, ip: &IpAddr) -> bool {
        self.blocked_ips.contains_key(ip)
    }

    /// Blocks an IP address.
    pub fn block_ip(&self, ip: IpAddr) {
        self.blocked_ips.insert(ip, Instant::now());
    }

    /// Unblocks an IP address.
    pub fn unblock_ip(&self, ip: &IpAddr) {
        self.blocked_ips.remove(ip);
    }

    /// Creates a new session for the given address.
    ///
    /// Returns `Ok(())` if the session was created, or an error if the
    /// server is full, the IP is blocked, or a session already exists.
    pub fn create_session(
        &self,
        address: SocketAddr,
        mtu_size: u16,
        client_guid: i64,
    ) -> Result<(), RakNetError> {
        // Check if IP is blocked
        if self.is_ip_blocked(&address.ip()) {
            return Err(RakNetError::ConnectionRejected(
                "IP address is blocked".to_string(),
            ));
        }

        // Check if server is full
        if self.is_full() {
            return Err(RakNetError::ServerFull(
                "maximum session count reached".to_string(),
            ));
        }

        // Check if session already exists
        if self.sessions.contains_key(&address) {
            return Err(RakNetError::ConnectionRejected(format!(
                "session already exists for {}",
                address
            )));
        }

        let session = Session::new(address, mtu_size, client_guid);
        self.sessions.insert(address, session);

        // Fire session created callback
        if let Some(callback) = self.on_session_created.read().as_ref() {
            callback(address);
        }

        Ok(())
    }

    /// Removes a session by address.
    pub fn remove_session(&self, address: &SocketAddr, reason: &str) {
        if self.sessions.remove(address).is_some() {
            if let Some(callback) = self.on_session_closed.read().as_ref() {
                callback(*address, reason);
            }
        }
    }

    /// Returns `true` if a session exists for the given address.
    pub fn has_session(&self, address: &SocketAddr) -> bool {
        self.sessions.contains_key(address)
    }

    /// Gets a session by address for reading.
    pub fn get_session(&self, address: &SocketAddr) -> Option<crate::session_ref::SessionRef<'_>> {
        self.sessions.get(address).map(|refm| crate::session_ref::SessionRef::new(refm))
    }

    /// Updates a session by address.
    ///
    /// The closure receives a mutable reference to the session.
    pub fn update_session<F, R>(&self, address: &SocketAddr, f: F) -> Option<R>
    where
        F: FnOnce(&mut Session) -> R,
    {
        let mut entry = self.sessions.get_mut(address)?;
        Some(f(&mut entry))
    }

    /// Handles an incoming datagram for an existing session.
    ///
    /// Returns the list of fully received encapsulated packets, or an error.
    pub fn handle_datagram(
        &self,
        address: &SocketAddr,
        data: &[u8],
    ) -> Result<Vec<crate::encapsulated::EncapsulatedPacket>, RakNetError> {
        let mut entry = self
            .sessions
            .get_mut(address)
            .ok_or_else(|| RakNetError::SessionNotFound(*address))?;

        entry.handle_datagram(data)
    }

    /// Sends an encapsulated packet to the session at the given address.
    pub fn send_to_session(
        &self,
        address: &SocketAddr,
        packet: crate::encapsulated::EncapsulatedPacket,
    ) -> Result<(), RakNetError> {
        let mut entry = self
            .sessions
            .get_mut(address)
            .ok_or_else(|| RakNetError::SessionNotFound(*address))?;

        entry.send_encapsulated(packet);
        Ok(())
    }

    /// Sends raw data to the session with the specified reliability and channel.
    pub fn send_data(
        &self,
        address: &SocketAddr,
        data: Vec<u8>,
        reliability: crate::reliability::Reliability,
        channel: u8,
    ) -> Result<(), RakNetError> {
        let mut entry = self
            .sessions
            .get_mut(address)
            .ok_or_else(|| RakNetError::SessionNotFound(*address))?;

        entry.send(data, reliability, channel);
        Ok(())
    }

    /// Performs a tick on all sessions.
    ///
    /// Returns a list of (address, packets) pairs for packets that need to be sent.
    pub fn tick(&self) -> Vec<(SocketAddr, Vec<Vec<u8>>)> {
        let mut results = Vec::new();
        let mut to_remove = Vec::new();

        // First pass: tick sessions and collect outgoing packets / timeout removals
        {
            for mut entry in self.sessions.iter_mut() {
                let address = entry.key().clone();
                let session = entry.value_mut();

                // Check for timed-out sessions
                if session.is_timed_out() && session.state != SessionState::Disconnected {
                    session.mark_disconnected();
                    to_remove.push((address.clone(), "timeout".to_string()));
                    continue;
                }

                // Tick the session to get outgoing packets
                let packets = session.tick();
                if !packets.is_empty() {
                    results.push((address, packets));
                }
            }
        }

        // Remove disconnected sessions (outside the iterator scope)
        for (address, reason) in to_remove {
            self.remove_session(&address, &reason);
        }

        results
    }

    /// Returns a list of all active session addresses.
    pub fn get_addresses(&self) -> Vec<SocketAddr> {
        self.sessions.iter().map(|e| *e.key()).collect()
    }

    /// Sets the callback for when a packet is received from a session.
    pub fn set_on_packet_received(&self, callback: PacketCallback) {
        *self.on_packet_received.write() = Some(callback);
    }

    /// Sets the callback for when a new session is created.
    pub fn set_on_session_created(&self, callback: SessionCreateCallback) {
        *self.on_session_created.write() = Some(callback);
    }

    /// Sets the callback for when a session is closed.
    pub fn set_on_session_closed(&self, callback: SessionCloseCallback) {
        *self.on_session_closed.write() = Some(callback);
    }

    /// Fires the packet received callback.
    pub fn fire_packet_received(&self, address: SocketAddr, data: Vec<u8>) {
        if let Some(callback) = self.on_packet_received.read().as_ref() {
            callback(address, data);
        }
    }

    /// Closes all sessions.
    pub fn close_all(&self) {
        let addresses: Vec<SocketAddr> = self.sessions.iter().map(|e| *e.key()).collect();
        for address in addresses {
            self.remove_session(&address, "server shutdown");
        }
    }
}
