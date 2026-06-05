//! Core Server struct that ties all PeRust subsystems together.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use uuid::Uuid;

use perust_command::CommandDispatcher;
use perust_config::{BanList, OpsList, ServerProperties, Whitelist};
use perust_console::Console;
use perust_entity::EntityManager;
use perust_event::EventDispatcher;
use perust_network::NetworkManager;
use perust_player::{Player, PlayerList};
use perust_plugin::PluginManager;
use perust_world::World;

use crate::handlers;
use crate::tick;

/// The core Server struct that ties everything together.
///
/// Holds all subsystems, shared state, and provides the main API for
/// server operations like broadcasting messages, managing players, etc.
pub struct Server {
    // === Configuration ===
    /// Main server configuration (server.properties).
    pub config: Arc<RwLock<ServerProperties>>,
    /// Server operator list.
    pub ops: Arc<Mutex<OpsList>>,
    /// Player whitelist.
    pub whitelist: Arc<Mutex<Whitelist>>,
    /// Player/IP/CID ban list.
    pub ban_list: Arc<Mutex<BanList>>,

    // === Core systems ===
    /// Network manager handling RakNet + MCPE protocol.
    pub network_manager: Arc<NetworkManager>,
    /// Tick-based task scheduler.
    pub scheduler: Arc<Mutex<perust_scheduler::Scheduler>>,
    /// Command registry and dispatcher.
    pub command_dispatcher: Arc<Mutex<CommandDispatcher>>,
    /// Event dispatcher for plugin events.
    pub event_dispatcher: Arc<Mutex<EventDispatcher>>,
    /// Plugin lifecycle manager.
    pub plugin_manager: Arc<Mutex<PluginManager>>,
    /// Console input reader.
    pub console: Console,
    /// Entity ID allocator and manager.
    pub entity_manager: Arc<EntityManager>,

    // === World management ===
    /// All loaded worlds keyed by name.
    pub worlds: HashMap<String, Arc<Mutex<World>>>,
    /// The default (overworld) world.
    pub default_world: Arc<Mutex<World>>,

    // === Player management ===
    /// All online players keyed by UUID.
    pub players: Arc<DashMap<Uuid, Player>>,
    /// Mapping from network address to player UUID (for packet routing).
    pub address_to_uuid: Arc<DashMap<SocketAddr, Uuid>>,
    /// Thread-safe player list (tab list).
    pub player_list: Arc<PlayerList>,

    // === State ===
    /// Whether the server is running.
    pub running: Arc<AtomicBool>,
    /// Monotonically increasing tick counter.
    pub tick_counter: u64,
    /// Time the server was started.
    pub start_time: Instant,
    /// Current ticks per second.
    pub tps: f64,
}

impl Server {
    /// Creates a new Server instance with the given configuration.
    ///
    /// This initializes all subsystems but does not start them.
    /// Call `start()` to begin accepting connections.
    pub async fn new(config: ServerProperties) -> Self {
        let bind_addr = if config.server_ip.is_empty() {
            "0.0.0.0"
        } else {
            &config.server_ip
        };
        let port = config.server_port;

        // Create the network manager
        let network_manager = Arc::new(
            NetworkManager::new_async(bind_addr, port).await,
        );

        // Create the default world
        let level_name = config.level_name.clone();
        let level_type = config.level_type.clone();
        let data_dir = std::path::PathBuf::from("data");
        let world_folder = data_dir.join("worlds").join(&level_name);

        let default_world = match level_type.as_str() {
            "FLAT" => Arc::new(Mutex::new(World::flat(level_name.clone(), world_folder))),
            "VOID" => Arc::new(Mutex::new(World::void(level_name.clone(), world_folder))),
            _ => {
                let seed = chrono::Utc::now().timestamp_millis() as i64;
                Arc::new(Mutex::new(World::normal(level_name.clone(), world_folder, seed)))
            }
        };

        let mut worlds = HashMap::new();
        worlds.insert(level_name.clone(), Arc::clone(&default_world));

        // Load configuration files
        let config_arc = Arc::new(RwLock::new(config));
        let data_path = std::path::Path::new("data");

        let ops = Arc::new(Mutex::new(
            OpsList::load(&data_path.join("ops.json")).unwrap_or_default(),
        ));
        let whitelist = Arc::new(Mutex::new(
            Whitelist::load(&data_path.join("whitelist.json")).unwrap_or_default(),
        ));
        let ban_list = Arc::new(Mutex::new(
            BanList::load(data_path).unwrap_or_default(),
        ));

        let scheduler = Arc::new(Mutex::new(perust_scheduler::Scheduler::new()));
        let command_dispatcher = Arc::new(Mutex::new(CommandDispatcher::new()));
        let event_dispatcher = Arc::new(Mutex::new(EventDispatcher::new()));
        let plugin_manager = Arc::new(Mutex::new(PluginManager::new()));
        let entity_manager = Arc::new(EntityManager::new());

        let players: Arc<DashMap<Uuid, Player>> = Arc::new(DashMap::new());
        let address_to_uuid: Arc<DashMap<SocketAddr, Uuid>> = Arc::new(DashMap::new());
        let player_list = Arc::new(PlayerList::new());

        let running = Arc::new(AtomicBool::new(true));
        let console = Console::new();

        let server = Self {
            config: config_arc,
            ops,
            whitelist,
            ban_list,
            network_manager,
            scheduler,
            command_dispatcher,
            event_dispatcher,
            plugin_manager,
            console,
            entity_manager,
            worlds,
            default_world,
            players,
            address_to_uuid,
            player_list,
            running,
            tick_counter: 0,
            start_time: Instant::now(),
            tps: 20.0,
        };

        // Register default commands
        {
            let mut dispatcher = server.command_dispatcher.lock();
            perust_command::defaults::register_defaults(&mut dispatcher);
        }

        server
    }

