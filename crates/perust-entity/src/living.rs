//! Living entity type for Minecraft Bedrock Edition.
//!
//! This module provides [`LivingEntity`], which extends the base [`Entity`]
//! with effects, equipment, and attributes.

use std::collections::HashMap;
use perust_utils::math::Vector3f;
use perust_protocol::types::ItemInstance;
use crate::entity::{Entity, EntityDataType};
use crate::effect::{Effect, EffectId};
use crate::attribute::Attribute;

// ---------------------------------------------------------------------------
// Equipment slots
// ---------------------------------------------------------------------------

/// Equipment slot indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    /// Main hand.
    MainHand = 0,
    /// Off hand.
    OffHand = 1,
    /// Helmet / head.
    Head = 2,
    /// Chestplate / chest.
    Chest = 3,
    /// Leggings / legs.
    Legs = 4,
    /// Boots / feet.
    Feet = 5,
}

impl EquipmentSlot {
    /// Converts a slot index to an [`EquipmentSlot`].
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(EquipmentSlot::MainHand),
            1 => Some(EquipmentSlot::OffHand),
            2 => Some(EquipmentSlot::Head),
            3 => Some(EquipmentSlot::Chest),
            4 => Some(EquipmentSlot::Legs),
            5 => Some(EquipmentSlot::Feet),
            _ => None,
        }
    }

    /// Returns the slot index.
    pub fn as_index(self) -> usize {
        self as usize
    }
}

// ---------------------------------------------------------------------------
// LivingEntity
// ---------------------------------------------------------------------------

/// A living entity with effects, equipment, and attributes.
///
/// This extends the base [`Entity`] with features common to all living
/// entities (players, mobs, etc.).
pub struct LivingEntity {
    /// The base entity.
    pub entity: Entity,
    /// Active status effects, keyed by effect ID.
    pub effects: HashMap<u8, Effect>,
    /// Equipment slots.
    pub equipment: [Option<ItemInstance>; 6],
    /// Entity attributes.
    pub attributes: Vec<Attribute>,
    /// Whether the entity can pick up items.
    pub can_pickup_items: bool,
    /// Whether the entity is using an item.
    pub is_using_item: bool,
    /// Ticks the entity has been using an item.
    pub item_use_ticks: i32,
    /// Whether the entity is blocking (shield).
    pub is_blocking: bool,
    /// Last damage source (simplified).
    pub last_damage_source: String,
    /// Death time in ticks (0 = not dying).
    pub death_time: i32,
    /// No damage ticks (invulnerability frames).
    pub no_damage_ticks: i32,
    /// Maximum no damage ticks.
    pub max_no_damage_ticks: i32,
}

impl LivingEntity {
    /// Creates a new living entity with the given type and position.
    pub fn new(entity_type: EntityDataType, position: Vector3f) -> Self {
        Self {
            entity: Entity::new(entity_type, position),
            effects: HashMap::new(),
            equipment: [None, None, None, None, None, None],
            attributes: vec![
                crate::attribute::attribute_health_with_max(entity_type.default_max_health()),
                crate::attribute::attribute_movement(),
                crate::attribute::attribute_knockback_resistance(),
                crate::attribute::attribute_follow_range(),
            ],
            can_pickup_items: true,
            is_using_item: false,
            item_use_ticks: 0,
            is_blocking: false,
            last_damage_source: String::new(),
            death_time: 0,
            no_damage_ticks: 0,
            max_no_damage_ticks: 20,
        }
    }

    /// Adds a status effect to this entity.
    ///
    /// If an effect of the same type already exists, it is replaced if the
    /// new effect has a higher amplifier or longer duration.
    pub fn add_effect(&mut self, effect: Effect) {
        if let Some(existing) = self.effects.get(&effect.id) {
            // Replace if new effect is stronger or longer
            if effect.amplifier >= existing.amplifier
                && (effect.duration > existing.duration || effect.amplifier > existing.amplifier)
            {
                self.effects.insert(effect.id, effect);
            }
        } else {
            self.effects.insert(effect.id, effect);
        }
    }

    /// Removes a status effect from this entity.
    pub fn remove_effect(&mut self, effect_id: u8) -> Option<Effect> {
        self.effects.remove(&effect_id)
    }

    /// Removes all status effects from this entity.
    pub fn clear_effects(&mut self) {
        self.effects.clear();
    }

    /// Returns `true` if this entity has the given effect.
    pub fn has_effect(&self, effect_id: u8) -> bool {
        self.effects.contains_key(&effect_id)
    }

    /// Gets an effect by ID.
    pub fn get_effect(&self, effect_id: u8) -> Option<&Effect> {
        self.effects.get(&effect_id)
    }

