use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::ItemInstance;
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// MobEquipmentPacket
// ============================================================================

/// Entity equipment (held item) packet.
#[derive(Debug, Clone)]
pub struct MobEquipmentPacket {
    pub entity_runtime_id: u64,
    pub item: ItemInstance,
    pub slot: i8,
    pub selected_slot: i8,
    pub window_id: i8,
}

impl Packet for MobEquipmentPacket {
    const PACKET_ID: u8 = protocol_info::MOB_EQUIPMENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        self.item.encode(writer);
        writer.write_i8(self.slot);
        writer.write_i8(self.selected_slot);
        writer.write_i8(self.window_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let item = ItemInstance::decode(reader)?;
        let slot = reader.read_i8()?;
        let selected_slot = reader.read_i8()?;
        let window_id = reader.read_i8()?;
        Ok(Self {
            entity_runtime_id,
            item,
            slot,
            selected_slot,
            window_id,
        })
    }

    fn packet_name(&self) -> &'static str {
        "MobEquipmentPacket"
    }
}

// ============================================================================
// MobArmorEquipmentPacket
// ============================================================================

/// Entity armor equipment packet.
#[derive(Debug, Clone)]
pub struct MobArmorEquipmentPacket {
    pub entity_runtime_id: u64,
    pub helmet: ItemInstance,
    pub chestplate: ItemInstance,
    pub leggings: ItemInstance,
    pub boots: ItemInstance,
}

impl Packet for MobArmorEquipmentPacket {
    const PACKET_ID: u8 = protocol_info::MOB_ARMOR_EQUIPMENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u64(self.entity_runtime_id);
        self.helmet.encode(writer);
        self.chestplate.encode(writer);
        self.leggings.encode(writer);
        self.boots.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let entity_runtime_id = reader.read_u64()?;
        let helmet = ItemInstance::decode(reader)?;
        let chestplate = ItemInstance::decode(reader)?;
        let leggings = ItemInstance::decode(reader)?;
        let boots = ItemInstance::decode(reader)?;
        Ok(Self {
            entity_runtime_id,
            helmet,
            chestplate,
            leggings,
            boots,
        })
    }

    fn packet_name(&self) -> &'static str {
        "MobArmorEquipmentPacket"
    }
}

// ============================================================================
// ContainerOpenPacket
// ============================================================================

/// Opens a container window.
#[derive(Debug, Clone)]
pub struct ContainerOpenPacket {
    pub window_id: i8,
    pub container_type: i8,
    pub slot_count: i32,
    pub entity_unique_id: i64,
}

impl Packet for ContainerOpenPacket {
    const PACKET_ID: u8 = protocol_info::CONTAINER_OPEN_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        writer.write_i8(self.container_type);
        writer.write_i32(self.slot_count);
        writer.write_i64(self.entity_unique_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let window_id = reader.read_i8()?;
        let container_type = reader.read_i8()?;
        let slot_count = reader.read_i32()?;
        let entity_unique_id = reader.read_i64()?;
        Ok(Self {
            window_id,
            container_type,
            slot_count,
            entity_unique_id,
        })
    }

    fn packet_name(&self) -> &'static str {
        "ContainerOpenPacket"
    }
}

// ============================================================================
// ContainerClosePacket
// ============================================================================

/// Closes a container window.
#[derive(Debug, Clone)]
pub struct ContainerClosePacket {
    pub window_id: i8,
}

impl Packet for ContainerClosePacket {
    const PACKET_ID: u8 = protocol_info::CONTAINER_CLOSE_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            window_id: reader.read_i8()?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "ContainerClosePacket"
    }
}

// ============================================================================
// ContainerSetSlotPacket
// ============================================================================

/// Sets a slot in a container.
#[derive(Debug, Clone)]
pub struct ContainerSetSlotPacket {
    pub window_id: i8,
    pub slot: i32,
    pub item: ItemInstance,
}

impl Packet for ContainerSetSlotPacket {
    const PACKET_ID: u8 = protocol_info::CONTAINER_SET_SLOT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        writer.write_i32(self.slot);
        self.item.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let window_id = reader.read_i8()?;
        let slot = reader.read_i32()?;
        let item = ItemInstance::decode(reader)?;
        Ok(Self { window_id, slot, item })
    }

    fn packet_name(&self) -> &'static str {
        "ContainerSetSlotPacket"
    }
}

// ============================================================================
// ContainerSetDataPacket
// ============================================================================

/// Sets data property in a container.
#[derive(Debug, Clone)]
pub struct ContainerSetDataPacket {
    pub window_id: i8,
    pub property: i32,
    pub value: i32,
}

impl Packet for ContainerSetDataPacket {
    const PACKET_ID: u8 = protocol_info::CONTAINER_SET_DATA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        writer.write_i32(self.property);
        writer.write_i32(self.value);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let window_id = reader.read_i8()?;
        let property = reader.read_i32()?;
        let value = reader.read_i32()?;
        Ok(Self { window_id, property, value })
    }

    fn packet_name(&self) -> &'static str {
        "ContainerSetDataPacket"
    }
}

// ============================================================================
// ContainerSetContentPacket
// ============================================================================

/// Sets the full content of a container.
#[derive(Debug, Clone)]
pub struct ContainerSetContentPacket {
    pub window_id: i8,
    pub items: Vec<ItemInstance>,
}

