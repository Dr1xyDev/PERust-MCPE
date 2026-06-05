use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

use crate::item::{Item, ToolType, ToolTier};
use crate::item_ids;

/// Global item registry.
pub static ITEM_REGISTRY: Lazy<RwLock<ItemRegistry>> = Lazy::new(|| {
    let registry = ItemRegistry::new();
    RwLock::new(registry)
});

/// Registry of all item types in the game.
pub struct ItemRegistry {
    items: HashMap<i32, Item>,
    name_to_id: HashMap<String, i32>,
}

impl ItemRegistry {
    /// Creates a new item registry with all vanilla items registered.
    pub fn new() -> Self {
        let mut registry = Self {
            items: HashMap::new(),
            name_to_id: HashMap::new(),
        };
        registry.register_vanilla_items();
        registry
    }

    /// Registers an item in the registry.
    pub fn register(&mut self, item: Item) {
        self.name_to_id.insert(item.name.clone(), item.id);
        self.items.insert(item.id, item);
    }

    /// Gets an item by its ID.
    pub fn get_by_id(&self, id: i32) -> Option<&Item> {
        self.items.get(&id)
    }

    /// Gets an item by its name.
    pub fn get_by_name(&self, name: &str) -> Option<&Item> {
        self.name_to_id.get(name).and_then(|&id| self.items.get(&id))
    }

    /// Creates an `ItemStack` from an item ID with the given count.
    ///
    /// Uses `perust_inventory::ItemStack` if available, otherwise returns
    /// the item definition directly. For now, returns a tuple of (id, count, max_stack).
    pub fn to_item_stack(&self, id: i32, count: u8) -> Option<(i32, u8, u8)> {
        self.items.get(&id).map(|item| (item.id, count, item.max_stack))
    }

    /// Returns the number of registered items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    fn register_vanilla_items(&mut self) {
        // === Tools - Iron ===
        self.register(Item::tool(item_ids::IRON_SHOVEL, "Iron Shovel", ToolType::Shovel, ToolTier::Iron, 2.5));
        self.register(Item::tool(item_ids::IRON_PICKAXE, "Iron Pickaxe", ToolType::Pickaxe, ToolTier::Iron, 3.0));
        self.register(Item::tool(item_ids::IRON_AXE, "Iron Axe", ToolType::Axe, ToolTier::Iron, 5.0));
        self.register(Item::tool(item_ids::IRON_SWORD, "Iron Sword", ToolType::Sword, ToolTier::Iron, 6.0));
        self.register(Item::tool(item_ids::IRON_HOE, "Iron Hoe", ToolType::Hoe, ToolTier::Iron, 0.0));

        // === Tools - Wooden ===
        self.register(Item::tool(item_ids::WOODEN_SWORD, "Wooden Sword", ToolType::Sword, ToolTier::Wooden, 4.0));
        self.register(Item::tool(item_ids::WOODEN_SHOVEL, "Wooden Shovel", ToolType::Shovel, ToolTier::Wooden, 1.5));
        self.register(Item::tool(item_ids::WOODEN_PICKAXE, "Wooden Pickaxe", ToolType::Pickaxe, ToolTier::Wooden, 2.0));
        self.register(Item::tool(item_ids::WOODEN_AXE, "Wooden Axe", ToolType::Axe, ToolTier::Wooden, 4.0));
        self.register(Item::tool(item_ids::WOODEN_HOE, "Wooden Hoe", ToolType::Hoe, ToolTier::Wooden, 0.0));

        // === Tools - Stone ===
        self.register(Item::tool(item_ids::STONE_SWORD, "Stone Sword", ToolType::Sword, ToolTier::Stone, 5.0));
        self.register(Item::tool(item_ids::STONE_SHOVEL, "Stone Shovel", ToolType::Shovel, ToolTier::Stone, 2.0));
        self.register(Item::tool(item_ids::STONE_PICKAXE, "Stone Pickaxe", ToolType::Pickaxe, ToolTier::Stone, 2.5));
        self.register(Item::tool(item_ids::STONE_AXE, "Stone Axe", ToolType::Axe, ToolTier::Stone, 4.5));
        self.register(Item::tool(item_ids::STONE_HOE, "Stone Hoe", ToolType::Hoe, ToolTier::Stone, 0.0));

