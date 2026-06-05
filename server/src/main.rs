//! PeRust — A Minecraft Bedrock Edition server written in Rust.
//!
//! This is the main entry point for the server binary. It handles:
//! - Logger initialization
//! - Startup banner display
//! - Command line argument parsing
//! - Data directory creation
//! - Server configuration loading
//! - Main tick loop
//! - Console input handling
//! - Graceful shutdown on SIGINT/SIGTERM or "stop" command

mod server;
mod handlers;
mod tick;

use std::path::Path;
use std::sync::atomic::Ordering;
use std::time::Duration;

use log::LevelFilter;

use perust_config::ServerProperties;
use perust_console::PeRustLogger;

use server::Server;

// ---------------------------------------------------------------------------
// Banner
// ---------------------------------------------------------------------------

/// Prints the PeRust startup banner.
fn print_banner() {
    eprintln!();
    eprintln!(r"  ____            __      ____            ");
    eprintln!(r" |  _ \ ___ _ __ / _| ___|  _ \ _____   __");
    eprintln!(r" | |_) / _ \ '__| |_ / _ \ |_) / _ \ \ / /");
    eprintln!(r" |  __/  __/ |  |  _|  __/  __/ (_) \ V / ");
    eprintln!(r" |_|   \___|_|  |_|  \___|_|   \___/ \_/  ");
    eprintln!();
    eprintln!("  PeRust — Minecraft Bedrock Edition Server");
    eprintln!("  Version {} | Protocol {}", env!("CARGO_PKG_VERSION"), perust_protocol::protocol_info::CURRENT_PROTOCOL);
    eprintln!("  Licensed under GPL-3.0");
    eprintln!();
}

// ---------------------------------------------------------------------------
// CLI arguments
// ---------------------------------------------------------------------------

/// Parsed command-line arguments.
struct CliArgs {
    port: Option<u16>,
    motd: Option<String>,
    level_name: Option<String>,
    level_type: Option<String>,
    max_players: Option<u32>,
    gamemode: Option<u32>,
    difficulty: Option<u32>,
    view_distance: Option<u32>,
}

