use perust_protocol::types::ItemInstance;

/// Represents a stack of items in an inventory slot.
#[derive(Debug, Clone)]
pub struct ItemStack {
    /// The network ID of the item.
    pub item_id: i32,
    /// The auxiliary value (damage/variant).
    pub aux_value: i16,
    /// The number of items in this stack.
    pub count: u8,
    /// Serialized NBT data attached to this item stack.
    pub nbt: Option<Vec<u8>>,
}

impl ItemStack {
    /// Creates a new `ItemStack` with the given item ID, auxiliary value, count, and optional NBT.
    pub fn new(item_id: i32, aux_value: i16, count: u8, nbt: Option<Vec<u8>>) -> Self {
        Self {
            item_id,
            aux_value,
            count,
            nbt,
        }
    }

    /// Creates an air (empty) item stack.
    pub fn air() -> Self {
        Self {
            item_id: 0,
            aux_value: 0,
            count: 0,
            nbt: None,
        }
    }

    /// Returns `true` if this item stack represents air (empty).
    pub fn is_air(&self) -> bool {
        self.item_id == 0
    }

    /// Creates a new `ItemStack` with a different count.
    pub fn with_count(&self, count: u8) -> Self {
        Self {
            item_id: self.item_id,
            aux_value: self.aux_value,
            count,
            nbt: self.nbt.clone(),
        }
    }

    /// Creates a new `ItemStack` with a different damage (auxiliary) value.
    pub fn with_damage(&self, damage: i16) -> Self {
        Self {
            item_id: self.item_id,
            aux_value: damage,
            count: self.count,
            nbt: self.nbt.clone(),
        }
    }

    /// Converts this `ItemStack` into a protocol `ItemInstance` for network serialization.
    pub fn to_network_item(&self) -> ItemInstance {
        ItemInstance::new(self.item_id, self.aux_value, self.nbt.clone())
    }

    /// Returns the default maximum stack size for this item.
    ///
    /// In general, most items stack to 64. Some exceptions:
    /// - Air: 0
    /// - Tools/armor: 1
    /// - Ender pearls, snowballs, eggs: 16
    /// - Buckets, signs, etc.: 1
    ///
    /// This is a simplified implementation; the full registry in `perust-items`
    /// provides accurate values.
    pub fn max_stack_size(&self) -> u8 {
        if self.is_air() {
            return 0;
        }

        // Tools and armor (item IDs 256+ for tools, 298-317 for armor, etc.)
        if self.is_tool_or_armor() {
            return 1;
        }

        // Special items with stack size 16
        match self.item_id {
            332 => 16, // Snowball
            344 => 16, // Egg
            368 => 16, // Ender Pearl
            384 => 16, // Bottle o' Enchanting
            _ => 64,
        }
    }

    /// Returns `true` if this item is a tool or piece of armor (non-stackable).
    fn is_tool_or_armor(&self) -> bool {
        let id = self.item_id;
        // Tools: 256-259, 267-279, 283-286, 290-294, 346, 359, 398, 455
        // Armor: 298-317
        // Other non-stackable: 325 (bucket), 323 (sign), 328 (minecart), 333 (boat), 329 (saddle), 355 (bed)
        matches!(id,
            256..=259 | 267..=279 | 283..=286 | 290..=294 |
            298..=317 | 323 | 325 | 328 | 329 | 333 |
            346 | 355 | 359 | 398 | 416 | 426 | 450 | 455
        )
    }
}

impl PartialEq for ItemStack {
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id && self.aux_value == other.aux_value
    }
}

impl Eq for ItemStack {}

impl Default for ItemStack {
    fn default() -> Self {
        Self::air()
    }
}
