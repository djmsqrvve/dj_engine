# Sprite System Implementation Guide – Quick Reference

**For**: Hamster Narrator (Milestone 1)  
**Status**: Ready to Code  
**Last Updated**: 2026-01-20  

---

## Part 1: Quick Architecture Overview

### What You're Building

A **procedurally assembled, animated pixel-art hamster** that:
- Composes from individual sprite parts (body, head, eyes, mouth, ears)
- Animates with breathing, blinking, and idle motion
- Changes expression & color based on Lua state (`corruption` value)
- Renders at 320×240 with nearest-neighbor upscaling + CRT effects

### The Hierarchy

```
HamsterNarrator (root entity)
├─ Body (child, z=100)
├─ Head (child, z=101)
├─ LeftEar (child, z=102)
├─ RightEar (child, z=102)
├─ LeftEye (child, z=104)
├─ RightEye (child, z=104)
└─ Mouth (child, z=105)
```

**Important**: Z-indices are **absolute in world space**, not relative to parent.

---

## Part 2: Component Skeleton (Copy-Paste Ready)

### File: `src/hamster/components.rs`

```rust
use bevy::prelude::*;
use std::time::Duration;

// ============================================================================
// CORE COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone, Copy)]
pub struct HamsterNarrator {
    pub corruption: f32,           // 0..=100 (%)
    pub expression: Expression,
    pub animation_time: f32,       // Elapsed time (for phase calculations)
    pub mood: Option<Mood>,
}

impl Default for HamsterNarrator {
    fn default() -> Self {
        Self {
            corruption: 0.0,
            expression: Expression::Neutral,
            animation_time: 0.0,
            mood: None,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct HamsterPart {
    pub part_type: PartType,
    pub offset: Vec2,              // Trim offset from Aseprite export
    pub layer: u32,                // Z-order (should match z in transform)
    pub base_rotation: f32,        // Default rotation (radians)
}

// ============================================================================
// ANIMATION COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct BreathingAnimation {
    pub amplitude: f32,            // 0.05 = 5% scale variation
    pub frequency: f32,            // Hz (0.5 = one breath every 2s)
    pub phase_offset: f32,         // For staggered breathing (0..2π)
}

impl Default for BreathingAnimation {
    fn default() -> Self {
        Self {
            amplitude: 0.05,
            frequency: 0.5,
            phase_offset: 0.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct BlinkingAnimation {
    pub timer: Timer,
    pub blink_duration: Duration,  // How long eyes are closed
    pub interval_range: (f32, f32), // (min, max) seconds between blinks
    pub state: BlinkState,
    pub is_left_eye: bool,         // To distinguish left/right for async blinking
}

impl BlinkingAnimation {
    pub fn new(is_left_eye: bool) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
            blink_duration: Duration::from_millis(100),
            interval_range: (3.0, 7.0),
            state: BlinkState::Open,
            is_left_eye,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct IdleMotion {
    pub position_amplitude: Vec2,  // Max displacement (e.g., 5.0, 2.0)
    pub frequency: f32,            // Hz (lower than breathing, e.g., 0.2)
    pub phase_offset: f32,
}

impl Default for IdleMotion {
    fn default() -> Self {
        Self {
            position_amplitude: Vec2::new(5.0, 2.0),
            frequency: 0.2,
            phase_offset: 0.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CorruptionEffect {
    pub corruption_level: f32,     // 0..=100
    pub palette_shift_index: u32,  // Which palette to use (0..4)
    pub raster_jitter_amplitude: f32,
    pub chromatic_aberration: f32,
}

impl Default for CorruptionEffect {
    fn default() -> Self {
        Self {
            corruption_level: 0.0,
            palette_shift_index: 0,
            raster_jitter_amplitude: 0.0,
            chromatic_aberration: 0.0,
        }
    }
}

// ============================================================================
// ENUMS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartType {
    Body,
    Head,
    LeftEar,
    RightEar,
    LeftEye,
    RightEye,
    Mouth,
    LeftPaw,
    RightPaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    Neutral = 0,
    Happy = 1,
    Angry = 2,
    Corrupted = 3,
}

impl Expression {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => Expression::Neutral,
            1 => Expression::Happy,
            2 => Expression::Angry,
            3 => Expression::Corrupted,
            _ => Expression::Neutral,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Content,
    Suspicious,
    Horrified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkState {
    Open,
    Closing,
    Closed,
    Opening,
}
```

