# PeRust Protocol Documentation

This document describes the MCPE v113 (Minecraft Bedrock Edition v1.1.7) protocol implementation in PeRust, including the underlying RakNet transport layer.

---

## 1. RakNet Protocol

RakNet is the UDP-based transport protocol used by Minecraft Bedrock Edition. PeRust implements RakNet protocol version 6.

### 1.1 Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `RAKNET_PROTOCOL_VERSION` | 6 | RakNet protocol version for Bedrock |
| `RAKNET_MAGIC` | `00 ff ff 00 fe fe fe fe fd fd fd fd 12 34 56 78` | Identifies valid RakNet packets |
| `MIN_MTU_SIZE` | 400 | Minimum MTU during negotiation |
| `DEFAULT_MTU_SIZE` | 1492 | Default maximum MTU |

### 1.2 Connection Handshake

The RakNet connection is established through a multi-step handshake:

```
Client                              Server
  │                                    │
  │──── UnconnectedPing ────────────▶  │  (server discovery)
  │◀─── UnconnectedPong ────────────│  (MOTD response)
  │                                    │
  │──── OpenConnectionRequest1 ─────▶  │  (propose MTU)
  │◀─── OpenConnectionReply1 ────────│  (accept MTU)
  │                                    │
  │──── OpenConnectionRequest2 ─────▶  │  (confirm address + MTU)
  │◀─── OpenConnectionReply2 ────────│  (confirm client address)
  │                                    │
  │──── ConnectionRequest ──────────▶  │  (finalize connection)
  │◀─── ConnectionRequestAccepted ──│  (accept connection)
  │                                    │
  │──── NewIncomingConnection ──────▶  │  (client confirms)
  │                                    │
  │═══ Connected Session ══════════════│
```

### 1.3 RakNet Packet IDs

| ID | Name | Description |
|----|------|-------------|
| `0x00` | `ID_CONNECTED_PING` | Latency measurement on established connection |
| `0x01` | `ID_UNCONNECTED_PING` | Server discovery ping (before connection) |
| `0x02` | `ID_UNCONNECTED_PING_OPEN_CONNECTIONS` | Ping only if server has open slots |
| `0x03` | `ID_CONNECTED_PONG` | Response to ConnectedPing |
| `0x04` | `ID_DETECT_LOST_CONNECTIONS` | Probe for lost connections |
| `0x05` | `ID_OPEN_CONNECTION_REQUEST_1` | First handshake step |
| `0x06` | `ID_OPEN_CONNECTION_REPLY_1` | Server response to OCR1 |
| `0x07` | `ID_OPEN_CONNECTION_REQUEST_2` | Second handshake step |
| `0x08` | `ID_OPEN_CONNECTION_REPLY_2` | Server response to OCR2 |
| `0x09` | `ID_CONNECTION_REQUEST` | Finalize connection |
| `0x10` | `ID_CONNECTION_REQUEST_ACCEPTED` | Server accepts connection |
| `0x13` | `ID_NEW_INCOMING_CONNECTION` | Client confirms connection |
| `0x15` | `ID_DISCONNECTION_NOTIFICATION` | Graceful disconnect |
| `0x16` | `ID_CONNECTION_LOST` | Connection lost |
| `0x19` | `ID_INCOMPATIBLE_PROTOCOL_VERSION` | Protocol version mismatch |
| `0x1c` | `ID_UNCONNECTED_PONG` | Server discovery response |
| `0x1d` | `ID_ADVERTISE_SYSTEM` | Server advertisement broadcast |
| `0x84` | `ID_DATAGRAM` | Datagram carrying encapsulated packets |
| `0xc0` | `ID_ACK` | Acknowledgment |
| `0xa0` | `ID_NACK` | Negative acknowledgment |

### 1.4 Packet Structures

#### UnconnectedPing

| Field | Type | Description |
|-------|------|-------------|
| Packet ID | `u8` | `0x01` |
| Time | `i64` BE | Client timestamp |
| Magic | `[u8; 16]` | RakNet magic bytes |
| Client GUID | `i64` BE | Client unique ID |

#### UnconnectedPong

| Field | Type | Description |
|-------|------|-------------|
| Packet ID | `u8` | `0x1c` |
| Time | `i64` BE | Echoed client timestamp |
| Server GUID | `i64` BE | Server unique ID |
| Magic | `[u8; 16]` | RakNet magic bytes |
| MOTD Length | `u16` BE | Length of MOTD string |
| MOTD | `UTF-8 bytes` | Server MOTD |

#### OpenConnectionRequest1

