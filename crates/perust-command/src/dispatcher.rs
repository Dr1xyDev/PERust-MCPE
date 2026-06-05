//! Command dispatcher — registry, parsing, and execution of commands.
//!
//! The [`CommandDispatcher`] stores registered commands and their executors,
//! parses raw input strings, and dispatches to the correct executor.

use std::collections::HashMap;

use crate::command::{Command, CommandError, CommandExecutor, CommandResult};
use crate::sender::CommandSender;

/// Central registry and dispatcher for commands.
pub struct CommandDispatcher {
    /// Maps primary command names to (command definition, executor) pairs.
    commands: HashMap<String, (Command, Box<dyn CommandExecutor>)>,
    /// Maps alias strings to the primary command name they resolve to.
    aliases: HashMap<String, String>,
}

impl CommandDispatcher {
    /// Creates a new, empty `CommandDispatcher`.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    /// Registers a command and its executor.
    ///
    /// All aliases defined on the command are also registered so that they
    /// resolve to the same executor.
    pub fn register(&mut self, command: Command, executor: Box<dyn CommandExecutor>) {
        let name = command.name.clone();
        // Register aliases before moving the command.
        for alias in &command.aliases {
            self.aliases.insert(alias.clone(), name.clone());
        }
        self.commands.insert(name, (command, executor));
    }

    /// Dispatches a raw input string from the given sender.
    ///
    /// The input is split by whitespace. The first token is treated as the
    /// command name (checked against primary names and then aliases). Remaining
    /// tokens become the arguments list.
    ///
    /// # Errors
    ///
    /// Returns [`CommandError::NotFound`] if no matching command exists.
    /// Returns [`CommandError::PermissionDenied`] if the sender lacks the required permission.
    pub fn dispatch(&self, sender: &CommandSender, input: &str) -> CommandResult {
        let tokens: Vec<String> = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if tokens.is_empty() {
            return Err(CommandError::NotFound(String::new()));
        }

        let cmd_name = tokens[0].to_lowercase();
        let args: Vec<String> = tokens[1..].to_vec();

        // Resolve the command — try primary name first, then aliases.
        let resolved_name = if self.commands.contains_key(&cmd_name) {
            cmd_name.clone()
        } else if let Some(primary) = self.aliases.get(&cmd_name) {
            primary.clone()
        } else {
            return Err(CommandError::NotFound(cmd_name));
        };

        let (command, executor) = self.commands.get(&resolved_name).unwrap();

        // Permission check.
        if let Some(ref perm) = command.permission {
            if !perm.is_empty() && !sender.has_permission(perm) {
                return Err(CommandError::PermissionDenied(perm.clone()));
            }
        }

        executor.execute(sender, command, &args)
    }

    /// Returns a reference to a command by its primary name, or `None`.
    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name).map(|(cmd, _)| cmd)
    }

    /// Returns references to all registered commands.
    pub fn get_all_commands(&self) -> Vec<&Command> {
        self.commands.values().map(|(cmd, _)| cmd).collect()
    }

    /// Provides tab-completion suggestions for the given partial input.
    ///
    /// Currently returns the names of commands that start with the first token.
    /// If the first token fully matches a command, sub-command names are offered.
    pub fn tab_complete(&self, _sender: &CommandSender, input: &str) -> Vec<String> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {
            return self.commands.keys().cloned().collect();
        }

        let partial = tokens[0].to_lowercase();

        // If there is only one token (or the input ends with a space after it),
        // complete command names.
        if tokens.len() == 1 && !input.ends_with(' ') {
            let mut results: Vec<String> = self
                .commands
                .keys()
                .filter(|k| k.starts_with(&partial))
                .cloned()
                .collect();
            results.extend(
                self.aliases
                    .keys()
                    .filter(|k| k.starts_with(&partial))
                    .cloned(),
            );
            results.sort();
            results.dedup();
            return results;
        }

        // First token matches a command — offer sub-command names.
        let resolved = if let Some(primary) = self.aliases.get(&partial) {
            primary.clone()
        } else {
            partial
        };

        if let Some((command, _)) = self.commands.get(&resolved) {
            if tokens.len() == 2 || (tokens.len() == 1 && input.ends_with(' ')) {
                let sub_partial = if tokens.len() > 1 {
                    tokens[1].to_lowercase()
                } else {
                    String::new()
                };
                let mut subs: Vec<String> = command
                    .sub_commands
                    .iter()
                    .filter(|sc| sc.name.starts_with(&sub_partial))
                    .map(|sc| sc.name.clone())
                    .collect();
                subs.sort();
                return subs;
            }
        }

        Vec::new()
    }
}

impl Default for CommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
