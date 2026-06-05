//! Login state tracking for the player connection lifecycle.

/// Represents the current state of a player's login process.
///
/// The login flow progresses through these states in order:
/// 1. `Disconnected` → Player has no active connection
/// 2. `Connecting` → RakNet connection established, waiting for LoginPacket
/// 3. `LoggingIn` → LoginPacket received, processing authentication
/// 4. `ResourcePacks` → Waiting for resource pack response from client
/// 5. `Spawning` → StartGamePacket sent, waiting for client to acknowledge spawn
/// 6. `Playing` → Fully in-game, normal gameplay state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoginState {
    /// No active connection.
    Disconnected,
    /// RakNet connected, waiting for LoginPacket from client.
    Connecting,
    /// LoginPacket received, processing authentication.
    LoggingIn,
    /// Waiting for resource pack response from client.
    ResourcePacks,
    /// StartGamePacket sent, waiting for client to acknowledge spawn.
    Spawning,
    /// Fully in-game, normal gameplay.
    Playing,
}

impl LoginState {
    /// Returns `true` if the player is in a connected state (any state except Disconnected).
    pub fn is_connected(self) -> bool {
        self != LoginState::Disconnected
    }

    /// Returns `true` if the player is fully in-game.
    pub fn is_playing(self) -> bool {
        self == LoginState::Playing
    }

    /// Returns `true` if the player is still in the login process.
    pub fn is_logging_in(self) -> bool {
        matches!(
            self,
            LoginState::Connecting
                | LoginState::LoggingIn
                | LoginState::ResourcePacks
                | LoginState::Spawning
        )
    }
}

impl std::fmt::Display for LoginState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginState::Disconnected => write!(f, "Disconnected"),
            LoginState::Connecting => write!(f, "Connecting"),
            LoginState::LoggingIn => write!(f, "LoggingIn"),
            LoginState::ResourcePacks => write!(f, "ResourcePacks"),
            LoginState::Spawning => write!(f, "Spawning"),
            LoginState::Playing => write!(f, "Playing"),
        }
    }
}
