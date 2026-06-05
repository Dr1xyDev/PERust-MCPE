//! Packet handler implementations for the PeRust server.
//!
//! Each handler function processes a specific packet type received from
//! a client. The handlers are called from the network manager's packet
//! handler callbacks and operate on shared server state.

use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use uuid::Uuid;

use perust_command::{CommandDispatcher, CommandSender};
use perust_config::{BanList, OpsList, ServerProperties, Whitelist};
use perust_entity::EntityManager;
use perust_event::EventDispatcher;
use perust_network::{LoginResult, NetworkManager};
use perust_player::{LoginState, Player, PlayerList};
use perust_protocol::packets::{
    ChunkRadiusUpdatedPacket, PlayStatusPacket, PlayerListPacket,
    ResourcePacksInfoPacket, ResourcePackStackPacket,
    StartGamePacket,
};
use perust_protocol::protocol_info;
use perust_protocol::types::{
    GameMode, GameRule, PlayStatus, PlayerListEntry as ProtocolPlayerListEntry, SkinData,
};
use perust_world::World;

/// Handles a login packet from a client.
///
/// This is the entry point for the login flow:
/// 1. Validate protocol version
/// 2. Check ban list, whitelist, player count
/// 3. Fire PlayerLoginEvent
/// 4. Send PlayStatusPacket (LOGIN_SUCCESS)
/// 5. Send ResourcePacksInfoPacket
/// 6. After resource pack response → send StartGamePacket, chunks, etc.
#[allow(clippy::too_many_arguments)]
pub fn handle_login(
    address: SocketAddr,
    result: LoginResult,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
    player_list: &Arc<PlayerList>,
    config: &Arc<RwLock<ServerProperties>>,
    ops: &Arc<Mutex<OpsList>>,
    whitelist: &Arc<Mutex<Whitelist>>,
    ban_list: &Arc<Mutex<BanList>>,
    network_manager: &Arc<NetworkManager>,
    default_world: &Arc<Mutex<World>>,
    event_dispatcher: &Arc<Mutex<EventDispatcher>>,
    entity_manager: &Arc<EntityManager>,
    running: &Arc<AtomicBool>,
) {
    let cfg = config.read();

    // 1. Validate protocol version
    let accepted = protocol_info::ACCEPTED_PROTOCOLS.contains(&result.protocol);
    if !accepted {
        log::warn!(
            "Player {} has unsupported protocol {} (expected {})",
            result.username,
            result.protocol,
            protocol_info::CURRENT_PROTOCOL,
        );
        let status_packet = PlayStatusPacket {
            status: PlayStatus::LoginFailedClient,
        };
        let _ = network_manager.send_typed_packet_to(address, &status_packet);
        return;
    }

    // 2. Check ban list
    {
        let bans = ban_list.lock();
        if bans.is_banned(&result.username) {
            log::info!("Rejected login from banned player: {}", result.username);
            let disconnect = perust_protocol::packets::DisconnectPacket {
                message: "You have been banned from this server.".to_string(),
            };
            let _ = network_manager.send_typed_packet_to(address, &disconnect);
            return;
        }
        if bans.is_ip_banned(&address.ip().to_string()) {
            log::info!("Rejected login from banned IP: {}", address.ip());
            let disconnect = perust_protocol::packets::DisconnectPacket {
                message: "Your IP address has been banned.".to_string(),
            };
            let _ = network_manager.send_typed_packet_to(address, &disconnect);
            return;
        }
    }

    // 3. Check whitelist
    {
        let wl = whitelist.lock();
        if cfg.white_list && !wl.is_whitelisted(&result.username) {
            log::info!("Rejected login from non-whitelisted player: {}", result.username);
            let disconnect = perust_protocol::packets::DisconnectPacket {
                message: "You are not whitelisted on this server.".to_string(),
            };
            let _ = network_manager.send_typed_packet_to(address, &disconnect);
            return;
        }
    }

    // 4. Check player count
    if players.len() >= cfg.max_players as usize {
        log::info!("Rejected login from {}: server is full", result.username);
        let disconnect = perust_protocol::packets::DisconnectPacket {
            message: "The server is full.".to_string(),
        };
        let _ = network_manager.send_typed_packet_to(address, &disconnect);
        return;
    }

    // 5. Fire PlayerLoginEvent
    {
        let mut ed = event_dispatcher.lock();
        let mut login_event = perust_event::events::PlayerLoginEvent {
            cancel: perust_event::CancellableEvent::new(),
            player_name: result.username.clone(),
            kick_message: None,
        };
        ed.dispatch(&mut login_event);

        if login_event.cancel.is_cancelled() {
            log::info!("Login event cancelled for player: {}", result.username);
            let msg = login_event.kick_message.unwrap_or_else(|| "Login denied.".to_string());
            let disconnect = perust_protocol::packets::DisconnectPacket { message: msg };
            let _ = network_manager.send_typed_packet_to(address, &disconnect);
            return;
        }
    }

    // 6. Create the Player object
    let player_uuid = result.uuid.unwrap_or_else(|| Uuid::new_v4());
    let runtime_id = entity_manager.allocate_id();

    let mut player = Player::new(runtime_id, address);
    player.uuid = player_uuid;
    player.username = result.username.clone();
    player.display_name = result.username.clone();
    player.xuid = result.xuid.clone();
    player.protocol = result.protocol;
    player.login_state = LoginState::LoggingIn;

    // Set the player's gamemode from config
    let gamemode = GameMode::from_i32(cfg.gamemode as i32).unwrap_or(GameMode::Survival);
    player.set_gamemode(gamemode);

    // Check if player is an operator
    {
        let ops_list = ops.lock();
        player.is_op = ops_list.is_op(&result.username);
        player.permissions.is_op = player.is_op;
    }

    // Set spawn position from world
    let spawn_pos = {
        let world = default_world.lock();
        world.get_spawn_position()
    };
    player.spawn_position = spawn_pos;
    player.position.x = spawn_pos.x as f32;
    player.position.y = spawn_pos.y as f32;
    player.position.z = spawn_pos.z as f32;

    // Insert the player
    players.insert(player_uuid, player);
    address_to_uuid.insert(address, player_uuid);

    log::info!(
        "Player {} ({}) logged in from {} [OP={}]",
        result.username, player_uuid, address, players.get(&player_uuid).map(|p| p.is_op).unwrap_or(false),
    );

    // 7. Send PlayStatusPacket (LOGIN_SUCCESS)
    let status_packet = PlayStatusPacket {
        status: PlayStatus::LoginSuccess,
    };
    if let Err(e) = network_manager.send_typed_packet_to(address, &status_packet) {
        log::error!("Failed to send PlayStatusPacket to {}: {:?}", address, e);
        return;
    }

    // 8. Send ResourcePacksInfoPacket
    let resource_packs_info = ResourcePacksInfoPacket {
        must_accept: false,
        has_scripts: false,
        behavior_pack_infos: Vec::new(),
        resource_pack_infos: Vec::new(),
    };
    if let Err(e) = network_manager.send_typed_packet_to(address, &resource_packs_info) {
        log::error!("Failed to send ResourcePacksInfoPacket to {}: {:?}", address, e);
        return;
    }

    // Update player login state to ResourcePacks
    if let Some(mut p) = players.get_mut(&player_uuid) {
        p.login_state = LoginState::ResourcePacks;
    }

    drop(cfg);
}

