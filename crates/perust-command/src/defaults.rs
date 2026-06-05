//! Default command implementations shipped with the server.
//!
//! Each struct implements [`CommandExecutor`] and can be registered with a
//! [`crate::dispatcher::CommandDispatcher`]. The helper function
//! [`register_defaults`] registers all of them at once.

use crate::command::{Command, CommandError, CommandExecutor, CommandResult};
use crate::sender::CommandSender;

// ---------------------------------------------------------------------------
// HelpCommand
// ---------------------------------------------------------------------------

/// Displays a list of available commands.
pub struct HelpCommand;

impl CommandExecutor for HelpCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, _args: &[String]) -> CommandResult {
        // In a full implementation this would query the dispatcher for the
        // command list. Here we just log a placeholder.
        log::info!("Available commands: help, stop, list, gamemode, give, tp, op, deop, kick, say, time, whitelist, ban, pardon, seed, version, plugins, difficulty, spawnpoint, kill");
        Ok(())
    }
}

/// Builds the `/help` command definition.
pub fn help_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("help")
            .with_description("Shows the list of available commands")
            .with_usage("/help [page]")
            .with_alias("?"),
        Box::new(HelpCommand),
    )
}

// ---------------------------------------------------------------------------
// StopCommand
// ---------------------------------------------------------------------------

/// Stops the server gracefully.
pub struct StopCommand;

impl CommandExecutor for StopCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, _args: &[String]) -> CommandResult {
        log::info!("Stopping server...");
        // In a full implementation this would signal the server to shut down.
        Ok(())
    }
}

/// Builds the `/stop` command definition.
pub fn stop_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("stop")
            .with_description("Stops the server")
            .with_usage("/stop")
            .with_permission("perust.command.stop"),
        Box::new(StopCommand),
    )
}

// ---------------------------------------------------------------------------
// ListCommand
// ---------------------------------------------------------------------------

/// Lists online players.
pub struct ListCommand;

impl CommandExecutor for ListCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, _args: &[String]) -> CommandResult {
        // In a full implementation this would query the player manager.
        log::info!("Online players: 0");
        Ok(())
    }
}

/// Builds the `/list` command definition.
pub fn list_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("list")
            .with_description("Lists online players")
            .with_usage("/list"),
        Box::new(ListCommand),
    )
}

// ---------------------------------------------------------------------------
// GamemodeCommand
// ---------------------------------------------------------------------------

/// Changes a player's game mode.
pub struct GamemodeCommand;

impl CommandExecutor for GamemodeCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/gamemode <mode> [player]".to_string()));
        }
        let mode = &args[0];
        log::info!("Setting gamemode to {}", mode);
        Ok(())
    }
}

/// Builds the `/gamemode` command definition.
pub fn gamemode_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("gamemode")
            .with_description("Changes a player's game mode")
            .with_usage("/gamemode <mode> [player]")
            .with_permission("perust.command.gamemode")
            .with_alias("gm"),
        Box::new(GamemodeCommand),
    )
}

// ---------------------------------------------------------------------------
// GiveCommand
// ---------------------------------------------------------------------------

/// Gives items to a player.
pub struct GiveCommand;

impl CommandExecutor for GiveCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/give <player> <item> [amount]".to_string()));
        }
        log::info!("Giving item to player");
        Ok(())
    }
}

/// Builds the `/give` command definition.
pub fn give_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("give")
            .with_description("Gives items to a player")
            .with_usage("/give <player> <item> [amount]")
            .with_permission("perust.command.give"),
        Box::new(GiveCommand),
    )
}

// ---------------------------------------------------------------------------
// TeleportCommand
// ---------------------------------------------------------------------------

/// Teleports a player to another player or location.
pub struct TeleportCommand;

impl CommandExecutor for TeleportCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/tp <target> [destination]".to_string()));
        }
        log::info!("Teleporting player");
        Ok(())
    }
}

