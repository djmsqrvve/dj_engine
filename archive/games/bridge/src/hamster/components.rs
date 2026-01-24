//! Character sprite components for doomexe.

use bevy::prelude::*;

/// Root marker for assembled characters.
#[derive(Component, Default)]
pub struct CharacterRoot {
    pub corruption: f32,
    pub expression: Expression,
}

impl CharacterRoot {
    /// Create a new character root with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Character expression state.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Expression {
    #[default]
    Neutral,
    Happy,
    Angry,
}

impl Expression {
    /// Cycle to next expression.
    pub fn next(self) -> Self {
        match self {
            Expression::Neutral => Expression::Happy,
            Expression::Happy => Expression::Angry,
            Expression::Angry => Expression::Neutral,
        }
    }

    /// Get sprite suffix for this expression.
    #[allow(dead_code)]
    pub fn sprite_suffix(&self) -> &'static str {
        match self {
            Expression::Neutral => "head.png",
            Expression::Happy => "head_happy.png",
            Expression::Angry => "head_angry.png",
        }
    }
}

/// Marker for individual sprite parts.
#[derive(Component)]
pub struct SpritePart {
    #[allow(dead_code)]
    pub kind: PartKind,
    #[allow(dead_code)]
    pub z_layer: u32,
}

/// Types of character parts.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PartKind {
    Body,
    Head,
    LeftEye,
    RightEye,
    Mouth,
    LeftPaw,
    RightPaw,
    LeftFoot,
    RightFoot,
}

/// Breathing animation - smooth scale oscillation.
#[derive(Component)]
pub struct BreathingAnimation {
    pub amplitude: f32,
    pub frequency: f32,
}

impl Default for BreathingAnimation {
    fn default() -> Self {
        Self {
            amplitude: 0.03,
            frequency: 0.5,
        }
    }
}

impl BreathingAnimation {
    /// Create with hamster-specific defaults.
    pub fn hamster_default() -> Self {
        Self {
            amplitude: 0.04,
            frequency: 0.6,
        }
    }
}

/// Blinking animation - random eye closure.
#[derive(Component)]
pub struct BlinkingAnimation {
    pub timer: Timer,
    pub is_closed: bool,
    pub min_interval: f32,
    pub max_interval: f32,
}

impl Default for BlinkingAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(4.0, TimerMode::Once),
            is_closed: false,
            min_interval: 3.0,
            max_interval: 7.0,
        }
    }
}

/// Idle motion - subtle head movement.
#[derive(Component)]
pub struct IdleMotion {
    pub amplitude: Vec2,
    pub frequency: f32,
    pub base_offset: Vec2,
}

impl Default for IdleMotion {
    fn default() -> Self {
        Self {
            amplitude: Vec2::new(3.0, 1.5),
            frequency: 0.15,
            base_offset: Vec2::ZERO,
        }
    }
}

/// Corruption visual effect parameters.
#[derive(Component, Default)]
pub struct CorruptionEffect {
    pub level: f32,
    pub palette_index: u32,
    pub jitter_amplitude: f32,
    pub chromatic_aberration: f32,
}

impl CorruptionEffect {
    /// Update effect parameters based on corruption level.
    pub fn update_from_level(&mut self, corruption: f32) {
        self.level = corruption;
        self.palette_index = (corruption / 25.0).floor() as u32;
        self.jitter_amplitude = corruption / 200.0;
        self.chromatic_aberration = (corruption / 100.0) * 2.0;
    }
}

/// Marker for head parts that respond to expression changes.
#[derive(Component)]
pub struct ExpressionSprite {
    pub neutral: Handle<Image>,
    pub happy: Handle<Image>,
    pub angry: Handle<Image>,
}

impl ExpressionSprite {
    /// Get sprite handle for given expression.
    pub fn handle_for(&self, expr: Expression) -> &Handle<Image> {
        match expr {
            Expression::Neutral => &self.neutral,
            Expression::Happy => &self.happy,
            Expression::Angry => &self.angry,
        }
    }
}
