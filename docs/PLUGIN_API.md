# PeRust Plugin API

This document describes the PeRust plugin system — how to create, configure, and distribute plugins that extend the server's functionality.

---

## 1. Overview

PeRust plugins are dynamically loaded modules that implement the `Plugin` trait. They have access to the full server API through a `PluginContext`, including:

- **Event handling** — Listen for and respond to server events
- **Command registration** — Add custom commands to the server
- **Scheduler** — Schedule tasks to run at specific tick intervals
- **World access** — Read and modify world data
- **Player management** — Interact with online players
- **Configuration** — Load and save plugin-specific configuration

---

## 2. Creating a Plugin

### 2.1 Implement the Plugin Trait

Every plugin must implement the `perust_plugin::Plugin` trait:

```rust
use perust_plugin::{Plugin, PluginContext};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    /// Called when the plugin is first discovered and loaded.
    /// Use this to read configuration and prepare resources.
    fn on_load(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("MyPlugin is loading...");
    }

    /// Called when the plugin is enabled.
    /// Use this to register commands, event listeners, and start services.
    fn on_enable(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("MyPlugin is enabled!");
    }

    /// Called when the plugin is disabled (server stop or reload).
    /// Use this to clean up resources and save state.
    fn on_disable(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("MyPlugin is disabled!");
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
```

### 2.2 Lifecycle

Plugins follow a three-phase lifecycle:

```
┌──────────┐     ┌────────────┐     ┌─────────────┐
│  on_load  │────▶│  on_enable │────▶│  on_disable │
└──────────┘     └────────────┘     └─────────────┘
   Discovery       Activation          Deactivation
   & parsing       & registration      & cleanup
```

1. **`on_load`** — The plugin has been discovered and its `plugin.yml` parsed. Use this to:
   - Read configuration files
   - Initialize data structures
   - Validate dependencies

2. **`on_enable`** — The plugin is being activated. Use this to:
   - Register commands
   - Register event listeners
   - Schedule tasks
   - Start background services

3. **`on_disable`** — The plugin is being deactivated. Use this to:
   - Save state and configuration
   - Clean up resources
   - Unregister listeners (automatic)

---

## 3. Plugin Manifest (plugin.yml)

Each plugin must include a `plugin.yml` file in its root directory. This file describes the plugin to the server:

```yaml
# Required fields
name: MyPlugin              # Plugin identifier (must match the struct name)
version: "1.0.0"            # Semantic version string
main: MyPlugin              # The struct name implementing the Plugin trait

# Optional fields
author: YourName            # Author name
description: A cool plugin  # Short description
api: ["0.1.0"]              # Compatible PeRust API versions

# Dependencies
depend: []                  # Required dependencies (plugin names)
softdepend: []              # Optional dependencies
loadbefore: []              # Plugins that should load after this one

# Permissions (optional)
permissions:
  myplugin.use:
    description: Allows using MyPlugin features
    default: op
  myplugin.admin:
    description: Allows admin MyPlugin commands
    default: op
```

### Plugin Directory Structure

```
data/plugins/
└── MyPlugin/
    ├── plugin.yml          # Plugin manifest
    ├── config.yml          # Plugin configuration (optional)
    └── data/               # Plugin data directory (created automatically)
```

---

## 4. PluginContext

The `PluginContext` is passed to each lifecycle method and provides access to server APIs:

```rust
pub struct PluginContext {
    /// The plugin's private data folder (e.g., data/plugins/MyPlugin/data/)
    pub data_folder: PathBuf,

    /// The plugin's configuration, if a config file was found
    pub config: Option<serde_json::Value>,

    /// A logger that prefixes messages with the plugin name
    pub logger: PluginLogger,

    /// Names of commands registered by this plugin
    pub registered_commands: Vec<String>,

    /// TypeIds of event listeners registered by this plugin
    pub registered_listeners: Vec<TypeId>,
}
```

### 4.1 PluginLogger

The logger automatically prefixes all messages with `[PluginName]`:

