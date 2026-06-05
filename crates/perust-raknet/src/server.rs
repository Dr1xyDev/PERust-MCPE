//! Async UDP RakNet server implementation.
//!
//! The RakNetServer binds a UDP socket and processes incoming RakNet packets,
//! routing unconnected packets (pings, connection requests) and connected
//! packets (datagrams) to the appropriate handlers.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

use crate::error::RakNetError;
use crate::protocol::{
    IncompatibleProtocolVersion, OpenConnectionReply1, OpenConnectionReply2, PacketId,
    RAKNET_PROTOCOL_VERSION,
};
use crate::reliability::Reliability;
use crate::session_manager::SessionManager;

/// The maximum UDP receive buffer size.
const MAX_UDP_PACKET_SIZE: usize = 65535;

/// The tick interval for the session manager (100 TPS = 10ms).
const TICK_INTERVAL: Duration = Duration::from_millis(10);

/// Callback type for when a packet is received from a connected session.
pub type PacketHandler = Box<dyn Fn(SocketAddr, Vec<u8>) + Send + Sync>;

/// Callback type for when a new session is established.
pub type SessionHandler = Box<dyn Fn(SocketAddr) + Send + Sync>;

/// Callback type for when a session is closed.
pub type SessionCloseHandler = Box<dyn Fn(SocketAddr, &str) + Send + Sync>;

/// The async RakNet UDP server.
pub struct RakNetServer {
    /// The UDP socket.
    socket: Arc<UdpSocket>,
    /// The session manager.
    session_manager: Arc<SessionManager>,
    /// The server's unique GUID.
    server_guid: i64,
    /// Whether the server is running.
    running: Arc<AtomicBool>,
    /// The server's MOTD.
    motd: Arc<RwLock<String>>,
    /// Maximum number of players.
    max_players: Arc<RwLock<usize>>,
    /// Handler for game packets received from sessions.
    on_packet: Arc<RwLock<Option<PacketHandler>>>,
    /// Handler for new session connections.
    on_session_open: Arc<RwLock<Option<SessionHandler>>>,
    /// Handler for session closures.
    on_session_close: Arc<RwLock<Option<SessionCloseHandler>>>,
}

impl RakNetServer {
    /// Creates a new RakNet server bound to the given address and port.
    ///
    /// This does not start the server; call `start()` to begin processing.
    pub async fn new(bind_addr: &str, port: u16) -> Result<Self, RakNetError> {
        let addr = format!("{}:{}", bind_addr, port);
        let socket = UdpSocket::bind(&addr).await?;
        let server_guid = rand::random::<i64>();

        let session_manager = Arc::new(SessionManager::new(server_guid, 4096));

        Ok(RakNetServer {
            socket: Arc::new(socket),
            session_manager,
            server_guid,
            running: Arc::new(AtomicBool::new(false)),
            motd: Arc::new(RwLock::new(String::new())),
            max_players: Arc::new(RwLock::new(4096)),
            on_packet: Arc::new(RwLock::new(None)),
            on_session_open: Arc::new(RwLock::new(None)),
            on_session_close: Arc::new(RwLock::new(None)),
        })
    }

