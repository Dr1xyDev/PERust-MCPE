# PeRust Architecture

This document describes the technical architecture of PeRust, a Minecraft Bedrock Edition server implemented in Rust.

---

## 1. Project Overview

PeRust is a modular, multi-crate Rust workspace that implements a Minecraft Bedrock Edition server targeting protocol version 113 (MCPE v1.1.7). The project is organized as a Cargo workspace with 18 internal crates and one binary crate, each responsible for a distinct subsystem.

The design philosophy emphasizes:
- **Modularity** — Each subsystem is an independent crate with a well-defined API
- **Safety** — Rust's ownership model and type system prevent data races and null pointer errors
- **Performance** — Zero-copy parsing, fine-grained locking, and efficient data structures
- **Extensibility** — Plugin API, event system, and command framework for runtime extension

---

## 2. Crate Dependency Graph

```
                    ┌─────────────┐
                    │   perust    │  (server binary)
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼──────┐  ┌─────▼─────┐  ┌──────▼──────┐
    │ perust-     │  │ perust-   │  │ perust-     │
    │ network     │  │ world     │  │ player      │
    └──┬──────┬───┘  └──┬────┬──┘  └──────┬──────┘
       │      │         │    │             │
  ┌────▼──┐  │    ┌────▼──┐ │      ┌──────▼──────┐
  │perust-│  │    │perust-│ │      │  perust-    │
  │raknet │  │    │ nbt   │ │      │ inventory   │
  └───────┘  │    └───────┘ │      └─────────────┘
  ┌──────▼───▼──┐    ┌──────▼──┐
  │  perust-    │    │ perust- │
  │  protocol   │    │ utils   │
  └──────┬──────┘    └─────────┘
         │
    ┌────▼────┐
    │ perust- │
    │  nbt    │
    └─────────┘

    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
    │  perust-     │  │  perust-     │  │  perust-     │
    │  command     │  │  event       │  │  plugin      │
    └──────────────┘  └──────────────┘  └──────────────┘

    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
    │  perust-     │  │  perust-     │  │  perust-     │
    │  scheduler   │  │  console     │  │  config      │
    └──────────────┘  └──────────────┘  └──────┬───────┘
                                               │
                                         ┌──────▼───────┐
                                         │  perust-     │
                                         │  storage     │
                                         └──────────────┘

    ┌──────────────┐  ┌──────────────┐
    │  perust-     │  │  perust-     │
    │  blocks      │  │  items       │
    └──────────────┘  └──────────────┘

    ┌──────────────┐
    │  perust-     │
    │  entity      │
    └──────────────┘
```

### Crate Summary

| Crate | Responsibility |
|-------|---------------|
| `perust-raknet` | RakNet protocol v6: connection handshake, reliable delivery, sessions |
| `perust-protocol` | MCPE v113 packet definitions, codec, protocol constants |
| `perust-network` | Network manager, session routing, packet handler callbacks |
| `perust-world` | World management, chunk columns, terrain generators, biomes, regions |
| `perust-entity` | Entity runtime ID allocation, entity metadata, attributes, effects |
| `perust-player` | Player state, login flow, permissions, player list |
| `perust-inventory` | Inventory types (player, container), item stacks, transactions |
| `perust-blocks` | Block registry, block state definitions, block IDs |
| `perust-items` | Item registry, item IDs, enchantments |
| `perust-command` | Command framework, dispatcher, built-in commands |
| `perust-event` | Event dispatcher, priority system, event definitions |
| `perust-plugin` | Plugin trait, plugin lifecycle manager, plugin description |
| `perust-scheduler` | Tick-based task scheduler, async task pool |
| `perust-nbt` | NBT reader/writer (big-endian & little-endian) |
| `perust-config` | Server properties, ops list, whitelist, ban list |
| `perust-storage` | YAML, JSON, NBT, and region-based storage backends |
| `perust-console` | Console input reader, colored logger |
| `perust-utils` | Binary I/O, VarInt, math, color, object pools, singletons |

---

## 3. Server Lifecycle

The server follows a well-defined lifecycle:

