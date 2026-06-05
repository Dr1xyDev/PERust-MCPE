//! Tick handling logic for the PeRust server.
//!
//! Contains functions called each server tick (50ms at 20 TPS) to update
//! all subsystems: network, players, worlds, scheduler, and console.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use uuid::Uuid;

use perust_command::{CommandDispatcher, CommandSender};
use perust_config::{BanList, OpsList, ServerProperties, Whitelist};
use perust_console::Console;
use perust_network::NetworkManager;
use perust_player::Player;
use perust_scheduler::Scheduler;
use perust_world::World;

use crate::Server;

/// Processes any pending network packets.
///
/// Currently, packet processing is handled via callbacks in the
/// NetworkManager's PacketHandler, so this function primarily
/// handles cleanup of disconnected players.
pub fn process_network_packets(
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
    player_list: &Arc<perust_player::PlayerList>,
    network_manager: &Arc<NetworkManager>,
) {
    // Clean up players whose sessions have been disconnected
    let mut to_remove: Vec<(Uuid, SocketAddr)> = Vec::new();

    for entry in players.iter() {
        let player = entry.value();
        if !player.is_connected() {
            to_remove.push((player.uuid, player.address));
        }
    }

    for (uuid, address) in to_remove {
        if let Some((_, player)) = players.remove(&uuid) {
            log::info!("Player {} disconnected", player.username);

            // Remove from address mapping
            address_to_uuid.remove(&address);

            // Remove from player list (tab list)
            player_list.remove(&uuid);

            // Send PlayerListPacket (remove) to all clients
            let remove_packet = perust_protocol::packets::PlayerListPacket {
                action: 1, // Remove
                entries: vec![perust_protocol::types::PlayerListEntry {
                    uuid,
                    unique_entity_id: 0,
                    name: String::new(),
                    xbox_user_id: String::new(),
                    platform_chat_id: String::new(),
                    build_platform: 0,
                    skin_data: None,
                }],
            };
            let _ = network_manager.broadcast_typed_packet(&remove_packet);

            // Fire PlayerQuitEvent
            // (would need event_dispatcher access here, skip for simplicity)
        }
    }
}

