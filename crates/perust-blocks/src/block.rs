use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

use crate::block_ids;

/// The type of tool required to efficiently mine a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolType {
    None,
    Pickaxe,
    Axe,
    Shovel,
    Hoe,
    Sword,
    Shears,
}

impl ToolType {
    /// Returns the name of this tool type.
    pub fn name(&self) -> &'static str {
        match self {
            ToolType::None => "none",
            ToolType::Pickaxe => "pickaxe",
            ToolType::Axe => "axe",
            ToolType::Shovel => "shovel",
            ToolType::Hoe => "hoe",
            ToolType::Sword => "sword",
            ToolType::Shears => "shears",
        }
    }
}

/// Represents a block type with all its properties.
#[derive(Debug, Clone)]
pub struct Block {
    /// The block ID.
    pub id: u8,
    /// The block metadata/damage value.
    pub meta: u8,
    /// The human-readable name of this block.
    pub name: String,
    /// How hard the block is to break. -1.0 means unbreakable.
    pub hardness: f32,
    /// Blast resistance of the block.
    pub resistance: f32,
    /// The light level this block emits (0-15).
    pub light_level: u8,
    /// How much light this block filters (0-15).
    pub light_filter: u8,
    /// Whether this block is solid (entities cannot walk through it).
    pub is_solid: bool,
    /// Whether this block is transparent (light can pass through).
    pub is_transparent: bool,
    /// Whether this block can catch fire.
    pub is_flammable: bool,
    /// The type of tool required to efficiently mine this block.
    pub tool_type: ToolType,
}

impl Block {
    /// Creates a new block with the given properties.
    pub fn new(
        id: u8,
        meta: u8,
        name: &str,
        hardness: f32,
        resistance: f32,
        light_level: u8,
        light_filter: u8,
        is_solid: bool,
        is_transparent: bool,
        is_flammable: bool,
        tool_type: ToolType,
    ) -> Self {
        Self {
            id,
            meta,
            name: name.to_string(),
            hardness,
            resistance,
            light_level,
            light_filter,
            is_solid,
            is_transparent,
            is_flammable,
            tool_type,
        }
    }

    /// Returns `true` if this block is air.
    pub fn is_air(&self) -> bool {
        self.id == block_ids::AIR
    }

    /// Returns `true` if this block can be broken (hardness >= 0).
    pub fn is_breakable(&self) -> bool {
        self.hardness >= 0.0
    }
}

/// Global block registry storing all registered blocks.
pub static BLOCK_REGISTRY: Lazy<RwLock<BlockRegistry>> = Lazy::new(|| {
    let registry = BlockRegistry::new();
    RwLock::new(registry)
});

/// Registry of all block types in the game.
pub struct BlockRegistry {
    blocks: Vec<Option<Block>>,
    name_to_id: HashMap<String, u8>,
}

impl BlockRegistry {
    /// Creates a new block registry with all vanilla blocks registered.
    pub fn new() -> Self {
        let mut registry = Self {
            blocks: vec![None; 256],
            name_to_id: HashMap::new(),
        };
        registry.register_vanilla_blocks();
        registry
    }

    /// Registers a block in the registry.
    pub fn register(&mut self, block: Block) {
        let id = block.id as usize;
        if id < self.blocks.len() {
            self.name_to_id.insert(block.name.clone(), block.id);
            self.blocks[id] = Some(block);
        }
    }

    /// Gets a block by its ID.
    pub fn get_by_id(&self, id: u8) -> Option<&Block> {
        self.blocks.get(id as usize).and_then(|opt| opt.as_ref())
    }

    /// Gets a block by its name.
    pub fn get_by_name(&self, name: &str) -> Option<&Block> {
        self.name_to_id.get(name).and_then(|&id| self.get_by_id(id))
    }

    /// Gets the runtime ID for a block (simplified: just the block ID).
    pub fn get_runtime_id(&self, id: u8) -> u32 {
        id as u32
    }

    /// Returns the number of registered blocks.
    pub fn count(&self) -> usize {
        self.blocks.iter().filter(|b| b.is_some()).count()
    }