        // === Tools - Diamond ===
        self.register(Item::tool(item_ids::DIAMOND_SWORD, "Diamond Sword", ToolType::Sword, ToolTier::Diamond, 7.0));
        self.register(Item::tool(item_ids::DIAMOND_SHOVEL, "Diamond Shovel", ToolType::Shovel, ToolTier::Diamond, 3.5));
        self.register(Item::tool(item_ids::DIAMOND_PICKAXE, "Diamond Pickaxe", ToolType::Pickaxe, ToolTier::Diamond, 4.0));
        self.register(Item::tool(item_ids::DIAMOND_AXE, "Diamond Axe", ToolType::Axe, ToolTier::Diamond, 6.0));
        self.register(Item::tool(item_ids::DIAMOND_HOE, "Diamond Hoe", ToolType::Hoe, ToolTier::Diamond, 0.0));

        // === Tools - Gold ===
        self.register(Item::tool(item_ids::GOLDEN_SWORD, "Golden Sword", ToolType::Sword, ToolTier::Gold, 4.0));
        self.register(Item::tool(item_ids::GOLDEN_SHOVEL, "Golden Shovel", ToolType::Shovel, ToolTier::Gold, 2.0));
        self.register(Item::tool(item_ids::GOLDEN_PICKAXE, "Golden Pickaxe", ToolType::Pickaxe, ToolTier::Gold, 2.5));
        self.register(Item::tool(item_ids::GOLDEN_AXE, "Golden Axe", ToolType::Axe, ToolTier::Gold, 4.0));
        self.register(Item::tool(item_ids::GOLDEN_HOE, "Golden Hoe", ToolType::Hoe, ToolTier::Gold, 0.0));

        // === Special tools ===
        self.register(Item {
            id: item_ids::BOW,
            meta: 0,
            name: "Bow".to_string(),
            max_stack: 1,
            durability: 384,
            tool_type: ToolType::Bow,
            tool_tier: ToolTier::None,
            is_tool: true,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 1,
        });
        self.register(Item {
            id: item_ids::SHEARS,
            meta: 0,
            name: "Shears".to_string(),
            max_stack: 1,
            durability: 238,
            tool_type: ToolType::Shears,
            tool_tier: ToolTier::None,
            is_tool: true,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        });
        self.register(Item {
            id: item_ids::FISHING_ROD,
            meta: 0,
            name: "Fishing Rod".to_string(),
            max_stack: 1,
            durability: 65,
            tool_type: ToolType::FishingRod,
            tool_tier: ToolTier::None,
            is_tool: true,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 1,
        });
        self.register(Item {
            id: item_ids::FLINT_STEEL,
            meta: 0,
            name: "Flint and Steel".to_string(),
            max_stack: 1,
            durability: 64,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: true,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        });
        self.register(Item {
            id: item_ids::CARROT_ON_A_STICK,
            meta: 0,
            name: "Carrot on a Stick".to_string(),
            max_stack: 1,
            durability: 25,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: true,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        });

        // === Armor - Leather ===
        self.register(Item::armor(item_ids::LEATHER_HELMET, "Leather Helmet", ToolTier::None, 1, 0.0));
        self.register(Item::armor(item_ids::LEATHER_CHESTPLATE, "Leather Chestplate", ToolTier::None, 3, 0.0));
        self.register(Item::armor(item_ids::LEATHER_LEGGINGS, "Leather Leggings", ToolTier::None, 2, 0.0));
        self.register(Item::armor(item_ids::LEATHER_BOOTS, "Leather Boots", ToolTier::None, 1, 0.0));

        // === Armor - Chain ===
        self.register(Item::armor(item_ids::CHAIN_HELMET, "Chain Helmet", ToolTier::None, 2, 0.0));
        self.register(Item::armor(item_ids::CHAIN_CHESTPLATE, "Chain Chestplate", ToolTier::None, 5, 0.0));
        self.register(Item::armor(item_ids::CHAIN_LEGGINGS, "Chain Leggings", ToolTier::None, 4, 0.0));
        self.register(Item::armor(item_ids::CHAIN_BOOTS, "Chain Boots", ToolTier::None, 1, 0.0));

