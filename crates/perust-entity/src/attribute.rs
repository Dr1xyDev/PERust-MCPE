//! Entity attribute system for Minecraft Bedrock Edition.
//!
//! This module provides the [`Attribute`] struct and predefined attribute
//! constants matching the MCPE entity attribute system.

// ---------------------------------------------------------------------------
// Attribute
// ---------------------------------------------------------------------------

/// An entity attribute with min/max bounds and current/default values.
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The attribute name (e.g., "minecraft:health").
    pub name: String,
    /// Minimum allowed value.
    pub min: f32,
    /// Maximum allowed value.
    pub max: f32,
    /// Current value.
    pub value: f32,
    /// Default value.
    pub default: f32,
}

impl Attribute {
    /// Creates a new attribute with the given parameters.
    pub fn new(name: impl Into<String>, min: f32, max: f32, value: f32, default: f32) -> Self {
        Self {
            name: name.into(),
            min,
            max,
            value: value.clamp(min, max),
            default,
        }
    }

    /// Sets the current value, clamping to [min, max].
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Gets the current value.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Resets the value to the default.
    pub fn reset(&mut self) {
        self.value = self.default;
    }

    /// Converts to the protocol attribute type.
    pub fn to_protocol(&self) -> perust_protocol::types::Attribute {
        perust_protocol::types::Attribute::new(
            self.name.clone(),
            self.min,
            self.max,
            self.value,
            self.default,
        )
    }
}

// ---------------------------------------------------------------------------
// Predefined attributes
// ---------------------------------------------------------------------------

/// Creates the default health attribute (0–20, default 20).
pub fn attribute_health() -> Attribute {
    Attribute::new("minecraft:health", 0.0, 20.0, 20.0, 20.0)
}

/// Creates the default health attribute with custom max.
pub fn attribute_health_with_max(max: f32) -> Attribute {
    Attribute::new("minecraft:health", 0.0, max, max, max)
}

/// Creates the default movement speed attribute (0–1024, default 0.1).
pub fn attribute_movement() -> Attribute {
    Attribute::new("minecraft:movement", 0.0, 1024.0, 0.1, 0.1)
}

/// Creates the default knockback resistance attribute (0–1, default 0).
pub fn attribute_knockback_resistance() -> Attribute {
    Attribute::new("minecraft:knockback_resistance", 0.0, 1.0, 0.0, 0.0)
}

/// Creates the default follow range attribute (0–2048, default 16).
pub fn attribute_follow_range() -> Attribute {
    Attribute::new("minecraft:follow_range", 0.0, 2048.0, 16.0, 16.0)
}

/// Creates the default hunger attribute (0–20, default 20).
pub fn attribute_hunger() -> Attribute {
    Attribute::new("minecraft:player.hunger", 0.0, 20.0, 20.0, 20.0)
}

/// Creates the default saturation attribute (0–20, default 20).
pub fn attribute_saturation() -> Attribute {
    Attribute::new("minecraft:player.saturation", 0.0, 20.0, 20.0, 20.0)
}

/// Creates the default experience attribute (0–24791, default 0).
pub fn attribute_experience() -> Attribute {
    Attribute::new("minecraft:player.experience", 0.0, 24791.0, 0.0, 0.0)
}

/// Creates the default experience level attribute (0–24791, default 0).
pub fn attribute_experience_level() -> Attribute {
    Attribute::new("minecraft:player.level", 0.0, 24791.0, 0.0, 0.0)
}

/// Creates the default attack damage attribute (0–2048, default 1).
pub fn attribute_attack_damage() -> Attribute {
    Attribute::new("minecraft:attack_damage", 0.0, 2048.0, 1.0, 1.0)
}

/// Creates the default absorption attribute (0–254, default 0).
pub fn attribute_absorption() -> Attribute {
    Attribute::new("minecraft:absorption", 0.0, 254.0, 0.0, 0.0)
}

/// Creates the default luck attribute (-1024–1024, default 0).
pub fn attribute_luck() -> Attribute {
    Attribute::new("minecraft:luck", -1024.0, 1024.0, 0.0, 0.0)
}

/// Returns the list of default player attributes.
pub fn default_player_attributes() -> Vec<Attribute> {
    vec![
        attribute_health(),
        attribute_movement(),
        attribute_knockback_resistance(),
        attribute_follow_range(),
        attribute_hunger(),
        attribute_saturation(),
        attribute_experience(),
        attribute_experience_level(),
        attribute_absorption(),
    ]
}

/// Returns the list of default mob attributes.
pub fn default_mob_attributes(max_health: f32, follow_range: f32, movement_speed: f32) -> Vec<Attribute> {
    vec![
        attribute_health_with_max(max_health),
        Attribute::new("minecraft:movement", 0.0, 1024.0, movement_speed, movement_speed),
        attribute_knockback_resistance(),
        Attribute::new("minecraft:follow_range", 0.0, 2048.0, follow_range, follow_range),
    ]
}