| Field | Type | Description |
|-------|------|-------------|
| Packet ID | `u8` | `0x05` |
| Protocol | `u8` | RakNet protocol version (6) |
| Magic | `[u8; 16]` | RakNet magic bytes |
| Padding | `[u8; N]` | Zero-padded to MTU size |

#### OpenConnectionReply1

| Field | Type | Description |
|-------|------|-------------|
| Packet ID | `u8` | `0x06` |
| Magic | `[u8; 16]` | RakNet magic bytes |
| Server GUID | `i64` BE | Server unique ID |
| Use Security | `u8` | 0 = no encryption |
| MTU Size | `u16` BE | Negotiated MTU |

#### OpenConnectionRequest2

| Field | Type | Description |
|-------|------|-------------|
| Packet ID | `u8` | `0x07` |
| Magic | `[u8; 16]` | RakNet magic bytes |
| Server Address | `SocketAddress` | Server address + port |
| MTU Size | `u16` BE | Negotiated MTU |
| Client GUID | `i64` BE | Client unique ID |

#### OpenConnectionReply2

| Field | Type | Description |
|-------|------|-------------|
| Packet ID | `u8` | `0x08` |
| Magic | `[u8; 16]` | RakNet magic bytes |
| Server GUID | `i64` BE | Server unique ID |
| Client Address | `SocketAddress` | Client address as seen by server |
| MTU Size | `u16` BE | Negotiated MTU |
| Use Security | `u8` | 0 = no encryption |

#### SocketAddress Encoding

| Field | Type | Description |
|-------|------|-------------|
| Version | `u8` | 4 = IPv4, 6 = IPv6 |
| IP | `[u8; 4]` or `[u8; 16]` | IP address bytes |
| Port | `u16` BE | Port number |

### 1.5 Reliable Delivery

Connected sessions use encapsulated packets with reliability levels:

- **Reliable** — Guaranteed delivery via ACK/NACK
- **ReliableOrdered** — Guaranteed delivery + ordering
- **Unreliable** — No guarantee, no overhead

ACK/NACK packets use run-length encoding for efficient sequence number representation:

```
ACK/NACK:
  u8:  packet ID (0xc0 or 0xa0)
  u16: record count (big-endian)
  For each record:
    u8:  is_range (0 = single, 1 = range)
    u24: start sequence number
    u24: end sequence number (only if is_range)
```

---

## 2. MCPE Protocol (v113)

### 2.1 Protocol Constants

| Constant | Value |
|----------|-------|
| Protocol Version | 113 |
| Minecraft Version | 1.1.7 |
| Accepted Protocols | 110, 111, 112, 113 |

### 2.2 Packet Flow

#### Login Flow

```
Client                              Server
  │                                    │
  │──── LoginPacket ────────────────▶  │
  │◀─── ServerToClientHandshake ─────│  (encryption start, if enabled)
  │──── ClientToServerHandshake ────▶  │
  │◀─── PlayStatusPacket ────────────│  (login success)
  │◀─── ResourcePacksInfo ───────────│  (resource pack list)
  │──── ResourcePackClientResponse ─▶  │
  │◀─── ResourcePackStack ───────────│  │
  │──── ResourcePackClientResponse ─▶  │
  │◀─── StartGamePacket ─────────────│  │
  │◀─── PlayerListPacket ────────────│  │
  │──── RequestChunkRadius ─────────▶  │
  │◀─── ChunkRadiusUpdated ──────────│  │
  │◀─── FullChunkDataPacket × N ─────│  │
  │                                    │
  │═══ Gameplay ═══════════════════════│
```

#### Gameplay

During gameplay, the following packet types are exchanged:

- **Client → Server**: `MovePlayerPacket`, `PlayerActionPacket`, `TextPacket`, `InventoryTransactionPacket`, `InteractPacket`, `UseItemPacket`, `RequestChunkRadiusPacket`
- **Server → Client**: `MovePlayerPacket`, `TextPacket`, `SetTimePacket`, `UpdateBlockPacket`, `AddPlayerPacket`, `AddEntityPacket`, `RemoveEntityPacket`, `FullChunkDataPacket`, `SetEntityDataPacket`, `SetHealthPacket`, `ContainerSetContentPacket`, `PlayerListPacket`

#### Disconnection

```
Client                              Server
  │                                    │
  │──── DisconnectPacket ───────────▶  │  (or)
  │◀─── DisconnectPacket ────────────│
  │                                    │
  │──── DisconnectionNotification ──▶  │  (RakNet level)
```

### 2.3 MCPE Packet IDs