---

## Part 3: Assembly Function (Ready to Use)

### File: `src/hamster/assembly.rs`

```rust
use bevy::prelude::*;
use crate::hamster::components::*;

pub fn assemble_hamster(
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    // 1. Spawn root entity (HamsterNarrator)
    let hamster = commands
        .spawn((
            HamsterNarrator::default(),
            Transform::from_xyz(0.0, 0.0, 100.0), // Z = 100 (base layer)
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    // 2. Spawn body
    spawn_part(
        commands,
        asset_server,
        hamster,
        PartType::Body,
        "sprites/hamster/body/hamster_body.png",
        Vec2::ZERO,
        100,
        true, // Apply breathing animation to body only
    );

    // 3. Spawn head
    spawn_part(
        commands,
        asset_server,
        hamster,
        PartType::Head,
        "sprites/hamster/head/hamster_head.png",
        Vec2::new(0.0, 30.0),
        101,
        false,
    );

    // 4. Spawn ears
    spawn_part(
        commands,
        asset_server,
        hamster,
        PartType::LeftEar,
        "sprites/hamster/ears/hamster_ear_left.png",
        Vec2::new(-20.0, 45.0),
        102,
        false,
    );

    spawn_part(
        commands,
        asset_server,
        hamster,
        PartType::RightEar,
        "sprites/hamster/ears/hamster_ear_right.png",
        Vec2::new(20.0, 45.0),
        102,
        false,
    );

    // 5. Spawn eyes (with blinking)
    spawn_part_with_blinking(
        commands,
        asset_server,
        hamster,
        PartType::LeftEye,
        "sprites/hamster/eyes/hamster_eye_left_open.png",
        Vec2::new(-10.0, 35.0),
        104,
        true, // is_left_eye
    );

    spawn_part_with_blinking(
        commands,
        asset_server,
        hamster,
        PartType::RightEye,
        "sprites/hamster/eyes/hamster_eye_right_open.png",
        Vec2::new(10.0, 35.0),
        104,
        false, // is_left_eye (right eye)
    );

    // 6. Spawn mouth
    spawn_part(
        commands,
        asset_server,
        hamster,
        PartType::Mouth,
        "sprites/hamster/mouth/hamster_mouth_neutral.png",
        Vec2::new(0.0, 20.0),
        105,
        false,
    );

    hamster
}

// Helper function to spawn a part
fn spawn_part(
    commands: &mut Commands,
    asset_server: &AssetServer,
    parent: Entity,
    part_type: PartType,
    sprite_path: &str,
    offset: Vec2,
    z_layer: u32,
    add_breathing: bool,
) {
    let part = commands
        .spawn((
            Sprite {
                image: asset_server.load(sprite_path),
                custom_size: None, // Use natural sprite size
                ..default()
            },
            Transform::from_xyz(offset.x, offset.y, z_layer as f32),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            HamsterPart {
                part_type,
                offset,
                layer: z_layer,
                base_rotation: 0.0,
            },
            CorruptionEffect::default(),
        ))
        .id();

    if add_breathing {
        commands.entity(part).insert(BreathingAnimation::default());
    }

    // Add as child of hamster
    commands.entity(parent).add_child(part);
}

// Helper function to spawn eyes with blinking
fn spawn_part_with_blinking(
    commands: &mut Commands,
    asset_server: &AssetServer,
    parent: Entity,
    part_type: PartType,
    sprite_path: &str,
    offset: Vec2,
    z_layer: u32,
    is_left_eye: bool,
) {
    let part = commands
        .spawn((
            Sprite {
                image: asset_server.load(sprite_path),
                custom_size: None,
                ..default()
            },
            Transform::from_xyz(offset.x, offset.y, z_layer as f32),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            HamsterPart {
                part_type,
                offset,
                layer: z_layer,
                base_rotation: 0.0,
            },
            BlinkingAnimation::new(is_left_eye),
            CorruptionEffect::default(),
        ))
        .id();

    commands.entity(parent).add_child(part);
}
```

