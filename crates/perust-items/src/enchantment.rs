/// Enchantment types matching MCPE protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnchantmentType {
    Protection = 0,
    FireProtection = 1,
    FallProtection = 2,
    ExplosionProtection = 3,
    ProjectileProtection = 4,
    Thorns = 5,
    WaterBreathing = 6,
    WaterSpeed = 7,
    WaterAffinity = 8,
    Sharpness = 9,
    Smite = 10,
    Arthropods = 11,
    Knockback = 12,
    FireAspect = 13,
    Looting = 14,
    Efficiency = 15,
    SilkTouch = 16,
    Durability = 17,
    Fortune = 18,
    BowPower = 19,
    BowKnockback = 20,
    BowFlame = 21,
    BowInfinity = 22,
    FishingFortune = 23,
    FishingLure = 24,
}

impl EnchantmentType {
    /// Creates an enchantment type from its ID.
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(EnchantmentType::Protection),
            1 => Some(EnchantmentType::FireProtection),
            2 => Some(EnchantmentType::FallProtection),
            3 => Some(EnchantmentType::ExplosionProtection),
            4 => Some(EnchantmentType::ProjectileProtection),
            5 => Some(EnchantmentType::Thorns),
            6 => Some(EnchantmentType::WaterBreathing),
            7 => Some(EnchantmentType::WaterSpeed),
            8 => Some(EnchantmentType::WaterAffinity),
            9 => Some(EnchantmentType::Sharpness),
            10 => Some(EnchantmentType::Smite),
            11 => Some(EnchantmentType::Arthropods),
            12 => Some(EnchantmentType::Knockback),
            13 => Some(EnchantmentType::FireAspect),
            14 => Some(EnchantmentType::Looting),
            15 => Some(EnchantmentType::Efficiency),
            16 => Some(EnchantmentType::SilkTouch),
            17 => Some(EnchantmentType::Durability),
            18 => Some(EnchantmentType::Fortune),
            19 => Some(EnchantmentType::BowPower),
            20 => Some(EnchantmentType::BowKnockback),
            21 => Some(EnchantmentType::BowFlame),
            22 => Some(EnchantmentType::BowInfinity),
            23 => Some(EnchantmentType::FishingFortune),
            24 => Some(EnchantmentType::FishingLure),
            _ => None,
        }
    }

    /// Returns the ID of this enchantment type.
    pub fn id(&self) -> u8 {
        *self as u8
    }

    /// Returns the minimum enchantment level required to apply this enchantment.
    pub fn min_level(&self) -> u8 {
        1
    }

    /// Returns the maximum level for this enchantment type.
    pub fn max_level(&self) -> u8 {
        match self {
            EnchantmentType::Protection => 4,
            EnchantmentType::FireProtection => 4,
            EnchantmentType::FallProtection => 4,
            EnchantmentType::ExplosionProtection => 4,
            EnchantmentType::ProjectileProtection => 4,
            EnchantmentType::Thorns => 3,
            EnchantmentType::WaterBreathing => 3,
            EnchantmentType::WaterSpeed => 3,
            EnchantmentType::WaterAffinity => 1,
            EnchantmentType::Sharpness => 5,
            EnchantmentType::Smite => 5,
            EnchantmentType::Arthropods => 5,
            EnchantmentType::Knockback => 2,
            EnchantmentType::FireAspect => 2,
            EnchantmentType::Looting => 3,
            EnchantmentType::Efficiency => 5,
            EnchantmentType::SilkTouch => 1,
            EnchantmentType::Durability => 3,
            EnchantmentType::Fortune => 3,
            EnchantmentType::BowPower => 5,
            EnchantmentType::BowKnockback => 2,
            EnchantmentType::BowFlame => 1,
            EnchantmentType::BowInfinity => 1,
            EnchantmentType::FishingFortune => 3,
            EnchantmentType::FishingLure => 3,
        }
    }

    /// Returns the name of this enchantment type.
    pub fn name(&self) -> &'static str {
        match self {
            EnchantmentType::Protection => "Protection",
            EnchantmentType::FireProtection => "Fire Protection",
            EnchantmentType::FallProtection => "Feather Falling",
            EnchantmentType::ExplosionProtection => "Blast Protection",
            EnchantmentType::ProjectileProtection => "Projectile Protection",
            EnchantmentType::Thorns => "Thorns",
            EnchantmentType::WaterBreathing => "Respiration",
            EnchantmentType::WaterSpeed => "Depth Strider",
            EnchantmentType::WaterAffinity => "Aqua Affinity",
            EnchantmentType::Sharpness => "Sharpness",
            EnchantmentType::Smite => "Smite",
            EnchantmentType::Arthropods => "Bane of Arthropods",
            EnchantmentType::Knockback => "Knockback",
            EnchantmentType::FireAspect => "Fire Aspect",
            EnchantmentType::Looting => "Looting",
            EnchantmentType::Efficiency => "Efficiency",
            EnchantmentType::SilkTouch => "Silk Touch",
            EnchantmentType::Durability => "Unbreaking",
            EnchantmentType::Fortune => "Fortune",
            EnchantmentType::BowPower => "Power",
            EnchantmentType::BowKnockback => "Punch",
            EnchantmentType::BowFlame => "Flame",
            EnchantmentType::BowInfinity => "Infinity",
            EnchantmentType::FishingFortune => "Luck of the Sea",
            EnchantmentType::FishingLure => "Lure",
        }
    }

    /// Returns the minimum enchantability required at the given level.
    pub fn min_enchantability(&self, level: u8) -> i32 {
        let lvl = level as i32;
        match self {
            EnchantmentType::Protection => 1 + (lvl - 1) * 11,
            EnchantmentType::FireProtection => 10 + (lvl - 1) * 8,
            EnchantmentType::FallProtection => 5 + (lvl - 1) * 6,
            EnchantmentType::ExplosionProtection => 5 + (lvl - 1) * 8,
            EnchantmentType::ProjectileProtection => 3 + (lvl - 1) * 6,
            EnchantmentType::Thorns => 10 + (lvl - 1) * 20,
            EnchantmentType::WaterBreathing => 10 + (lvl - 1) * 10,
            EnchantmentType::WaterSpeed => lvl * 10,
            EnchantmentType::WaterAffinity => 1,
            EnchantmentType::Sharpness => 1 + (lvl - 1) * 11,
            EnchantmentType::Smite => 5 + (lvl - 1) * 8,
            EnchantmentType::Arthropods => 5 + (lvl - 1) * 8,
            EnchantmentType::Knockback => 5 + (lvl - 1) * 20,
            EnchantmentType::FireAspect => 10 + (lvl - 1) * 20,
            EnchantmentType::Looting => 15 + (lvl - 1) * 9,
            EnchantmentType::Efficiency => 1 + (lvl - 1) * 10,
            EnchantmentType::SilkTouch => 15,
            EnchantmentType::Durability => 5 + (lvl - 1) * 8,
            EnchantmentType::Fortune => 15 + (lvl - 1) * 9,
            EnchantmentType::BowPower => 1 + (lvl - 1) * 10,
            EnchantmentType::BowKnockback => 12 + (lvl - 1) * 20,
            EnchantmentType::BowFlame => 20,
            EnchantmentType::BowInfinity => 20,
            EnchantmentType::FishingFortune => 15 + (lvl - 1) * 9,
            EnchantmentType::FishingLure => 15 + (lvl - 1) * 9,
        }
    }

    /// Returns the maximum enchantability at the given level.
    pub fn max_enchantability(&self, level: u8) -> i32 {
        self.min_enchantability(level) + 25
    }
}

/// A specific enchantment instance with a type and level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enchantment {
    /// The enchantment type ID.
    pub id: u8,
    /// The level of this enchantment (1-based).
    pub level: u8,
}

impl Enchantment {
    /// Creates a new enchantment with the given type and level.
    pub fn new(id: u8, level: u8) -> Self {
        Self { id, level }
    }

    /// Creates a new enchantment from an `EnchantmentType` and level.
    pub fn from_type(enchantment_type: EnchantmentType, level: u8) -> Self {
        Self {
            id: enchantment_type.id(),
            level,
        }
    }

    /// Returns the enchantment type, if valid.
    pub fn enchantment_type(&self) -> Option<EnchantmentType> {
        EnchantmentType::from_id(self.id)
    }

    /// Returns the name of this enchantment.
    pub fn name(&self) -> &'static str {
        self.enchantment_type()
            .map_or("Unknown", |t| t.name())
    }

    /// Returns `true` if this enchantment level is valid.
    pub fn is_valid(&self) -> bool {
        if let Some(etype) = self.enchantment_type() {
            self.level >= etype.min_level() && self.level <= etype.max_level()
        } else {
            false
        }
    }
}