/// Handles a movement packet from a player.
///
/// Validates the movement and applies it to the player's state.
pub fn handle_move_player(
    address: SocketAddr,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
    _body_yaw: f32,
    on_ground: bool,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
) {
    let uuid = match address_to_uuid.get(&address) {
        Some(entry) => *entry.value(),
        None => return,
    };

    if let Some(mut player) = players.get_mut(&uuid) {
        if !player.is_playing() {
            return;
        }
        player.handle_move(x, y, z, yaw, pitch, yaw, on_ground);
    }
}

/// Handles a text (chat) packet from a player.
///
/// Processes chat messages and commands (messages starting with `/`).
#[allow(clippy::too_many_arguments)]
pub fn handle_text(
    address: SocketAddr,
    _source: String,
    message: String,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
    network_manager: &Arc<NetworkManager>,
    event_dispatcher: &Arc<Mutex<EventDispatcher>>,
    command_dispatcher: &Arc<Mutex<CommandDispatcher>>,
) {
    let uuid = match address_to_uuid.get(&address) {
        Some(entry) => *entry.value(),
        None => return,
    };

    let (username, runtime_id) = {
        match players.get(&uuid) {
            Some(player) => (player.username.clone(), player.runtime_id),
            None => return,
        }
    };

    // Check if the message is a command
    if message.starts_with('/') {
        let command_input = &message[1..]; // strip the '/'
        log::info!("{} issued command: {}", username, command_input);

        let sender = CommandSender::Player {
            runtime_id,
            name: username.clone(),
        };

        let dispatcher = command_dispatcher.lock();
        match dispatcher.dispatch(&sender, command_input) {
            Ok(()) => {}
            Err(e) => {
                log::info!("Command error for {}: {}", username, e);
            }
        }
    } else {
        // Chat message — fire PlayerChatEvent
        let cancelled = {
            let mut ed = event_dispatcher.lock();
            let mut chat_event = perust_event::events::PlayerChatEvent {
                cancel: perust_event::CancellableEvent::new(),
                runtime_id,
                player_name: username.clone(),
                message: message.clone(),
            };
            ed.dispatch(&mut chat_event);
            chat_event.cancel.is_cancelled()
        };

        if !cancelled {
            log::info!("<{}> {}", username, message);
            // Broadcast the chat message to all players
            let text_packet = perust_protocol::packets::TextPacket {
                text_type: perust_protocol::types::TextPacketType::Chat,
                source: username.clone(),
                message: message.clone(),
                parameters: Vec::new(),
                xuid: String::new(),
                platform_chat_id: String::new(),
            };
            let _ = network_manager.broadcast_typed_packet(&text_packet);
        }
    }
}