    /// Gets a mutable reference to an effect by ID.
    pub fn get_effect_mut(&mut self, effect_id: u8) -> Option<&mut Effect> {
        self.effects.get_mut(&effect_id)
    }

    /// Gets the amplifier level of an effect, or 0 if not present.
    pub fn get_effect_amplifier(&self, effect_id: u8) -> u8 {
        self.effects.get(&effect_id).map(|e| e.amplifier).unwrap_or(0)
    }

    /// Sets equipment in the given slot.
    pub fn set_equipment(&mut self, slot: EquipmentSlot, item: Option<ItemInstance>) {
        self.equipment[slot.as_index()] = item;
    }

    /// Gets equipment from the given slot.
    pub fn get_equipment(&self, slot: EquipmentSlot) -> Option<&ItemInstance> {
        self.equipment[slot.as_index()].as_ref()
    }

    /// Gets a mutable reference to equipment in the given slot.
    pub fn get_equipment_mut(&mut self, slot: EquipmentSlot) -> Option<&mut Option<ItemInstance>> {
        Some(&mut self.equipment[slot.as_index()])
    }

    /// Gets an attribute by name.
    pub fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.iter().find(|a| a.name == name)
    }

    /// Gets a mutable reference to an attribute by name.
    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut Attribute> {
        self.attributes.iter_mut().find(|a| a.name == name)
    }

    /// Sets the value of an attribute by name.
    pub fn set_attribute_value(&mut self, name: &str, value: f32) {
        if let Some(attr) = self.attributes.iter_mut().find(|a| a.name == name) {
            attr.set_value(value);
        }
    }

    /// Performs a tick for this living entity.
    pub fn tick(&mut self) {
        // Tick base entity
        self.entity.tick();

        // Tick effects
        let mut expired = Vec::new();
        for (&id, effect) in self.effects.iter_mut() {
            if !effect.tick() {
                expired.push(id);
            }
        }
        for id in expired {
            self.effects.remove(&id);
        }

        // No damage ticks countdown
        if self.no_damage_ticks > 0 {
            self.no_damage_ticks -= 1;
        }

        // Item use ticks
        if self.is_using_item {
            self.item_use_ticks += 1;
        }

        // Death animation
        if !self.entity.alive {
            self.death_time += 1;
        }
    }

    /// Damages this living entity.
    ///
    /// Respects no-damage ticks (invulnerability frames).
    pub fn damage(&mut self, amount: f32) -> bool {
        if self.no_damage_ticks > 0 {
            return false;
        }
        let died = self.entity.damage(amount);
        if !died && self.entity.alive {
            self.no_damage_ticks = self.max_no_damage_ticks;
        }
        died
    }

    /// Heals this living entity.
    pub fn heal(&mut self, amount: f32) {
        self.entity.heal(amount);
    }

    /// Returns `true` if this entity is alive.
    pub fn is_alive(&self) -> bool {
        self.entity.is_alive()
    }

    /// Returns the movement speed, modified by effects.
    pub fn get_movement_speed(&self) -> f32 {
        let base = self
            .get_attribute("minecraft:movement")
            .map(|a| a.value)
            .unwrap_or(0.1);

        let mut speed = base;

        // Speed effect
        if let Some(effect) = self.get_effect(EffectId::Speed.as_id()) {
            speed *= 1.0 + 0.2 * (effect.amplifier as f32 + 1.0);
        }

        // Slowness effect
        if let Some(effect) = self.get_effect(EffectId::Slowness.as_id()) {
            speed *= 1.0 - 0.15 * (effect.amplifier as f32 + 1.0);
        }

        speed
    }

    /// Returns the attack damage, modified by effects.
    pub fn get_attack_damage(&self) -> f32 {
        let base = self
            .get_attribute("minecraft:attack_damage")
            .map(|a| a.value)
            .unwrap_or(1.0);

        let mut damage = base;

        // Strength effect
        if let Some(effect) = self.get_effect(EffectId::Strength.as_id()) {
            damage *= 1.0 + 0.3 * (effect.amplifier as f32 + 1.0);
        }

        // Weakness effect
        if let Some(effect) = self.get_effect(EffectId::Weakness.as_id()) {
            damage *= 1.0 - 0.2 * (effect.amplifier as f32 + 1.0);
        }

        damage
    }

    /// Returns the knockback resistance, modified by attributes.
    pub fn get_knockback_resistance(&self) -> f32 {
        self.get_attribute("minecraft:knockback_resistance")
            .map(|a| a.value)
            .unwrap_or(0.0)
    }

    /// Returns the list of active effect IDs.
    pub fn active_effect_ids(&self) -> Vec<u8> {
        self.effects.keys().copied().collect()
    }
}
