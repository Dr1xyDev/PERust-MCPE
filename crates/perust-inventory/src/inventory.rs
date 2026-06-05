use crate::error::InventoryError;
use crate::inventory_type::InventoryType;
use crate::item_stack::ItemStack;

/// Trait representing a generic inventory that can hold items in slots.
pub trait Inventory: Send + Sync {
    /// Returns the type of this inventory.
    fn inventory_type(&self) -> InventoryType;

    /// Returns the number of slots in this inventory.
    fn size(&self) -> usize;

    /// Gets a reference to the item in the given slot, if any.
    fn get_item(&self, slot: usize) -> Option<&ItemStack>;

    /// Sets the item in the given slot. Passing `None` clears the slot.
    fn set_item(&mut self, slot: usize, item: Option<ItemStack>);

    /// Clears all slots in this inventory.
    fn clear(&mut self);

    /// Returns `true` if this inventory contains at least one of the specified item.
    fn contains(&self, item: &ItemStack) -> bool;

    /// Returns the index of the first empty slot, or `None` if the inventory is full.
    fn first_empty(&self) -> Option<usize>;

    /// Attempts to add an item to the inventory.
    ///
    /// First tries to stack with existing items of the same type, then fills
    /// empty slots. Returns `Some(leftover)` if not all items could be added,
    /// or `None` if all items were added successfully.
    fn add_item(&mut self, item: ItemStack) -> Option<ItemStack>;

    /// Removes one instance of the specified item from the inventory.
    ///
    /// Returns `true` if an item was successfully removed.
    fn remove_item(&mut self, item: &ItemStack) -> bool;

    /// Returns the maximum stack size for this inventory.
    fn max_stack_size(&self) -> u8;

    /// Returns a slice of all slots in this inventory.
    fn slots(&self) -> &[Option<ItemStack>];
}

/// A basic inventory implementation with a fixed number of slots.
pub struct BaseInventory {
    inventory_type: InventoryType,
    slots: Vec<Option<ItemStack>>,
    max_stack_size: u8,
    viewers: Vec<u64>,
}

impl BaseInventory {
    /// Creates a new `BaseInventory` with the given type and size.
    pub fn new(inventory_type: InventoryType) -> Self {
        let size = inventory_type.default_size();
        Self {
            inventory_type,
            slots: vec![None; size],
            max_stack_size: inventory_type.default_max_stack_size(),
            viewers: Vec::new(),
        }
    }

    /// Creates a new `BaseInventory` with a custom size.
    pub fn with_size(inventory_type: InventoryType, size: usize) -> Self {
        Self {
            inventory_type,
            slots: vec![None; size],
            max_stack_size: inventory_type.default_max_stack_size(),
            viewers: Vec::new(),
        }
    }

    /// Adds a viewer (player runtime ID) to the inventory.
    pub fn add_viewer(&mut self, runtime_id: u64) {
        if !self.viewers.contains(&runtime_id) {
            self.viewers.push(runtime_id);
        }
    }

    /// Removes a viewer from the inventory.
    pub fn remove_viewer(&mut self, runtime_id: u64) {
        self.viewers.retain(|&id| id != runtime_id);
    }

    /// Returns a reference to the list of viewers.
    pub fn viewers(&self) -> &[u64] {
        &self.viewers
    }

    /// Sets the maximum stack size for this inventory.
    pub fn set_max_stack_size(&mut self, size: u8) {
        self.max_stack_size = size;
    }

    fn validate_slot(&self, slot: usize) -> Result<(), InventoryError> {
        if slot >= self.slots.len() {
            Err(InventoryError::SlotOutOfRange(slot, self.slots.len()))
        } else {
            Ok(())
        }
    }
}

impl Inventory for BaseInventory {
    fn inventory_type(&self) -> InventoryType {
        self.inventory_type
    }

    fn size(&self) -> usize {
        self.slots.len()
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
        self.slots.iter().position(|slot| slot.is_none())
    }

    fn add_item(&mut self, mut item: ItemStack) -> Option<ItemStack> {
        if item.is_air() {
            return None;
        }

        let max_stack = self.max_stack_size.min(item.max_stack_size());

        // First, try to stack with existing items of the same type
        for slot in &mut self.slots {
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

        // Then, fill empty slots
        for slot in &mut self.slots {
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
        for slot in &mut self.slots {
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
