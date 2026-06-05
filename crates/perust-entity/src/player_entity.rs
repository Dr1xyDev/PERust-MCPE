//! Player entity type for Minecraft Bedrock Edition.
//!
//! This module provides [`PlayerEntity`], which extends [`LivingEntity`]
//! with player-specific properties like gamemode, food, experience, and abilities.

use perust_utils::math::Vector3f;
use perust_protocol::types::GameMode;
use crate::entity::EntityDataType;
use crate::living::LivingEntity;
use crate::attribute;

// ---------------------------------------------------------------------------
// AdventureSettings
// ---------------------------------------------------------------------------

/// Adventure settings flags for a player.
#[derive(Debug, Clone, Copy, Default)]
pub struct AdventureSettings {
    /// Whether the player can fly.
    pub allow_flight: bool,
    /// Whether the player is currently flying.
    pub flying: bool,
    /// Whether the player can build (place/break blocks).
    pub can_build: bool,
    /// Whether the player can mine (break blocks).
    pub can_mine: bool,
    /// Whether the player can interact with entities/blocks.
    pub can_interact: bool,
    /// Whether the player can attack entities.
    pub can_attack: bool,
    /// Whether the player's world is immutable.
    pub world_immutable: bool,
    /// Whether the player is muted (cannot send chat).
    pub muted: bool,
    /// Whether no-clip is enabled.
    pub no_clip: bool,
}

