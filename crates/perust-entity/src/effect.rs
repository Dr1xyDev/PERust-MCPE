//! Status effects for Minecraft Bedrock Edition.
//!
//! This module provides the [`Effect`] struct and [`EffectId`] enum matching
//! the MCPE status effect IDs.

// ---------------------------------------------------------------------------
// EffectId
// ---------------------------------------------------------------------------

/// Status effect IDs matching the MCPE protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EffectId {
    Speed = 1,
    Slowness = 2,
    Haste = 3,
    Fatigue = 4,
    Strength = 5,
    Healing = 6,
    Harming = 7,
    Jump = 8,
    Nausea = 9,
    Regeneration = 10,
    Resistance = 11,
    FireResistance = 12,
    WaterBreathing = 13,
    Invisibility = 14,
    Blindness = 15,
    NightVision = 16,
    Hunger = 17,
    Weakness = 18,
    Poison = 19,
    Wither = 20,
    HealthBoost = 21,
    Absorption = 22,
    Saturation = 23,
    Levitation = 24,
}

impl EffectId {
    /// Converts an effect ID byte to an [`EffectId`] variant.
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(EffectId::Speed),
            2 => Some(EffectId::Slowness),
            3 => Some(EffectId::Haste),
            4 => Some(EffectId::Fatigue),
            5 => Some(EffectId::Strength),
            6 => Some(EffectId::Healing),
            7 => Some(EffectId::Harming),
            8 => Some(EffectId::Jump),
            9 => Some(EffectId::Nausea),
            10 => Some(EffectId::Regeneration),
            11 => Some(EffectId::Resistance),
            12 => Some(EffectId::FireResistance),
            13 => Some(EffectId::WaterBreathing),
            14 => Some(EffectId::Invisibility),
            15 => Some(EffectId::Blindness),
            16 => Some(EffectId::NightVision),
            17 => Some(EffectId::Hunger),
            18 => Some(EffectId::Weakness),
            19 => Some(EffectId::Poison),
            20 => Some(EffectId::Wither),
            21 => Some(EffectId::HealthBoost),
            22 => Some(EffectId::Absorption),
            23 => Some(EffectId::Saturation),
            24 => Some(EffectId::Levitation),
            _ => None,
        }
    }

    /// Returns the protocol ID for this effect.
    pub fn as_id(self) -> u8 {
        self as u8
    }

    /// Returns `true` if this effect is beneficial.
    pub fn is_beneficial(self) -> bool {
        matches!(
            self,
            EffectId::Speed
                | EffectId::Haste
                | EffectId::Strength
                | EffectId::Jump
                | EffectId::Regeneration
                | EffectId::Resistance
                | EffectId::FireResistance
                | EffectId::WaterBreathing
                | EffectId::Invisibility
                | EffectId::NightVision
                | EffectId::HealthBoost
                | EffectId::Absorption
                | EffectId::Saturation
                | EffectId::Levitation
        )
    }

    /// Returns `true` if this effect is harmful.
    pub fn is_harmful(self) -> bool {
        matches!(
            self,
            EffectId::Slowness
                | EffectId::Fatigue
                | EffectId::Harming
                | EffectId::Nausea
                | EffectId::Blindness
                | EffectId::Hunger
                | EffectId::Weakness
                | EffectId::Poison
                | EffectId::Wither
        )
    }
}

// ---------------------------------------------------------------------------
// Effect
// ---------------------------------------------------------------------------

/// A status effect applied to an entity.
#[derive(Debug, Clone)]
pub struct Effect {
    /// The effect type ID.
    pub id: u8,
    /// The amplifier level (0 = level I, 1 = level II, etc.).
    pub amplifier: u8,
    /// Duration in ticks (-1 = infinite).
    pub duration: i32,
    /// Whether the effect was applied from an ambient source (beacon).
    pub ambient: bool,
    /// Whether the effect particles are visible.
    pub visible: bool,
}

impl Effect {
    /// Creates a new effect with the given parameters.
    pub fn new(id: u8, amplifier: u8, duration: i32, ambient: bool, visible: bool) -> Self {
        Self {
            id,
            amplifier,
            duration,
            ambient,
            visible,
        }
    }

    /// Creates a new effect from an [`EffectId`].
    pub fn from_effect_id(effect_id: EffectId, amplifier: u8, duration: i32) -> Self {
        Self {
            id: effect_id.as_id(),
            amplifier,
            duration,
            ambient: false,
            visible: true,
        }
    }

    /// Returns the [`EffectId`] for this effect, if valid.
    pub fn effect_id(&self) -> Option<EffectId> {
        EffectId::from_id(self.id)
    }

    /// Returns `true` if this effect has infinite duration.
    pub fn is_infinite(&self) -> bool {
        self.duration == -1
    }

    /// Returns `true` if this effect has expired.
    pub fn is_expired(&self) -> bool {
        self.duration == 0
    }

    /// Ticks the effect, reducing its duration by 1.
    ///
    /// Returns `true` if the effect is still active.
    pub fn tick(&mut self) -> bool {
        if self.duration > 0 {
            self.duration -= 1;
        }
        self.duration != 0 || self.is_infinite()
    }

    /// Returns the amplifier as a Roman numeral level string.
    pub fn level_string(&self) -> String {
        match self.amplifier {
            0 => "I".to_string(),
            1 => "II".to_string(),
            2 => "III".to_string(),
            3 => "IV".to_string(),
            4 => "V".to_string(),
            n => format!("{}", n + 1),
        }
    }
}

impl Default for Effect {
    fn default() -> Self {
        Self::new(1, 0, 600, false, true) // Speed I for 30 seconds
    }
}
