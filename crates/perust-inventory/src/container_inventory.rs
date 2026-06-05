use crate::inventory::BaseInventory;
use crate::inventory_type::InventoryType;

/// A container inventory for block-based containers (chest, furnace, etc.).
///
/// This wraps a `BaseInventory` and associates it with a specific block position
/// in the world.
pub struct ContainerInventory {
    inner: BaseInventory,
    /// The inventory window ID used in the protocol.
    window_id: u8,
}

impl ContainerInventory {
    /// Creates a new container inventory for the given inventory type.
    pub fn new(inventory_type: InventoryType, window_id: u8) -> Self {
        Self {
            inner: BaseInventory::new(inventory_type),
            window_id,
        }
    }

    /// Creates a new container inventory with a custom size.
    pub fn with_size(inventory_type: InventoryType, size: usize, window_id: u8) -> Self {
        Self {
            inner: BaseInventory::with_size(inventory_type, size),
            window_id,
        }
    }

    /// Returns the window ID for protocol communication.
    pub fn window_id(&self) -> u8 {
        self.window_id
    }

    /// Sets the window ID.
    pub fn set_window_id(&mut self, id: u8) {
        self.window_id = id;
    }

    /// Returns the inner `BaseInventory`.
    pub fn inner(&self) -> &BaseInventory {
        &self.inner
    }

    /// Returns a mutable reference to the inner `BaseInventory`.
    pub fn inner_mut(&mut self) -> &mut BaseInventory {
        &mut self.inner
    }
}

// Delegate Inventory trait to inner BaseInventory
impl crate::inventory::Inventory for ContainerInventory {
    fn inventory_type(&self) -> InventoryType {
        self.inner.inventory_type()
    }

    fn size(&self) -> usize {
        self.inner.size()
    }

    fn get_item(&self, slot: usize) -> Option<&crate::item_stack::ItemStack> {
        self.inner.get_item(slot)
    }

    fn set_item(&mut self, slot: usize, item: Option<crate::item_stack::ItemStack>) {
        self.inner.set_item(slot, item)
    }

    fn clear(&mut self) {
        self.inner.clear()
    }

    fn contains(&self, item: &crate::item_stack::ItemStack) -> bool {
        self.inner.contains(item)
    }

    fn first_empty(&self) -> Option<usize> {
        self.inner.first_empty()
    }

    fn add_item(&mut self, item: crate::item_stack::ItemStack) -> Option<crate::item_stack::ItemStack> {
        self.inner.add_item(item)
    }

    fn remove_item(&mut self, item: &crate::item_stack::ItemStack) -> bool {
        self.inner.remove_item(item)
    }

    fn max_stack_size(&self) -> u8 {
        self.inner.max_stack_size()
    }

    fn slots(&self) -> &[Option<crate::item_stack::ItemStack>] {
        self.inner.slots()
    }
}