        // === Armor - Iron ===
        self.register(Item::armor(item_ids::IRON_HELMET, "Iron Helmet", ToolTier::Iron, 2, 0.0));
        self.register(Item::armor(item_ids::IRON_CHESTPLATE, "Iron Chestplate", ToolTier::Iron, 6, 0.0));
        self.register(Item::armor(item_ids::IRON_LEGGINGS, "Iron Leggings", ToolTier::Iron, 5, 0.0));
        self.register(Item::armor(item_ids::IRON_BOOTS, "Iron Boots", ToolTier::Iron, 2, 0.0));

        // === Armor - Diamond ===
        self.register(Item::armor(item_ids::DIAMOND_HELMET, "Diamond Helmet", ToolTier::Diamond, 3, 2.0));
        self.register(Item::armor(item_ids::DIAMOND_CHESTPLATE, "Diamond Chestplate", ToolTier::Diamond, 8, 2.0));
        self.register(Item::armor(item_ids::DIAMOND_LEGGINGS, "Diamond Leggings", ToolTier::Diamond, 6, 2.0));
        self.register(Item::armor(item_ids::DIAMOND_BOOTS, "Diamond Boots", ToolTier::Diamond, 3, 2.0));

        // === Armor - Gold ===
        self.register(Item::armor(item_ids::GOLDEN_HELMET, "Golden Helmet", ToolTier::Gold, 2, 0.0));
        self.register(Item::armor(item_ids::GOLDEN_CHESTPLATE, "Golden Chestplate", ToolTier::Gold, 5, 0.0));
        self.register(Item::armor(item_ids::GOLDEN_LEGGINGS, "Golden Leggings", ToolTier::Gold, 3, 0.0));
        self.register(Item::armor(item_ids::GOLDEN_BOOTS, "Golden Boots", ToolTier::Gold, 1, 0.0));

        // === Materials ===
        self.register(Item::material(item_ids::COAL, "Coal"));
        self.register(Item::material(item_ids::DIAMOND, "Diamond"));
        self.register(Item::material(item_ids::IRON_INGOT, "Iron Ingot"));
        self.register(Item::material(item_ids::GOLD_INGOT, "Gold Ingot"));
        self.register(Item::material(item_ids::STICK, "Stick"));
        self.register(Item::material(item_ids::BOWL, "Bowl"));
        self.register(Item::material(item_ids::STRING, "String"));
        self.register(Item::material(item_ids::FEATHER, "Feather"));
        self.register(Item::material(item_ids::GUNPOWDER, "Gunpowder"));
        self.register(Item::material(item_ids::FLINT, "Flint"));
        self.register(Item::material(item_ids::LEATHER, "Leather"));
        self.register(Item::material(item_ids::BRICK, "Brick"));
        self.register(Item::material(item_ids::CLAY, "Clay"));
        self.register(Item::material(item_ids::PAPER, "Paper"));
        self.register(Item::material(item_ids::BOOK, "Book"));
        self.register(Item::material(item_ids::SLIMEBALL, "Slimeball"));
        self.register(Item::material(item_ids::GLOWSTONE_DUST, "Glowstone Dust"));
        self.register(Item::material(item_ids::DYE, "Dye"));
        self.register(Item::material(item_ids::BONE, "Bone"));
        self.register(Item::material(item_ids::SUGAR, "Sugar"));
        self.register(Item::material(item_ids::IRON_DOOR, "Iron Door"));
        self.register(Item::material(item_ids::REDSTONE, "Redstone"));
        self.register(Item::material(item_ids::GOLD_NUGGET, "Gold Nugget"));
        self.register(Item::material(item_ids::BLAZE_ROD, "Blaze Rod"));
        self.register(Item::material(item_ids::GHAST_TEAR, "Ghast Tear"));
        self.register(Item::material(item_ids::BLAZE_POWDER, "Blaze Powder"));
        self.register(Item::material(item_ids::MAGMA_CREAM, "Magma Cream"));
        self.register(Item::material(item_ids::ENDER_PEARL, "Ender Pearl"));
        self.register(Item::material(item_ids::SPIDER_EYE, "Spider Eye"));
        self.register(Item::material(item_ids::FERMENTED_SPIDER_EYE, "Fermented Spider Eye"));
        self.register(Item::material(item_ids::ENDER_EYE, "Ender Eye"));
        self.register(Item::material(item_ids::EMERALD, "Emerald"));
        self.register(Item::material(item_ids::NETHER_STAR, "Nether Star"));
        self.register(Item::material(item_ids::NETHER_BRICK, "Nether Brick"));
        self.register(Item::material(item_ids::QUARTZ, "Quartz"));
        self.register(Item::material(item_ids::PRISMARINE_SHARD, "Prismarine Shard"));
        self.register(Item::material(item_ids::PRISMARINE_CRYSTALS, "Prismarine Crystals"));
        self.register(Item::material(item_ids::RABBIT_HIDE, "Rabbit Hide"));
        self.register(Item::material(item_ids::RABBIT_FOOT, "Rabbit Foot"));
        self.register(Item::material(item_ids::IRON_NUGGET, "Iron Nugget"));
        self.register(Item::material(item_ids::SHULKER_SHELL, "Shulker Shell"));