```
┌─────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Init    │───▶│  Start   │───▶│  Tick    │───▶│  Stop    │
│          │    │          │    │  Loop    │    │          │
└─────────┘    └──────────┘    └──────────┘    └──────────┘
     │              │               │               │
     ▼              ▼               ▼               ▼
  - Parse CLI   - Load world    - 20 TPS loop    - Fire
  - Init logger - Set MOTD      - Process pkts   ServerStop
  - Create dirs - Setup pkt     - Tick scheduler - Disable
  - Load config   handlers      - Update worlds    plugins
  - Build       - Start network- Update players  - Save
    Server      - Load plugins - Process console  worlds
  - Register    - Start console- Calculate TPS   - Save
    commands    - Fire            - Auto-save      config
                ServerStart                      - Stop
                                                 network
```

### Initialization (`main.rs`)

1. **Logger init** — `PeRustLogger` with configurable level filter
2. **Banner display** — ASCII art + version info
3. **CLI parsing** — `--port`, `--motd`, `--level`, etc.
4. **Data directories** — Create `data/`, `data/worlds/`, `data/plugins/`
5. **Config loading** — Load or create `data/server.properties`
6. **Server creation** — `Server::new(config).await` builds all subsystems
7. **Server start** — `server.start().await` begins accepting connections

### Tick Loop

The main loop runs at **20 TPS** (50ms per tick) using `tokio::time::interval`:

```rust
while running.load(Ordering::SeqCst) {
    tick_interval.tick().await;
    process_network_packets(...);
    server.tick();
}
```

Each tick:
1. **Network** — Flush session buffers, send queued packets
2. **Scheduler** — Execute due scheduled tasks
3. **Worlds** — Update each loaded world
4. **Players** — Send chunks, sync positions
5. **Console** — Process console input commands
6. **TPS** — Calculate current ticks per second
7. **Auto-save** — Every 6000 ticks (5 minutes)

### Shutdown

1. Fire `ServerStopEvent`
2. Disable all plugins
3. Save and unload worlds
4. Save configuration files
5. Disconnect all players
6. Stop network manager
7. Stop console reader

---

## 4. Network Architecture

### Stack Overview

```
┌─────────────────────────────────┐
│        Minecraft Client         │
└──────────────┬──────────────────┘
               │ UDP
┌──────────────▼──────────────────┐
│           RakNet Layer          │
│  (Connection, Reliability,      │
│   MTU Negotiation, Sessions)    │
├─────────────────────────────────┤
│        MCPE Protocol Layer      │
│  (Packet codec, BatchPacket,    │
│   Compression, Encryption)      │
├─────────────────────────────────┤
│       Packet Handler Layer      │
│  (Callbacks: on_login, on_move, │
│   on_text, on_inventory, etc.)  │
├─────────────────────────────────┤
│         Server Logic            │
│  (World, Player, Entity,        │
│   Command, Event, Plugin)       │
└─────────────────────────────────┘
```

### RakNet Layer (`perust-raknet`)

The RakNet layer handles the UDP-based reliable transport:

- **Server** (`server.rs`) — Binds UDP socket, receives/sends raw datagrams, manages the server GUID and MOTD
- **SessionManager** (`session_manager.rs`) — Tracks all active sessions, routes packets to the correct session
- **Session** (`session.rs`) — Per-client state: sequence numbers, ACK/NACK tracking, reliable/ordered delivery queues
- **Protocol** (`protocol.rs`) — Packet structures: `UnconnectedPing/Pong`, `OpenConnectionRequest/Reply1/2`, `ConnectionRequest/Accepted`, `NewIncomingConnection`, `Datagram`, `ACK/NACK`
- **Encapsulated** (`encapsulated.rs`) — Encapsulated packet framing within datagrams
- **Reliability** (`reliability.rs`) — Reliability levels: Reliable, ReliableOrdered, Unreliable, etc.

### MCPE Protocol Layer (`perust-protocol`)

Handles Minecraft-specific packet encoding/decoding:

- **Packet** (`packet.rs`) — Base `Packet` trait with `encode`/`decode` methods
- **Codec** (`codec.rs`) — Batch packet decompression/decoding, VarInt reading
- **Protocol Info** (`protocol_info.rs`) — Constants: protocol version 113, packet IDs
- **Types** (`types.rs`) — Shared types: `TextPacketType`, `GameType`, etc.
- **Packets** — Organized by domain:
  - `login.rs` — `LoginPacket`, `PlayStatusPacket`, handshake packets
  - `player.rs` — `MovePlayerPacket`, `PlayerActionPacket`
  - `text.rs` — `TextPacket` (chat, raw, tip, popup)
  - `world.rs` — `FullChunkDataPacket`, `SetTimePacket`, `UpdateBlockPacket`
  - `entity.rs` — `AddEntityPacket`, `AddPlayerPacket`, `SetEntityDataPacket`
  - `inventory.rs` — `InventoryTransactionPacket`, `ContainerClosePacket`
  - `resource_pack.rs` — `ResourcePacksInfoPacket`, `ResourcePackResponsePacket`
  - `misc.rs` — Various utility packets