/// Handles a player action packet.
///
/// Updates player flags based on the action type (sprinting, sneaking, etc.).
pub fn handle_player_action(
    address: SocketAddr,
    action: perust_protocol::types::PlayerAction,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
) {
    let uuid = match address_to_uuid.get(&address) {
        Some(entry) => *entry.value(),
        None => return,
    };

    if let Some(mut player) = players.get_mut(&uuid) {
        if !player.is_playing() {
            return;
        }
        player.handle_player_action(action);
    }
}

/// Handles an inventory transaction packet.
///
/// Forwards the raw data to the player's inventory handler.
pub fn handle_inventory_transaction(
    address: SocketAddr,
    _data: Vec<u8>,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
) {
    let uuid = match address_to_uuid.get(&address) {
        Some(entry) => *entry.value(),
        None => return,
    };

    if let Some(mut player) = players.get_mut(&uuid) {
        if !player.is_playing() {
            return;
        }
        // TODO: Parse the transaction data and validate it
        log::trace!("Inventory transaction from {}", player.username);
    }
}

/// Handles a chunk radius request packet.
///
/// Sends the requested chunks around the player and responds with
/// the actual chunk radius that will be used.
#[allow(clippy::too_many_arguments)]
pub fn handle_request_chunk_radius(
    address: SocketAddr,
    radius: u32,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
    default_world: &Arc<Mutex<World>>,
    network_manager: &Arc<NetworkManager>,
    config: &Arc<RwLock<ServerProperties>>,
) {
    let uuid = match address_to_uuid.get(&address) {
        Some(entry) => *entry.value(),
        None => return,
    };

    // Clamp the requested radius to the server's view distance
    let cfg = config.read();
    let actual_radius = radius.min(cfg.view_distance).max(1);
    drop(cfg);

    // Update player's chunk radius
    if let Some(mut player) = players.get_mut(&uuid) {
        player.chunk_radius = actual_radius;
    }

    // Send ChunkRadiusUpdatedPacket
    let radius_packet = ChunkRadiusUpdatedPacket {
        radius: actual_radius as i32,
    };
    if let Err(e) = network_manager.send_typed_packet_to(address, &radius_packet) {
        log::debug!("Failed to send ChunkRadiusUpdatedPacket to {}: {:?}", address, e);
        return;
    }

    // Send chunks around the player
    send_chunks_around_player(uuid, address, actual_radius, players, default_world, network_manager);
}