/// Builds the `/tp` command definition.
pub fn teleport_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("tp")
            .with_description("Teleports a player")
            .with_usage("/tp <target> [destination]")
            .with_permission("perust.command.tp")
            .with_alias("teleport"),
        Box::new(TeleportCommand),
    )
}

// ---------------------------------------------------------------------------
// OpCommand
// ---------------------------------------------------------------------------

/// Grants operator status to a player.
pub struct OpCommand;

impl CommandExecutor for OpCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/op <player>".to_string()));
        }
        log::info!("Opping player: {}", args[0]);
        Ok(())
    }
}

/// Builds the `/op` command definition.
pub fn op_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("op")
            .with_description("Grants operator status to a player")
            .with_usage("/op <player>")
            .with_permission("perust.command.op"),
        Box::new(OpCommand),
    )
}

// ---------------------------------------------------------------------------
// DeopCommand
// ---------------------------------------------------------------------------

/// Revokes operator status from a player.
pub struct DeopCommand;

impl CommandExecutor for DeopCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/deop <player>".to_string()));
        }
        log::info!("De-opping player: {}", args[0]);
        Ok(())
    }
}

/// Builds the `/deop` command definition.
pub fn deop_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("deop")
            .with_description("Revokes operator status from a player")
            .with_usage("/deop <player>")
            .with_permission("perust.command.deop"),
        Box::new(DeopCommand),
    )
}

// ---------------------------------------------------------------------------
// KickCommand
// ---------------------------------------------------------------------------

/// Kicks a player from the server.
pub struct KickCommand;

impl CommandExecutor for KickCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/kick <player> [reason]".to_string()));
        }
        let reason = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            "Kicked by an operator".to_string()
        };
        log::info!("Kicking player: {} (reason: {})", args[0], reason);
        Ok(())
    }
}

/// Builds the `/kick` command definition.
pub fn kick_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("kick")
            .with_description("Kicks a player from the server")
            .with_usage("/kick <player> [reason]")
            .with_permission("perust.command.kick"),
        Box::new(KickCommand),
    )
}

// ---------------------------------------------------------------------------
// SayCommand
// ---------------------------------------------------------------------------

/// Broadcasts a message to all players.
pub struct SayCommand;

impl CommandExecutor for SayCommand {
    fn execute(&self, sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/say <message>".to_string()));
        }
        let message = args.join(" ");
        log::info!("[{}] {}", sender.name(), message);
        Ok(())
    }
}

/// Builds the `/say` command definition.
pub fn say_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("say")
            .with_description("Broadcasts a message to all players")
            .with_usage("/say <message>")
            .with_permission("perust.command.say"),
        Box::new(SayCommand),
    )
}

// ---------------------------------------------------------------------------
// TimeCommand
// ---------------------------------------------------------------------------

/// Sets or queries the world time.
pub struct TimeCommand;

impl CommandExecutor for TimeCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/time <set|add|query> <value>".to_string()));
        }
        match args[0].as_str() {
            "set" => {
                if args.len() < 2 {
                    return Err(CommandError::InvalidUsage("/time set <value>".to_string()));
                }
                log::info!("Setting time to {}", args[1]);
            }
            "add" => {
                if args.len() < 2 {
                    return Err(CommandError::InvalidUsage("/time add <value>".to_string()));
                }
                log::info!("Adding {} to time", args[1]);
            }
            "query" => {
                log::info!("Querying time");
            }
            _ => return Err(CommandError::InvalidUsage("/time <set|add|query> <value>".to_string())),
        }
        Ok(())
    }
}

/// Builds the `/time` command definition.
pub fn time_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("time")
            .with_description("Sets or queries the world time")
            .with_usage("/time <set|add|query> <value>")
            .with_permission("perust.command.time")
            .add_sub_command(Command::new("set"))
            .add_sub_command(Command::new("add"))
            .add_sub_command(Command::new("query")),
        Box::new(TimeCommand),
    )
}

// ---------------------------------------------------------------------------
// WhitelistCommand
// ---------------------------------------------------------------------------