```rust
ctx.logger.info("This prints as: [MyPlugin] This prints as:");
ctx.logger.warn("Warning message");
ctx.logger.error("Error message");
ctx.logger.debug("Debug message (only in debug builds)");
ctx.logger.trace("Trace message (only in trace builds)");
```

### 4.2 Tracking Registrations

The context tracks what the plugin has registered for cleanup:

```rust
// Record a command registration
ctx.add_registered_command("mycommand");

// Record an event listener registration
ctx.add_registered_listener::<PlayerJoinEvent>();
```

---

## 5. Event Handling

### 5.1 Event System Overview

PeRust uses a priority-based event dispatcher. Events are dispatched in priority order:

| Priority | Value | Use Case |
|----------|-------|----------|
| `Lowest` | 0 | Setting defaults |
| `Low` | 1 | Early processing |
| `Normal` | 2 | Default handling |
| `High` | 3 | Override default behavior |
| `Highest` | 4 | Final modifications |
| `Monitor` | 5 | Read-only observation |

### 5.2 Registering an Event Listener

```rust
use perust_event::{EventDispatcher, EventPriority, Event};
use perust_event::events::PlayerJoinEvent;

fn register_listeners(event_dispatcher: &mut EventDispatcher) {
    event_dispatcher.register::<PlayerJoinEvent>(
        EventPriority::Normal,
        Box::new(|event| {
            let e = event.downcast_mut::<PlayerJoinEvent>().unwrap();
            e.join_message = Some(format!("Welcome, {}!", e.player_name));
        })
    );
}
```

### 5.3 Cancellable Events

Some events can be cancelled to prevent the default action:

```rust
use perust_event::events::PlayerLoginEvent;

event_dispatcher.register::<PlayerLoginEvent>(
    EventPriority::Normal,
    Box::new(|event| {
        let e = event.downcast_mut::<PlayerLoginEvent>().unwrap();
        if e.player_name == "BannedPlayer" {
            e.cancel.set_cancelled(true);
            e.kick_message = Some("You are not allowed!".to_string());
        }
    })
);
```

### 5.4 Available Events

#### Server Events
| Event | Cancellable | Description |
|-------|-------------|-------------|
| `ServerStartEvent` | No | Server has finished starting |
| `ServerStopEvent` | No | Server is about to stop |
| `DataPacketReceiveEvent` | No | Raw packet received |
| `DataPacketSendEvent` | No | Raw packet about to be sent |

#### Player Events
| Event | Cancellable | Description |
|-------|-------------|-------------|
| `PlayerLoginEvent` | Yes | Player attempts to log in |
| `PlayerJoinEvent` | No | Player has joined the server |
| `PlayerQuitEvent` | No | Player has left the server |
| `PlayerMoveEvent` | No | Player moved |
| `PlayerChatEvent` | Yes | Player sent a chat message |
| `PlayerDeathEvent` | No | Player died |
| `PlayerInteractEvent` | Yes | Player interacted |
| `PlayerBreakBlockEvent` | Yes | Player broke a block |
| `PlayerPlaceBlockEvent` | Yes | Player placed a block |

#### Entity Events
| Event | Cancellable | Description |
|-------|-------------|-------------|
| `EntityDamageEvent` | Yes | Entity took damage |
| `EntityDeathEvent` | No | Entity died |
| `EntitySpawnEvent` | No | Entity spawned |
| `EntityDespawnEvent` | No | Entity was removed |

#### Block Events
| Event | Cancellable | Description |
|-------|-------------|-------------|
| `BlockBreakEvent` | Yes | Block was broken |
| `BlockPlaceEvent` | Yes | Block was placed |
| `BlockUpdateEvent` | No | Block was updated |

#### Level Events
| Event | Cancellable | Description |
|-------|-------------|-------------|
| `ChunkLoadEvent` | No | Chunk was loaded |
| `ChunkUnloadEvent` | No | Chunk was unloaded |
| `LevelLoadEvent` | No | Level/dimension was loaded |