    /// Starts all server subsystems.
    ///
    /// This loads worlds, sets up network callbacks, starts the network
    /// manager, loads plugins, and begins accepting connections.
    pub async fn start(&mut self) {
        log::info!("Starting PeRust server...");

        // Load the default world
        {
            let mut world = self.default_world.lock();
            if let Err(e) = world.load() {
                log::error!("Failed to load default world: {:?}", e);
            }
        }

        // Set MOTD on the network manager
        {
            let config = self.config.read();
            self.network_manager.set_motd(&config.motd);
            // Note: set_max_players requires &mut self on NetworkManager,
            // which is not possible through Arc. The RakNet server
            // defaults to a high capacity which is sufficient.
        }

        // Set up packet handler callbacks
        self.setup_packet_handlers();

        // Start the network manager
        if let Err(e) = self.network_manager.start().await {
            log::error!("Failed to start network manager: {:?}", e);
            self.running.store(false, Ordering::SeqCst);
            return;
        }

        // Load plugins
        {
            let mut plugin_manager = self.plugin_manager.lock();
            let plugins_dir = std::path::Path::new("data/plugins");
            if let Ok(loaded) = plugin_manager.load_plugins(plugins_dir) {
                log::info!("Loaded {} plugin(s)", loaded.len());
                plugin_manager.enable_all();
            }
        }

        // Start the console reader
        self.console.start();

        // Fire server start event
        {
            let event_dispatcher = self.event_dispatcher.lock();
            let mut event = perust_event::events::ServerStartEvent;
            event_dispatcher.dispatch(&mut event);
        }

        let config = self.config.read();
        log::info!(
            "PeRust server started on {}:{} [{}]",
            if config.server_ip.is_empty() { "0.0.0.0" } else { &config.server_ip },
            config.server_port,
            config.level_name,
        );
        log::info!(
            "Game mode: {} | Difficulty: {} | Max players: {}",
            config.gamemode_name(),
            config.difficulty_name(),
            config.max_players,
        );
    }