    /// Starts the server's main loop.
    ///
    /// Returns a `JoinHandle` for the server task.
    pub fn start(&self) -> JoinHandle<()> {
        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let socket = self.socket.clone();
        let session_manager = self.session_manager.clone();
        let server_guid = self.server_guid;
        let motd = self.motd.clone();
        let max_players = self.max_players.clone();
        let on_packet = self.on_packet.clone();
        let on_session_open = self.on_session_open.clone();
        let on_session_close = self.on_session_close.clone();

        // Set up session manager callbacks that bridge to the server's handlers
        let sm = session_manager.clone();
        sm.set_on_session_created(Box::new(move |addr| {
            if let Some(handler) = on_session_open.read().as_ref() {
                handler(addr);
            }
        }));

        let sm = session_manager.clone();
        sm.set_on_session_closed(Box::new(move |addr, reason| {
            if let Some(handler) = on_session_close.read().as_ref() {
                handler(addr, reason);
            }
        }));

        tokio::spawn(async move {
            let mut buf = [0u8; MAX_UDP_PACKET_SIZE];
            let mut tick_interval = time::interval(TICK_INTERVAL);

            loop {
                if !running.load(Ordering::SeqCst) {
                    break;
                }

                tokio::select! {
                    result = socket.recv_from(&mut buf) => {
                        match result {
                            Ok((len, src_addr)) => {
                                let data = &buf[..len];
                                if let Err(e) = Self::handle_incoming(
                                    &socket,
                                    &session_manager,
                                    server_guid,
                                    &motd,
                                    &max_players,
                                    &on_packet,
                                    src_addr,
                                    data,
                                ).await {
                                    log::warn!("Error handling packet from {}: {:?}", src_addr, e);
                                }
                            }
                            Err(e) => {
                                log::error!("UDP recv error: {:?}", e);
                            }
                        }
                    }
                    _ = tick_interval.tick() => {
                        // Tick the session manager
                        let outgoing = session_manager.tick();
                        for (addr, packets) in outgoing {
                            for packet in packets {
                                if let Err(e) = socket.send_to(&packet, addr).await {
                                    log::debug!("Failed to send to {}: {:?}", addr, e);
                                }
                            }
                        }
                    }
                }
            }

            // Clean up all sessions
            session_manager.close_all();
        })
    }

    /// Stops the server.
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Handles an incoming UDP packet.
    async fn handle_incoming(
        socket: &Arc<UdpSocket>,
        session_manager: &Arc<SessionManager>,
        server_guid: i64,
        motd: &Arc<RwLock<String>>,
        max_players: &Arc<RwLock<usize>>,
        on_packet: &Arc<RwLock<Option<PacketHandler>>>,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        if data.is_empty() {
            return Ok(());
        }

        let packet_id = PacketId::from(data[0]);

        match packet_id {
            // --- Unconnected packets (pre-session) ---
            PacketId::UnconnectedPing | PacketId::UnconnectedPingOpenConnections => {
                Self::handle_unconnected_ping(
                    socket,
                    server_guid,
                    motd,
                    max_players,
                    session_manager,
                    src_addr,
                    data,
                )
                .await
            }

            PacketId::OpenConnectionRequest1 => {
                Self::handle_open_connection_request_1(
                    socket,
                    session_manager,
                    server_guid,
                    src_addr,
                    data,
                )
                .await
            }

            PacketId::OpenConnectionRequest2 => {
                Self::handle_open_connection_request_2(
                    socket,
                    session_manager,
                    server_guid,
                    src_addr,
                    data,
                )
                .await
            }

            // --- Connected packets (datagrams) ---
            PacketId::Datagram | PacketId::Ack | PacketId::Nack => {
                Self::handle_connected_packet(
                    socket,
                    session_manager,
                    on_packet,
                    src_addr,
                    data,
                )
                .await
            }

            // --- Session handshake packets ---
            PacketId::ConnectionRequest => {
                Self::handle_connection_request(
                    socket,
                    session_manager,
                    src_addr,
                    data,
                )
                .await
            }

            PacketId::NewIncomingConnection => {
                Self::handle_new_incoming_connection(
                    session_manager,
                    src_addr,
                    data,
                )
                .await
            }

            PacketId::ConnectedPing => {
                Self::handle_connected_ping(socket, session_manager, src_addr, data).await
            }

            PacketId::DisconnectionNotification => {
                session_manager.remove_session(&src_addr, "client disconnected");
                Ok(())
            }

            _ => {
                log::trace!(
                    "Unhandled packet ID 0x{:02x} from {}",
                    data[0],
                    src_addr
                );
                Ok(())
            }
        }
    }