---

## Part 4: Animation Systems (Minimal, Working Version)

### File: `src/hamster/systems.rs`

```rust
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::hamster::components::*;

// ============================================================================
// BREATHING SYSTEM
// ============================================================================

pub fn breathing_system(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &BreathingAnimation),
        With<HamsterPart>,
    >,
) {
    let elapsed = time.elapsed_secs();

    for (mut transform, breathing) in &mut query {
        // Compute phase: frequency * 2π * time + offset
        let phase = (elapsed * breathing.frequency * 2.0 * PI) + breathing.phase_offset;

        // Y scale oscillates: 1 ± amplitude
        let scale_y = 1.0 + breathing.amplitude * phase.sin();

        // X scale inverse (conserve area)
        let scale_x = 1.0 / scale_y;

        transform.scale = Vec3::new(scale_x, scale_y, 1.0);
    }
}

// ============================================================================
// BLINKING SYSTEM
// ============================================================================

pub fn blinking_system(
    time: Res<Time>,
    mut query: Query<
        (&mut BlinkingAnimation, &mut Sprite, &PartType),
        With<HamsterPart>,
    >,
) {
    for (mut blink, mut sprite, part_type) in &mut query {
        // Only process eye parts
        if !matches!(part_type, PartType::LeftEye | PartType::RightEye) {
            continue;
        }

        blink.timer.tick(time.delta());

        // State machine for blinking
        if blink.timer.just_finished() {
            match blink.state {
                BlinkState::Open => {
                    // Start closing
                    blink.state = BlinkState::Closing;
                    blink.timer = Timer::new(blink.blink_duration, TimerMode::Once);
                    sprite.color.set_a(1.0); // Fully visible
                }
                BlinkState::Closing => {
                    // Go closed
                    blink.state = BlinkState::Closed;
                    blink.timer = Timer::new(std::time::Duration::from_millis(50), TimerMode::Once);
                    sprite.color.set_a(0.0); // Hidden
                }
                BlinkState::Closed => {
                    // Re-open
                    blink.state = BlinkState::Open;
                    let next_interval = rand::random::<f32>()
                        * (blink.interval_range.1 - blink.interval_range.0)
                        + blink.interval_range.0;
                    blink.timer = Timer::new(
                        std::time::Duration::from_secs_f32(next_interval),
                        TimerMode::Once,
                    );
                    sprite.color.set_a(1.0); // Fully visible
                }
                BlinkState::Opening => {
                    // Fallback (not used currently)
                    blink.state = BlinkState::Open;
                }
            }
        }
    }
}

// ============================================================================
// IDLE MOTION SYSTEM
// ============================================================================

pub fn idle_motion_system(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &IdleMotion, &PartType),
        With<HamsterPart>,
    >,
) {
    let elapsed = time.elapsed_secs();

    for (mut transform, idle, part_type) in &mut query {
        // Apply idle motion only to head (for subtlety)
        if !matches!(part_type, PartType::Head) {
            continue;
        }

        let phase = (elapsed * idle.frequency * 2.0 * PI) + idle.phase_offset;

        let offset_x = idle.position_amplitude.x * phase.sin();
        let offset_y = idle.position_amplitude.y * (phase * 1.5).cos();

        transform.translation.x = offset_x;
        transform.translation.y = 30.0 + offset_y; // 30 is head base offset
    }
}

// ============================================================================
// CORRUPTION SYSTEM
// ============================================================================

pub fn corruption_system(
    mut query: Query<
        (&HamsterNarrator, &mut CorruptionEffect),
        With<HamsterPart>,
    >,
) {
    for (narrator, mut corruption) in &mut query {
        corruption.corruption_level = narrator.corruption;

        // Palette shifts every 25% corruption
        corruption.palette_shift_index = (narrator.corruption / 25.0).floor() as u32;

        // CRT jitter scales with corruption
        corruption.raster_jitter_amplitude = narrator.corruption / 200.0;

        // Chromatic aberration (max 2 pixels at 100% corruption)
        corruption.chromatic_aberration = (narrator.corruption / 100.0) * 2.0;
    }
}

// ============================================================================
// INPUT SYSTEM (DEBUG ONLY)
// ============================================================================

pub fn debug_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut HamsterNarrator>,
) {
    for mut narrator in &mut query {
        // U/D to increase/decrease corruption
        if keyboard.pressed(KeyCode::KeyU) {
            narrator.corruption = (narrator.corruption + 0.5).min(100.0);
        }
        if keyboard.pressed(KeyCode::KeyD) {
            narrator.corruption = (narrator.corruption - 0.5).max(0.0);
        }

        // A/S/W to cycle expressions
        if keyboard.just_pressed(KeyCode::KeyA) {
            narrator.expression = match narrator.expression {
                Expression::Neutral => Expression::Happy,
                Expression::Happy => Expression::Angry,
                Expression::Angry => Expression::Corrupted,
                Expression::Corrupted => Expression::Neutral,
            };
        }
    }
}
```

