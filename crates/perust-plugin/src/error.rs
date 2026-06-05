//! Plugin error types.

use thiserror::Error;

/// Errors that can occur during plugin operations.
#[derive(Debug, Error)]
pub enum PluginError {
    /// The requested plugin was not found.
    #[error("Plugin not found: {0}")]
    NotFound(String),

    /// The plugin description file is invalid or missing required fields.
    #[error("Invalid plugin description: {0}")]
    InvalidDescription(String),

    /// The plugin failed to load.
    #[error("Failed to load plugin '{name}': {reason}")]
    LoadError {
        /// Plugin name.
        name: String,
        /// Failure reason.
        reason: String,
    },

    /// A required dependency is missing.
    #[error("Missing dependency '{dependency}' required by plugin '{plugin}'")]
    DependencyMissing {
        /// The plugin that requires the dependency.
        plugin: String,
        /// The missing dependency name.
        dependency: String,
    },

    /// A circular dependency was detected.
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    /// The plugin is already loaded.
    #[error("Plugin already loaded: {0}")]
    AlreadyLoaded(String),
}
