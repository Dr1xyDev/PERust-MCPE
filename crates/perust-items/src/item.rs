/// Tool tier classification for items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolTier {
    None = 0,
    Wooden = 1,
    Stone = 2,
    Iron = 3,
    Diamond = 4,
    Gold = 5,
}

impl ToolTier {
    /// Returns the attack damage bonus for this tool tier.
    pub fn attack_damage(&self) -> f32 {
        match self {
            ToolTier::None => 0.0,
            ToolTier::Wooden => 0.0,
            ToolTier::Stone => 1.0,
            ToolTier::Iron => 2.0,
            ToolTier::Diamond => 3.0,
            ToolTier::Gold => 0.0,
        }
    }

    /// Returns the mining speed multiplier for this tool tier.
    pub fn speed(&self) -> f32 {
        match self {
            ToolTier::None => 1.0,
            ToolTier::Wooden => 2.0,
            ToolTier::Stone => 4.0,
            ToolTier::Iron => 6.0,
            ToolTier::Diamond => 8.0,
            ToolTier::Gold => 12.0,
        }
    }

    /// Returns the durability for this tool tier (0 means no durability).
    pub fn durability(&self) -> i16 {
        match self {
            ToolTier::None => 0,
            ToolTier::Wooden => 59,
            ToolTier::Stone => 131,
            ToolTier::Iron => 250,
            ToolTier::Diamond => 1561,
            ToolTier::Gold => 32,
        }
    }

    /// Returns the enchantability level for this tool tier.
    pub fn enchantability(&self) -> i32 {
        match self {
            ToolTier::None => 0,
            ToolTier::Wooden => 15,
            ToolTier::Stone => 5,
            ToolTier::Iron => 14,
            ToolTier::Diamond => 10,
            ToolTier::Gold => 22,
        }
    }

    /// Returns the name of this tool tier.
    pub fn name(&self) -> &'static str {
        match self {
            ToolTier::None => "none",
            ToolTier::Wooden => "wooden",
            ToolTier::Stone => "stone",
            ToolTier::Iron => "iron",
            ToolTier::Diamond => "diamond",
            ToolTier::Gold => "gold",
        }
    }
}

/// Tool type classification for items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolType {
    None,
    Pickaxe,
    Axe,
    Shovel,
    Hoe,
    Sword,
    Shears,
    Bow,
    FishingRod,
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
            ToolType::Bow => "bow",
            ToolType::FishingRod => "fishing_rod",
        }
    }
}

/// Represents an item type with all its properties.
#[derive(Debug, Clone)]
pub struct Item {
    /// The item ID (network ID).
    pub id: i32,
    /// The item metadata/damage value.
    pub meta: i16,
    /// The human-readable name of this item.
    pub name: String,
    /// Maximum stack size for this item.
    pub max_stack: u8,
    /// Maximum durability (0 = no durability / not damageable).
    pub durability: i16,
    /// The type of tool this item is (if any).
    pub tool_type: ToolType,
    /// The tier of tool this item is (if any).
    pub tool_tier: ToolTier,
    /// Whether this item is a tool.
    pub is_tool: bool,
    /// Whether this item is armor.
    pub is_armor: bool,
    /// Whether this item is food.
    pub is_food: bool,
    /// Food restoration value (hunger points).
    pub food_restauration: i32,
    /// Armor points provided by this armor item.
    pub armor_points: u8,
    /// Armor toughness provided by this armor item.
    pub armor_toughness: f32,
    /// Attack damage bonus for this item.
    pub damage: f32,
    /// Enchantability level for this item.
    pub enchantability: i32,
}

impl Item {
    /// Creates a new item with the given properties.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        meta: i16,
        name: &str,
        max_stack: u8,
        durability: i16,
        tool_type: ToolType,
        tool_tier: ToolTier,
        is_tool: bool,
        is_armor: bool,
        is_food: bool,
        food_restauration: i32,
        armor_points: u8,
        armor_toughness: f32,
        damage: f32,
        enchantability: i32,
    ) -> Self {
        Self {
            id,
            meta,
            name: name.to_string(),
            max_stack,
            durability,
            tool_type,
            tool_tier,
            is_tool,
            is_armor,
            is_food,
            food_restauration,
            armor_points,
            armor_toughness,
            damage,
            enchantability,
        }
    }

    /// Creates a simple material item with standard stack size (64).
    pub fn material(id: i32, name: &str) -> Self {
        Self {
            id,
            meta: 0,
            name: name.to_string(),
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
            damage: 0.0,
            enchantability: 0,
        }
    }

    /// Creates a tool item with the given properties.
    pub fn tool(id: i32, name: &str, tool_type: ToolType, tool_tier: ToolTier, damage: f32) -> Self {
        let base_durability = tool_tier.durability();
        let adjusted_durability = match tool_type {
            ToolType::Sword => (base_durability as f32 * 1.5) as i16,
            ToolType::Hoe => (base_durability as f32 * 0.7) as i16,
            _ => base_durability,
        };
        Self {
            id,
            meta: 0,
            name: name.to_string(),
            max_stack: 1,
            durability: if adjusted_durability > 0 { adjusted_durability } else { 0 },
            tool_type,
            tool_tier,
            is_tool: true,
            is_armor: false,
            is_food: false,
            food_restauration: 0,
            armor_points: 0,
            armor_toughness: 0.0,
            damage,
            enchantability: tool_tier.enchantability(),
        }
    }

    /// Creates an armor item with the given properties.
    pub fn armor(id: i32, name: &str, tool_tier: ToolTier, armor_points: u8, armor_toughness: f32) -> Self {
        let durability = match tool_tier {
            ToolTier::None => 0,
            ToolTier::Wooden | ToolTier::Gold => 80,
            ToolTier::Stone => 120,
            ToolTier::Iron => 240,
            ToolTier::Diamond => 528,
        };
        Self {
            id,
            meta: 0,
            name: name.to_string(),
            max_stack: 1,
            durability,
            tool_type: ToolType::None,
            tool_tier,
            is_tool: false,
            is_armor: true,
            is_food: false,
            food_restauration: 0,
            armor_points,
            armor_toughness,
            damage: 0.0,
            enchantability: tool_tier.enchantability(),
        }
    }

    /// Creates a food item with the given properties.
    pub fn food(id: i32, name: &str, food_restauration: i32, max_stack: u8) -> Self {
        Self {
            id,
            meta: 0,
            name: name.to_string(),
            max_stack,
            durability: 0,
            tool_type: ToolType::None,
            tool_tier: ToolTier::None,
            is_tool: false,
            is_armor: false,
            is_food: true,
            food_restauration,
            armor_points: 0,
            armor_toughness: 0.0,
            damage: 0.0,
            enchantability: 0,
        }
    }

    /// Returns `true` if this item has durability.
    pub fn has_durability(&self) -> bool {
        self.durability > 0
    }

    /// Returns `true` if this item can be enchanted.
    pub fn can_enchant(&self) -> bool {
        self.enchantability > 0
    }
}