    /// Sets up the packet handler callbacks on the network manager.
    fn setup_packet_handlers(&self) {
        let packet_handler = self.network_manager.packet_handler();
        let mut handler = packet_handler.write();

        // Capture Arc clones for use in callbacks
        let players = Arc::clone(&self.players);
        let address_to_uuid = Arc::clone(&self.address_to_uuid);
        let player_list = Arc::clone(&self.player_list);
        let config = Arc::clone(&self.config);
        let ops = Arc::clone(&self.ops);
        let whitelist = Arc::clone(&self.whitelist);
        let ban_list = Arc::clone(&self.ban_list);
        let network_manager = Arc::clone(&self.network_manager);
        let default_world = Arc::clone(&self.default_world);
        let event_dispatcher = Arc::clone(&self.event_dispatcher);
        let entity_manager = Arc::clone(&self.entity_manager);
        let running = Arc::clone(&self.running);

        // Login callback
        handler.on_login(Box::new(move |address, result| {
            handlers::handle_login(
                address,
                result,
                &players,
                &address_to_uuid,
                &player_list,
                &config,
                &ops,
                &whitelist,
                &ban_list,
                &network_manager,
                &default_world,
                &event_dispatcher,
                &entity_manager,
                &running,
            );
        }));

        // Move callback
        let players_move = Arc::clone(&self.players);
        let address_to_uuid_move = Arc::clone(&self.address_to_uuid);
        handler.on_move(Box::new(move |address, x, y, z, yaw, pitch, body_yaw, on_ground| {
            handlers::handle_move_player(
                address, x, y, z, yaw, pitch, body_yaw, on_ground,
                &players_move, &address_to_uuid_move,
            );
        }));

        // Player action callback
        let players_action = Arc::clone(&self.players);
        let address_to_uuid_action = Arc::clone(&self.address_to_uuid);
        handler.on_player_action(Box::new(move |address, action| {
            handlers::handle_player_action(address, action, &players_action, &address_to_uuid_action);
        }));

        // Text (chat) callback
        let players_text = Arc::clone(&self.players);
        let address_to_uuid_text = Arc::clone(&self.address_to_uuid);
        let network_manager_text = Arc::clone(&self.network_manager);
        let event_dispatcher_text = Arc::clone(&self.event_dispatcher);
        let command_dispatcher_text = Arc::clone(&self.command_dispatcher);
        handler.on_text(Box::new(move |address, source, message| {
            handlers::handle_text(
                address, source, message,
                &players_text, &address_to_uuid_text,
                &network_manager_text, &event_dispatcher_text,
                &command_dispatcher_text,
            );
        }));

        // Inventory transaction callback
        let players_inv = Arc::clone(&self.players);
        let address_to_uuid_inv = Arc::clone(&self.address_to_uuid);
        handler.on_inventory_transaction(Box::new(move |address, data| {
            handlers::handle_inventory_transaction(address, data, &players_inv, &address_to_uuid_inv);
        }));

        // Chunk radius request callback
        let players_chunk = Arc::clone(&self.players);
        let address_to_uuid_chunk = Arc::clone(&self.address_to_uuid);
        let default_world_chunk = Arc::clone(&self.default_world);
        let network_manager_chunk = Arc::clone(&self.network_manager);
        let config_chunk = Arc::clone(&self.config);
        handler.on_chunk_radius_request(Box::new(move |address, radius| {
            handlers::handle_request_chunk_radius(
                address, radius,
                &players_chunk, &address_to_uuid_chunk,
                &default_world_chunk, &network_manager_chunk, &config_chunk,
            );
        }));

        // Resource pack response callback
        let players_rp = Arc::clone(&self.players);
        let address_to_uuid_rp = Arc::clone(&self.address_to_uuid);
        let network_manager_rp = Arc::clone(&self.network_manager);
        let default_world_rp = Arc::clone(&self.default_world);
        let player_list_rp = Arc::clone(&self.player_list);
        let config_rp = Arc::clone(&self.config);
        let entity_manager_rp = Arc::clone(&self.entity_manager);
        handler.on_resource_pack_response(Box::new(move |address, status| {
            handlers::handle_resource_pack_response(
                address, status,
                &players_rp, &address_to_uuid_rp,
                &network_manager_rp, &default_world_rp,
                &player_list_rp, &config_rp, &entity_manager_rp,
            );
        }));

        // Container close callback
        let players_cc = Arc::clone(&self.players);
        let address_to_uuid_cc = Arc::clone(&self.address_to_uuid);
        handler.on_container_close(Box::new(move |address, window_id| {
            handlers::handle_container_close(address, window_id, &players_cc, &address_to_uuid_cc);
        }));
    }

    /// Stops the server gracefully.
    ///
    /// Kicks all players, saves worlds, disables plugins, and shuts down
    /// the network manager.
    pub fn stop(&mut self) {
        log::info!("Stopping PeRust server...");

        // Fire server stop event
        {
            let event_dispatcher = self.event_dispatcher.lock();
            let mut event = perust_event::events::ServerStopEvent;
            event_dispatcher.dispatch(&mut event);
        }

        // Disable plugins
        {
            let mut plugin_manager = self.plugin_manager.lock();
            plugin_manager.disable_all();
        }

        // Save and unload worlds
        for (name, world) in &self.worlds {
            let world = world.lock();
            if let Err(e) = world.save() {
                log::error!("Failed to save world '{}': {:?}", name, e);
            }
        }
        log::info!("Worlds saved");

        // Save configuration
        {
            let data_path = std::path::Path::new("data");
            let config = self.config.read();
            if let Err(e) = config.save(&data_path.join("server.properties")) {
                log::error!("Failed to save server.properties: {:?}", e);
            }
            let ops = self.ops.lock();
            if let Err(e) = ops.save(&data_path.join("ops.json")) {
                log::error!("Failed to save ops.json: {:?}", e);
            }
            let whitelist = self.whitelist.lock();
            if let Err(e) = whitelist.save(&data_path.join("whitelist.json")) {
                log::error!("Failed to save whitelist.json: {:?}", e);
            }
            let ban_list = self.ban_list.lock();
            if let Err(e) = ban_list.save(data_path) {
                log::error!("Failed to save ban lists: {:?}", e);
            }
        }

        // Disconnect all players
        self.players.clear();
        self.address_to_uuid.clear();
        self.player_list.clear();

        // Stop the network manager
        self.network_manager.stop();

        // Stop the console reader
        self.console.stop();

        self.running.store(false, Ordering::SeqCst);
        log::info!("PeRust server stopped");
    }