    /// Handles an UnconnectedPing packet.
    async fn handle_unconnected_ping(
        socket: &Arc<UdpSocket>,
        server_guid: i64,
        motd: &Arc<RwLock<String>>,
        max_players: &Arc<RwLock<usize>>,
        session_manager: &Arc<SessionManager>,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        let ping = crate::protocol::UnconnectedPing::decode(&data[1..])?;

        let motd_str = motd.read().clone();
        let max_p = *max_players.read();
        let online_count = session_manager.session_count();

        // Build Bedrock MOTD format
        // MCPE;ServerName;ProtocolVersion;VersionName;OnlineCount;MaxPlayers;GUID;WorldName;GameMode
        let motd_full = format!(
            "MCPE;{};{};{};{};{};{};Default;Survival",
            motd_str,
            RAKNET_PROTOCOL_VERSION,
            "1.21.0",
            online_count,
            max_p,
            server_guid
        );

        let pong = crate::protocol::UnconnectedPong::new(ping.time, server_guid, motd_full);
        let response = pong.encode();
        socket.send_to(&response, src_addr).await?;

        Ok(())
    }

    /// Handles an OpenConnectionRequest1 packet.
    async fn handle_open_connection_request_1(
        socket: &Arc<UdpSocket>,
        session_manager: &Arc<SessionManager>,
        server_guid: i64,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        let request = crate::protocol::OpenConnectionRequest1::decode(&data[1..])?;

        // Check protocol version
        if request.protocol != RAKNET_PROTOCOL_VERSION {
            let reply = IncompatibleProtocolVersion::new(RAKNET_PROTOCOL_VERSION, server_guid);
            socket.send_to(&reply.encode(), src_addr).await?;
            return Err(RakNetError::ProtocolMismatch {
                expected: RAKNET_PROTOCOL_VERSION,
                actual: request.protocol,
            });
        }

        // Check if server is full
        if session_manager.is_full() {
            return Err(RakNetError::ServerFull(
                "server is at maximum capacity".to_string(),
            ));
        }

        // Negotiate MTU: clamp to [400, 1492]
        let mtu = request.mtu_size.clamp(400, 1492);

        let reply = OpenConnectionReply1::new(server_guid, mtu);
        socket.send_to(&reply.encode(), src_addr).await?;

        Ok(())
    }

    /// Handles an OpenConnectionRequest2 packet.
    async fn handle_open_connection_request_2(
        socket: &Arc<UdpSocket>,
        session_manager: &Arc<SessionManager>,
        server_guid: i64,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        let request = crate::protocol::OpenConnectionRequest2::decode(&data[1..])?;

        // Negotiate MTU
        let mtu = request.mtu_size.clamp(400, 1492);

        // Create the session
        session_manager.create_session(src_addr, mtu, request.client_guid)?;

        let client_address = crate::protocol::SocketAddress::from(src_addr);
        let reply = OpenConnectionReply2::new(server_guid, client_address, mtu);
        socket.send_to(&reply.encode(), src_addr).await?;

        Ok(())
    }

    /// Handles a ConnectionRequest packet (session handshake finalization).
    async fn handle_connection_request(
        socket: &Arc<UdpSocket>,
        session_manager: &Arc<SessionManager>,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        let request = crate::protocol::ConnectionRequest::decode(&data[1..])?;

        let request_time = request.request_time;
        let accepted_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        let client_address = crate::protocol::SocketAddress::from(src_addr);
        let reply = crate::protocol::ConnectionRequestAccepted::new(
            client_address,
            request_time,
            accepted_time,
        );
        socket.send_to(&reply.encode(), src_addr).await?;

        // Update the session's GUID
        session_manager.update_session(&src_addr, |session| {
            session.guid = request.client_guid;
        });

        Ok(())
    }