### Network Manager (`perust-network`)

Ties RakNet and MCPE protocol together:

- **NetworkManager** — Holds the RakNet server, routes packets, provides broadcast APIs
- **NetworkSession** — Per-player session state (login phase, compression, encryption)
- **PacketHandler** — Callback-based handler system:
  - `on_login` — Player login attempt
  - `on_move` — Player movement
  - `on_text` — Chat/command messages
  - `on_player_action` — Player actions (jump, sneak, etc.)
  - `on_inventory_transaction` — Inventory changes
  - `on_chunk_radius_request` — Chunk loading requests
  - `on_resource_pack_response` — Resource pack ACK
  - `on_container_close` — Container UI close

---

## 5. World / Chunk System

### World (`perust-world`)

A `World` represents a dimension (Overworld, Nether, End):

- Holds a `ChunkManager` for loaded chunks
- Has a `Generator` for terrain generation
- Tracks spawn position, time, weather state
- Supports `load()` and `save()` operations
- Factory methods: `World::normal()`, `World::flat()`, `World::void()`

### Chunk (`perust-world::chunk`)

A chunk column is 16×256×16 blocks:

```
Chunk
 ├── sub_chunks[0..16]  — 16 SubChunks (16×16×16 each)
 │    ├── block_ids[4096]     — 1 byte per block
 │    ├── block_data[2048]    — nibble (4-bit) packed
 │    ├── sky_light[2048]     — nibble packed
 │    └── block_light[2048]   — nibble packed
 ├── height_map[256]    — highest non-air Y+1 per column
 ├── biomes[256]        — biome ID per column
 └── extra_data         — HashMap for extended block metadata
```

**Block indexing**: XZY ordering: `(x << 8) | (z << 4) | y`

**Nibble packing**: Two 4-bit values per byte. High nibble for odd indices, low nibble for even.

**Network serialization**: Sub-chunks are serialized with a version byte + raw data, followed by height map (u16 BE), biome array, and extra data count.

### Generators (`perust-world::generator`)

Three built-in generators:

| Generator | Description |
|-----------|-------------|
| `FlatGenerator` | Configurable flat layers (default: bedrock + dirt + grass) |
| `VoidGenerator` | Completely empty world (all air, Void biome) |
| `NormalGenerator` | Noise-based terrain with multi-octave value noise, biome selection |

The `NormalGenerator` uses a hash-based value noise with bilinear interpolation and 4 octaves. Biome selection is based on temperature/rainfall noise.

---

## 6. Entity System

### Entity Manager (`perust-entity`)

- Allocates monotonically increasing runtime entity IDs (starting from 1)
- Thread-safe via `AtomicU64`

### Entity Types

| Type | Description |
|------|-------------|
| `Entity` | Base entity: position, rotation, motion, on-ground flag |
| `Living` | Extends Entity: health, max health, damage ticks |
| `PlayerEntity` | Extends Living: player-specific data |

### Entity Metadata

Metadata entries typed as:
- `Byte(i8)`, `Short(i16)`, `Int(i32)`, `Float(f32)`
- `String(String)`, `Compound(NBT)`, `BlockPos(i32,i32,i32)`
- `Long(i64)`, `Vec3(f32,f32,f32)`

### Attributes

Standard Minecraft attributes: `health`, `movement_speed`, `follow_range`, `knockback_resistance`, `attack_damage`, `absorption`, `luck`.

### Effects

Status effects: `Speed`, `Slowness`, `Haste`, `MiningFatigue`, `Strength`, `InstantHealth`, `InstantDamage`, `JumpBoost`, `Nausea`, `Regeneration`, `Resistance`, `FireResistance`, `WaterBreathing`, `Invisibility`, `Blindness`, `NightVision`, `Hunger`, `Weakness`, `Poison`, `Wither`, `HealthBoost`, `Absorption`, `Saturation`, `Levitation`.

---

## 7. Plugin API Design

### Plugin Trait

Every plugin implements `Plugin`:

