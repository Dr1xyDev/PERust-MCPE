use crate::error::InventoryError;
use crate::inventory::Inventory;
use crate::inventory_type::InventoryType;
use crate::item_stack::ItemStack;

/// The number of main inventory slots (9 hotbar + 27 backpack).
const MAIN_SIZE: usize = 36;
/// The number of armor slots.
const ARMOR_SIZE: usize = 4;
/// The number of offhand slots.
const OFFHAND_SIZE: usize = 1;
/// The total number of slots.
const TOTAL_SIZE: usize = MAIN_SIZE + ARMOR_SIZE + OFFHAND_SIZE;

/// The index where armor slots start in the internal slot array.
const ARMOR_OFFSET: usize = MAIN_SIZE;
/// The index where the offhand slot is in the internal slot array.
const OFFHAND_OFFSET: usize = MAIN_SIZE + ARMOR_SIZE;

/// Player inventory with main slots, armor, offhand, and held item tracking.
pub struct PlayerInventory {
    slots: Vec<Option<ItemStack>>,
    held_item_slot: usize,
    max_stack_size: u8,
}

impl PlayerInventory {
    /// Creates a new empty player inventory.
    pub fn new() -> Self {
        Self {
            slots: vec![None; TOTAL_SIZE],
            held_item_slot: 0,
            max_stack_size: 64,
        }
    }

    /// Returns the currently held item (from the hotbar).
    pub fn get_held_item(&self) -> Option<&ItemStack> {
        self.slots.get(self.held_item_slot).and_then(|opt| opt.as_ref())
    }

    /// Returns the currently held item as mutable.
    pub fn get_held_item_mut(&mut self) -> Option<&mut ItemStack> {
        self.slots.get_mut(self.held_item_slot).and_then(|opt| opt.as_mut())
    }

    /// Sets the held item slot (hotbar index 0-8).
    pub fn set_held_item(&mut self, slot: usize) -> Result<(), InventoryError> {
        if slot > 8 {
            return Err(InventoryError::SlotOutOfRange(slot, 9));
        }
        self.held_item_slot = slot;
        Ok(())
    }

    /// Returns the held item slot index (0-8).
    pub fn held_item_slot(&self) -> usize {
        self.held_item_slot
    }

    /// Gets an armor item by armor slot index (0=helmet, 1=chestplate, 2=leggings, 3=boots).
    pub fn get_armor(&self, armor_slot: usize) -> Option<&ItemStack> {
        if armor_slot >= ARMOR_SIZE {
            return None;
        }
        self.slots.get(ARMOR_OFFSET + armor_slot).and_then(|opt| opt.as_ref())
    }

    /// Sets an armor item by armor slot index.
    pub fn set_armor(&mut self, armor_slot: usize, item: Option<ItemStack>) {
        if armor_slot < ARMOR_SIZE {
            self.slots[ARMOR_OFFSET + armor_slot] = item;
        }
    }

    /// Gets the offhand item.
    pub fn get_offhand(&self) -> Option<&ItemStack> {
        self.slots.get(OFFHAND_OFFSET).and_then(|opt| opt.as_ref())
    }

    /// Sets the offhand item.
    pub fn set_offhand(&mut self, item: Option<ItemStack>) {
        self.slots[OFFHAND_OFFSET] = item;
    }

    /// Gets a hotbar slot by index (0-8).
    pub fn get_hotbar_slot(&self, index: usize) -> Option<&ItemStack> {
        if index > 8 {
            return None;
        }
        self.slots.get(index).and_then(|opt| opt.as_ref())
    }

    /// Sets a hotbar slot by index.
    pub fn set_hotbar_slot(&mut self, index: usize, item: Option<ItemStack>) {
        if index <= 8 {
            self.slots[index] = item;
        }
    }

    /// Returns the main inventory slots (indices 0-35).
    pub fn main_slots(&self) -> &[Option<ItemStack>] {
        &self.slots[..MAIN_SIZE]
    }

    /// Returns the armor slots (indices 36-39 internally).
    pub fn armor_slots(&self) -> &[Option<ItemStack>] {
        &self.slots[ARMOR_OFFSET..ARMOR_OFFSET + ARMOR_SIZE]
    }

    /// Converts an internal slot index to a protocol window slot index.
    ///
    /// In MCPE protocol, the slot mapping differs:
    /// - Main inventory: 0-35 (same as internal)
    /// - Armor: 36-39 (same as internal offset)
    /// - Offhand: varies by implementation
    pub fn slot_to_protocol(&self, slot: usize) -> i32 {
        slot as i32
    }

    /// Converts a protocol slot index to an internal slot index.
    pub fn protocol_to_slot(&self, protocol_slot: i32) -> usize {
        protocol_slot as usize
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl Inventory for PlayerInventory {
    fn inventory_type(&self) -> InventoryType {
        InventoryType::Player
    }

    fn size(&self) -> usize {
        TOTAL_SIZE
    }

    fn get_item(&self, slot: usize) -> Option<&ItemStack> {
        self.slots.get(slot).and_then(|opt| opt.as_ref())
    }

    fn set_item(&mut self, slot: usize, item: Option<ItemStack>) {
        if slot < self.slots.len() {
            self.slots[slot] = item;
        }
    }

    fn clear(&mut self) {
        for slot in &mut self.slots {
            *slot = None;
        }
    }

    fn contains(&self, item: &ItemStack) -> bool {
        self.slots.iter().any(|slot| {
            slot.as_ref().map_or(false, |s| s == item)
        })
    }

    fn first_empty(&self) -> Option<usize> {
        // Only look in main inventory slots (0-35), not armor/offhand
        self.slots[..MAIN_SIZE].iter().position(|slot| slot.is_none())
    }

    fn add_item(&mut self, mut item: ItemStack) -> Option<ItemStack> {
        if item.is_air() {
            return None;
        }

        let max_stack = self.max_stack_size.min(item.max_stack_size());

        // First, try to stack with existing items of the same type (main inventory only)
        for slot in &mut self.slots[..MAIN_SIZE] {
            if let Some(existing) = slot {
                if *existing == item && existing.count < max_stack {
                    let space = max_stack - existing.count;
                    if item.count <= space {
                        existing.count += item.count;
                        return None;
                    } else {
                        existing.count = max_stack;
                        item.count -= space;
                    }
                }
            }
        }

        // Then, fill empty slots (main inventory only)
        for slot in &mut self.slots[..MAIN_SIZE] {
            if slot.is_none() {
                if item.count <= max_stack {
                    *slot = Some(item.with_count(item.count));
                    return None;
                } else {
                    *slot = Some(item.with_count(max_stack));
                    item.count -= max_stack;
                }
            }
        }

        // Return leftover items
        if item.count > 0 {
            Some(item)
        } else {
            None
        }
    }

    fn remove_item(&mut self, item: &ItemStack) -> bool {
        for slot in &mut self.slots[..MAIN_SIZE] {
            if let Some(existing) = slot {
                if existing == item {
                    if existing.count > 1 {
                        existing.count -= 1;
                    } else {
                        *slot = None;
                    }
                    return true;
                }
            }
        }
        false
    }

    fn max_stack_size(&self) -> u8 {
        self.max_stack_size
    }

    fn slots(&self) -> &[Option<ItemStack>] {
        &self.slots
    }
}
