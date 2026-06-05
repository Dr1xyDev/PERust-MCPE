/// Types of inventories in the game, corresponding to different container types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryType {
    Chest,
    DoubleChest,
    Player,
    Crafting,
    Workbench,
    Furnace,
    EnchantTable,
    BrewingStand,
    Anvil,
    Dispenser,
    Dropper,
    Hopper,
    EnderChest,
    Beacon,
    ShulkerBox,
}

impl InventoryType {
    /// Returns the default number of slots for this inventory type.
    pub fn default_size(&self) -> usize {
        match self {
            InventoryType::Chest => 27,
            InventoryType::DoubleChest => 54,
            InventoryType::Player => 36,
            InventoryType::Crafting => 4,
            InventoryType::Workbench => 9,
            InventoryType::Furnace => 3,
            InventoryType::EnchantTable => 2,
            InventoryType::BrewingStand => 5,
            InventoryType::Anvil => 3,
            InventoryType::Dispenser => 9,
            InventoryType::Dropper => 9,
            InventoryType::Hopper => 5,
            InventoryType::EnderChest => 27,
            InventoryType::Beacon => 1,
            InventoryType::ShulkerBox => 27,
        }
    }

    /// Returns the default max stack size for this inventory type.
    pub fn default_max_stack_size(&self) -> u8 {
        64
    }
}