```rust
pub trait Plugin: Send + Sync {
    fn on_load(&mut self, context: &mut PluginContext);
    fn on_enable(&mut self, context: &mut PluginContext);
    fn on_disable(&mut self, context: &mut PluginContext);
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}
```

### Plugin Context

`PluginContext` provides:
- `data_folder: PathBuf` — Plugin's private data directory
- `config: Option<serde_json::Value>` — Loaded configuration
- `logger: PluginLogger` — Namespaced logger (`[PluginName] message`)
- `registered_commands: Vec<String>` — Track registered commands
- `registered_listeners: Vec<TypeId>` — Track registered event listeners

### Plugin Manager

`PluginManager` handles:
- Scanning the `data/plugins/` directory
- Loading plugin manifests (`plugin.yml`)
- Instantiating plugins via dynamic loading
- Calling lifecycle methods (`load_all`, `enable_all`, `disable_all`)

### Plugin Description (`plugin.yml`)

```yaml
name: MyPlugin
version: "1.0.0"
author: AuthorName
description: Plugin description
main: MyPlugin    # Struct name implementing Plugin trait
api: ["0.1.0"]   # Compatible PeRust API versions
```

---

## 8. Thread Model

```
┌──────────────────────────────────────┐
│         Tokio Async Runtime          │
│  ┌────────────────────────────────┐  │
│  │        Main Tick Loop          │  │
│  │  (20 TPS, drives all logic)    │  │
│  └────────────────────────────────┘  │
│                                      │
│  ┌──────────────┐  ┌──────────────┐  │
│  │   Network    │  │   Console    │  │
│  │   Receive    │  │   Input      │  │
│  │   Task       │  │   Task       │  │
│  └──────────────┘  └──────────────┘  │
└──────────────────────────────────────┘

┌──────────────────────────────────────┐
│        Scheduler Async Pool          │
│  (Async tasks run on thread pool)    │
└──────────────────────────────────────┘
```

### Concurrency Strategy

- **Main tick loop** — Single-threaded, sequential processing of game logic
- **Network I/O** — Tokio async tasks for UDP send/receive
- **Console input** — Separate thread reading from stdin
- **Player map** — `DashMap` for lock-free concurrent access
- **Shared state** — `Arc<RwLock<T>>` for read-heavy, `Arc<Mutex<T>>` for exclusive access
- **Entity IDs** — `AtomicU64` for lock-free ID allocation
- **Async tasks** — `Scheduler` dispatches to a thread pool for non-tick work

### Synchronization Points

| Resource | Lock Type | Reason |
|----------|-----------|--------|
| `ServerProperties` | `RwLock` | Read frequently, write rarely |
| `World` | `Mutex` | Chunk access needs exclusive access |
| `CommandDispatcher` | `Mutex` | Commands registered once, dispatched often |
| `EventDispatcher` | `Mutex` | Handlers registered once, dispatched often |
| `PluginManager` | `Mutex` | Load once, read during events |
| `Player map` | `DashMap` | Lock-free concurrent player lookups |
| `Entity IDs` | `AtomicU64` | Lock-free ID generation |

---

## 9. Performance Optimizations

### Build Profile

The release profile is aggressively optimized:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

- **Fat LTO** — Cross-crate inlining and dead code elimination
- **Single codegen unit** — Better optimization at the cost of compile time
- **Strip** — Remove debug symbols for smaller binaries
- **Abort on panic** — Avoids unwinding overhead

### Data Structure Choices

- **Nibble arrays** — Block data, sky light, and block light use 4 bits per value (50% memory savings vs 1 byte per value)
- **`DashMap`** — Lock-free concurrent hash map for player lookups
- **`parking_lot`** — Smaller, faster mutexes than `std::sync`
- **`indexmap::IndexMap`** — Ordered hash map for deterministic NBT compound iteration
- **`Vec<Option<SubChunk>>`** — Sparse sub-chunk storage; `None` means all-air (no allocation)

### Network

- **BatchPacket** — Multiple MCPE packets compressed into one datagram
- **VarInt encoding** — Compact integer encoding for protocol fields
- **Zero-copy parsing** — Packet reading uses byte slices without copying where possible
- **Async I/O** — Tokio-based UDP handling avoids blocking the tick loop

### Chunk System

- **Lazy sub-chunk allocation** — Sub-chunks are only allocated when a block is set
- **Height map caching** — Avoids scanning columns for the highest block
- **Dirty flag** — Only save chunks that have been modified
- **Network serialization** — Direct binary writing with pre-allocated buffers
