//! Core plugin trait and context types.
//!
//! Every plugin must implement the [`Plugin`] trait. The server calls
//! `on_load`, `on_enable`, and `on_disable` at the appropriate lifecycle
//! points, passing a [`PluginContext`] that gives the plugin access to its
//! data folder, configuration, logger, and registration records.

use std::any::TypeId;
use std::path::PathBuf;

/// The core trait that all plugins must implement.
///
/// Lifecycle callbacks are called in the following order:
/// 1. `on_load` — the plugin has been discovered and its description parsed.
/// 2. `on_enable` — the plugin is being activated.
/// 3. `on_disable` — the plugin is being deactivated (server stop or reload).
pub trait Plugin: Send + Sync {
    /// Called when the plugin is first loaded (before enabling).
    ///
    /// Use this to read configuration and register event listeners.
    fn on_load(&mut self, context: &mut PluginContext);

    /// Called when the plugin is enabled.
    ///
    /// Use this to register commands and start services.
    fn on_enable(&mut self, context: &mut PluginContext);

    /// Called when the plugin is disabled.
    ///
    /// Use this to clean up resources and save state.
    fn on_disable(&mut self, context: &mut PluginContext);

    /// Returns the plugin's name.
    fn name(&self) -> &str;

    /// Returns the plugin's version string.
    fn version(&self) -> &str;
}

/// Runtime context provided to a plugin during lifecycle callbacks.
///
/// The context gives the plugin access to its private data folder, its
/// loaded configuration, a namespaced logger, and records of what it has
/// registered (commands and listeners).
pub struct PluginContext {
    /// The plugin's private data folder (e.g. `plugins/MyPlugin/`).
    pub data_folder: PathBuf,
    /// The plugin's configuration, if a config file was found.
    pub config: Option<serde_json::Value>,
    /// A logger that prefixes messages with the plugin name.
    pub logger: PluginLogger,
    /// Names of commands registered by this plugin.
    pub registered_commands: Vec<String>,
    /// TypeIds of event listeners registered by this plugin.
    pub registered_listeners: Vec<TypeId>,
}

impl PluginContext {
    /// Creates a new context for the given plugin name and data folder.
    pub fn new(plugin_name: &str, data_folder: PathBuf) -> Self {
        Self {
            data_folder,
            config: None,
            logger: PluginLogger {
                plugin_name: plugin_name.to_string(),
            },
            registered_commands: Vec::new(),
            registered_listeners: Vec::new(),
        }
    }

    /// Records that the plugin has registered a command with the given name.
    pub fn add_registered_command(&mut self, name: &str) {
        self.registered_commands.push(name.to_string());
    }

    /// Records that the plugin has registered a listener for the given event type.
    pub fn add_registered_listener<E: 'static>(&mut self) {
        self.registered_listeners.push(TypeId::of::<E>());
    }
}

/// A simple logger that prefixes messages with the plugin name.
///
/// Uses the `log` crate under the hood.
pub struct PluginLogger {
    /// The plugin name used as a log prefix.
    pub plugin_name: String,
}

impl PluginLogger {
    /// Logs at `info` level with the plugin name prefix.
    pub fn info(&self, msg: &str) {
        log::info!("[{}] {}", self.plugin_name, msg);
    }

    /// Logs at `warn` level with the plugin name prefix.
    pub fn warn(&self, msg: &str) {
        log::warn!("[{}] {}", self.plugin_name, msg);
    }

    /// Logs at `error` level with the plugin name prefix.
    pub fn error(&self, msg: &str) {
        log::error!("[{}] {}", self.plugin_name, msg);
    }

    /// Logs at `debug` level with the plugin name prefix.
    pub fn debug(&self, msg: &str) {
        log::debug!("[{}] {}", self.plugin_name, msg);
    }

    /// Logs at `trace` level with the plugin name prefix.
    pub fn trace(&self, msg: &str) {
        log::trace!("[{}] {}", self.plugin_name, msg);
    }
}
