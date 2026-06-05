//! # perust-command
//!
//! Command system crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **Command**: Command definition with name, description, aliases, permissions, and sub-commands
//! - **CommandExecutor**: Trait for command execution logic
//! - **CommandSender**: Represents the source of a command (console or player)
//! - **CommandDispatcher**: Registry, parser, and dispatcher for commands
//! - **defaults**: Built-in command implementations (help, stop, list, etc.)

pub mod command;
pub mod sender;
pub mod dispatcher;
pub mod defaults;

pub use command::{Command, CommandExecutor, CommandResult, CommandError};
pub use sender::CommandSender;
pub use dispatcher::CommandDispatcher;