    /// Handles a NewIncomingConnection packet.
    async fn handle_new_incoming_connection(
        session_manager: &Arc<SessionManager>,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        let _request = crate::protocol::NewIncomingConnection::decode(&data[1..])?;

        // Mark the session as connected
        session_manager.update_session(&src_addr, |session| {
            session.mark_connected();
        });

        log::info!("Session {} is now connected", src_addr);

        Ok(())
    }

    /// Handles a ConnectedPing packet within a session.
    async fn handle_connected_ping(
        socket: &Arc<UdpSocket>,
        session_manager: &Arc<SessionManager>,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        if data.len() < 9 {
            return Err(RakNetError::InvalidPacket(
                "connected ping too short".to_string(),
            ));
        }

        // ConnectedPing format: ID (1) + time (8)
        let ping_time = i64::from_be_bytes([
            data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
        ]);

        // Update session last update time
        session_manager.update_session(&src_addr, |session| {
            session.last_update = std::time::Instant::now();
        });

        // Respond with ConnectedPong
        let mut pong = Vec::with_capacity(1 + 8 + 8);
        pong.push(0x03); // ID_CONNECTED_PONG
        pong.extend_from_slice(&ping_time.to_be_bytes());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        pong.extend_from_slice(&now.to_be_bytes());

        socket.send_to(&pong, src_addr).await?;

        Ok(())
    }

    /// Handles connected packets (datagrams, ACK, NACK) for existing sessions.
    async fn handle_connected_packet(
        socket: &Arc<UdpSocket>,
        session_manager: &Arc<SessionManager>,
        on_packet: &Arc<RwLock<Option<PacketHandler>>>,
        src_addr: SocketAddr,
        data: &[u8],
    ) -> Result<(), RakNetError> {
        let packets = session_manager.handle_datagram(&src_addr, data)?;

        // Forward received packets to the game handler
        for packet in packets {
            if let Some(handler) = on_packet.read().as_ref() {
                handler(src_addr, packet.buffer.clone());
            }
        }

        // After processing, immediately send any ACK/NACK responses
        let outgoing = session_manager.tick();
        for (addr, packets) in outgoing {
            for packet in packets {
                if let Err(e) = socket.send_to(&packet, addr).await {
                    log::debug!("Failed to send to {}: {:?}", addr, e);
                }
            }
        }

        Ok(())
    }

    /// Sends raw data to a specific session.
    pub fn send_to(&self, address: SocketAddr, data: Vec<u8>, reliability: Reliability, channel: u8) {
        if let Err(e) = self.session_manager.send_data(&address, data, reliability, channel) {
            log::debug!("Failed to queue data for {}: {:?}", address, e);
        }
    }

    /// Sets the server's MOTD.
    pub fn set_motd(&self, motd: &str) {
        *self.motd.write() = motd.to_string();
    }

    /// Sets the maximum number of players.
    pub fn set_max_players(&self, max: usize) {
        *self.max_players.write() = max;
    }

    /// Sets the callback for game packets received from sessions.
    pub fn set_on_packet(&self, handler: PacketHandler) {
        *self.on_packet.write() = Some(handler);
    }

    /// Sets the callback for new session connections.
    pub fn set_on_session_open(&self, handler: SessionHandler) {
        *self.on_session_open.write() = Some(handler);
    }

    /// Sets the callback for session closures.
    pub fn set_on_session_close(&self, handler: SessionCloseHandler) {
        *self.on_session_close.write() = Some(handler);
    }

    /// Returns a reference to the session manager.
    pub fn session_manager(&self) -> &Arc<SessionManager> {
        &self.session_manager
    }

    /// Returns the server's GUID.
    pub fn server_guid(&self) -> i64 {
        self.server_guid
    }

    /// Returns the local address the server is bound to.
    pub fn local_addr(&self) -> Result<SocketAddr, RakNetError> {
        self.socket.local_addr().map_err(RakNetError::Io)
    }

    /// Returns the number of currently connected sessions.
    pub fn session_count(&self) -> usize {
        self.session_manager.session_count()
    }
}