| ID | Name | Direction | Description |
|----|------|-----------|-------------|
| `0x01` | `LoginPacket` | C→S | Client login data |
| `0x02` | `PlayStatusPacket` | S→C | Login status / spawn state |
| `0x03` | `ServerToClientHandshake` | S→C | Encryption handshake |
| `0x04` | `ClientToServerHandshake` | C→S | Encryption response |
| `0x05` | `DisconnectPacket` | Both | Disconnect notification |
| `0x06` | `ResourcePacksInfoPacket` | S→C | Available resource packs |
| `0x07` | `ResourcePackStackPacket` | S→C | Resource pack stack order |
| `0x08` | `ResourcePackClientResponse` | C→S | Client resource pack response |
| `0x09` | `TextPacket` | Both | Chat / tip / popup messages |
| `0x0a` | `SetTimePacket` | S→C | World time update |
| `0x0b` | `StartGamePacket` | S→C | Game start / spawn data |
| `0x0c` | `AddPlayerPacket` | S→C | Player entity spawn |
| `0x0d` | `AddEntityPacket` | S→C | Non-player entity spawn |
| `0x0e` | `RemoveEntityPacket` | S→C | Entity despawn |
| `0x0f` | `AddItemEntityPacket` | S→C | Dropped item entity |
| `0x12` | `MoveEntityPacket` | S→C | Entity position update |
| `0x13` | `MovePlayerPacket` | Both | Player position update |
| `0x16` | `UpdateBlockPacket` | S→C | Block change notification |
| `0x18` | `ExplodePacket` | S→C | Explosion effect |
| `0x19` | `LevelSoundEventPacket` | Both | Sound event |
| `0x1a` | `LevelEventPacket` | S→C | Level-wide event (particles, etc.) |
| `0x1b` | `BlockEventPacket` | S→C | Block event (chest open, etc.) |
| `0x1c` | `EntityEventPacket` | Both | Entity event (hurt animation, etc.) |
| `0x1d` | `MobEffectPacket` | S→C | Add/remove status effect |
| `0x1e` | `UpdateAttributesPacket` | S→C | Entity attribute update |
| `0x1f` | `MobEquipmentPacket` | C→S | Held item change |
| `0x20` | `MobArmorEquipmentPacket` | S→C | Armor update |
| `0x21` | `InteractPacket` | C→S | Player interaction |
| `0x23` | `UseItemPacket` | C→S | Use item action |
| `0x24` | `PlayerActionPacket` | C→S | Player action (jump, sneak, etc.) |
| `0x27` | `SetEntityDataPacket` | S→C | Entity metadata update |
| `0x28` | `SetEntityMotionPacket` | S→C | Entity velocity update |
| `0x2a` | `SetHealthPacket` | S→C | Player health update |
| `0x2b` | `SetSpawnPositionPacket` | S→C | Spawn point update |
| `0x2c` | `AnimatePacket` | S→C | Animation (arm swing, etc.) |
| `0x2d` | `RespawnPacket` | S→C | Player respawn |
| `0x2f` | `InventoryActionPacket` | C→S | Inventory action |
| `0x30` | `ContainerOpenPacket` | S→C | Open container UI |
| `0x31` | `ContainerClosePacket` | Both | Close container UI |
| `0x32` | `ContainerSetSlotPacket` | S→C | Set slot content |
| `0x33` | `ContainerSetDataPacket` | S→C | Set container property |
| `0x34` | `ContainerSetContentPacket` | S→C | Set all container contents |
| `0x35` | `CraftingDataPacket` | S→C | Crafting recipe list |
| `0x37` | `AdventureSettingsPacket` | S→C | Player flags (flying, etc.) |
| `0x3a` | `FullChunkDataPacket` | S→C | Complete chunk data |
| `0x3b` | `SetCommandsEnabledPacket` | S→C | Enable/disable commands |
| `0x3c` | `SetDifficultyPacket` | S→C | World difficulty |
| `0x3d` | `ChangeDimensionPacket` | S→C | Dimension change |
| `0x3e` | `SetPlayerGameTypePacket` | S→C | Player gamemode change |
| `0x3f` | `PlayerListPacket` | S→C | Player list (tab) update |
| `0x45` | `RequestChunkRadiusPacket` | C→S | Client requests chunk radius |
| `0x46` | `ChunkRadiusUpdatedPacket` | S→C | Server confirms chunk radius |
| `0x4e` | `AvailableCommandsPacket` | S→C | Command list for client |
| `0x59` | `SetTitlePacket` | S→C | Title / subtitle / action bar |
| `0xfe` | `BatchPacket` | S→C | Compressed batch of packets |