/// Manages the server whitelist.
pub struct WhitelistCommand;

impl CommandExecutor for WhitelistCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/whitelist <on|off|add|remove|list>".to_string()));
        }
        match args[0].as_str() {
            "on" => log::info!("Whitelist enabled"),
            "off" => log::info!("Whitelist disabled"),
            "add" => {
                if args.len() < 2 {
                    return Err(CommandError::InvalidUsage("/whitelist add <player>".to_string()));
                }
                log::info!("Adding {} to whitelist", args[1]);
            }
            "remove" => {
                if args.len() < 2 {
                    return Err(CommandError::InvalidUsage("/whitelist remove <player>".to_string()));
                }
                log::info!("Removing {} from whitelist", args[1]);
            }
            "list" => log::info!("Whitelist: (empty)"),
            _ => return Err(CommandError::InvalidUsage("/whitelist <on|off|add|remove|list>".to_string())),
        }
        Ok(())
    }
}

/// Builds the `/whitelist` command definition.
pub fn whitelist_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("whitelist")
            .with_description("Manages the server whitelist")
            .with_usage("/whitelist <on|off|add|remove|list>")
            .with_permission("perust.command.whitelist")
            .with_alias("wl"),
        Box::new(WhitelistCommand),
    )
}

// ---------------------------------------------------------------------------
// BanCommand
// ---------------------------------------------------------------------------

/// Bans a player from the server.
pub struct BanCommand;

impl CommandExecutor for BanCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/ban <player> [reason]".to_string()));
        }
        let reason = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            "Banned by an operator".to_string()
        };
        log::info!("Banning player: {} (reason: {})", args[0], reason);
        Ok(())
    }
}

/// Builds the `/ban` command definition.
pub fn ban_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("ban")
            .with_description("Bans a player from the server")
            .with_usage("/ban <player> [reason]")
            .with_permission("perust.command.ban"),
        Box::new(BanCommand),
    )
}

// ---------------------------------------------------------------------------
// PardonCommand
// ---------------------------------------------------------------------------

/// Pardons (unbans) a player.
pub struct PardonCommand;

impl CommandExecutor for PardonCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/pardon <player>".to_string()));
        }
        log::info!("Pardoning player: {}", args[0]);
        Ok(())
    }
}

/// Builds the `/pardon` command definition.
pub fn pardon_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("pardon")
            .with_description("Pardons (unbans) a player")
            .with_usage("/pardon <player>")
            .with_permission("perust.command.pardon")
            .with_alias("unban"),
        Box::new(PardonCommand),
    )
}

// ---------------------------------------------------------------------------
// SeedCommand
// ---------------------------------------------------------------------------

/// Displays the world seed.
pub struct SeedCommand;

impl CommandExecutor for SeedCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, _args: &[String]) -> CommandResult {
        log::info!("Seed: [placeholder]");
        Ok(())
    }
}

/// Builds the `/seed` command definition.
pub fn seed_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("seed")
            .with_description("Displays the world seed")
            .with_usage("/seed")
            .with_permission("perust.command.seed"),
        Box::new(SeedCommand),
    )
}

// ---------------------------------------------------------------------------
// VersionCommand
// ---------------------------------------------------------------------------

/// Displays server version information.
pub struct VersionCommand;

impl CommandExecutor for VersionCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, _args: &[String]) -> CommandResult {
        log::info!("PeRust v0.1.0 (Minecraft Bedrock Edition server)");
        Ok(())
    }
}

/// Builds the `/version` command definition.
pub fn version_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("version")
            .with_description("Displays server version information")
            .with_usage("/version")
            .with_alias("ver")
            .with_alias("about"),
        Box::new(VersionCommand),
    )
}

// ---------------------------------------------------------------------------
// PluginsCommand
// ---------------------------------------------------------------------------

/// Lists loaded plugins.
pub struct PluginsCommand;

impl CommandExecutor for PluginsCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, _args: &[String]) -> CommandResult {
        log::info!("Plugins (0): ");
        Ok(())
    }
}

