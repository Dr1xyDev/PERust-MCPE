use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

/// Represents a block state with its name, data value, and runtime ID mapping.
///
/// In the Bedrock protocol, block states are used to identify specific variants
/// of blocks (e.g., different colors of wool, directions of stairs, etc.).
/// The runtime ID is used in network packets for efficient transfer.
#[derive(Debug, Clone)]
pub struct BlockState {
    /// The namespaced name of the block (e.g., "minecraft:stone").
    pub name: String,
    /// The packed state data: (block_id << 4) | meta.
    pub state_data: u16,
    /// The runtime ID assigned to this block state for network transfer.
    pub runtime_id: u32,
}

impl BlockState {
    /// Creates a new block state.
    pub fn new(name: &str, block_id: u8, meta: u8, runtime_id: u32) -> Self {
        Self {
            name: name.to_string(),
            state_data: ((block_id as u16) << 4) | (meta as u16),
            runtime_id,
        }
    }

    /// Returns the block ID extracted from the state data.
    pub fn block_id(&self) -> u8 {
        (self.state_data >> 4) as u8
    }

    /// Returns the meta value extracted from the state data.
    pub fn meta(&self) -> u8 {
        (self.state_data & 0x0F) as u8
    }
}

/// Global block state registry.
pub static BLOCK_STATE_REGISTRY: Lazy<RwLock<BlockStateRegistry>> = Lazy::new(|| {
    let registry = BlockStateRegistry::new();
    RwLock::new(registry)
});

/// Registry mapping between block states and runtime IDs.
pub struct BlockStateRegistry {
    states: Vec<BlockState>,
    state_data_to_runtime: HashMap<u16, u32>,
    runtime_to_state_data: HashMap<u32, u16>,
}

impl BlockStateRegistry {
    /// Creates a new block state registry with vanilla block states registered.
    pub fn new() -> Self {
        let mut registry = Self {
            states: Vec::new(),
            state_data_to_runtime: HashMap::new(),
            runtime_to_state_data: HashMap::new(),
        };
        registry.register_vanilla_states();
        registry
    }

    /// Registers a block state.
    pub fn register(&mut self, state: BlockState) {
        self.state_data_to_runtime.insert(state.state_data, state.runtime_id);
        self.runtime_to_state_data.insert(state.runtime_id, state.state_data);
        self.states.push(state);
    }

    /// Gets the runtime ID for a block ID and meta combination.
    pub fn get_runtime_id(&self, block_id: u8, meta: u8) -> Option<u32> {
        let state_data = ((block_id as u16) << 4) | (meta as u16);
        self.state_data_to_runtime.get(&state_data).copied()
    }

    /// Gets the block ID and meta from a runtime ID.
    pub fn get_from_runtime_id(&self, runtime_id: u32) -> Option<(u8, u8)> {
        self.runtime_to_state_data.get(&runtime_id).map(|&state_data| {
            ((state_data >> 4) as u8, (state_data & 0x0F) as u8)
        })
    }

    /// Returns the number of registered block states.
    pub fn count(&self) -> usize {
        self.states.len()
    }