/// Handles a resource pack client response packet.
///
/// After the client acknowledges resource packs, sends the StartGamePacket
/// and completes the login flow.
#[allow(clippy::too_many_arguments)]
pub fn handle_resource_pack_response(
    address: SocketAddr,
    status: u8,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
    network_manager: &Arc<NetworkManager>,
    default_world: &Arc<Mutex<World>>,
    player_list: &Arc<PlayerList>,
    config: &Arc<RwLock<ServerProperties>>,
    entity_manager: &Arc<EntityManager>,
) {
    const RESOURCE_PACK_RESPONSE_REFUSED: u8 = 1;
    const RESOURCE_PACK_RESPONSE_SEND_PACKS: u8 = 2;
    const RESOURCE_PACK_RESPONSE_HAVE_ALL_PACKS: u8 = 3;
    const RESOURCE_PACK_RESPONSE_COMPLETED: u8 = 4;

    let uuid = match address_to_uuid.get(&address) {
        Some(entry) => *entry.value(),
        None => return,
    };

    match status {
        RESOURCE_PACK_RESPONSE_REFUSED => {
            log::info!("Player at {} refused resource packs, disconnecting", address);
            let disconnect = perust_protocol::packets::DisconnectPacket {
                message: "You must accept resource packs to play.".to_string(),
            };
            let _ = network_manager.send_typed_packet_to(address, &disconnect);
            // Remove the player
            players.remove(&uuid);
            address_to_uuid.remove(&address);
        }
        RESOURCE_PACK_RESPONSE_SEND_PACKS => {
            // We have no packs to send, so just proceed
            send_resource_pack_stack(address, network_manager);
        }
        RESOURCE_PACK_RESPONSE_HAVE_ALL_PACKS => {
            // Client has all packs, send stack
            send_resource_pack_stack(address, network_manager);
        }
        RESOURCE_PACK_RESPONSE_COMPLETED => {
            // Client has completed resource pack loading — send StartGame
            complete_login(uuid, address, players, network_manager, default_world, player_list, config, entity_manager);
        }
        _ => {
            log::warn!("Unknown resource pack response status {} from {}", status, address);
        }
    }
}

