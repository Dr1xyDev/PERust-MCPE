//! Command definition and execution traits.
//!
//! A [`Command`] holds metadata (name, description, aliases, permissions, sub-commands)
//! while a [`CommandExecutor`] provides the runtime behaviour.

use thiserror::Error;

/// Errors that can occur during command execution.
#[derive(Debug, Error)]
pub enum CommandError {
    /// The command was not found.
    #[error("Command not found: {0}")]
    NotFound(String),

    /// The sender does not have permission to run the command.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// The command was used incorrectly (wrong arguments, etc.).
    #[error("Usage: {0}")]
    InvalidUsage(String),

    /// A generic runtime error produced by a command.
    #[error("{0}")]
    General(String),
}

/// The result type returned by command execution.
pub type CommandResult = Result<(), CommandError>;

/// A command definition containing metadata and sub-commands.
///
/// Commands are constructed using the builder pattern:
///
/// ```rust,ignore
/// let cmd = Command::new("gamemode")
///     .with_description("Change the game mode")
///     .with_usage("/gamemode <mode> [player]")
///     .with_permission("perust.command.gamemode")
///     .add_sub_command(Command::new("survival"))
///     .add_sub_command(Command::new("creative"));
/// ```
#[derive(Debug, Clone)]
pub struct Command {
    /// The primary name of the command (e.g. `"help"`).
    pub name: String,
    /// A short description shown in the help listing.
    pub description: String,
    /// Usage string (e.g. `"/give <player> <item> [amount]"`).
    pub usage: String,
    /// Alternative names that map to this command.
    pub aliases: Vec<String>,
    /// Optional permission node required to execute this command.
    pub permission: Option<String>,
    /// Sub-commands (e.g. `whitelist add`, `whitelist remove`).
    pub sub_commands: Vec<Command>,
}

impl Command {
    /// Creates a new command with the given name and empty defaults.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            description: String::new(),
            usage: String::new(),
            aliases: Vec::new(),
            permission: None,
            sub_commands: Vec::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Sets the usage string.
    pub fn with_usage(mut self, usage: &str) -> Self {
        self.usage = usage.to_string();
        self
    }

    /// Adds an alias.
    pub fn with_alias(mut self, alias: &str) -> Self {
        self.aliases.push(alias.to_lowercase());
        self
    }

    /// Sets the required permission node.
    pub fn with_permission(mut self, perm: &str) -> Self {
        self.permission = Some(perm.to_string());
        self
    }

    /// Adds a sub-command.
    pub fn add_sub_command(mut self, cmd: Command) -> Self {
        self.sub_commands.push(cmd);
        self
    }
}

/// Trait for command execution logic.
///
/// Implementors provide the behaviour that runs when a command is invoked.
pub trait CommandExecutor: Send + Sync {
    /// Executes the command.
    ///
    /// - `sender`: The source of the command (console or player).
    /// - `command`: The command definition that was matched.
    /// - `args`: The arguments passed after the command name.
    fn execute(&self, sender: &crate::sender::CommandSender, command: &Command, args: &[String]) -> CommandResult;
}
