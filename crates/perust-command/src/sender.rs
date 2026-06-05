//! Command sender types.
//!
//! A [`CommandSender`] represents the origin of a command — either the server
//! console or an in-game player.

/// Identifies who issued a command.
#[derive(Debug, Clone)]
pub enum CommandSender {
    /// The server console (always has full permissions).
    Console,
    /// An in-game player identified by runtime ID and name.
    Player {
        /// The player's runtime entity ID.
        runtime_id: u64,
        /// The player's display name.
        name: String,
    },
}

impl CommandSender {
    /// Returns `true` if this sender is the console.
    pub fn is_console(&self) -> bool {
        matches!(self, CommandSender::Console)
    }

    /// Returns `true` if this sender is an in-game player.
    pub fn is_player(&self) -> bool {
        matches!(self, CommandSender::Player { .. })
    }

    /// Returns the display name of the sender.
    ///
    /// For console this is `"Console"`, for players it is their player name.
    pub fn name(&self) -> &str {
        match self {
            CommandSender::Console => "Console",
            CommandSender::Player { name, .. } => name,
        }
    }

    /// Checks whether the sender has the given permission node.
    ///
    /// The console always returns `true`. Players currently return `false`
    /// for any non-empty permission string unless they hold the `perust.admin`
    /// wildcard permission. A full permission system would replace this stub.
    pub fn has_permission(&self, perm: &str) -> bool {
        match self {
            CommandSender::Console => true,
            CommandSender::Player { .. } => {
                // Stub: in a full implementation this would check the player's
                // permission set. For now, operators get all permissions.
                perm.is_empty() || perm == "perust.admin"
            }
        }
    }

    /// Returns the player's runtime ID, or `None` if this is the console.
    pub fn runtime_id(&self) -> Option<u64> {
        match self {
            CommandSender::Console => None,
            CommandSender::Player { runtime_id, .. } => Some(*runtime_id),
        }
    }
}
