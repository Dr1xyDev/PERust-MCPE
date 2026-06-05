//! Network manager coordinating RakNet transport with MCPE protocol handling.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::RwLock;

use perust_protocol::packet::Packet;
use perust_raknet::{RakNetServer, Reliability};

use crate::error::NetworkError;
use crate::network_session::{NetworkSession, SessionState, reliability_for_packet};
use crate::packet_handler::PacketHandler;

/// The default channel for MCPE game packets.
const GAME_CHANNEL: u8 = 0;

/// The default maximum number of players.
const DEFAULT_MAX_PLAYERS: usize = 20;

/// The network manager that coordinates RakNet and MCPE protocol layers.
///
/// Manages RakNet server lifecycle, session creation/removal, packet
/// routing, and broadcasting.
pub struct NetworkManager {
    /// The underlying RakNet server.
    raknet_server: Arc<RakNetServer>,
    /// Active MCPE sessions keyed by remote address.
    sessions: Arc<DashMap<SocketAddr, NetworkSession>>,
    /// Zlib compression level for batch packets.
    compression_level: u32,
    /// Maximum number of concurrent players.
    max_players: usize,
    /// Whether the network manager is running.
    running: Arc<AtomicBool>,
    /// The packet handler for processing incoming packets.
    packet_handler: Arc<RwLock<PacketHandler>>,
    /// Session ID counter.
    next_session_id: Arc<parking_lot::Mutex<u64>>,
}

impl NetworkManager {
    /// Creates a new network manager bound to the given address and port.
    ///
    /// This creates the underlying RakNet server but does not start it.
    /// Call `start()` to begin processing connections.
    pub fn new(bind_addr: &str, port: u16) -> Self {
        let runtime = tokio::runtime::Handle::current();
        let raknet_server = runtime.block_on(async {
            RakNetServer::new(bind_addr, port)
                .await
                .expect("Failed to create RakNet server")
        });

        Self {
            raknet_server: Arc::new(raknet_server),
            sessions: Arc::new(DashMap::new()),
            compression_level: 6,
            max_players: DEFAULT_MAX_PLAYERS,
            running: Arc::new(AtomicBool::new(false)),
            packet_handler: Arc::new(RwLock::new(PacketHandler::new())),
            next_session_id: Arc::new(parking_lot::Mutex::new(1)),
        }
    }

    /// Creates a new network manager (async version for use within tokio context).
    pub async fn new_async(bind_addr: &str, port: u16) -> Self {
        let raknet_server = RakNetServer::new(bind_addr, port)
            .await
            .expect("Failed to create RakNet server");

        Self {
            raknet_server: Arc::new(raknet_server),
            sessions: Arc::new(DashMap::new()),
            compression_level: 6,
            max_players: DEFAULT_MAX_PLAYERS,
            running: Arc::new(AtomicBool::new(false)),
            packet_handler: Arc::new(RwLock::new(PacketHandler::new())),
            next_session_id: Arc::new(parking_lot::Mutex::new(1)),
        }
    }

    /// Starts the network manager and RakNet server.
    ///
    /// Sets up packet routing from the RakNet layer to the MCPE session handler.
    pub async fn start(&self) -> Result<(), NetworkError> {
        self.running.store(true, Ordering::SeqCst);

        // Set up RakNet callbacks
        let sessions = self.sessions.clone();
        let next_session_id = self.next_session_id.clone();
        let max_players = self.max_players;
        // Session open callback - create a new MCPE session when RakNet connects
        self.raknet_server.set_on_session_open(Box::new(move |addr| {
            log::info!("New RakNet session from {}", addr);

            // Check capacity
            if sessions.len() >= max_players {
                log::warn!("Server is full, rejecting connection from {}", addr);
                return;
            }

            let session_id = {
                let mut guard = next_session_id.lock();
                let id = *guard;
                *guard = id.wrapping_add(1);
                id
            };
            let session = NetworkSession::new(addr, session_id);
            sessions.insert(addr, session);
        }));

        // Session close callback - remove the MCPE session
        let sessions_close = self.sessions.clone();
        self.raknet_server.set_on_session_close(Box::new(move |addr, reason| {
            log::info!("RakNet session {} closed: {}", addr, reason);
            if let Some((_, mut session)) = sessions_close.remove(&addr) {
                session.state = SessionState::Disconnected;
            }
        }));

        // Packet callback - route RakNet packets to MCPE session handler
        let sessions_packet = self.sessions.clone();
        let packet_handler_packet = self.packet_handler.clone();
        let next_sid_packet = self.next_session_id.clone();
        self.raknet_server.set_on_packet(Box::new(move |addr, data| {
            // Get or create session
            if !sessions_packet.contains_key(&addr) {
                // Session might not exist yet if the first packet arrives
                // before the session open callback
                let session_id = {
                    let mut guard = next_sid_packet.lock();
                    let id = *guard;
                    *guard = id.wrapping_add(1);
                    id
                };
                let session = NetworkSession::new(addr, session_id);
                sessions_packet.insert(addr, session);
            }

            // Process the packet through the session
            if let Some(mut session_entry) = sessions_packet.get_mut(&addr) {
                match session_entry.handle_incoming(&data) {
                    Ok(Some(packet_type)) => {
                        let handler = packet_handler_packet.read();
                        handler.handle_packet(addr, packet_type, &data);
                    }
                    Ok(None) => {
                        // Packet not recognized or empty, ignore
                    }
                    Err(e) => {
                        log::warn!(
                            "Error handling packet from {}: {:?}",
                            addr,
                            e
                        );
                    }
                }
            }
        }));

        // Start the RakNet server
        self.raknet_server.start();

        log::info!("Network manager started");
        Ok(())
    }