        // Stack size 16 items
        self.register(Item {
            id: item_ids::SNOWBALL,
            meta: 0,
            name: "Snowball".to_string(),
            max_stack: 16,
            durability: 0,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: false,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        });
        self.register(Item {
            id: item_ids::EGG,
            meta: 0,
            name: "Egg".to_string(),
            max_stack: 16,
            durability: 0,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: false,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        });
        self.register(Item {
            id: item_ids::ENDER_PEARL,
            meta: 0,
            name: "Ender Pearl".to_string(),
            max_stack: 16,
            durability: 0,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: false,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        });
        self.register(Item {
            id: item_ids::BOTTLE_O_ENCHANTING,
            meta: 0,
            name: "Bottle o' Enchanting".to_string(),
            max_stack: 16,
            durability: 0,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: false,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        });

        // === Food ===
        self.register(Item::food(item_ids::APPLE, "Apple", 4, 64));
        self.register(Item::food(item_ids::MUSHROOM_STEW, "Mushroom Stew", 6, 1));
        self.register(Item::food(item_ids::BREAD, "Bread", 5, 64));
        self.register(Item::food(item_ids::PORKCHOP, "Raw Porkchop", 3, 64));
        self.register(Item::food(item_ids::COOKED_PORKCHOP, "Cooked Porkchop", 8, 64));
        self.register(Item::food(item_ids::GOLDEN_APPLE, "Golden Apple", 4, 64));
        self.register(Item::food(item_ids::RAW_FISH, "Raw Fish", 2, 64));
        self.register(Item::food(item_ids::COOKED_FISH, "Cooked Fish", 5, 64));
        self.register(Item::food(item_ids::MELON, "Melon", 2, 64));
        self.register(Item::food(item_ids::RAW_BEEF, "Raw Beef", 3, 64));
        self.register(Item::food(item_ids::STEAK, "Steak", 8, 64));
        self.register(Item::food(item_ids::RAW_CHICKEN, "Raw Chicken", 2, 64));
        self.register(Item::food(item_ids::COOKED_CHICKEN, "Cooked Chicken", 6, 64));
        self.register(Item::food(item_ids::ROTTEN_FLESH, "Rotten Flesh", 4, 64));
        self.register(Item::food(item_ids::CARROT, "Carrot", 3, 64));
        self.register(Item::food(item_ids::POTATO, "Potato", 1, 64));
        self.register(Item::food(item_ids::BAKED_POTATO, "Baked Potato", 5, 64));
        self.register(Item::food(item_ids::POISONOUS_POTATO, "Poisonous Potato", 2, 64));
        self.register(Item::food(item_ids::GOLDEN_CARROT, "Golden Carrot", 6, 64));
        self.register(Item::food(item_ids::PUMPKIN_PIE, "Pumpkin Pie", 8, 64));
        self.register(Item::food(item_ids::COOKIE, "Cookie", 2, 64));
        self.register(Item::food(item_ids::CAKE, "Cake", 2, 1));
        self.register(Item::food(item_ids::RABBIT, "Raw Rabbit", 3, 64));
        self.register(Item::food(item_ids::COOKED_RABBIT, "Cooked Rabbit", 5, 64));
        self.register(Item::food(item_ids::RABBIT_STEW, "Rabbit Stew", 10, 1));
        self.register(Item::food(item_ids::RAW_MUTTON, "Raw Mutton", 2, 64));
        self.register(Item::food(item_ids::COOKED_MUTTON, "Cooked Mutton", 6, 64));

