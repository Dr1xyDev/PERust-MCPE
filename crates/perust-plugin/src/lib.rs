//! # perust-plugin
//!
//! Plugin system crate for PeRust, a Minecraft Bedrock Edition server.
//!
//! This crate provides:
//! - **Plugin**: Core trait that all plugins must implement
//! - **PluginContext**: Runtime context provided to plugins (data folder, config, logger)
//! - **PluginDescription**: Metadata loaded from plugin descriptor files (JSON/YAML)
//! - **PluginManager**: Lifecycle manager for loading, enabling, and disabling plugins
//! - **PluginError**: Error types for plugin operations

pub mod error;
pub mod plugin;
pub mod plugin_description;
pub mod plugin_manager;

pub use error::PluginError;
pub use plugin::{Plugin, PluginContext, PluginLogger};
pub use plugin_description::{
    CommandEntry, PermissionDefault, PermissionEntry, PluginDescription, PluginLoadOrder,
};
pub use plugin_manager::PluginManager;
