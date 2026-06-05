//! Base entity type for Minecraft Bedrock Edition.
//!
//! This module provides [`EntityDataType`] for entity type classification
//! and [`Entity`] as the base entity structure.

use perust_utils::math::{BoundingBox, Vector3f};
use perust_nbt::Tag;
use crate::metadata::EntityMetadata;
use crate::error::EntityError;

// ---------------------------------------------------------------------------
// EntityDataType
// ---------------------------------------------------------------------------

/// Entity type classification matching MCPE network IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EntityDataType {
    // Players
    Player = 63,
    // Monsters
    Zombie = 32,
    Skeleton = 34,
    Creeper = 33,
    Enderman = 38,
    Spider = 35,
    // Animals
    Cow = 11,
    Pig = 12,
    Sheep = 13,
    Chicken = 10,
    Horse = 23,
    // Projectiles
    Arrow = 80,
    Snowball = 81,
    Egg = 82,
    EnderPearl = 87,
    // Objects
    Item = 64,
    FallingSand = 66,
    PrimedTnt = 65,
    XpOrb = 69,
    Painting = 83,
    Lightning = 93,
    FishingHook = 77,
}

impl EntityDataType {
    /// Converts a network type ID to an [`EntityDataType`].
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            63 => Some(EntityDataType::Player),
            32 => Some(EntityDataType::Zombie),
            34 => Some(EntityDataType::Skeleton),
            33 => Some(EntityDataType::Creeper),
            38 => Some(EntityDataType::Enderman),
            35 => Some(EntityDataType::Spider),
            11 => Some(EntityDataType::Cow),
            12 => Some(EntityDataType::Pig),
            13 => Some(EntityDataType::Sheep),
            10 => Some(EntityDataType::Chicken),
            23 => Some(EntityDataType::Horse),
            80 => Some(EntityDataType::Arrow),
            81 => Some(EntityDataType::Snowball),
            82 => Some(EntityDataType::Egg),
            87 => Some(EntityDataType::EnderPearl),
            64 => Some(EntityDataType::Item),
            66 => Some(EntityDataType::FallingSand),
            65 => Some(EntityDataType::PrimedTnt),
            69 => Some(EntityDataType::XpOrb),
            83 => Some(EntityDataType::Painting),
            93 => Some(EntityDataType::Lightning),
            77 => Some(EntityDataType::FishingHook),
            _ => None,
        }
    }

    /// Returns the MCPE network type ID for this entity type.
    pub fn as_id(self) -> i32 {
        self as i32
    }

    /// Returns the default eye height for this entity type.
    pub fn default_eye_height(self) -> f32 {
        match self {
            EntityDataType::Player => 1.62,
            EntityDataType::Zombie => 1.62,
            EntityDataType::Skeleton => 1.62,
            EntityDataType::Creeper => 1.32,
            EntityDataType::Enderman => 2.16,
            EntityDataType::Spider => 0.46,
            EntityDataType::Cow => 0.86,
            EntityDataType::Pig => 0.68,
            EntityDataType::Sheep => 0.78,
            EntityDataType::Chicken => 0.52,
            EntityDataType::Horse => 1.52,
            _ => 0.0,
        }
    }

    /// Returns the default bounding box for this entity type.
    pub fn default_bounding_box(self) -> BoundingBox {
        let (half_w, height) = match self {
            EntityDataType::Player => (0.3, 1.8),
            EntityDataType::Zombie => (0.3, 1.8),
            EntityDataType::Skeleton => (0.3, 1.8),
            EntityDataType::Creeper => (0.3, 1.5),
            EntityDataType::Enderman => (0.3, 2.6),
            EntityDataType::Spider => (0.5, 0.5),
            EntityDataType::Cow => (0.45, 1.1),
            EntityDataType::Pig => (0.45, 0.85),
            EntityDataType::Sheep => (0.45, 1.0),
            EntityDataType::Chicken => (0.2, 0.65),
            EntityDataType::Horse => (0.7, 1.6),
            _ => (0.3, 0.5),
        };
        BoundingBox::from_center_half_size(
            Vector3f::new(0.0, height / 2.0, 0.0),
            Vector3f::new(half_w, height / 2.0, half_w),
        )
    }

    /// Returns the default max health for this entity type.
    pub fn default_max_health(self) -> f32 {
        match self {
            EntityDataType::Player => 20.0,
            EntityDataType::Zombie => 20.0,
            EntityDataType::Skeleton => 20.0,
            EntityDataType::Creeper => 20.0,
            EntityDataType::Enderman => 40.0,
            EntityDataType::Spider => 16.0,
            EntityDataType::Cow => 10.0,
            EntityDataType::Pig => 10.0,
            EntityDataType::Sheep => 8.0,
            EntityDataType::Chicken => 4.0,
            EntityDataType::Horse => 20.0,
            _ => 1.0,
        }
    }

    /// Returns the default step height for this entity type.
    pub fn default_step_height(self) -> f32 {
        match self {
            EntityDataType::Player => 0.6,
            EntityDataType::Horse => 1.0,
            EntityDataType::Spider => 0.6,
            _ => 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Entity
// ---------------------------------------------------------------------------

/// Base entity structure.
///
/// Contains the core properties shared by all entities in the game.
pub struct Entity {
    /// Runtime entity ID (unique per session, assigned by EntityManager).
    pub id: u64,
    /// Unique persistent entity ID.
    pub unique_id: i64,
    /// The type of this entity.
    pub entity_type: EntityDataType,
    /// Position in world space.
    pub position: Vector3f,
    /// Motion (velocity) vector.
    pub motion: Vector3f,
    /// Rotation: (pitch, yaw, head_yaw).
    pub rotation: Vector3f,
    /// Whether the entity is on the ground.
    pub on_ground: bool,
    /// Current health.
    pub health: f32,
    /// Maximum health.
    pub max_health: f32,
    /// Entity metadata.
    pub metadata: EntityMetadata,
    /// Distance fallen since last on_ground = true.
    pub fall_distance: f32,
    /// Ticks remaining on fire (-1 = not on fire, 0 = not on fire, >0 = on fire).
    pub fire_ticks: i16,
    /// Ticks of air remaining.
    pub air_ticks: i16,
    /// Whether the entity is invulnerable.
    pub invulnerable: bool,
    /// Whether the entity is alive.
    pub alive: bool,
    /// Damage from the last attack.
    pub last_damage: f32,
    /// Bounding box for collision.
    pub bounding_box: BoundingBox,
    /// Eye height from the bottom of the entity.
    pub eye_height: f32,
    /// Step height for auto-stepping.
    pub step_height: f32,
}

impl Entity {
    /// Creates a new entity with the given type and position.
    pub fn new(entity_type: EntityDataType, position: Vector3f) -> Self {
        Self {
            id: 0,
            unique_id: 0,
            entity_type,
            position,
            motion: Vector3f::zero(),
            rotation: Vector3f::new(0.0, 0.0, 0.0),
            on_ground: false,
            health: entity_type.default_max_health(),
            max_health: entity_type.default_max_health(),
            metadata: EntityMetadata::new(),
            fall_distance: 0.0,
            fire_ticks: 0,
            air_ticks: 300,
            invulnerable: false,
            alive: true,
            last_damage: 0.0,
            bounding_box: entity_type.default_bounding_box(),
            eye_height: entity_type.default_eye_height(),
            step_height: entity_type.default_step_height(),
        }
    }

    /// Performs a tick for this entity.
    pub fn tick(&mut self) {
        // Fire tick countdown
        if self.fire_ticks > 0 {
            self.fire_ticks -= 1;
        }

        // Air supply (if submerged, decrease; otherwise, refill)
        // Simplified: just decrement when not on ground
        if !self.on_ground {
            // Simplified: no actual water check here
        } else {
            self.air_ticks = 300;
        }

        // Fall distance tracking
        if self.on_ground {
            self.fall_distance = 0.0;
        }

        // Update bounding box position
        self.bounding_box = self.entity_type.default_bounding_box().offset(self.position);
    }

    /// Damages this entity by the given amount.
    ///
    /// Returns `true` if the entity died from this damage.
    pub fn damage(&mut self, amount: f32) -> bool {
        if self.invulnerable || !self.alive {
            return false;
        }
        self.last_damage = amount;
        self.health = (self.health - amount).max(0.0);
        if self.health <= 0.0 {
            self.kill();
            return true;
        }
        false
    }

    /// Heals this entity by the given amount.
    pub fn heal(&mut self, amount: f32) {
        if !self.alive {
            return;
        }
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Kills this entity.
    pub fn kill(&mut self) {
        self.health = 0.0;
        self.alive = false;
    }

    /// Teleports the entity to the given position.
    pub fn teleport(&mut self, position: Vector3f) {
        self.position = position;
        self.motion = Vector3f::zero();
        self.bounding_box = self.entity_type.default_bounding_box().offset(position);
    }

    /// Returns `true` if this entity is alive.
    pub fn is_alive(&self) -> bool {
        self.alive && self.health > 0.0
    }

    /// Returns the MCPE network type ID for this entity.
    pub fn get_network_type_id(&self) -> i32 {
        self.entity_type.as_id()
    }

    /// Saves entity data to NBT format.
    pub fn save_nbt(&self) -> Tag {
        let mut compound = indexmap::IndexMap::new();

        compound.insert("id".to_string(), Tag::Int(self.entity_type.as_id()));
        compound.insert("EntityUniqueID".to_string(), Tag::Long(self.unique_id as i64));
        compound.insert("Health".to_string(), Tag::Float(self.health));
        compound.insert("MaxHealth".to_string(), Tag::Float(self.max_health));
        compound.insert("Pos".to_string(), Tag::List(vec![
            Tag::Float(self.position.x),
            Tag::Float(self.position.y),
            Tag::Float(self.position.z),
        ]));
        compound.insert("Motion".to_string(), Tag::List(vec![
            Tag::Float(self.motion.x),
            Tag::Float(self.motion.y),
            Tag::Float(self.motion.z),
        ]));
        compound.insert("Rotation".to_string(), Tag::List(vec![
            Tag::Float(self.rotation.x),
            Tag::Float(self.rotation.y),
        ]));
        compound.insert("FallDistance".to_string(), Tag::Float(self.fall_distance));
        compound.insert("Fire".to_string(), Tag::Short(self.fire_ticks));
        compound.insert("Air".to_string(), Tag::Short(self.air_ticks));
        compound.insert("OnGround".to_string(), Tag::Byte(self.on_ground as i8));
        compound.insert("Invulnerable".to_string(), Tag::Byte(self.invulnerable as i8));

        Tag::Compound(compound)
    }

    /// Loads entity data from NBT format.
    pub fn load_nbt(&mut self, tag: &Tag) -> Result<(), EntityError> {
        let compound = tag.get_compound()?;

        if let Some(Tag::Long(id)) = compound.get("EntityUniqueID") {
            self.unique_id = *id;
        }
        if let Some(Tag::Float(health)) = compound.get("Health") {
            self.health = *health;
        }
        if let Some(Tag::Float(max_health)) = compound.get("MaxHealth") {
            self.max_health = *max_health;
        }
        if let Some(Tag::List(pos)) = compound.get("Pos") {
            if pos.len() >= 3 {
                if let (Tag::Float(x), Tag::Float(y), Tag::Float(z)) = (&pos[0], &pos[1], &pos[2]) {
                    self.position = Vector3f::new(*x, *y, *z);
                }
            }
        }
        if let Some(Tag::List(motion)) = compound.get("Motion") {
            if motion.len() >= 3 {
                if let (Tag::Float(x), Tag::Float(y), Tag::Float(z)) = (&motion[0], &motion[1], &motion[2]) {
                    self.motion = Vector3f::new(*x, *y, *z);
                }
            }
        }
        if let Some(Tag::List(rot)) = compound.get("Rotation") {
            if rot.len() >= 2 {
                if let (Tag::Float(pitch), Tag::Float(yaw)) = (&rot[0], &rot[1]) {
                    self.rotation.x = *pitch;
                    self.rotation.y = *yaw;
                }
            }
        }
        if let Some(Tag::Float(fd)) = compound.get("FallDistance") {
            self.fall_distance = *fd;
        }
        if let Some(Tag::Short(fire)) = compound.get("Fire") {
            self.fire_ticks = *fire;
        }
        if let Some(Tag::Short(air)) = compound.get("Air") {
            self.air_ticks = *air;
        }
        if let Some(Tag::Byte(on_ground)) = compound.get("OnGround") {
            self.on_ground = *on_ground != 0;
        }
        if let Some(Tag::Byte(invuln)) = compound.get("Invulnerable") {
            self.invulnerable = *invuln != 0;
        }

        self.alive = self.health > 0.0;
        Ok(())
    }
}