    fn register_vanilla_blocks(&mut self) {
        // Basic blocks
        self.register(Block::new(block_ids::AIR, 0, "Air", -1.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::STONE, 0, "Stone", 1.5, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::GRASS, 0, "Grass", 0.6, 3.0, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::DIRT, 0, "Dirt", 0.5, 2.5, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::COBBLESTONE, 0, "Cobblestone", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::PLANKS, 0, "Planks", 2.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::SAPLING, 0, "Sapling", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::BEDROCK, 0, "Bedrock", -1.0, 18000000.0, 0, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::WATER, 0, "Water", 100.0, 500.0, 0, 2, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::STILL_WATER, 0, "Still Water", 100.0, 500.0, 0, 2, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::LAVA, 0, "Lava", 100.0, 500.0, 15, 2, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::STILL_LAVA, 0, "Still Lava", 100.0, 500.0, 15, 2, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::SAND, 0, "Sand", 0.5, 2.5, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::GRAVEL, 0, "Gravel", 0.6, 3.0, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::GOLD_ORE, 0, "Gold Ore", 3.0, 15.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::IRON_ORE, 0, "Iron Ore", 3.0, 15.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::COAL_ORE, 0, "Coal Ore", 3.0, 15.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::WOOD, 0, "Wood", 2.0, 10.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::LEAVES, 0, "Leaves", 0.2, 1.0, 0, 1, true, true, true, ToolType::Shears));
        self.register(Block::new(block_ids::SPONGE, 0, "Sponge", 0.6, 3.0, 0, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::GLASS, 0, "Glass", 0.3, 1.5, 0, 0, true, true, false, ToolType::None));
        self.register(Block::new(block_ids::LAPIS_ORE, 0, "Lapis Ore", 3.0, 15.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::SANDSTONE, 0, "Sandstone", 0.8, 4.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::NOTE_BLOCK, 0, "Note Block", 0.8, 4.0, 0, 15, true, false, false, ToolType::Axe));
        self.register(Block::new(block_ids::BED, 0, "Bed", 0.2, 1.0, 0, 0, false, true, true, ToolType::None));
        self.register(Block::new(block_ids::POWERED_RAIL, 0, "Powered Rail", 0.7, 3.5, 0, 0, false, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::DETECTOR_RAIL, 0, "Detector Rail", 0.7, 3.5, 0, 0, false, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::STICKY_PISTON, 0, "Sticky Piston", 0.5, 2.5, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::COBWEB, 0, "Cobweb", 4.0, 20.0, 0, 1, false, true, false, ToolType::Sword));
        self.register(Block::new(block_ids::TALL_GRASS, 0, "Tall Grass", 0.0, 0.0, 0, 0, false, true, true, ToolType::None));
        self.register(Block::new(block_ids::DEAD_BUSH, 0, "Dead Bush", 0.0, 0.0, 0, 0, false, true, true, ToolType::None));
        self.register(Block::new(block_ids::PISTON, 0, "Piston", 0.5, 2.5, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::PISTON_HEAD, 0, "Piston Head", 0.5, 2.5, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::WOOL, 0, "Wool", 0.8, 4.0, 0, 15, true, false, true, ToolType::None));
        self.register(Block::new(block_ids::DANDELION, 0, "Dandelion", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::POPPY, 0, "Poppy", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::BROWN_MUSHROOM, 0, "Brown Mushroom", 0.0, 0.0, 1, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::RED_MUSHROOM, 0, "Red Mushroom", 0.0, 0.0, 1, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::GOLD_BLOCK, 0, "Gold Block", 3.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::IRON_BLOCK, 0, "Iron Block", 5.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::DOUBLE_STONE_SLAB, 0, "Double Stone Slab", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::STONE_SLAB, 0, "Stone Slab", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::BRICKS, 0, "Bricks", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::TNT, 0, "TNT", 0.0, 0.0, 0, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::BOOKSHELF, 0, "Bookshelf", 1.5, 7.5, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::MOSSY_STONE, 0, "Mossy Stone", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::OBSIDIAN, 0, "Obsidian", 50.0, 6000.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::TORCH, 0, "Torch", 0.0, 0.0, 14, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::FIRE, 0, "Fire", 0.0, 0.0, 15, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::STAIRS_WOOD, 0, "Wood Stairs", 2.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::CHEST, 0, "Chest", 2.5, 12.5, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::REDSTONE_WIRE, 0, "Redstone Wire", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::DIAMOND_ORE, 0, "Diamond Ore", 3.0, 15.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::DIAMOND_BLOCK, 0, "Diamond Block", 5.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::CRAFTING_TABLE, 0, "Crafting Table", 2.5, 12.5, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::FARMLAND, 0, "Farmland", 0.6, 3.0, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::FURNACE, 0, "Furnace", 3.5, 17.5, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::LADDER, 0, "Ladder", 0.4, 2.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::RAIL, 0, "Rail", 0.7, 3.5, 0, 0, false, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::STONE_STAIRS, 0, "Stone Stairs", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::WALL_SIGN, 0, "Wall Sign", 1.0, 5.0, 0, 0, false, true, true, ToolType::Axe));
        self.register(Block::new(block_ids::LEVER, 0, "Lever", 0.5, 2.5, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::WOODEN_PRESSURE_PLATE, 0, "Wooden Pressure Plate", 0.5, 2.5, 0, 0, false, true, true, ToolType::Axe));
        self.register(Block::new(block_ids::REDSTONE_ORE, 0, "Redstone Ore", 3.0, 15.0, 9, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::STONE_PRESSURE_PLATE, 0, "Stone Pressure Plate", 0.5, 2.5, 0, 0, false, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::SNOW, 0, "Snow", 0.5, 2.5, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::ICE, 0, "Ice", 0.5, 2.5, 0, 2, true, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::SNOW_BLOCK, 0, "Snow Block", 0.2, 1.0, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::CACTUS, 0, "Cactus", 0.4, 2.0, 0, 0, true, true, false, ToolType::None));
        self.register(Block::new(block_ids::CLAY, 0, "Clay", 0.6, 3.0, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::SUGARCANE, 0, "Sugarcane", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::JUKEBOX, 0, "Jukebox", 2.0, 10.0, 0, 15, true, false, false, ToolType::Axe));
        self.register(Block::new(block_ids::FENCE, 0, "Fence", 2.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::PUMPKIN, 0, "Pumpkin", 1.0, 5.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::NETHERRACK, 0, "Netherrack", 0.4, 2.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::SOUL_SAND, 0, "Soul Sand", 0.5, 2.5, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::GLOWSTONE, 0, "Glowstone", 0.3, 1.5, 15, 0, true, true, false, ToolType::None));
        self.register(Block::new(block_ids::NETHER_PORTAL, 0, "Nether Portal", -1.0, 0.0, 11, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::LIT_PUMPKIN, 0, "Lit Pumpkin", 1.0, 5.0, 15, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::CAKE, 0, "Cake", 0.5, 2.5, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::REPEATER, 0, "Repeater", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::STAINED_GLASS, 0, "Stained Glass", 0.3, 1.5, 0, 0, true, true, false, ToolType::None));
        self.register(Block::new(block_ids::TRAPDOOR, 0, "Trapdoor", 3.0, 15.0, 0, 15, true, true, true, ToolType::Axe));
        self.register(Block::new(block_ids::MONSTER_EGG, 0, "Monster Egg", 0.75, 3.75, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::STONE_BRICK, 0, "Stone Brick", 1.5, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::BROWN_MUSHROOM_BLOCK, 0, "Brown Mushroom Block", 0.2, 1.0, 0, 15, true, false, false, ToolType::Axe));
        self.register(Block::new(block_ids::RED_MUSHROOM_BLOCK, 0, "Red Mushroom Block", 0.2, 1.0, 0, 15, true, false, false, ToolType::Axe));
        self.register(Block::new(block_ids::IRON_BARS, 0, "Iron Bars", 5.0, 30.0, 0, 15, true, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::GLASS_PANE, 0, "Glass Pane", 0.3, 1.5, 0, 0, true, true, false, ToolType::None));
        self.register(Block::new(block_ids::MELON, 0, "Melon", 1.0, 5.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::PUMPKIN_STEM, 0, "Pumpkin Stem", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::MELON_STEM, 0, "Melon Stem", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::VINE, 0, "Vine", 0.2, 1.0, 0, 0, false, true, true, ToolType::Axe));
        self.register(Block::new(block_ids::FENCE_GATE, 0, "Fence Gate", 2.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::BRICK_STAIRS, 0, "Brick Stairs", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::STONE_BRICK_STAIRS, 0, "Stone Brick Stairs", 1.5, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::MYCELIUM, 0, "Mycelium", 0.6, 3.0, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::LILY_PAD, 0, "Lily Pad", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::NETHER_BRICK, 0, "Nether Brick", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::NETHER_BRICK_FENCE, 0, "Nether Brick Fence", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::NETHER_BRICK_STAIRS, 0, "Nether Brick Stairs", 2.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::ENCHANTING_TABLE, 0, "Enchanting Table", 5.0, 6000.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::END_PORTAL, 0, "End Portal", -1.0, 18000000.0, 15, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::END_PORTAL_FRAME, 0, "End Portal Frame", -1.0, 18000000.0, 1, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::END_STONE, 0, "End Stone", 3.0, 45.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::DRAGON_EGG, 0, "Dragon Egg", 3.0, 45.0, 0, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::REDSTONE_LAMP, 0, "Redstone Lamp", 0.3, 1.5, 0, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::DROPPER, 0, "Dropper", 3.5, 17.5, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::ACTIVATOR_RAIL, 0, "Activator Rail", 0.7, 3.5, 0, 0, false, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::COCOA, 0, "Cocoa", 0.2, 1.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::SANDSTONE_STAIRS, 0, "Sandstone Stairs", 0.8, 4.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::EMERALD_ORE, 0, "Emerald Ore", 3.0, 15.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::ENDER_CHEST, 0, "Ender Chest", 22.5, 3000.0, 7, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::TRIPWIRE_HOOK, 0, "Tripwire Hook", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::TRIPWIRE, 0, "Tripwire", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::EMERALD_BLOCK, 0, "Emerald Block", 5.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::SPRUCE_STAIRS, 0, "Spruce Stairs", 2.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::BIRCH_STAIRS, 0, "Birch Stairs", 2.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::JUNGLE_STAIRS, 0, "Jungle Stairs", 2.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::COMMAND_BLOCK, 0, "Command Block", -1.0, 18000000.0, 0, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::BEACON, 0, "Beacon", 3.0, 15.0, 15, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::COBBLESTONE_WALL, 0, "Cobblestone Wall", 2.0, 30.0, 0, 15, true, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::ANVIL, 0, "Anvil", 5.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::TRAPPED_CHEST, 0, "Trapped Chest", 2.5, 12.5, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::LIGHT_WEIGHTED_PRESSURE_PLATE, 0, "Light Weighted Pressure Plate", 0.5, 2.5, 0, 0, false, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::HEAVY_WEIGHTED_PRESSURE_PLATE, 0, "Heavy Weighted Pressure Plate", 0.5, 2.5, 0, 0, false, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::DAYLIGHT_DETECTOR, 0, "Daylight Detector", 0.2, 1.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::REDSTONE_BLOCK, 0, "Redstone Block", 5.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::HOPPER, 0, "Hopper", 3.0, 24.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::QUARTZ, 0, "Quartz", 0.8, 4.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::QUARTZ_STAIRS, 0, "Quartz Stairs", 0.8, 4.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::SLIME, 0, "Slime", 0.0, 0.0, 0, 15, true, false, false, ToolType::None));
        self.register(Block::new(block_ids::IRON_TRAPDOOR, 0, "Iron Trapdoor", 5.0, 25.0, 0, 15, true, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::PRISMARINE, 0, "Prismarine", 1.5, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::SEA_LANTERN, 0, "Sea Lantern", 0.3, 1.5, 15, 0, true, true, false, ToolType::None));
        self.register(Block::new(block_ids::HAY_BALE, 0, "Hay Bale", 0.5, 2.5, 0, 15, true, false, true, ToolType::None));
        self.register(Block::new(block_ids::CARPET, 0, "Carpet", 0.1, 0.5, 0, 0, false, true, true, ToolType::None));
        self.register(Block::new(block_ids::COAL_BLOCK, 0, "Coal Block", 5.0, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::PACKED_ICE, 0, "Packed Ice", 0.5, 2.5, 0, 2, true, true, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::DOUBLE_PLANT, 0, "Double Plant", 0.0, 0.0, 0, 0, false, true, true, ToolType::None));
        self.register(Block::new(block_ids::STANDING_SIGN, 0, "Standing Sign", 1.0, 5.0, 0, 0, false, true, true, ToolType::Axe));
        self.register(Block::new(block_ids::SPRUCE_DOOR, 0, "Spruce Door", 3.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::BIRCH_DOOR, 0, "Birch Door", 3.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::JUNGLE_DOOR, 0, "Jungle Door", 3.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::ACACIA_DOOR, 0, "Acacia Door", 3.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::DARK_OAK_DOOR, 0, "Dark Oak Door", 3.0, 15.0, 0, 15, true, false, true, ToolType::Axe));
        self.register(Block::new(block_ids::GRASS_PATH, 0, "Grass Path", 0.6, 3.0, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::ITEM_FRAME, 0, "Item Frame", 0.0, 0.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::CHORUS_PLANT, 0, "Chorus Plant", 0.4, 2.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::CHORUS_FLOWER, 0, "Chorus Flower", 0.4, 2.0, 0, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::PURPUR, 0, "Purpur", 1.5, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::PURPUR_STAIRS, 0, "Purpur Stairs", 1.5, 30.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::END_ROD, 0, "End Rod", 0.0, 0.0, 14, 0, false, true, false, ToolType::None));
        self.register(Block::new(block_ids::SHULKER_BOX, 0, "Shulker Box", 2.0, 10.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::PURPLE_GLAZED_TERRACOTTA, 0, "Purple Glazed Terracotta", 1.4, 7.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::WHITE_GLAZED_TERRACOTTA, 0, "White Glazed Terracotta", 1.4, 7.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::ORANGE_GLAZED_TERRACOTTA, 0, "Orange Glazed Terracotta", 1.4, 7.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::CONCRETE, 0, "Concrete", 1.8, 9.0, 0, 15, true, false, false, ToolType::Pickaxe));
        self.register(Block::new(block_ids::CONCRETE_POWDER, 0, "Concrete Powder", 0.5, 2.5, 0, 15, true, false, false, ToolType::Shovel));
        self.register(Block::new(block_ids::OBSERVER, 0, "Observer", 3.5, 17.5, 0, 15, true, false, false, ToolType::Pickaxe));
    }
}

impl Default for BlockRegistry {
    fn default() -> Self {
        Self::new()
    }
}
