//! Biome types for Minecraft Bedrock Edition.
//!
//! This module provides the [`Biome`] enum matching the MCPE biome IDs,
//! and a [`BiomeSelector`] for determining biomes based on noise values.

// ---------------------------------------------------------------------------
// Biome
// ---------------------------------------------------------------------------

/// Minecraft Bedrock Edition biome types with their protocol IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Biome {
    Ocean = 0,
    Plain = 1,
    Desert = 2,
    ExtremeHills = 3,
    Forest = 4,
    Taiga = 5,
    Swampland = 6,
    River = 7,
    Hell = 8,
    Sky = 9,
    FrozenOcean = 10,
    FrozenRiver = 11,
    IcePlains = 12,
    IceMountains = 13,
    MushroomIsland = 14,
    MushroomIslandShore = 15,
    Beach = 16,
    DesertHills = 17,
    ForestHills = 18,
    TaigaHills = 19,
    ExtremeHillsEdge = 20,
    Jungle = 21,
    JungleHills = 22,
    JungleEdge = 23,
    DeepOcean = 24,
    StoneBeach = 25,
    ColdBeach = 26,
    BirchForest = 27,
    BirchForestHills = 28,
    RoofedForest = 29,
    ColdTaiga = 30,
    ColdTaigaHills = 31,
    MegaTaiga = 32,
    MegaTaigaHills = 33,
    ExtremeHillsPlus = 34,
    Savanna = 35,
    SavannaPlateau = 36,
    Mesa = 37,
    MesaPlateauF = 38,
    MesaPlateau = 39,
    Void = 127,
}

impl Biome {
    /// Converts a biome ID byte to a [`Biome`] variant.
    ///
    /// Returns `Biome::Plain` as a fallback for unknown IDs.
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Biome::Ocean,
            1 => Biome::Plain,
            2 => Biome::Desert,
            3 => Biome::ExtremeHills,
            4 => Biome::Forest,
            5 => Biome::Taiga,
            6 => Biome::Swampland,
            7 => Biome::River,
            8 => Biome::Hell,
            9 => Biome::Sky,
            10 => Biome::FrozenOcean,
            11 => Biome::FrozenRiver,
            12 => Biome::IcePlains,
            13 => Biome::IceMountains,
            14 => Biome::MushroomIsland,
            15 => Biome::MushroomIslandShore,
            16 => Biome::Beach,
            17 => Biome::DesertHills,
            18 => Biome::ForestHills,
            19 => Biome::TaigaHills,
            20 => Biome::ExtremeHillsEdge,
            21 => Biome::Jungle,
            22 => Biome::JungleHills,
            23 => Biome::JungleEdge,
            24 => Biome::DeepOcean,
            25 => Biome::StoneBeach,
            26 => Biome::ColdBeach,
            27 => Biome::BirchForest,
            28 => Biome::BirchForestHills,
            29 => Biome::RoofedForest,
            30 => Biome::ColdTaiga,
            31 => Biome::ColdTaigaHills,
            32 => Biome::MegaTaiga,
            33 => Biome::MegaTaigaHills,
            34 => Biome::ExtremeHillsPlus,
            35 => Biome::Savanna,
            36 => Biome::SavannaPlateau,
            37 => Biome::Mesa,
            38 => Biome::MesaPlateauF,
            39 => Biome::MesaPlateau,
            127 => Biome::Void,
            _ => Biome::Plain,
        }
    }

    /// Returns the protocol ID for this biome.
    pub fn as_id(self) -> u8 {
        self as u8
    }

    /// Returns `true` if this is a cold biome.
    pub fn is_cold(self) -> bool {
        matches!(
            self,
            Biome::FrozenOcean
                | Biome::FrozenRiver
                | Biome::IcePlains
                | Biome::IceMountains
                | Biome::ColdBeach
                | Biome::ColdTaiga
                | Biome::ColdTaigaHills
        )
    }

    /// Returns `true` if this is a hot/dry biome.
    pub fn is_hot(self) -> bool {
        matches!(
            self,
            Biome::Desert | Biome::DesertHills | Biome::Hell | Biome::Mesa | Biome::MesaPlateauF | Biome::MesaPlateau
        )
    }

    /// Returns `true` if this is an ocean/water biome.
    pub fn is_ocean(self) -> bool {
        matches!(self, Biome::Ocean | Biome::DeepOcean | Biome::FrozenOcean)
    }
}

impl Default for Biome {
    fn default() -> Self {
        Biome::Plain
    }
}

// ---------------------------------------------------------------------------
// BiomeSelector
// ---------------------------------------------------------------------------

/// Selects biomes based on temperature and rainfall noise values.
///
/// This is a simplified biome selection system that maps 2D noise values
/// to biome types, similar to the vanilla Minecraft biome generation.
pub struct BiomeSelector {
    /// Temperature threshold for cold biomes.
    pub cold_threshold: f64,
    /// Temperature threshold for hot biomes.
    pub hot_threshold: f64,
    /// Rainfall threshold for dry biomes.
    pub dry_threshold: f64,
    /// Rainfall threshold for wet biomes.
    pub wet_threshold: f64,
}

impl BiomeSelector {
    /// Creates a new biome selector with default thresholds.
    pub fn new() -> Self {
        Self {
            cold_threshold: -0.3,
            hot_threshold: 0.3,
            dry_threshold: -0.2,
            wet_threshold: 0.3,
        }
    }

    /// Selects a biome based on temperature and rainfall noise values.
    ///
    /// `temperature` and `rainfall` should be in the range approximately [-1, 1].
    pub fn select(&self, temperature: f64, rainfall: f64) -> Biome {
        if temperature < self.cold_threshold {
            // Cold biomes
            if rainfall > self.wet_threshold {
                Biome::ColdTaiga
            } else {
                Biome::IcePlains
            }
        } else if temperature > self.hot_threshold {
            // Hot biomes
            if rainfall < self.dry_threshold {
                Biome::Desert
            } else if rainfall > self.wet_threshold {
                Biome::Jungle
            } else {
                Biome::Savanna
            }
        } else {
            // Temperate biomes
            if rainfall < self.dry_threshold {
                Biome::Plain
            } else if rainfall > self.wet_threshold {
                Biome::Forest
            } else {
                Biome::BirchForest
            }
        }
    }
}

impl Default for BiomeSelector {
    fn default() -> Self {
        Self::new()
    }
}
