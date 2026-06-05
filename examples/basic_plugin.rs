// Example PeRust plugin (conceptual)
// In a real scenario, this would be compiled as a dynamic library
// and placed in the data/plugins/ directory.
//
// This example demonstrates the basic plugin API pattern.

use perust_plugin::{Plugin, PluginContext};
use perust_event::EventPriority;

/// A simple example plugin that logs its lifecycle events.
struct MyPlugin;

impl Plugin for MyPlugin {
    /// Called when the plugin is first discovered and loaded.
    /// Use this to read configuration and prepare resources.
    fn on_load(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("MyPlugin loading...");

        // Read plugin configuration (if a config file exists)
        if let Some(ref config) = ctx.config {
            ctx.logger.info(&format!("Config loaded: {:?}", config));
        }
    }

    /// Called when the plugin is enabled.
    /// Use this to register commands, event listeners, and start services.
    fn on_enable(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("MyPlugin enabled!");

        // Register commands
        // In a real plugin, you would access the CommandDispatcher
        // through the server context and register custom commands.
        ctx.add_registered_command("mycommand");

        // Register event listeners
        // In a real plugin, you would access the EventDispatcher
        // and register handlers for specific event types.
        ctx.add_registered_listener::<perust_event::events::PlayerJoinEvent>();
    }

    /// Called when the plugin is disabled (server stop or reload).
    /// Use this to clean up resources and save state.
    fn on_disable(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("MyPlugin disabled!");
    }

    /// Returns the plugin's display name.
    fn name(&self) -> &str {
        "MyPlugin"
    }

    /// Returns the plugin's version string.
    fn version(&self) -> &str {
        "1.0.0"
    }
}

// In a real plugin, you would also need a plugin.yml manifest:
//
// name: MyPlugin
// version: "1.0.0"
// author: YourName
// description: A simple example plugin
// main: MyPlugin    # The struct name implementing the Plugin trait
// api: ["0.1.0"]    # Compatible PeRust API versions
