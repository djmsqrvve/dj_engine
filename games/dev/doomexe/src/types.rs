//! Game-specific types for doomexe.
//!
//! These types were moved from engine/src/types.rs to decouple
//! game-specific concepts from the core engine.

use bevy::prelude::*;

/// The main hamster character component with state tracking.
#[derive(Component, Resource, Default, Clone)]
pub struct HamsterNarrator {
    /// Corruption level (0.0–100.0)
    pub corruption: f32,
    /// Current facial expression
    pub expression: Expression,
    /// Animation time accumulator
    pub _animation_time: f32,
    /// Current mood state
    pub _mood: Mood,
}

impl HamsterNarrator {
    /// Creates a new hamster narrator with default state.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            _animation_time: 0.0,
            _mood: Mood::Neutral,
            ..Default::default()
        }
    }

    /// Sets corruption, clamping to valid range.
    pub fn set_corruption(&mut self, value: f32) {
        self.corruption = value.clamp(0.0, 100.0);
    }

    /// Gets corruption as normalized value (0.0–1.0).
    pub fn corruption_normalized(&self) -> f32 {
        self.corruption / 100.0
    }
}

/// Facial expression variants for the hamster.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Expression {
    #[default]
    Neutral,
    Happy,
    Angry,
    Sad,
    Corrupted,
    Confused,
    Amused,
}

impl Expression {
    /// Converts a string to an Expression, returning None if invalid.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "neutral" => Some(Self::Neutral),
            "happy" => Some(Self::Happy),
            "angry" => Some(Self::Angry),
            "sad" => Some(Self::Sad),
            "corrupted" => Some(Self::Corrupted),
            "confused" => Some(Self::Confused),
            "amused" => Some(Self::Amused),
            _ => None,
        }
    }

    /// Returns the expression name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Happy => "happy",
            Self::Angry => "angry",
            Self::Sad => "sad",
            Self::Corrupted => "corrupted",
            Self::Confused => "confused",
            Self::Amused => "amused",
        }
    }
}

/// Mood state for the hamster, affects animation intensity.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[allow(dead_code)]
pub enum Mood {
    #[default]
    Normal,
    Excited,
    Melancholy,
    Neutral,
}

/// Represents a hamster sprite part (child entity).
#[derive(Component, Clone)]
#[allow(dead_code)]
pub struct HamsterPart {
    /// Part type identifier (e.g., "body", "head", "eye_left")
    pub part_type: String,
    /// Offset from parent
    pub offset: Vec2,
    /// Z-order layer (0 = back, higher = front)
    pub layer: u32,
}

/// Shader uniform data for corruption effects.
#[derive(Clone, Copy, Default)]
#[allow(dead_code)]
pub struct CorruptionUniforms {
    /// Corruption level (0.0–1.0, normalized)
    pub corruption: f32,
    /// Time for animated effects
    pub time: f32,
    /// Which palette variant to use (0, 1, 2, 3)
    pub palette_shift: i32,
}