/// Builds the `/plugins` command definition.
pub fn plugins_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("plugins")
            .with_description("Lists loaded plugins")
            .with_usage("/plugins")
            .with_alias("pl"),
        Box::new(PluginsCommand),
    )
}

// ---------------------------------------------------------------------------
// DifficultyCommand
// ---------------------------------------------------------------------------

/// Sets the world difficulty.
pub struct DifficultyCommand;

impl CommandExecutor for DifficultyCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/difficulty <peaceful|easy|normal|hard>".to_string()));
        }
        log::info!("Setting difficulty to {}", args[0]);
        Ok(())
    }
}

/// Builds the `/difficulty` command definition.
pub fn difficulty_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("difficulty")
            .with_description("Sets the world difficulty")
            .with_usage("/difficulty <peaceful|easy|normal|hard>")
            .with_permission("perust.command.difficulty")
            .with_alias("diff"),
        Box::new(DifficultyCommand),
    )
}

// ---------------------------------------------------------------------------
// SpawnpointCommand
// ---------------------------------------------------------------------------

/// Sets a player's spawn point.
pub struct SpawnpointCommand;

impl CommandExecutor for SpawnpointCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/spawnpoint [player] [x] [y] [z]".to_string()));
        }
        log::info!("Setting spawnpoint");
        Ok(())
    }
}

/// Builds the `/spawnpoint` command definition.
pub fn spawnpoint_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("spawnpoint")
            .with_description("Sets a player's spawn point")
            .with_usage("/spawnpoint [player] [x] [y] [z]")
            .with_permission("perust.command.spawnpoint")
            .with_alias("setspawn"),
        Box::new(SpawnpointCommand),
    )
}

// ---------------------------------------------------------------------------
// KillCommand
// ---------------------------------------------------------------------------

/// Kills a player or entity.
pub struct KillCommand;

impl CommandExecutor for KillCommand {
    fn execute(&self, _sender: &CommandSender, _command: &Command, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return Err(CommandError::InvalidUsage("/kill [player]".to_string()));
        }
        log::info!("Killing target: {}", args[0]);
        Ok(())
    }
}

/// Builds the `/kill` command definition.
pub fn kill_command() -> (Command, Box<dyn CommandExecutor>) {
    (
        Command::new("kill")
            .with_description("Kills a player or entity")
            .with_usage("/kill [player]")
            .with_permission("perust.command.kill"),
        Box::new(KillCommand),
    )
}

// ---------------------------------------------------------------------------
// Bulk registration helper
// ---------------------------------------------------------------------------

/// Registers all default commands with the given dispatcher.
pub fn register_defaults(dispatcher: &mut crate::dispatcher::CommandDispatcher) {
    dispatcher.register(help_command().0, help_command().1);
    dispatcher.register(stop_command().0, stop_command().1);
    dispatcher.register(list_command().0, list_command().1);
    dispatcher.register(gamemode_command().0, gamemode_command().1);
    dispatcher.register(give_command().0, give_command().1);
    dispatcher.register(teleport_command().0, teleport_command().1);
    dispatcher.register(op_command().0, op_command().1);
    dispatcher.register(deop_command().0, deop_command().1);
    dispatcher.register(kick_command().0, kick_command().1);
    dispatcher.register(say_command().0, say_command().1);
    dispatcher.register(time_command().0, time_command().1);
    dispatcher.register(whitelist_command().0, whitelist_command().1);
    dispatcher.register(ban_command().0, ban_command().1);
    dispatcher.register(pardon_command().0, pardon_command().1);
    dispatcher.register(seed_command().0, seed_command().1);
    dispatcher.register(version_command().0, version_command().1);
    dispatcher.register(plugins_command().0, plugins_command().1);
    dispatcher.register(difficulty_command().0, difficulty_command().1);
    dispatcher.register(spawnpoint_command().0, spawnpoint_command().1);
    dispatcher.register(kill_command().0, kill_command().1);
}