        // === Crops / Seeds ===
        self.register(Item::material(item_ids::SEEDS, "Seeds"));
        self.register(Item::material(item_ids::WHEAT, "Wheat"));
        self.register(Item::material(item_ids::SUGARCANE, "Sugarcane"));
        self.register(Item::material(item_ids::PUMPKIN_SEEDS, "Pumpkin Seeds"));
        self.register(Item::material(item_ids::MELON_SEEDS, "Melon Seeds"));
        self.register(Item::material(item_ids::NETHER_WART, "Nether Wart"));

        // === Special items with stack size 1 ===
        let special_items = [
            (item_ids::BUCKET, "Bucket"),
            (item_ids::MINECART, "Minecart"),
            (item_ids::SADDLE, "Saddle"),
            (item_ids::BOAT, "Boat"),
            (item_ids::SIGN, "Sign"),
            (item_ids::BED, "Bed"),
            (item_ids::PAINTING, "Painting"),
            (item_ids::MUSHROOM_STEW, "Mushroom Stew"),
            (item_ids::CHEST_MINECART, "Chest Minecart"),
            (item_ids::COMPASS, "Compass"),
            (item_ids::CLOCK, "Clock"),
            (item_ids::MAP, "Map"),
            (item_ids::GLASS_BOTTLE, "Glass Bottle"),
            (item_ids::POTION, "Potion"),
            (item_ids::SPLASH_POTION, "Splash Potion"),
            (item_ids::BREWING_STAND, "Brewing Stand"),
            (item_ids::CAULDRON, "Cauldron"),
            (item_ids::FLOWER_POT, "Flower Pot"),
            (item_ids::ITEM_FRAME, "Item Frame"),
            (item_ids::ARMOR_STAND, "Armor Stand"),
            (item_ids::BANNER, "Banner"),
            (item_ids::END_CRYSTAL, "End Crystal"),
            (item_ids::SKULL, "Skull"),
            (item_ids::TNT_MINECART, "TNT Minecart"),
            (item_ids::HOPPER_MINECART, "Hopper Minecart"),
            (item_ids::SPAWN_EGG, "Spawn Egg"),
            (item_ids::FIRE_CHARGE, "Fire Charge"),
            (item_ids::REPEATER, "Repeater"),
            (item_ids::COMPARATOR, "Comparator"),
            (item_ids::ENCHANTED_BOOK, "Enchanted Book"),
            (item_ids::TOTEM, "Totem of Undying"),
            (item_ids::TRIDENT, "Trident"),
            (item_ids::GLISTERING_MELON, "Glistering Melon"),
        ];

        for (id, name) in special_items {
            // Skip items already registered
            if self.items.contains_key(&id) {
                continue;
            }
            let is_food = id == item_ids::MUSHROOM_STEW;
            let max_stack = if id == item_ids::SPAWN_EGG { 64 } else { 1 };
            self.register(Item {
                id,
                meta: 0,
                name: name.to_string(),
                max_stack,
                durability: 0,
                tool_type: ToolType::None,
                tool_tier: ToolTier::None,
                is_tool: false,
                is_armor: false,
                is_food,
                food_restauration: if is_food { 6 } else { 0 },
                armor_points: 0,
                armor_toughness: 0.0,
                damage: 0.0,
                enchantability: 0,
            });
        }

        // Special handling for Trident
        if let Some(trident) = self.items.get_mut(&item_ids::TRIDENT) {
            trident.is_tool = true;
            trident.durability = 250;
            trident.damage = 9.0;
            trident.enchantability = 15;
            trident.max_stack = 1;
        }

        // Special handling for Totem
        if let Some(totem) = self.items.get_mut(&item_ids::TOTEM) {
            totem.max_stack = 1;
        }

        // Arrow
        self.register(Item {
            id: item_ids::ARROW,
            meta: 0,
            name: "Arrow".to_string(),
            max_stack: 64,
            durability: 0,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: false,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 1.0,
            enchantability: 0,
        });
    }
}

impl Default for ItemRegistry {
    fn default() -> Self {
        Self::new()
    }
}