    /// Performs one tick of the server loop.
    ///
    /// Called at ~20 TPS from the main tick loop.
    pub fn tick(&mut self) {
        self.tick_counter += 1;

        // Network tick — flush session buffers
        self.network_manager.tick();

        // Process scheduled tasks
        tick::update_scheduler(&self.scheduler);

        // Update worlds
        tick::update_worlds(&self.worlds);

        // Update players (send chunks, sync positions)
        tick::update_players(
            &self.players,
            &self.address_to_uuid,
            &self.default_world,
            &self.network_manager,
            &self.config,
        );

        // Process console input
        tick::process_console_input(
            &mut self.console,
            &self.command_dispatcher,
            &self.running,
            &self.players,
            &self.network_manager,
            &self.config,
            &self.ops,
            &self.whitelist,
            &self.ban_list,
        );

        // Calculate TPS
        tick::calculate_tps(self);

        // Auto-save every 6000 ticks (5 minutes at 20 TPS)
        if self.tick_counter % 6000 == 0 {
            let config = self.config.read();
            if config.auto_save {
                drop(config);
                self.save_all();
            }
        }
    }

    /// Broadcasts a raw system message to all connected players.
    pub fn broadcast_message(&self, message: &str) {
        let text_packet = perust_protocol::packets::TextPacket {
            text_type: perust_protocol::types::TextPacketType::Raw,
            source: String::new(),
            message: message.to_string(),
            parameters: Vec::new(),
            xuid: String::new(),
            platform_chat_id: String::new(),
        };

        if let Err(e) = self.network_manager.broadcast_typed_packet(&text_packet) {
            log::debug!("Failed to broadcast message: {:?}", e);
        }
    }

    /// Broadcasts a chat message from a specific player.
    pub fn broadcast_chat(&self, from: &str, message: &str) {
        let text_packet = perust_protocol::packets::TextPacket {
            text_type: perust_protocol::types::TextPacketType::Chat,
            source: from.to_string(),
            message: message.to_string(),
            parameters: Vec::new(),
            xuid: String::new(),
            platform_chat_id: String::new(),
        };

        if let Err(e) = self.network_manager.broadcast_typed_packet(&text_packet) {
            log::debug!("Failed to broadcast chat: {:?}", e);
        }
    }

    /// Returns a list of references to all online players.
    pub fn get_online_players(&self) -> Vec<dashmap::mapref::multiple::RefMulti<'_, Uuid, Player>> {
        self.players.iter().collect()
    }

    /// Finds an online player by username (case-insensitive).
    pub fn get_player_by_name(&self, name: &str) -> Option<dashmap::mapref::one::Ref<'_, Uuid, Player>> {
        let uuid = self.players.iter().find(|entry| {
            entry.value().username.eq_ignore_ascii_case(name)
        }).map(|entry| *entry.key());
        uuid.and_then(|u| self.players.get(&u))
    }

    /// Saves all worlds and configuration.
    pub fn save_all(&self) {
        log::info!("Auto-saving...");

        for (name, world) in &self.worlds {
            let world = world.lock();
            if let Err(e) = world.save() {
                log::error!("Failed to save world '{}': {:?}", name, e);
            }
        }

        let data_path = std::path::Path::new("data");
        let config = self.config.read();
        let _ = config.save(&data_path.join("server.properties"));
        let ops = self.ops.lock();
        let _ = ops.save(&data_path.join("ops.json"));

        log::info!("Auto-save complete");
    }

    /// Handles an incoming packet from the network layer.
    ///
    /// Routes the packet to the appropriate handler based on its type.
    pub fn handle_packet(&mut self, address: SocketAddr, packet_type: perust_network::PacketType) {
        log::trace!("Handling packet {:?} from {}", packet_type, address);
        // Packet handling is done via callbacks set up in setup_packet_handlers()
    }
}