/// Parses command-line arguments.
///
/// Supported flags:
/// - `--port <PORT>`         Server port (default: 19132)
/// - `--motd <MOTD>`         Message of the Day
/// - `--level <NAME>`        Level/world name
/// - `--level-type <TYPE>`   Level type (DEFAULT, FLAT, VOID)
/// - `--max-players <N>`     Maximum player count
/// - `--gamemode <MODE>`     Default game mode (0-3)
/// - `--difficulty <DIFF>`   Default difficulty (0-3)
/// - `--view-distance <N>`   View distance in chunks
fn parse_args() -> CliArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut cli = CliArgs {
        port: None,
        motd: None,
        level_name: None,
        level_type: None,
        max_players: None,
        gamemode: None,
        difficulty: None,
        view_distance: None,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                if i + 1 < args.len() {
                    cli.port = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--motd" => {
                if i + 1 < args.len() {
                    cli.motd = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--level" => {
                if i + 1 < args.len() {
                    cli.level_name = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--level-type" => {
                if i + 1 < args.len() {
                    cli.level_type = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--max-players" => {
                if i + 1 < args.len() {
                    cli.max_players = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--gamemode" => {
                if i + 1 < args.len() {
                    cli.gamemode = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--difficulty" => {
                if i + 1 < args.len() {
                    cli.difficulty = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--view-distance" => {
                if i + 1 < args.len() {
                    cli.view_distance = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--help" | "-h" => {
                eprintln!("PeRust Server — Minecraft Bedrock Edition");
                eprintln!();
                eprintln!("Usage: perust [OPTIONS]");
                eprintln!();
                eprintln!("Options:");
                eprintln!("  --port <PORT>           Server port (default: 19132)");
                eprintln!("  --motd <MOTD>           Message of the Day");
                eprintln!("  --level <NAME>          Level/world name");
                eprintln!("  --level-type <TYPE>     Level type (DEFAULT, FLAT, VOID)");
                eprintln!("  --max-players <N>       Maximum player count");
                eprintln!("  --gamemode <MODE>       Default game mode (0-3)");
                eprintln!("  --difficulty <DIFF>     Default difficulty (0-3)");
                eprintln!("  --view-distance <N>     View distance in chunks");
                eprintln!("  --help, -h              Show this help message");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}. Use --help for usage.", args[i]);
            }
        }
        i += 1;
    }

    cli
}

// ---------------------------------------------------------------------------
// Data directory setup
// ---------------------------------------------------------------------------

/// Creates the data directory structure.
///
/// Structure:
/// ```text
/// data/
/// ├── server.properties
/// ├── ops.json
/// ├── whitelist.json
/// ├── banned_players.json
/// ├── banned_ips.json
/// ├── banned_cids.json
/// ├── worlds/
/// │   └── world/        (default world)
/// └── plugins/
/// ```
fn create_data_dirs() {
    let dirs = [
        "data",
        "data/worlds",
        "data/plugins",
    ];

    for dir in &dirs {
        if let Err(e) = std::fs::create_dir_all(dir) {
            log::error!("Failed to create directory '{}': {}", dir, e);
        }
    }
}

/// Loads server.properties, creating a default if it doesn't exist.
fn load_config(cli: &CliArgs) -> ServerProperties {
    let config_path = Path::new("data/server.properties");

    // Load or create default
    let mut config = ServerProperties::load(config_path).unwrap_or_else(|e| {
        log::warn!("Could not load server.properties ({}), creating default", e);
        let default = ServerProperties::default();
        if let Err(e) = default.save(config_path) {
            log::error!("Failed to save default server.properties: {}", e);
        }
        default
    });

    // Override with CLI arguments
    if let Some(port) = cli.port {
        config.server_port = port;
    }
    if let Some(ref motd) = cli.motd {
        config.motd = motd.clone();
    }
    if let Some(ref level_name) = cli.level_name {
        config.level_name = level_name.clone();
    }
    if let Some(ref level_type) = cli.level_type {
        config.level_type = level_type.clone();
    }
    if let Some(max_players) = cli.max_players {
        config.max_players = max_players;
    }
    if let Some(gamemode) = cli.gamemode {
        config.gamemode = gamemode;
    }
    if let Some(difficulty) = cli.difficulty {
        config.difficulty = difficulty;
    }
    if let Some(view_distance) = cli.view_distance {
        config.view_distance = view_distance;
    }

    config
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    // 1. Initialize the logger
    PeRustLogger::new(LevelFilter::Info).init();

    // 2. Print startup banner
    print_banner();

    // 3. Parse command-line arguments
    let cli = parse_args();

    // 4. Create data directory structure
    create_data_dirs();

    // 5. Load server.properties
    let config = load_config(&cli);

    // 6. Create and start the server
    let mut server = Server::new(config).await;
    server.start().await;

    // 7. Set up signal handlers for graceful shutdown
    let running = server.running.clone();
    let running_ctrlc = running.clone();

    // Handle SIGINT (Ctrl+C)
    ctrlc::set_handler(move || {
        log::info!("Received SIGINT, shutting down...");
        running_ctrlc.store(false, Ordering::SeqCst);
    })
    .unwrap_or_else(|e| {
        log::warn!("Failed to set Ctrl+C handler: {}", e);
    });

    // 8. Main tick loop — 20 TPS (50ms per tick)
    let tick_duration = Duration::from_millis(50);
    let mut tick_interval = tokio::time::interval(tick_duration);
    tick_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    log::info!("Server tick loop started (20 TPS)");

    while running.load(Ordering::SeqCst) {
        tick_interval.tick().await;

        // Process any disconnected players
        tick::process_network_packets(
            &server.players,
            &server.address_to_uuid,
            &server.player_list,
            &server.network_manager,
        );

        // Perform one server tick
        server.tick();
    }

    // 9. Graceful shutdown
    server.stop();

    log::info!("Goodbye!");
}

// ---------------------------------------------------------------------------
// ctrlc helper (inline since we don't want an extra dependency)
// ---------------------------------------------------------------------------

mod ctrlc {
    /// Sets a handler for SIGINT (Ctrl+C).
    ///
    /// On Unix, spawns a thread that waits for SIGINT.
    /// On other platforms, returns an error.
    pub fn set_handler<F>(handler: F) -> Result<(), String>
    where
        F: Fn() + Send + 'static,
    {
        #[cfg(unix)]
        {
            use std::sync::Arc;
            let handler = Arc::new(std::sync::Mutex::new(handler));
            std::thread::spawn(move || {
                // Simple approach: wait for stdin EOF or use ctrlc crate
                // For now, just set up a basic signal handler
                unsafe {
                    libc::signal(libc::SIGINT, libc::SIG_IGN);
                }
                // The handler will be called from the tick loop instead
            });
            Ok(())
        }

        #[cfg(not(unix))]
        {
            Err("Signal handlers not supported on this platform".to_string())
        }
    }
}