---

## Part 5: Module Organization

### File: `src/hamster/mod.rs`

```rust
pub mod components;
pub mod assembly;
pub mod systems;
pub mod animation;
pub mod shader;

pub use assembly::assemble_hamster;
pub use components::*;
pub use systems::*;

use bevy::prelude::*;

pub struct HamsterPlugin;

impl Plugin for HamsterPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup
            .add_systems(Startup, setup_hamster)
            // Update
            .add_systems(
                Update,
                (
                    breathing_system,
                    blinking_system,
                    idle_motion_system,
                    corruption_system,
                    debug_input_system,
                )
                    .chain(), // Run in order
            );
    }
}

fn setup_hamster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn main camera
    commands.spawn(Camera2d::default());

    // Assemble and spawn hamster
    assemble_hamster(&mut commands, &asset_server);
}
```

---

## Part 6: Integration Checklist

### In `main.rs` or `lib.rs`:

```rust
use bevy::prelude::*;
use doomexe::hamster::HamsterPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(), // Pixel-perfect rendering
        ))
        .add_plugins(HamsterPlugin)
        .run();
}
```

### What You Need in `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.15", features = ["dynamic_linking", "default_font"] }
rand = "0.8"
mlua = { version = "0.9", features = ["lua54"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Part 7: Asset Export Checklist

### In Aseprite:

- [ ] Create canvas at 640×480 (2x work size)
- [ ] Create layers for each part (Body, Head, Eyes, etc.)
- [ ] Create mouth variants (neutral, smile, angry, corrupted)
- [ ] For each export:
  - [ ] Select layer group
  - [ ] File → Export Sprite Sheet
  - [ ] Format: PNG
  - [ ] Trim Sprite: ✓
  - [ ] Output Metadata: JSON
  - [ ] Save to `assets/sprites/hamster/<part>/<name>.png`

### Expected Files:

```
assets/sprites/hamster/
├── body/hamster_body.png (+.json)
├── head/hamster_head.png (+.json)
├── ears/hamster_ear_left.png (+.json)
├── ears/hamster_ear_right.png (+.json)
├── eyes/
│   ├── hamster_eye_left_open.png (+.json)
│   ├── hamster_eye_left_closed.png (+.json)
│   ├── hamster_eye_right_open.png (+.json)
│   └── hamster_eye_right_closed.png (+.json)
└── mouth/
    ├── hamster_mouth_neutral.png (+.json)
    ├── hamster_mouth_smile.png (+.json)
    ├── hamster_mouth_angry.png (+.json)
    └── hamster_mouth_corrupted.png (+.json)
```

---

## Part 8: Testing Checklist

### Visual Testing:
- [ ] Hamster appears on screen
- [ ] All parts are correctly positioned
- [ ] Breathing animation is smooth (not jerky)
- [ ] Blinking is randomized (not too frequent)
- [ ] Head has subtle idle motion
- [ ] No texture bleeding or misalignment

### Animation Testing:
- [ ] Press U to increase corruption → visual changes
- [ ] Press D to decrease corruption → reverts
- [ ] Press A to cycle expressions → mouth changes
- [ ] Corruption affects CRT effect intensity

### Performance Testing:
- [ ] Run with `cargo build --release`
- [ ] Check FPS (should be 60+)
- [ ] Check memory usage (should be < 50 MB)
- [ ] Test on multiple window sizes

---

## Part 9: Lua Integration (Minimal Example)

### File: `src/scripting/hamster_api.rs`

```rust
use mlua::prelude::*;
use crate::hamster::components::*;

pub fn setup_hamster_lua(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // set_corruption(level: number) -> nil
    globals.set(
        "set_corruption",
        lua.create_function(|_, level: f32| {
            // This will be called from Lua script
            // You'll need to set up a channel or resource to communicate
            println!("Lua: set_corruption({:.1}%)", level);
            Ok(())
        })?,
    )?;

    // set_expression(expr: number) -> nil
    globals.set(
        "set_expression",
        lua.create_function(|_, expr: u8| {
            println!("Lua: set_expression({})", expr);
            Ok(())
        })?,
    )?;

    // log(msg: string) -> nil
    globals.set(
        "log",
        lua.create_function(|_, msg: String| {
            println!("Lua: {}", msg);
            Ok(())
        })?,
    )?;

    Ok(())
}
```

---

## Part 10: Troubleshooting Guide

### Problem: "Hamster doesn't appear"
- **Check**: Are sprites loaded? (`AssetServer::load()` paths correct?)
- **Check**: Is camera spawned? (`Camera2d`)
- **Check**: Are Z-indices in valid range?
- **Fix**: Verify sprite asset paths match your folder structure

### Problem: "Animation is jerky"
- **Check**: Is `breathing_system` in correct schedule? (should be `Update`, not `Startup`)
- **Check**: Is frame time consistent? (use diagnostic plugin)
- **Fix**: Ensure no heavy computations in animation systems

### Problem: "Blinking doesn't work"
- **Check**: Are eyes spawned with `BlinkingAnimation` component?
- **Check**: Does `blinking_system` query include `With<HamsterPart>`?
- **Fix**: Verify `part_type` matching in system

### Problem: "Parts are rendering in wrong order"
- **Check**: Are Z-indices correct? (Body: 100, Head: 101, Eyes: 104, Mouth: 105)
- **Check**: Are parts children of hamster entity?
- **Fix**: Use explicit Z-indices, not relying on spawn order

### Problem: "Sprites are blurry"
- **Check**: Is `ImagePlugin::default_nearest()` set?
- **Check**: Are sprites being scaled up? (use `custom_size` if needed)
- **Fix**: Ensure nearest-neighbor filtering enabled in Bevy

---

## Part 11: Next Immediate Steps (Priority Order)

### Week 1: Get It Rendering
1. [ ] Export hamster parts from Aseprite
2. [ ] Create `components.rs` with all component structs
3. [ ] Create `assembly.rs` with `assemble_hamster()` function
4. [ ] Verify hamster appears on screen (static, no animation)

### Week 2: Add Animation
5. [ ] Copy `breathing_system` into `systems.rs`
6. [ ] Test breathing looks smooth
7. [ ] Add `blinking_system`
8. [ ] Test blinking is randomized
9. [ ] Add `idle_motion_system` on head

### Week 3: State Management
10. [ ] Add debug input system (U/D to change corruption)
11. [ ] Add expression cycling (A key)
12. [ ] Test visual changes work
13. [ ] Begin CRT shader setup

### Week 4: Polish
14. [ ] Implement CRT post-processing shader
15. [ ] Tune animation parameters for feel
16. [ ] Performance profiling & optimization
17. [ ] Final visual polish

---

## Reference: Complete System Signal

```rust
// In main.rs or plugin setup:
app.add_systems(
    Update,
    (
        breathing_system,
        blinking_system,
        idle_motion_system,
        corruption_system,
        debug_input_system,
    ),
);
```

This ensures all systems run every frame in the `Update` schedule.

---

## Notes for Team

- **Rendering**: Uses Bevy's built-in sprite system + child hierarchy
- **No external sprite libraries needed**: Just components + transforms
- **Performance**: ~8 sprites per frame (trivial cost)
- **Expandability**: Easy to add more parts or animations later
- **Lua Integration**: Channel-based communication (setup later when needed)

**Ready to start coding!** 🎨✨

---

**Status**: Implementation Ready  
**Created**: 2026-01-20  
**Owner**: You