    fn register_vanilla_states(&mut self) {
        let mut runtime_id: u32 = 0;

        // Register basic blocks with their common meta values.
        // This is a simplified mapping for protocol v113.
        // In a full implementation, this would be loaded from the block palette.

        // Air
        self.register(BlockState::new("minecraft:air", 0, 0, runtime_id)); runtime_id += 1;
        // Stone variants
        self.register(BlockState::new("minecraft:stone", 1, 0, runtime_id)); runtime_id += 1;
        self.register(BlockState::new("minecraft:stone", 1, 1, runtime_id)); runtime_id += 1; // Granite
        self.register(BlockState::new("minecraft:stone", 1, 2, runtime_id)); runtime_id += 1; // Polished Granite
        self.register(BlockState::new("minecraft:stone", 1, 3, runtime_id)); runtime_id += 1; // Diorite
        self.register(BlockState::new("minecraft:stone", 1, 4, runtime_id)); runtime_id += 1; // Polished Diorite
        self.register(BlockState::new("minecraft:stone", 1, 5, runtime_id)); runtime_id += 1; // Andesite
        self.register(BlockState::new("minecraft:stone", 1, 6, runtime_id)); runtime_id += 1; // Polished Andesite
        // Grass
        self.register(BlockState::new("minecraft:grass", 2, 0, runtime_id)); runtime_id += 1;
        // Dirt variants
        self.register(BlockState::new("minecraft:dirt", 3, 0, runtime_id)); runtime_id += 1;
        self.register(BlockState::new("minecraft:dirt", 3, 1, runtime_id)); runtime_id += 1; // Coarse Dirt
        self.register(BlockState::new("minecraft:dirt", 3, 2, runtime_id)); runtime_id += 1; // Podzol
        // Cobblestone
        self.register(BlockState::new("minecraft:cobblestone", 4, 0, runtime_id)); runtime_id += 1;
        // Planks variants
        self.register(BlockState::new("minecraft:planks", 5, 0, runtime_id)); runtime_id += 1; // Oak
        self.register(BlockState::new("minecraft:planks", 5, 1, runtime_id)); runtime_id += 1; // Spruce
        self.register(BlockState::new("minecraft:planks", 5, 2, runtime_id)); runtime_id += 1; // Birch
        self.register(BlockState::new("minecraft:planks", 5, 3, runtime_id)); runtime_id += 1; // Jungle
        self.register(BlockState::new("minecraft:planks", 5, 4, runtime_id)); runtime_id += 1; // Acacia
        self.register(BlockState::new("minecraft:planks", 5, 5, runtime_id)); runtime_id += 1; // Dark Oak
        // Bedrock
        self.register(BlockState::new("minecraft:bedrock", 7, 0, runtime_id)); runtime_id += 1;
        // Sand
        self.register(BlockState::new("minecraft:sand", 12, 0, runtime_id)); runtime_id += 1;
        self.register(BlockState::new("minecraft:sand", 12, 1, runtime_id)); runtime_id += 1; // Red Sand
        // Gold Ore
        self.register(BlockState::new("minecraft:gold_ore", 14, 0, runtime_id)); runtime_id += 1;
        // Iron Ore
        self.register(BlockState::new("minecraft:iron_ore", 15, 0, runtime_id)); runtime_id += 1;
        // Coal Ore
        self.register(BlockState::new("minecraft:coal_ore", 16, 0, runtime_id)); runtime_id += 1;
        // Wood variants
        self.register(BlockState::new("minecraft:wood", 17, 0, runtime_id)); runtime_id += 1; // Oak
        self.register(BlockState::new("minecraft:wood", 17, 1, runtime_id)); runtime_id += 1; // Spruce
        self.register(BlockState::new("minecraft:wood", 17, 2, runtime_id)); runtime_id += 1; // Birch
        self.register(BlockState::new("minecraft:wood", 17, 3, runtime_id)); runtime_id += 1; // Jungle
        // Leaves variants
        self.register(BlockState::new("minecraft:leaves", 18, 0, runtime_id)); runtime_id += 1; // Oak
        self.register(BlockState::new("minecraft:leaves", 18, 1, runtime_id)); runtime_id += 1; // Spruce
        self.register(BlockState::new("minecraft:leaves", 18, 2, runtime_id)); runtime_id += 1; // Birch
        self.register(BlockState::new("minecraft:leaves", 18, 3, runtime_id)); runtime_id += 1; // Jungle
        // Glass
        self.register(BlockState::new("minecraft:glass", 20, 0, runtime_id)); runtime_id += 1;
        // Lapis Ore
        self.register(BlockState::new("minecraft:lapis_ore", 21, 0, runtime_id)); runtime_id += 1;
        // Sandstone variants
        self.register(BlockState::new("minecraft:sandstone", 24, 0, runtime_id)); runtime_id += 1;
        self.register(BlockState::new("minecraft:sandstone", 24, 1, runtime_id)); runtime_id += 1; // Chiseled
        self.register(BlockState::new("minecraft:sandstone", 24, 2, runtime_id)); runtime_id += 1; // Cut
        // Wool variants (all 16 colors)
        for meta in 0u8..16 {
            self.register(BlockState::new("minecraft:wool", 35, meta, runtime_id));
            runtime_id += 1;
        }
        // Gold Block
        self.register(BlockState::new("minecraft:gold_block", 41, 0, runtime_id)); runtime_id += 1;
        // Iron Block
        self.register(BlockState::new("minecraft:iron_block", 42, 0, runtime_id)); runtime_id += 1;
        // Diamond Ore
        self.register(BlockState::new("minecraft:diamond_ore", 56, 0, runtime_id)); runtime_id += 1;
        // Diamond Block
        self.register(BlockState::new("minecraft:diamond_block", 57, 0, runtime_id)); runtime_id += 1;
        // Chest
        self.register(BlockState::new("minecraft:chest", 54, 0, runtime_id)); runtime_id += 1;
        // Crafting Table
        self.register(BlockState::new("minecraft:crafting_table", 58, 0, runtime_id)); runtime_id += 1;
        // Furnace
        self.register(BlockState::new("minecraft:furnace", 61, 0, runtime_id)); runtime_id += 1;
        // Torch
        self.register(BlockState::new("minecraft:torch", 50, 0, runtime_id)); runtime_id += 1;
        // Obsidian
        self.register(BlockState::new("minecraft:obsidian", 49, 0, runtime_id)); runtime_id += 1;
        // Enchanting Table
        self.register(BlockState::new("minecraft:enchanting_table", 116, 0, runtime_id)); runtime_id += 1;
        // Ender Chest
        self.register(BlockState::new("minecraft:ender_chest", 130, 0, runtime_id)); runtime_id += 1;
        // Beacon
        self.register(BlockState::new("minecraft:beacon", 138, 0, runtime_id)); runtime_id += 1;
        // Anvil
        self.register(BlockState::new("minecraft:anvil", 145, 0, runtime_id)); runtime_id += 1;
        self.register(BlockState::new("minecraft:anvil", 145, 1, runtime_id)); runtime_id += 1; // Slightly Damaged
        self.register(BlockState::new("minecraft:anvil", 145, 2, runtime_id)); runtime_id += 1; // Very Damaged
        // Hopper
        self.register(BlockState::new("minecraft:hopper", 154, 0, runtime_id)); runtime_id += 1;
        // Shulker Box
        self.register(BlockState::new("minecraft:shulker_box", 218, 0, runtime_id)); runtime_id += 1;
        // End Stone
        self.register(BlockState::new("minecraft:end_stone", 121, 0, runtime_id)); runtime_id += 1;
    }
}

impl Default for BlockStateRegistry {
    fn default() -> Self {
        Self::new()
    }
}