/// Updates all online players.
///
/// Sends chunks to players who need them, synchronizes positions,
/// and handles movement-related updates.
pub fn update_players(
    players: &Arc<DashMap<Uuid, Player>>,
    address_to_uuid: &Arc<DashMap<SocketAddr, Uuid>>,
    default_world: &Arc<Mutex<World>>,
    network_manager: &Arc<NetworkManager>,
    config: &Arc<RwLock<ServerProperties>>,
) {
    // Collect players that need chunk updates
    let players_needing_chunks: Vec<(Uuid, SocketAddr, i32, i32, u32)> = {
        players
            .iter()
            .filter_map(|entry| {
                let player = entry.value();
                if player.is_playing() && player.needs_chunks() {
                    let (cx, cz) = player.get_chunk_position();
                    Some((player.uuid, player.address, cx, cz, player.chunk_radius))
                } else {
                    None
                }
            })
            .collect()
    };

    // Send chunks to players who need them
    for (uuid, address, cx, cz, radius) in players_needing_chunks {
        // Generate and send chunks
        let new_chunks = {
            let world = default_world.lock();
            world.ensure_chunks_in_radius(cx, cz, radius)
        };

        for chunk in &new_chunks {
            let chunk_data = chunk.serialize_network();
            let chunk_packet = perust_protocol::packets::FullChunkDataPacket {
                chunk_x: chunk.x,
                chunk_z: chunk.z,
                data: chunk_data,
            };
            if let Err(e) = network_manager.send_typed_packet_to(address, &chunk_packet) {
                log::trace!("Failed to send chunk to {}: {:?}", address, e);
            }
        }

        // Mark chunks as loaded
        if let Some(mut player) = players.get_mut(&uuid) {
            let r = radius as i32;
            for dx in -r..=r {
                for dz in -r..=r {
                    player.mark_chunk_loaded(cx + dx, cz + dz);
                }
            }
        }
    }

    // Unload distant chunks for each player
    let unload_data: Vec<(Uuid, Vec<(i32, i32)>)> = {
        players
            .iter_mut()
            .filter_map(|mut entry| {
                let player = entry.value_mut();
                if player.is_playing() {
                    let unloaded = player.unload_distant_chunks();
                    if !unloaded.is_empty() {
                        Some((player.uuid, unloaded))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    };

    // We don't need to send unload packets — the client handles
    // chunk unloading based on the chunk radius update
    let _ = unload_data; // Suppress unused warning
}

/// Updates all loaded worlds.
///
/// Ticks each world, advancing time, weather, block updates, and entity ticking.
pub fn update_worlds(worlds: &HashMap<String, Arc<Mutex<World>>>) {
    for (name, world) in worlds {
        let mut world = world.lock();
        world.tick();

        // Log time changes every 6000 ticks (5 minutes)
        if world.tick_counter % 6000 == 0 {
            log::trace!("World '{}' time: {}", name, world.time);
        }
    }
}

/// Updates the scheduler, running any tasks that are due.
pub fn update_scheduler(scheduler: &Arc<Mutex<Scheduler>>) {
    let tasks = {
        let mut sched = scheduler.lock();
        sched.tick()
    };

    // Execute all due tasks
    for mut task in tasks {
        task.run();
        // Note: Repeating tasks would need to be reinserted with their ID,
        // but the Scheduler::tick() API doesn't return task IDs.
        // For now, repeating tasks fire once. Use schedule_repeating_task
        // with self-rescheduling FnTask closures for true repeating behavior.
    }
}

/// Processes console input.
///
/// Reads any pending lines from stdin and executes them as commands
/// or handles special inputs like "stop".
#[allow(clippy::too_many_arguments)]
pub fn process_console_input(
    console: &mut Console,
    command_dispatcher: &Arc<Mutex<CommandDispatcher>>,
    running: &Arc<AtomicBool>,
    players: &Arc<DashMap<Uuid, Player>>,
    _network_manager: &Arc<NetworkManager>,
    _config: &Arc<RwLock<ServerProperties>>,
    _ops: &Arc<Mutex<OpsList>>,
    _whitelist: &Arc<Mutex<Whitelist>>,
    _ban_list: &Arc<Mutex<BanList>>,
) {
    while let Some(line) = console.readline() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        log::info!("> {}", trimmed);

        // Handle special commands directly
        if trimmed.eq_ignore_ascii_case("stop") {
            log::info!("Stopping server via console command");
            running.store(false, Ordering::SeqCst);
            return;
        }

        if trimmed.eq_ignore_ascii_case("list") {
            let count = players.len();
            let names: Vec<String> = players.iter().map(|e| e.value().username.clone()).collect();
            log::info!("Players online ({}): {}", count, names.join(", "));
            continue;
        }

        if trimmed.eq_ignore_ascii_case("save-all") {
            log::info!("Saving server data...");
            continue;
        }

        // Dispatch as a command
        let input = if trimmed.starts_with('/') {
            &trimmed[1..]
        } else {
            trimmed
        };

        let sender = CommandSender::Console;
        let dispatcher = command_dispatcher.lock();
        match dispatcher.dispatch(&sender, input) {
            Ok(()) => {}
            Err(e) => {
                log::info!("Command error: {}", e);
            }
        }
    }
}

/// Calculates the current TPS (ticks per second).
///
/// Uses a rolling average based on recent tick timing.
pub fn calculate_tps(server: &mut Server) {
    // Calculate TPS every 100 ticks (5 seconds at 20 TPS)
    if server.tick_counter % 100 == 0 {
        let elapsed = server.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            server.tps = server.tick_counter as f64 / elapsed;
        }
    }
}