impl Packet for ContainerSetContentPacket {
    const PACKET_ID: u8 = protocol_info::CONTAINER_SET_CONTENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        writer.write_u32_le(self.items.len() as u32);
        for item in &self.items {
            item.encode(writer);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let window_id = reader.read_i8()?;
        let count = reader.read_u32_le()? as usize;
        let mut items = Vec::with_capacity(count.min(512));
        for _ in 0..count {
            items.push(ItemInstance::decode(reader)?);
        }
        Ok(Self { window_id, items })
    }

    fn packet_name(&self) -> &'static str {
        "ContainerSetContentPacket"
    }
}

// ============================================================================
// InventoryActionPacket
// ============================================================================

/// Inventory transaction action.
#[derive(Debug, Clone)]
pub struct InventoryActionPacket {
    pub action_id: u32,
    pub source_type: u32,
    pub window_id: i32,
    pub source_flags: u32,
    pub inventory_slot: u32,
    pub old_item: ItemInstance,
    pub new_item: ItemInstance,
}

impl Packet for InventoryActionPacket {
    const PACKET_ID: u8 = protocol_info::INVENTORY_ACTION_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_u32_le(self.action_id);
        writer.write_u32_le(self.source_type);
        writer.write_var_int(self.window_id);
        writer.write_u32_le(self.source_flags);
        writer.write_u32_le(self.inventory_slot);
        self.old_item.encode(writer);
        self.new_item.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let action_id = reader.read_u32_le()?;
        let source_type = reader.read_u32_le()?;
        let window_id = reader.read_var_int()?;
        let source_flags = reader.read_u32_le()?;
        let inventory_slot = reader.read_u32_le()?;
        let old_item = ItemInstance::decode(reader)?;
        let new_item = ItemInstance::decode(reader)?;
        Ok(Self {
            action_id,
            source_type,
            window_id,
            source_flags,
            inventory_slot,
            old_item,
            new_item,
        })
    }

    fn packet_name(&self) -> &'static str {
        "InventoryActionPacket"
    }
}

// ============================================================================
// DropItemPacket
// ============================================================================

/// Drop item from inventory.
#[derive(Debug, Clone)]
pub struct DropItemPacket {
    pub item: ItemInstance,
}

impl Packet for DropItemPacket {
    const PACKET_ID: u8 = protocol_info::DROP_ITEM_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.item.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            item: ItemInstance::decode(reader)?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "DropItemPacket"
    }
}

// ============================================================================
// CraftingDataPacket
// ============================================================================

/// Crafting recipe data.
#[derive(Debug, Clone)]
pub struct CraftingDataPacket {
    pub crafting_data: Vec<u8>, // Raw crafting data
    pub clean_recipes: bool,
}

impl Packet for CraftingDataPacket {
    const PACKET_ID: u8 = protocol_info::CRAFTING_DATA_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_bytes(&self.crafting_data);
        writer.write_bool(self.clean_recipes);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let remaining = reader.read_remaining().to_vec();
        // The clean_recipes bool is the last byte
        if remaining.is_empty() {
            return Err(ProtocolError::DecodeError("Empty CraftingDataPacket".to_string()));
        }
        let clean_recipes = remaining[remaining.len() - 1] != 0;
        let crafting_data = remaining[..remaining.len() - 1].to_vec();
        Ok(Self { crafting_data, clean_recipes })
    }

    fn packet_name(&self) -> &'static str {
        "CraftingDataPacket"
    }
}

// ============================================================================
// CraftingEventPacket
// ============================================================================

/// Crafting event (player crafts an item).
#[derive(Debug, Clone)]
pub struct CraftingEventPacket {
    pub window_id: i8,
    pub recipe_type: i32,
    pub recipe_id: String,
    pub input: Vec<ItemInstance>,
    pub result: Vec<ItemInstance>,
}

impl Packet for CraftingEventPacket {
    const PACKET_ID: u8 = protocol_info::CRAFTING_EVENT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        writer.write_i8(self.window_id);
        writer.write_i32(self.recipe_type);
        writer.write_string(&self.recipe_id);
        writer.write_u32_le(self.input.len() as u32);
        for item in &self.input {
            item.encode(writer);
        }
        writer.write_u32_le(self.result.len() as u32);
        for item in &self.result {
            item.encode(writer);
        }
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let window_id = reader.read_i8()?;
        let recipe_type = reader.read_i32()?;
        let recipe_id = reader.read_string_owned()?;
        let input_count = reader.read_u32_le()? as usize;
        let mut input = Vec::with_capacity(input_count.min(512));
        for _ in 0..input_count {
            input.push(ItemInstance::decode(reader)?);
        }
        let result_count = reader.read_u32_le()? as usize;
        let mut result = Vec::with_capacity(result_count.min(512));
        for _ in 0..result_count {
            result.push(ItemInstance::decode(reader)?);
        }
        Ok(Self {
            window_id,
            recipe_type,
            recipe_id,
            input,
            result,
        })
    }

    fn packet_name(&self) -> &'static str {
        "CraftingEventPacket"
    }
}

// ============================================================================
// AddItemPacket
// ============================================================================

/// Add item packet.
#[derive(Debug, Clone)]
pub struct AddItemPacket {
    pub item: ItemInstance,
}

impl Packet for AddItemPacket {
    const PACKET_ID: u8 = protocol_info::ADD_ITEM_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.item.encode(writer);
        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        Ok(Self {
            item: ItemInstance::decode(reader)?,
        })
    }

    fn packet_name(&self) -> &'static str {
        "AddItemPacket"
    }
}