#### Inventory Events
| Event | Cancellable | Description |
|-------|-------------|-------------|
| `InventoryOpenEvent` | Yes | Inventory was opened |
| `InventoryCloseEvent` | No | Inventory was closed |
| `InventoryTransactionEvent` | Yes | Inventory transaction occurred |

---

## 6. Command Registration

### 6.1 Defining a Command

Commands are defined using the builder pattern:

```rust
use perust_command::{Command, CommandExecutor, CommandSender, CommandResult};

struct GreetCommand;

impl CommandExecutor for GreetCommand {
    fn execute(&self, sender: &CommandSender, command: &Command, args: &[String]) -> CommandResult {
        let name = args.first().map(|s| s.as_str()).unwrap_or("World");
        // In a real plugin, you would broadcast this message
        Ok(())
    }
}

// Register the command
let cmd = Command::new("greet")
    .with_description("Greet a player")
    .with_usage("/greet [player]")
    .with_permission("myplugin.greet")
    .with_alias("hello");

dispatcher.register(cmd, Box::new(GreetCommand));
```

### 6.2 Command Builder Methods

| Method | Description |
|--------|-------------|
| `Command::new(name)` | Create a command with the given name (lowercased) |
| `.with_description(desc)` | Set the help text description |
| `.with_usage(usage)` | Set the usage string (e.g., `/give <player> <item>`) |
| `.with_alias(alias)` | Add an alternative name |
| `.with_permission(perm)` | Set required permission node |
| `.add_sub_command(cmd)` | Add a sub-command |

### 6.3 Sub-Commands

```rust
let whitelist_cmd = Command::new("myplugin")
    .with_description("MyPlugin management")
    .add_sub_command(Command::new("reload").with_description("Reload configuration"))
    .add_sub_command(Command::new("status").with_description("Show plugin status"));
```

### 6.4 CommandSender

The `CommandSender` enum identifies who issued the command:

```rust
pub enum CommandSender {
    Console,                                          // Server console
    Player { runtime_id: u64, name: String },        // In-game player
}
```

Methods:
- `sender.is_console()` — Check if console
- `sender.is_player()` — Check if player
- `sender.name()` — Get display name
- `sender.has_permission(perm)` — Check permission (console always has all)
- `sender.runtime_id()` — Get player runtime ID (None for console)

### 6.5 Built-in Commands

PeRust includes these default commands:

| Command | Description | Permission |
|---------|-------------|------------|
| `help` | List available commands | — |
| `stop` | Stop the server | `perust.admin` |
| `list` | List online players | — |
| `say` | Broadcast a message | `perust.admin` |
| `op` | Grant operator status | `perust.admin` |
| `deop` | Revoke operator status | `perust.admin` |
| `whitelist` | Manage whitelist | `perust.admin` |
| `ban` | Ban a player | `perust.admin` |
| `kick` | Kick a player | `perust.admin` |
| `gamemode` | Change game mode | `perust.command.gamemode` |
| `tp` | Teleport a player | `perust.command.tp` |
| `time` | Set/query world time | `perust.command.time` |
| `seed` | Show world seed | `perust.command.seed` |
| `save-all` | Force save all worlds | `perust.admin` |

---

## 7. Scheduler Usage

### 7.1 Synchronous Tasks

Schedule tasks that run on the main tick thread:

```rust
use perust_scheduler::{Scheduler, FnTask};

let mut scheduler = Scheduler::new();

// Schedule a task to run on the next tick
let task = FnTask::new(|| {
    // This code runs on the main tick thread
    println!("Task executed!");
});
scheduler.schedule_task(Box::new(task));

// Each tick, collect and run due tasks
let tasks = scheduler.tick();
for mut task in tasks {
    task.run();
}
```

### 7.2 Async Tasks

For I/O-bound or long-running work, use the async task pool:

```rust
use perust_scheduler::AsyncPool;

let pool = AsyncPool::new(4); // 4 worker threads

pool.spawn(async move {
    // Long-running work here
    let data = fetch_data_from_disk().await;
    // Process data...
});
```