### 2.4 Key Packet Structures

#### TextPacket (0x09)

| Field | Type | Description |
|-------|------|-------------|
| Type | `u8` | 0=Raw, 1=Chat, 2=Translation, 3=Popup, 4=JukeboxPopup, 5=Tip, 6=System, 7=Whisper |
| Source | `String` | Message source (player name or empty) |
| Message | `String` | Message content |
| Parameters | `Vec<String>` | Translation parameters |
| XUID | `String` | Player XUID |
| Platform Chat ID | `String` | Platform identifier |

#### MovePlayerPacket (0x13)

| Field | Type | Description |
|-------|------|-------------|
| Runtime ID | `VarLong` | Entity runtime ID |
| X | `f32` LE | Position X |
| Y | `f32` LE | Position Y |
| Z | `f32` LE | Position Z |
| Yaw | `f32` LE | Body rotation |
| Pitch | `f32` LE | Vertical rotation |
| Head Yaw | `f32` LE | Head rotation |
| Mode | `u8` | 0=Normal, 1=Reset, 2=Teleport, 3=Rotation |
| On Ground | `bool` | Whether on ground |

#### FullChunkDataPacket (0x3a)

| Field | Type | Description |
|-------|------|-------------|
| Chunk X | `VarInt` | Chunk X coordinate |
| Chunk Z | `VarInt` | Chunk Z coordinate |
| Data | `byte[]` | Serialized chunk data |

**Chunk data format** (as serialized by `Chunk::serialize_network`):

```
1 byte:    sub-chunk count
[sub-chunks × count]:
  1 byte:    version (0x00)
  4096 bytes: block IDs
  2048 bytes: block data (nibble array)
  2048 bytes: sky light (nibble array)
  2048 bytes: block light (nibble array)
512 bytes:  height map (256 × u16 big-endian)
256 bytes:  biome array
VarInt:     extra data count (currently 0)
```

### 2.5 Serialization Format

#### VarInt / VarLong

PeRust uses the standard Minecraft VarInt/VarLong encoding:

- Each byte uses 7 bits for data and 1 bit as a continuation flag
- Least significant group first (little-endian byte order)
- `VarInt`: up to 5 bytes for `i32`
- `VarLong`: up to 10 bytes for `i64`

Example:
```
Value 300:
  Binary: 00000010 01011000
  VarInt: 0xAC 0x02  (10101100 00000010)
```

#### Strings

Strings are encoded as:
```
VarUInt: byte length
bytes:   UTF-8 encoded string data
```

#### BatchPacket (0xfe)

The `BatchPacket` compresses multiple MCPE packets into a single datagram:

```
u32:     compressed payload length (little-endian)
bytes:   zlib-compressed payload containing:
  [For each contained packet]:
    VarUInt: packet length
    bytes:   packet data (with header byte)
```

All MCPE packets sent during gameplay are wrapped in a `BatchPacket` and compressed with zlib (level 7) before being handed to the RakNet layer for reliable delivery.

---

## 3. NBT (Named Binary Tag)

PeRust implements a complete NBT reader/writer supporting both big-endian and little-endian formats.

### 3.1 Tag Types

| ID | Type | Description |
|----|------|-------------|
| 0 | `End` | End of compound tag |
| 1 | `Byte` | Signed 8-bit integer |
| 2 | `Short` | Signed 16-bit integer (BE) |
| 3 | `Int` | Signed 32-bit integer (BE) |
| 4 | `Long` | Signed 64-bit integer (BE) |
| 5 | `Float` | 32-bit IEEE 754 (BE) |
| 6 | `Double` | 64-bit IEEE 754 (BE) |
| 7 | `ByteArray` | Length-prefixed byte array |
| 8 | `String` | Length-prefixed UTF-8 string |
| 9 | `List` | Homogeneous list of tags |
| 10 | `Compound` | Ordered map of named tags |
| 11 | `IntArray` | Length-prefixed int array |
| 12 | `LongArray` | Length-prefixed long array |

### 3.2 Encoding

**Big-endian** — Used in Java Edition and some Bedrock structures (level.dat, player data)

**Little-endian** — Used in Bedrock Edition network NBT and some file formats

The `Endianness` enum controls the byte order for multi-byte numeric types.

### 3.3 Reading/Writing

```rust
// Writing
let mut writer = NbtWriter::new(Endianness::BigEndian);
writer.write_compound("root", &compound_tag);
let bytes = writer.into_bytes();

// Reading
let mut reader = NbtReader::new(&bytes, Endianness::BigEndian);
let result = reader.read_compound().unwrap();
```