impl AdventureSettings {
    /// Creates adventure settings appropriate for the given game mode.
    pub fn for_gamemode(gamemode: GameMode) -> Self {
        match gamemode {
            GameMode::Survival => Self {
                can_build: true,
                can_mine: true,
                can_interact: true,
                can_attack: true,
                ..Default::default()
            },
            GameMode::Creative => Self {
                allow_flight: true,
                can_build: true,
                can_mine: true,
                can_interact: true,
                can_attack: true,
                ..Default::default()
            },
            GameMode::Adventure => Self {
                can_interact: true,
                can_attack: true,
                world_immutable: true,
                ..Default::default()
            },
            GameMode::Spectator => Self {
                allow_flight: true,
                flying: true,
                no_clip: true,
                world_immutable: true,
                ..Default::default()
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Abilities
// ---------------------------------------------------------------------------

/// Player abilities (fly, build, mine, etc.).
#[derive(Debug, Clone, Copy, Default)]
pub struct Abilities {
    /// Whether the player can fly.
    pub fly: bool,
    /// Whether the player is currently flying.
    pub flying: bool,
    /// Whether the player can build.
    pub build: bool,
    /// Whether the player can mine.
    pub mine: bool,
    /// Whether the player has noclip.
    pub noclip: bool,
    /// Whether the player can use commands.
    pub operator: bool,
    /// Whether the player can teleport.
    pub teleport: bool,
    /// Whether the player is invulnerable.
    pub invulnerable: bool,
}

impl Abilities {
    /// Creates abilities appropriate for the given game mode.
    pub fn for_gamemode(gamemode: GameMode) -> Self {
        match gamemode {
            GameMode::Survival => Self {
                build: true,
                mine: true,
                ..Default::default()
            },
            GameMode::Creative => Self {
                fly: true,
                build: true,
                mine: true,
                invulnerable: true,
                ..Default::default()
            },
            GameMode::Adventure => Self {
                ..Default::default()
            },
            GameMode::Spectator => Self {
                fly: true,
                flying: true,
                noclip: true,
                invulnerable: true,
                ..Default::default()
            },
        }
    }
}

// ---------------------------------------------------------------------------
// PlayerEntity
// ---------------------------------------------------------------------------

/// A player entity with gamemode, food, experience, and abilities.
pub struct PlayerEntity {
    /// The base living entity.
    pub living: LivingEntity,
    /// Player game mode.
    pub gamemode: GameMode,
    /// Food/hunger level (0–20).
    pub food: i32,
    /// Food saturation level (0–20).
    pub food_saturation: f32,
    /// Food exhaustion level.
    pub food_exhaustion: f32,
    /// Food tick timer.
    pub food_tick_timer: i32,
    /// Experience points.
    pub experience: i32,
    /// Experience level.
    pub experience_level: i32,
    /// Experience progress toward next level (0.0–1.0).
    pub experience_progress: f32,
    /// Player abilities.
    pub abilities: Abilities,
    /// Adventure settings.
    pub adventure_settings: AdventureSettings,
    /// Player display name.
    pub display_name: String,
    /// Whether the player is sleeping.
    pub sleeping: bool,
    /// Spawn point (override), if set.
    pub spawn_point: Option<(i32, i32, i32)>,
    /// Whether the player has loaded the world.
    pub spawned: bool,
}

impl PlayerEntity {
    /// Creates a new player entity at the given position.
    pub fn new(position: Vector3f) -> Self {
        Self::with_gamemode(position, GameMode::Survival)
    }

    /// Creates a new player entity with the given game mode.
    pub fn with_gamemode(position: Vector3f, gamemode: GameMode) -> Self {
        let mut living = LivingEntity::new(EntityDataType::Player, position);
        living.entity.max_health = 20.0;
        living.entity.health = 20.0;
        living.attributes = attribute::default_player_attributes();

        let abilities = Abilities::for_gamemode(gamemode);
        let adventure_settings = AdventureSettings::for_gamemode(gamemode);

        Self {
            living,
            gamemode,
            food: 20,
            food_saturation: 20.0,
            food_exhaustion: 0.0,
            food_tick_timer: 0,
            experience: 0,
            experience_level: 0,
            experience_progress: 0.0,
            abilities,
            adventure_settings,
            display_name: String::new(),
            sleeping: false,
            spawn_point: None,
            spawned: false,
        }
    }

    /// Sets the player's game mode and updates abilities accordingly.
    pub fn set_gamemode(&mut self, gamemode: GameMode) {
        self.gamemode = gamemode;
        self.abilities = Abilities::for_gamemode(gamemode);
        self.adventure_settings = AdventureSettings::for_gamemode(gamemode);

        // Update entity properties based on gamemode
        match gamemode {
            GameMode::Creative | GameMode::Spectator => {
                self.living.entity.invulnerable = true;
                self.abilities.fly = true;
            }
            GameMode::Survival | GameMode::Adventure => {
                self.living.entity.invulnerable = false;
                self.abilities.fly = false;
                self.abilities.flying = false;
            }
        }
    }

    /// Adds experience points.
    ///
    /// Handles level-ups and experience progress updates.
    pub fn add_experience(&mut self, amount: i32) {
        self.experience += amount;
        while self.experience >= self.experience_for_next_level() {
            self.experience -= self.experience_for_next_level();
            self.experience_level += 1;
        }
        self.experience_progress = self.experience as f32 / self.experience_for_next_level() as f32;
    }

    /// Returns the experience required to advance from the current level to the next.
    pub fn experience_for_next_level(&self) -> i32 {
        if self.experience_level >= 30 {
            112 + (self.experience_level - 30) * 9
        } else if self.experience_level >= 15 {
            37 + (self.experience_level - 15) * 5
        } else {
            7 + self.experience_level * 2
        }
    }

    /// Gets the total experience (level + progress).
    pub fn total_experience(&self) -> i32 {
        let mut total = 0;
        for level in 0..self.experience_level {
            if level >= 30 {
                total += 112 + (level - 30) * 9;
            } else if level >= 15 {
                total += 37 + (level - 15) * 5;
            } else {
                total += 7 + level * 2;
            }
        }
        total + self.experience
    }

    /// Damages the player.
    pub fn damage(&mut self, amount: f32) -> bool {
        if self.gamemode == GameMode::Creative || self.gamemode == GameMode::Spectator {
            return false;
        }
        self.living.damage(amount)
    }

    /// Heals the player.
    pub fn heal(&mut self, amount: f32) {
        self.living.heal(amount);
    }

    /// Performs a tick for this player entity.
    pub fn tick(&mut self) {
        self.living.tick();

        // Food tick
        if self.gamemode == GameMode::Survival {
            self.food_tick_timer += 1;
            if self.food_tick_timer >= 80 {
                self.food_tick_timer = 0;
                self.tick_food();
            }
        }
    }

    /// Processes food-related mechanics.
    fn tick_food(&mut self) {
        if self.food >= 18 && self.living.entity.health < self.living.entity.max_health {
            // Natural regeneration
            self.living.heal(1.0);
            self.add_exhaustion(3.0);
        } else if self.food <= 0 {
            // Starvation damage
            let difficulty = 2; // Normal difficulty
            if difficulty > 0 {
                self.living.entity.damage(1.0);
            }
        }

        // Saturation depletion
        if self.food_saturation > 0.0 && self.food > 0 {
            self.food_saturation = (self.food_saturation - 0.1).max(0.0);
        } else if self.food > 0 {
            self.food = (self.food - 1).max(0);
        }
    }

    /// Adds exhaustion to the player (depletes saturation/food).
    pub fn add_exhaustion(&mut self, amount: f32) {
        self.food_exhaustion += amount;
        while self.food_exhaustion >= 4.0 {
            self.food_exhaustion -= 4.0;
            if self.food_saturation > 0.0 {
                self.food_saturation = (self.food_saturation - 1.0).max(0.0);
            } else {
                self.food = (self.food - 1).max(0);
            }
        }
    }

    /// Feeds the player, setting food and saturation levels.
    pub fn feed(&mut self, food: i32, saturation: f32) {
        self.food = (self.food + food).min(20);
        self.food_saturation = (self.food_saturation + saturation).min(self.food as f32);
    }

    /// Returns `true` if the player is alive.
    pub fn is_alive(&self) -> bool {
        self.living.is_alive()
    }

    /// Returns the player's position.
    pub fn position(&self) -> perust_utils::math::Vector3f {
        self.living.entity.position
    }

    /// Teleports the player to the given position.
    pub fn teleport(&mut self, position: perust_utils::math::Vector3f) {
        self.living.entity.teleport(position);
    }

    /// Returns the player's gamemode.
    pub fn gamemode(&self) -> GameMode {
        self.gamemode
    }

    /// Returns the player's health.
    pub fn health(&self) -> f32 {
        self.living.entity.health
    }

    /// Returns the player's max health.
    pub fn max_health(&self) -> f32 {
        self.living.entity.max_health
    }

    /// Returns the player's food level.
    pub fn food(&self) -> i32 {
        self.food
    }

    /// Sets the player's food level.
    pub fn set_food(&mut self, food: i32) {
        self.food = food.clamp(0, 20);
    }
}
