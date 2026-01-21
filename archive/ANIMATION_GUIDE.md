# Animation Implementation Guide

**For**: DJ Engine Sprite System  
**Status**: Ready to Code

---

## Quick Start

### 1. Add Dependencies

```toml
[dependencies]
bevy = { version = "0.15", features = ["default_font"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
```

### 2. Create Components

```rust
// src/hamster/components.rs
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CharacterRoot {
    pub corruption: f32,
    pub expression: Expression,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    #[default]
    Neutral,
    Happy,
    Angry,
}

#[derive(Component)]
pub struct SpritePart {
    pub kind: PartKind,
    pub z_layer: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartKind {
    Body, Head, LeftEye, RightEye, Mouth, LeftPaw, RightPaw, LeftFoot, RightFoot,
}

#[derive(Component)]
pub struct BreathingAnimation {
    pub amplitude: f32,
    pub frequency: f32,
}

impl Default for BreathingAnimation {
    fn default() -> Self {
        Self { amplitude: 0.03, frequency: 0.5 }
    }
}

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
```

---

### 3. Implement Systems

```rust
// src/hamster/systems.rs
use bevy::prelude::*;
use std::f32::consts::PI;
use super::components::*;

pub fn breathing_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &BreathingAnimation)>,
) {
    let t = time.elapsed_secs();
    for (mut transform, anim) in &mut query {
        let phase = t * anim.frequency * 2.0 * PI;
        let scale_y = 1.0 + anim.amplitude * phase.sin();
        let scale_x = 1.0 / scale_y;
        transform.scale = Vec3::new(scale_x, scale_y, 1.0);
    }
}

pub fn blinking_system(
    time: Res<Time>,
    mut query: Query<(&mut BlinkingAnimation, &mut Sprite)>,
) {
    for (mut blink, mut sprite) in &mut query {
        blink.timer.tick(time.delta());
        
        if blink.timer.just_finished() {
            blink.is_closed = !blink.is_closed;
            
            let duration = if blink.is_closed {
                0.1 // Eyes closed briefly
            } else {
                // Random interval until next blink
                blink.min_interval + rand::random::<f32>() * (blink.max_interval - blink.min_interval)
            };
            
            blink.timer = Timer::from_seconds(duration, TimerMode::Once);
        }
        
        sprite.color.set_alpha(if blink.is_closed { 0.0 } else { 1.0 });
    }
}

pub fn idle_motion_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &IdleMotion)>,
) {
    let t = time.elapsed_secs();
    for (mut transform, motion) in &mut query {
        let phase = t * motion.frequency * 2.0 * PI;
        let offset_x = motion.amplitude.x * phase.sin();
        let offset_y = motion.amplitude.y * (phase * 1.3).cos();
        transform.translation.x = motion.base_offset.x + offset_x;
        transform.translation.y = motion.base_offset.y + offset_y;
    }
}
```

---

### 4. Create Assembly Function

```rust
// src/hamster/assembly.rs
use bevy::prelude::*;
use super::components::*;

pub fn spawn_character(
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    let root = commands.spawn((
        CharacterRoot::default(),
        Transform::from_xyz(0.0, 0.0, 100.0),
        Visibility::default(),
    )).id();

    // Body (with breathing)
    let body = commands.spawn((
        Sprite {
            image: asset_server.load("sprites/hamster/body/body.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0),
        SpritePart { kind: PartKind::Body, z_layer: 100 },
        BreathingAnimation::default(),
    )).id();
    commands.entity(root).add_child(body);

    // Head (with idle motion)
    let head = commands.spawn((
        Sprite {
            image: asset_server.load("sprites/hamster/head/neutral.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 26.0, 101.0),
        SpritePart { kind: PartKind::Head, z_layer: 101 },
        IdleMotion { base_offset: Vec2::new(0.0, 26.0), ..default() },
    )).id();
    commands.entity(root).add_child(head);

    // Eyes (with blinking)
    for (kind, x_offset) in [(PartKind::LeftEye, -8.0), (PartKind::RightEye, 8.0)] {
        let eye = commands.spawn((
            Sprite {
                image: asset_server.load("sprites/hamster/eyes/open.png"),
                ..default()
            },
            Transform::from_xyz(x_offset, 32.0, 104.0),
            SpritePart { kind, z_layer: 104 },
            BlinkingAnimation::default(),
        )).id();
        commands.entity(root).add_child(eye);
    }

    // Paws
    for (kind, x) in [(PartKind::LeftPaw, -20.0), (PartKind::RightPaw, 20.0)] {
        let paw = commands.spawn((
            Sprite {
                image: asset_server.load("sprites/hamster/paws/paw.png"),
                flip_x: x > 0.0,
                ..default()
            },
            Transform::from_xyz(x, -5.0, 102.0),
            SpritePart { kind, z_layer: 102 },
        )).id();
        commands.entity(root).add_child(paw);
    }

    root
}
```

---

### 5. Register Plugin

```rust
// src/hamster/mod.rs
mod components;
mod systems;
mod assembly;

pub use components::*;
pub use assembly::spawn_character;

use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
           .add_systems(Update, (
               systems::breathing_system,
               systems::blinking_system,
               systems::idle_motion_system,
           ));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    spawn_character(&mut commands, &asset_server);
}
```

---

## Testing Checklist

- [ ] Character appears on screen
- [ ] Body has subtle breathing motion
- [ ] Eyes blink at random intervals
- [ ] Head has slight idle movement
- [ ] No z-fighting between parts
- [ ] 60+ FPS maintained

---

## Common Issues

| Problem | Solution |
|---------|----------|
| Sprites not visible | Check asset paths, verify camera spawned |
| Parts in wrong order | Use absolute Z values (100, 101, 104, 105) |
| Jerky animation | Ensure systems run in Update, not Startup |
| Blurry sprites | Add `ImagePlugin::default_nearest()` |

---

## Debug Controls (Optional)

```rust
pub fn debug_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CharacterRoot>,
) {
    for mut root in &mut query {
        if keys.just_pressed(KeyCode::Digit1) { root.expression = Expression::Neutral; }
        if keys.just_pressed(KeyCode::Digit2) { root.expression = Expression::Happy; }
        if keys.just_pressed(KeyCode::Digit3) { root.expression = Expression::Angry; }
        if keys.pressed(KeyCode::KeyU) { root.corruption = (root.corruption + 0.5).min(100.0); }
        if keys.pressed(KeyCode::KeyD) { root.corruption = (root.corruption - 0.5).max(0.0); }
    }
}
```