/// Handles a container close packet from a player.
pub fn handle_container_close(
    address: SocketAddr,
    _window_id: i8,
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
) {
    let uuid = match address_to_uuid.get(&address) {
        Some(entry) => *entry.value(),
        None => return,
    };

    if let Some(mut player) = players.get_mut(&uuid) {
        if player.is_playing() {
            player.open_inventory = None;
        }
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Sends a ResourcePackStackPacket (empty) to the client.
fn send_resource_pack_stack(address: SocketAddr, network_manager: &Arc<NetworkManager>) {
    let stack_packet = ResourcePackStackPacket {
        must_accept: false,
        behavior_pack_stack: Vec::new(),
        resource_pack_stack: Vec::new(),
    };
    if let Err(e) = network_manager.send_typed_packet_to(address, &stack_packet) {
        log::debug!("Failed to send ResourcePackStackPacket to {}: {:?}", address, e);
    }
}

/// Completes the login flow by sending StartGamePacket, PlayerListPacket,
/// chunks, and PlayStatusPacket (PLAYER_SPAWN).
#[allow(clippy::too_many_arguments)]
fn complete_login(
    uuid: Uuid,
    address: SocketAddr,
    players: &Arc<DashMap<Uuid, Player>>,
    network_manager: &Arc<NetworkManager>,
    default_world: &Arc<Mutex<World>>,
    player_list: &Arc<PlayerList>,
    config: &Arc<RwLock<ServerProperties>>,
    entity_manager: &Arc<EntityManager>,
) {
    let (username, runtime_id, unique_id, player_gamemode, x, y, z, pitch, yaw, chunk_radius) = {
        match players.get(&uuid) {
            Some(player) => (
                player.username.clone(),
                player.runtime_id,
                player.unique_id,
                player.gamemode.as_i32(),
                player.position.x,
                player.position.y,
                player.position.z,
                player.pitch,
                player.yaw,
                player.chunk_radius,
            ),
            None => return,
        }
    };

    let (seed, spawn_x, spawn_y, spawn_z, gamemode_val, difficulty_val, level_name) = {
        let cfg = config.read();
        let world = default_world.lock();
        (
            world.seed,
            world.spawn_position.x,
            world.spawn_position.y,
            world.spawn_position.z,
            cfg.gamemode as i32,
            cfg.difficulty as i32,
            cfg.level_name.clone(),
        )
    };

    // Determine generator type from level_name convention
    let generator = {
        let cfg = config.read();
        match cfg.level_type.as_str() {
            "FLAT" => 2,
            "VOID" => 3,
            _ => 1,
        }
    };

    // 1. Send StartGamePacket
    let start_game = StartGamePacket {
        entity_unique_id: unique_id,
        entity_runtime_id: runtime_id,
        player_gamemode,
        x,
        y,
        z,
        pitch,
        yaw,
        seed,
        dimension: 0, // Overworld
        generator,
        world_gamemode: gamemode_val,
        difficulty: difficulty_val,
        spawn_x,
        spawn_y,
        spawn_z,
        has_achievements_disabled: true,
        day_cycle_stop_time: -1,
        edu_mode: false,
        rain_level: 0.0,
        lightning_level: 0.0,
        commands_enabled: true,
        is_texture_packs_required: false,
        gamerules: vec![
            GameRule::bool_rule("announceachievements", false, true),
            GameRule::bool_rule("commandblockoutput", true, true),
            GameRule::bool_rule("dodaylightcycle", true, true),
            GameRule::bool_rule("doentitydrops", true, true),
            GameRule::bool_rule("dofiretick", true, true),
            GameRule::bool_rule("domobloot", true, true),
            GameRule::bool_rule("domobspawning", true, true),
            GameRule::bool_rule("dotiledrops", true, true),
            GameRule::bool_rule("doweathercycle", true, true),
            GameRule::bool_rule("keepinventory", false, true),
            GameRule::bool_rule("pvp", true, true),
            GameRule::bool_rule("showcoordinates", true, true),
        ],
        level_id: level_name.clone(),
        world_name: level_name,
        premium_world_template_id: String::new(),
    };

    if let Err(e) = network_manager.send_typed_packet_to(address, &start_game) {
        log::error!("Failed to send StartGamePacket to {}: {:?}", address, e);
        return;
    }

    // 2. Send PlayerListPacket (add this player)
    let player_list_entry = ProtocolPlayerListEntry {
        uuid,
        unique_entity_id: unique_id,
        name: username.clone(),
        xbox_user_id: String::new(),
        platform_chat_id: String::new(),
        build_platform: -1,
        skin_data: Some(SkinData {
            skin_id: format!("{}_{}", uuid, chrono::Utc::now().timestamp()),
            skin_resource_patch: String::new(),
            skin_data: vec![0u8; 8192], // 64x32 RGBA
            animation_data: Vec::new(),
            cape_data: Vec::new(),
            geometry_name: String::new(),
            geometry_data: Vec::new(),
            animated_image_data: Vec::new(),
        }),
    };

    let player_list_packet = PlayerListPacket {
        action: 0, // Add
        entries: vec![player_list_entry],
    };
    if let Err(e) = network_manager.broadcast_typed_packet(&player_list_packet) {
        log::debug!("Failed to broadcast PlayerListPacket: {:?}", e);
    }

    // Also add to server-side player list
    player_list.add(perust_player::player_list::PlayerListEntry {
        uuid,
        username: username.clone(),
        skin_id: String::new(),
        skin_data: Vec::new(),
        xuid: String::new(),
        platform_chat_id: String::new(),
    });

    // 3. Send PlayStatusPacket (PLAYER_SPAWN)
    let spawn_packet = PlayStatusPacket {
        status: PlayStatus::PlayerSpawn,
    };
    if let Err(e) = network_manager.send_typed_packet_to(address, &spawn_packet) {
        log::error!("Failed to send PlayStatusPacket (PlayerSpawn) to {}: {:?}", address, e);
        return;
    }

    // 4. Update player login state to Playing
    if let Some(mut player) = players.get_mut(&uuid) {
        player.login_state = LoginState::Playing;
    }

    log::info!("Player {} spawned successfully", username);

    // 5. Send initial chunks
    let cx = (x.floor() as i32) >> 4;
    let cz = (z.floor() as i32) >> 4;
    send_chunks_around_player(uuid, address, chunk_radius, players, default_world, network_manager);
}

/// Sends chunks around a player's position.
fn send_chunks_around_player(
    uuid: Uuid,
    address: SocketAddr,
    radius: u32,
    players: &Arc<DashMap<Uuid, Player>>,
    default_world: &Arc<Mutex<World>>,
    network_manager: &Arc<NetworkManager>,
) {
    let (cx, cz) = {
        match players.get(&uuid) {
            Some(player) => player.get_chunk_position(),
            None => return,
        }
    };

    // Ensure chunks are generated in the world
    let new_chunks = {
        let world = default_world.lock();
        world.ensure_chunks_in_radius(cx, cz, radius)
    };

    // Send chunk data packets for newly generated chunks
    for chunk in &new_chunks {
        let chunk_data = chunk.serialize_network();
        let chunk_packet = perust_protocol::packets::FullChunkDataPacket {
            chunk_x: chunk.x,
            chunk_z: chunk.z,
            data: chunk_data,
        };
        if let Err(e) = network_manager.send_typed_packet_to(address, &chunk_packet) {
            log::debug!("Failed to send chunk ({}, {}) to {}: {:?}", chunk.x, chunk.z, address, e);
        }
    }

    // Mark chunks as loaded for this player
    if let Some(mut player) = players.get_mut(&uuid) {
        let r = radius as i32;
        for dx in -r..=r {
            for dz in -r..=r {
                player.mark_chunk_loaded(cx + dx, cz + dz);
            }
        }
    }

    log::debug!("Sent {} chunks to player at {}", new_chunks.len(), address);
}