### 7.3 Delayed and Repeating Tasks

(TODO: Future API — planned for a future release)

```rust,ignore
// Run after 100 ticks (5 seconds)
scheduler.schedule_delayed(task, 100);

// Run every 20 ticks (1 second)
scheduler.schedule_repeating(task, 20);
```

---

## 8. World Access

### 8.1 Accessing Worlds

Plugins can access worlds through the server's world map:

```rust,ignore
// Access the default world
let world = server.default_world.lock();

// Get a chunk
let chunk = world.get_chunk(x, z);

// Get a block
let (block_id, block_data) = chunk.get_block(lx, y, lz);

// Set a block
chunk.set_block(lx, y, lz, new_block_id, new_block_data);
```

### 8.2 Custom Generators

Implement the `Generator` trait to create custom terrain:

```rust
use perust_world::generator::Generator;
use perust_world::chunk::Chunk;

struct MyGenerator { seed: i64 }

impl Generator for MyGenerator {
    fn generate_chunk(&self, x: i32, z: i32) -> Chunk {
        let mut chunk = Chunk::new(x, z);
        // Custom terrain generation logic
        chunk
    }

    fn name(&self) -> &str {
        "my_generator"
    }
}
```

Built-in generators:
- `FlatGenerator` — Flat world with configurable layers
- `VoidGenerator` — Empty world
- `NormalGenerator` — Noise-based terrain with biomes

---

## 9. Full Example

Here is a complete plugin that greets players when they join and provides a `/greet` command:

```rust
use perust_plugin::{Plugin, PluginContext};
use perust_event::{EventDispatcher, EventPriority};
use perust_event::events::PlayerJoinEvent;
use perust_command::{Command, CommandDispatcher, CommandExecutor, CommandSender, CommandResult};

pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    fn on_load(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("GreetPlugin loading...");
    }

    fn on_enable(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("GreetPlugin enabled!");

        // Register the /greet command
        ctx.add_registered_command("greet");

        // Register the PlayerJoinEvent listener
        ctx.add_registered_listener::<PlayerJoinEvent>();
    }

    fn on_disable(&mut self, ctx: &mut PluginContext) {
        ctx.logger.info("GreetPlugin disabled!");
    }

    fn name(&self) -> &str { "GreetPlugin" }
    fn version(&self) -> &str { "1.0.0" }
}
```

Corresponding `plugin.yml`:

```yaml
name: GreetPlugin
version: "1.0.0"
author: Example
description: Greets players when they join
main: GreetPlugin
api: ["0.1.0"]
permissions:
  greetplugin.greet:
    description: Allows using the /greet command
    default: true
```

---

## 10. Best Practices

### 10.1 General

- **Keep `on_load` fast** — Don't do heavy I/O or network calls in `on_load`
- **Register in `on_enable`** — All commands and listeners should be registered in `on_enable`
- **Clean up in `on_disable`** — Save state, close files, cancel tasks
- **Use namespaced permissions** — Prefix with your plugin name (e.g., `myplugin.command.greet`)
- **Handle errors gracefully** — Don't panic; log errors and continue

### 10.2 Performance

- **Avoid long-running tasks on the main thread** — Use the async scheduler for I/O
- **Don't lock for long** — Hold locks for the minimum time necessary
- **Batch operations** — Modify multiple blocks/entries at once when possible
- **Cache frequently accessed data** — Don't re-read config every tick

### 10.3 Events

- **Use the correct priority** — `Monitor` for read-only, `Lowest` for defaults
- **Don't modify events in `Monitor`** — This priority is for observation only
- **Check cancellation** — In `Normal` priority and above, check if a previous handler cancelled the event
- **Be careful with cancellable events** — Cancelling player actions may cause client-server desync

### 10.4 Commands

- **Provide usage strings** — Help players understand how to use your commands
- **Validate arguments** — Check arg count and types before processing
- **Return meaningful errors** — Use `CommandError::InvalidUsage`, `PermissionDenied`, etc.
- **Support console and player** — Check `sender.is_player()` for player-only commands