    /// Stops the network manager and RakNet server.
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        self.raknet_server.stop();

        // Disconnect all sessions
        for mut entry in self.sessions.iter_mut() {
            entry.value_mut().disconnect("Server shutting down");
        }
        self.sessions.clear();

        log::info!("Network manager stopped");
    }

    /// Broadcasts a packet to all connected sessions.
    ///
    /// The packet data should already be encoded with the packet ID prefix.
    pub fn broadcast_packet(&self, packet_data: Vec<u8>) {
        for mut entry in self.sessions.iter_mut() {
            entry.value_mut().queue_packet(packet_data.clone());
        }
    }

    /// Broadcasts a typed packet to all connected sessions.
    pub fn broadcast_typed_packet<P: Packet>(&self, packet: &P) -> Result<(), NetworkError> {
        let data = perust_protocol::packet::encode_packet(packet)?;
        self.broadcast_packet(data);
        Ok(())
    }

    /// Sends a raw packet to a specific session.
    pub fn send_packet_to(&self, address: SocketAddr, packet_data: Vec<u8>) {
        self.raknet_server.send_to(
            address,
            packet_data,
            Reliability::ReliableOrdered,
            GAME_CHANNEL,
        );
    }

    /// Sends a typed packet to a specific session.
    pub fn send_typed_packet_to<P: Packet>(
        &self,
        address: SocketAddr,
        packet: &P,
    ) -> Result<(), NetworkError> {
        let data = perust_protocol::packet::encode_packet(packet)?;
        let reliability = reliability_for_packet(P::PACKET_ID);
        self.raknet_server
            .send_to(address, data, reliability, GAME_CHANNEL);
        Ok(())
    }

    /// Returns the number of active sessions.
    pub fn get_session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Returns `true` if the server is at maximum capacity.
    pub fn is_full(&self) -> bool {
        self.sessions.len() >= self.max_players
    }

    /// Gets the MCPE session for a given address.
    pub fn get_session(&self, address: &SocketAddr) -> Option<dashmap::mapref::one::Ref<'_, SocketAddr, NetworkSession>> {
        self.sessions.get(address)
    }

    /// Updates the MCPE session for a given address.
    pub fn update_session<F, R>(&self, address: &SocketAddr, f: F) -> Option<R>
    where
        F: FnOnce(&mut NetworkSession) -> R,
    {
        let mut entry = self.sessions.get_mut(address)?;
        Some(f(&mut entry))
    }

    /// Removes a session by address.
    pub fn remove_session(&self, address: &SocketAddr) -> Option<NetworkSession> {
        self.sessions.remove(address).map(|(_, v)| v)
    }

    /// Sets the maximum number of players.
    pub fn set_max_players(&mut self, max: usize) {
        self.max_players = max;
    }

    /// Returns the maximum number of players.
    pub fn max_players(&self) -> usize {
        self.max_players
    }

    /// Sets the compression level for batch packets.
    pub fn set_compression_level(&mut self, level: u32) {
        self.compression_level = level.clamp(0, 9);
    }

    /// Sets the MOTD displayed in the server list.
    pub fn set_motd(&self, motd: &str) {
        self.raknet_server.set_motd(motd);
    }

    /// Performs a network tick.
    ///
    /// This should be called at a regular interval (e.g., 50ms for 20 TPS).
    /// Flushes all session send buffers and sends the data through RakNet.
    pub fn tick(&self) {
        // Flush all sessions and send their buffered packets
        for mut entry in self.sessions.iter_mut() {
            let addr = *entry.key();
            if let Some(batch_data) = entry.value_mut().flush() {
                let reliability = Reliability::ReliableOrdered;
                self.raknet_server
                    .send_to(addr, batch_data, reliability, GAME_CHANNEL);
            }
        }
    }

    /// Returns a reference to the packet handler for configuration.
    pub fn packet_handler(&self) -> &Arc<RwLock<PacketHandler>> {
        &self.packet_handler
    }

    /// Returns a reference to the RakNet server.
    pub fn raknet_server(&self) -> &Arc<RakNetServer> {
        &self.raknet_server
    }

    /// Returns all active session addresses.
    pub fn get_session_addresses(&self) -> Vec<SocketAddr> {
        self.sessions.iter().map(|e| *e.key()).collect()
    }
}
