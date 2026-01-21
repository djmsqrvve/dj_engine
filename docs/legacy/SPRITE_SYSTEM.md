# Sprite System Design – DJ Engine / Doomexe

**Date**: 2026-01-20  
**Purpose**: Technical specification for procedural sprite assembly, animation, and rendering  
**Scope**: Pixel-art sprite system for Hamster Narrator (Milestone 1)  
**Status**: Design Phase → Implementation Ready  

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Asset Pipeline](#asset-pipeline)
4. [Component Structure](#component-structure)
5. [Systems & Rendering](#systems--rendering)
6. [Animation Framework](#animation-framework)
7. [Procedural Assembly](#procedural-assembly)
8. [Corruption & Post-Processing](#corruption--post-processing)
9. [Implementation Strategy](#implementation-strategy)
10. [Performance Considerations](#performance-considerations)
11. [Testing & Validation](#testing--validation)

---

## Overview

### Goals

1. **Procedural Assembly**: Build characters from sprite parts (body, head, eyes, mouth, etc.)
2. **Pixel-Perfect Rendering**: 320×240 or 400×240 internal resolution with nearest-neighbor upscaling
3. **Smooth Animation**: Breathing, blinking, idle motion, corruption effects
4. **Lua Integration**: State changes driven by script (expression, corruption)
5. **Performance**: 60+ FPS on modest hardware with CRT post-processing

### Design Approach

- **Entity-Component-System (ECS)**: Leverage Bevy's strengths
- **Hierarchical Sprites**: Parent entities (body) with child entities (parts)
- **Texture Atlases**: Efficient sprite sheet loading via `TextureAtlasLayout`
- **Z-Indexing**: Careful layer management for proper depth ordering
- **Offscreen Render**: Low-res internal buffer + shader upscaling

---

## Architecture

### High-Level Flow

```
┌─────────────────────────────────────────────────────┐
│ Lua Script Layer                                     │
│  - State management (expression, corruption)        │
│  - Input handling                                   │
└──────────────────┬──────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────┐
│ Bevy Systems Layer                                   │
│  - HamsterAssemblySystem (spawn/update)             │
│  - AnimationSystems (breathing, blinking, idle)     │
│  - CorruptionSystem (palette shifts, CRT effects)   │
│  - InputSystem (debug controls)                     │
└──────────────────┬──────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────┐
│ Component Layer (Transform Hierarchy)                │
│                                                      │
│  HamsterNarrator (root)                             │
│    ├─ Body (child, layer=0)                         │
│    ├─ Head (child, layer=1)                         │
│    ├─ LeftEar (child, layer=2)                      │
│    ├─ RightEar (child, layer=2)                     │
│    ├─ LeftEye (child, layer=4)                      │
│    ├─ RightEye (child, layer=4)                     │
│    ├─ Mouth (child, layer=5)                        │
│    └─ LeftPaw (child, layer=3) [optional]           │
└──────────────────┬──────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────┐
│ Rendering Pipeline                                  │
│  - Sprite rendering (with parent transforms)       │
│  - Offscreen texture collection                     │
│  - Shader upscaling + CRT post-processing           │
│  - Screen output                                    │
└──────────────────────────────────────────────────────┘
```

### Core Systems

| System | Responsibility | Schedule |
|--------|----------------|----------|
| **HamsterAssemblySystem** | Spawn/despawn hamster + parts on startup | Startup |
| **BreathingSystem** | Scale oscillation (sine wave) on body | Update (every frame) |
| **BlinkingSystem** | Eye alpha/variant toggling | Update (via timer) |
| **IdleMotionSystem** | Subtle position/rotation noise on head | Update (every frame) |
| **CorruptionSystem** | Apply corruption color palette & CRT intensity | Update (when corruption changes) |
| **InputSystem** | Debug keyboard controls (debug builds only) | Update (when key pressed) |
| **AnimationControlSystem** | Route Lua state changes to components | Update (reactive) |

---

## Asset Pipeline

### Aseprite Export Workflow

#### Step 1: Prepare in Aseprite

1. **Create Master Canvas**: 640×480 (2x target render size for working room)
2. **Organize Layers**: Group by part (Body, Head, LeftEye, RightEye, Mouth, etc.)
3. **Create Variations**: Multiple mouth expressions (neutral, smile, angry, corrupted)
4. **Palette**: Use indexed color mode + shared palette for all parts

#### Step 2: Export Process

For each part variant:

```
File → Export Sprite Sheet
├─ Output Format: PNG
├─ Sprite Sheet Type: Packed (optional, or rows/columns)
├─ Trim Sprite: ✓ (removes empty space)
├─ Metadata: JSON (includes trimmed bounds and offsets)
└─ Filename: hamster_body.png, hamster_head_neutral.png, etc.
```

Each export generates:
- `hamster_<part>.png` – Sprite image (trimmed)
- `hamster_<part>.json` – Metadata with original bounds

#### Step 3: Metadata Structure

```json
{
  "frames": {
    "hamster_body.png": {
      "frame": { "x": 0, "y": 0, "w": 120, "h": 100 },
      "rotated": false,
      "trimmed": true,
      "spriteSourceSize": { "x": 10, "y": 20, "w": 140, "h": 140 },
      "sourceSize": { "w": 640, "h": 480 }
    }
  },
  "meta": {
    "app": "Aseprite",
    "version": "1.3",
    "image": "hamster_body.png",
    "format": "RGBA8888",
    "size": { "w": 256, "h": 256 },
    "scale": "1"
  }
}
```

**Key fields**:
- `spriteSourceSize`: Original offset (trim correction)
- `sourceSize`: Canvas size at export
- `frame`: Actual trimmed bounds

---

### Directory Structure

```
assets/
├── sprites/
│   ├── hamster/
│   │   ├── body/
│   │   │   ├── hamster_body.png
│   │   │   └── hamster_body.json
│   │   ├── head/
│   │   │   ├── hamster_head.png
│   │   │   └── hamster_head.json
│   │   ├── eyes/
│   │   │   ├── hamster_eye_left_open.png
│   │   │   ├── hamster_eye_left_open.json
│   │   │   ├── hamster_eye_left_closed.png
│   │   │   ├── hamster_eye_left_closed.json
│   │   │   └── ... (right eye variants)
│   │   ├── mouth/
│   │   │   ├── hamster_mouth_neutral.png
│   │   │   ├── hamster_mouth_neutral.json
│   │   │   ├── hamster_mouth_smile.png
│   │   │   ├── hamster_mouth_smile.json
│   │   │   ├── hamster_mouth_angry.png
│   │   │   ├── hamster_mouth_angry.json
│   │   │   └── ... (corrupted variants)
│   │   ├── ears/
│   │   │   ├── hamster_ear_left.png
│   │   │   ├── hamster_ear_left.json
│   │   │   ├── hamster_ear_right.png
│   │   │   └── hamster_ear_right.json
│   │   └── paws/
│   │       └── ... (optional for M1)
│   ├── background/
│   │   ├── corrupted_sky_01.png
│   │   └── corrupted_sky_02.png
│   └── post_processing/
│       ├── scanlines.png
│       └── vignette.png
├── shaders/
│   ├── sprite_upscale.wgsl
│   ├── crt_post.wgsl
│   └── scanlines.wgsl
└── scripts/
    ├── hamster_narrator.lua
    └── dialogue_simple.lua
```

---

## Component Structure

### Core Components

#### 1. **HamsterNarrator** (Root Entity Marker)

```rust
#[derive(Component)]
pub struct HamsterNarrator {
    pub corruption: f32,              // 0..=100 (%)
    pub expression: Expression,        // Enum: Neutral, Happy, Angry, Corrupted
    pub animation_time: f32,           // Elapsed time (for breathing phase)
    pub mood: Option<Mood>,           // (Optional) high-level state
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    Neutral = 0,
    Happy = 1,
    Angry = 2,
    Corrupted = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Content,
    Suspicious,
    Horrified,
}
```

#### 2. **HamsterPart** (Child Entity Marker)

```rust
#[derive(Component)]
pub struct HamsterPart {
    pub part_type: PartType,
    pub offset: Vec2,                 // Trim offset from Aseprite export
    pub layer: u32,                   // Z-order (0 = back, higher = front)
    pub base_rotation: f32,           // Default rotation (radians)
}

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
```

#### 3. **BreathingAnimation**

```rust
#[derive(Component)]
pub struct BreathingAnimation {
    pub amplitude: f32,               // Typical: 0.05 (5% scale variation)
    pub frequency: f32,               // Typical: 0.5 Hz
    pub phase_offset: f32,            // For staggered animations (0..=2π)
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
```

#### 4. **BlinkingAnimation**

```rust
#[derive(Component)]
pub struct BlinkingAnimation {
    pub timer: Timer,
    pub blink_duration: Duration,     // How long eyes are closed (0.1–0.15s typical)
    pub interval_range: (f32, f32),   // (min, max) seconds between blinks
    pub state: BlinkState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkState {
    Open,
    Closing,
    Closed,
    Opening,
}
```

#### 5. **CorruptionEffect**

```rust
#[derive(Component)]
pub struct CorruptionEffect {
    pub corruption_level: f32,         // 0..=100
    pub palette_shift_index: u32,      // Which corrupted palette to use
    pub raster_jitter_amplitude: f32,  // CRT effect intensity
    pub chromatic_aberration: f32,     // Color channel offset (pixels)
}
```

---

## Systems & Rendering

### Bevy Rendering Pipeline

#### Resolution & Upscaling

1. **Internal Render**: 320×240 (or 400×240)
   - All game elements render to offscreen texture
   - Uses `RenderLayers` to separate from UI/post-processing

2. **Upscaling**: Window-sized camera with shader
   - Nearest-neighbor filtering (pixel-perfect)
   - Optional scanlines overlay

3. **Post-Processing**:
   - CRT shader (scanlines + vignette)
   - Corruption-driven chromatic aberration

#### Z-Ordering (Critical for Hierarchy)

**Rule**: Z-index is **absolute in world space**, not relative to parent.

```rust
// Setup hamster with correct z-indices:
// Body:       z = 100
// Head:       z = 101
// Ears:       z = 102
// Paws:       z = 103
// Eyes:       z = 104
// Mouth:      z = 105

fn setup_hamster(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let hamster = commands
        .spawn((
            HamsterNarrator {
                corruption: 0.0,
                expression: Expression::Neutral,
                animation_time: 0.0,
                mood: None,
            },
            Transform::from_xyz(0.0, 0.0, 100.0), // Base Z
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .id();

    // Body (Z = 100)
    spawn_part(
        &mut commands,
        hamster,
        PartType::Body,
        "sprites/hamster/body/hamster_body.png",
        Vec2::new(0.0, 0.0),   // offset
        100,                    // z-index
    );

    // Head (Z = 101, offset from body)
    spawn_part(
        &mut commands,
        hamster,
        PartType::Head,
        "sprites/hamster/head/hamster_head.png",
        Vec2::new(0.0, 30.0),  // offset from body
        101,
    );

    // ... spawn other parts with increasing Z
}
```

**Key Points**:
- Parent `Transform` doesn't affect child Z directly (z is world-space)
- Each part has its own `Transform` as a **sibling transform** under parent
- Z-ordering determined by `transform.translation.z`
- Bevy automatically batches sprites by z-order

---

### System Implementation Examples

#### Breathing System

```rust
fn breathing_system(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &BreathingAnimation),
        With<HamsterPart>,
    >,
) {
    let elapsed = time.elapsed_secs();

    for (mut transform, breathing) in &mut query {
        // Compute phase
        let phase = (elapsed * breathing.frequency * 2.0 * PI) + breathing.phase_offset;

        // Y scale: oscillate around 1.0
        let scale_y = 1.0 + breathing.amplitude * phase.sin();

        // X scale: inverse to conserve area
        let scale_x = 1.0 / scale_y;

        transform.scale = Vec3::new(scale_x, scale_y, 1.0);
    }
}
```

#### Blinking System

```rust
fn blinking_system(
    time: Res<Time>,
    mut query: Query<
        (&mut BlinkingAnimation, &mut Sprite),
        With<HamsterPart>,
    >,
) {
    for (mut blink, mut sprite) in &mut query {
        blink.timer.tick(time.delta());

        if blink.timer.just_finished() {
            match blink.state {
                BlinkState::Open => {
                    // Start closing
                    blink.state = BlinkState::Closing;
                    blink.timer = Timer::new(blink.blink_duration, TimerMode::Once);
                }
                BlinkState::Closing => {
                    // Stay closed briefly
                    blink.state = BlinkState::Closed;
                    blink.timer = Timer::new(Duration::from_millis(50), TimerMode::Once);
                }
                BlinkState::Closed => {
                    // Re-open
                    blink.state = BlinkState::Open;
                    let next_interval = rand::random::<f32>()
                        * (blink.interval_range.1 - blink.interval_range.0)
                        + blink.interval_range.0;
                    blink.timer = Timer::new(Duration::from_secs_f32(next_interval), TimerMode::Once);
                }
                _ => {}
            }
        }

        // Update sprite visibility based on blink state
        sprite.color.set_a(match blink.state {
            BlinkState::Open | BlinkState::Closing => 1.0,
            BlinkState::Closed | BlinkState::Opening => 0.0,
        });
    }
}
```

#### Corruption System

```rust
fn corruption_system(
    mut query: Query<
        (&HamsterNarrator, &mut CorruptionEffect),
    >,
) {
    for (narrator, mut corruption) in &mut query {
        // Update palette shift based on corruption
        corruption.corruption_level = narrator.corruption;
        corruption.palette_shift_index = (narrator.corruption / 25.0).floor() as u32;

        // CRT intensity scales with corruption
        corruption.raster_jitter_amplitude = narrator.corruption / 200.0;

        // Chromatic aberration increases with corruption
        corruption.chromatic_aberration = (narrator.corruption / 100.0) * 2.0; // max 2px offset
    }
}
```

---

## Animation Framework

### Animation Phases

#### 1. **Breathing** (Continuous, on Body)

- **Amplitude**: 0.05 (5% scale variation)
- **Frequency**: 0.5 Hz (one breath every 2 seconds)
- **Formula**: `scale = 1.0 + 0.05 * sin(2π * 0.5 * t)`
- **Applied to**: Body, Head, Ears (with slight phase offsets)

#### 2. **Blinking** (Periodic, on Eyes)

- **Blink Duration**: 0.1–0.15 seconds
- **Interval**: Random 3–7 seconds between blinks
- **Mechanism**: Toggle eye texture or alpha to closed variant

#### 3. **Idle Motion** (Subtle, on Head)

- **Type**: Perlin noise or slow sine wave oscillation
- **Range**: ±5 pixels horizontal, ±2 pixels vertical
- **Frequency**: 0.2 Hz (lower than breathing for subtlety)

#### 4. **Expression Changes** (Discrete, on Mouth)

- **Neutral**: Default mouth sprite
- **Happy**: Smile sprite
- **Angry**: Frown sprite
- **Corrupted**: Distorted/multiple mouths

---

## Procedural Assembly

### Assembly Algorithm

```rust
fn assemble_hamster(
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    // 1. Spawn root entity
    let hamster = commands
        .spawn((
            HamsterNarrator::default(),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .id();

    // 2. Define parts to spawn (order = rendering order)
    let parts = vec![
        (PartType::Body, "sprites/hamster/body/hamster_body.png", Vec2::ZERO, 0),
        (PartType::Head, "sprites/hamster/head/hamster_head.png", Vec2::new(0.0, 30.0), 1),
        (PartType::LeftEar, "sprites/hamster/ears/hamster_ear_left.png", Vec2::new(-20.0, 45.0), 2),
        (PartType::RightEar, "sprites/hamster/ears/hamster_ear_right.png", Vec2::new(20.0, 45.0), 2),
        (PartType::LeftEye, "sprites/hamster/eyes/hamster_eye_left_open.png", Vec2::new(-10.0, 35.0), 4),
        (PartType::RightEye, "sprites/hamster/eyes/hamster_eye_right_open.png", Vec2::new(10.0, 35.0), 4),
        (PartType::Mouth, "sprites/hamster/mouth/hamster_mouth_neutral.png", Vec2::new(0.0, 20.0), 5),
    ];

    // 3. Spawn each part
    for (part_type, sprite_path, offset, z_layer) in parts {
        let part = commands
            .spawn((
                Sprite {
                    image: asset_server.load(sprite_path),
                    ..default()
                },
                Transform::from_xyz(offset.x, offset.y, 100 + z_layer as f32),
                GlobalTransform::default(),
                Visibility::default(),
                HamsterPart {
                    part_type,
                    offset,
                    layer: z_layer,
                    base_rotation: 0.0,
                },
                BreathingAnimation::default(), // on body only
                BlinkingAnimation::new(),      // on eyes only
                CorruptionEffect::default(),
            ))
            .id();

        // Make this part a child of hamster
        commands.entity(hamster).add_child(part);
    }

    hamster
}
```

---

## Corruption & Post-Processing

### Corruption Levels

| Level | Range | Effect |
|-------|-------|--------|
| **None** | 0–20% | Slight color tint, subtle scanlines |
| **Low** | 20–40% | More pronounced color shift, increased CRT |
| **Medium** | 40–60% | Noticeable palette change, raster jitter |
| **High** | 60–80% | Significant color distortion, chromatic aberration |
| **Extreme** | 80–100% | Maximum distortion, heavy glitch effects |

### Palette Shifting

Store 4–5 corrupted palette variants in a texture or shader constant:

```wgsl
// In CRT shader
var palette_variants: array<vec3<f32>, 5> = array(
    vec3<f32>(1.0, 1.0, 1.0),        // 0: Normal
    vec3<f32>(1.0, 0.8, 0.8),        // 1: Red tint
    vec3<f32>(0.8, 0.5, 0.8),        // 2: Purple tint
    vec3<f32>(0.5, 0.8, 1.0),        // 3: Blue tint
    vec3<f32>(1.0, 0.5, 0.5),        // 4: Red-pink tint
);

let palette_index = u32(corruption_level / 25.0);
let tint = palette_variants[min(palette_index, 4u)];
output_color *= tint;
```

### CRT Shader Post-Processing

```wgsl
@fragment
fn crt_fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(sprite_texture, sprite_sampler, in.uv);

    // 1. Scanlines
    let scanline = step(0.5, fract(in.uv.y * 240.0 * 2.0));
    color.rgb *= mix(0.7, 1.0, scanline);

    // 2. Vignette
    let distance_from_center = length(in.uv - vec2<f32>(0.5));
    let vignette = 1.0 - distance_from_center * 0.8;
    color.rgb *= vignette;

    // 3. Raster jitter (corruption-driven)
    let jitter = (sin(in.uv.y * 300.0 + time) * 0.5 + 0.5) * jitter_amplitude;
    color.rgb += vec3<f32>(jitter * 0.1);

    // 4. Chromatic aberration (corruption-driven)
    if (chromatic_aberration > 0.0) {
        let offset = chromatic_aberration / 1024.0; // screen-relative
        let r = textureSample(sprite_texture, sprite_sampler, in.uv + vec2<f32>(offset, 0.0)).r;
        let b = textureSample(sprite_texture, sprite_sampler, in.uv - vec2<f32>(offset, 0.0)).b;
        color.r = r;
        color.b = b;
    }

    return color;
}
```

---

## Implementation Strategy

### Phase 1: Sprite Loading & Display (Week 1)

- [ ] Export hamster parts from Aseprite with metadata
- [ ] Implement asset loader for sprites + JSON metadata
- [ ] Create `HamsterPart` component and `HamsterNarrator` marker
- [ ] Assemble hamster on-screen with correct Z-ordering
- [ ] Verify pixel-perfect rendering (320×240 → window with nearest-neighbor)

**Success Criteria**: Hamster displays as assembled parts, no visual glitches, clean edges

### Phase 2: Animation Systems (Week 2)

- [ ] Implement breathing system (sine wave scaling)
- [ ] Implement blinking system (eye toggle with random intervals)
- [ ] Implement idle motion (subtle noise-based offsets)
- [ ] Add animation timing utilities
- [ ] Test all systems at 60 FPS

**Success Criteria**: Breathing is smooth, blinking is randomized, idle motion is subtle

### Phase 3: Corruption & State Management (Week 2-3)

- [ ] Create corruption system (palette shifts, CRT intensity)
- [ ] Implement Lua integration for state changes
- [ ] Add expression switching (mouth variants)
- [ ] Debug keyboard controls (increase/decrease corruption, cycle expressions)

**Success Criteria**: Corruption slider works, visual changes are clear, Lua can trigger updates

### Phase 4: Post-Processing & Polish (Week 3)

- [ ] Implement CRT shader (scanlines, vignette)
- [ ] Add chromatic aberration (corruption-driven)
- [ ] Tune shader parameters for target aesthetic
- [ ] Performance profiling & optimization

**Success Criteria**: 60 FPS with CRT effects, visual matches target image

---

## Performance Considerations

### Optimization Strategies

1. **Sprite Batching**
   - Bevy automatically batches sprites by z-order and texture
   - Keep all hamster parts on same texture atlas if possible
   - Use `RenderLayers` to separate internal render from UI

2. **Transform Hierarchy**
   - Use `GlobalTransform` for efficient world-space computation
   - Child transforms computed automatically by Bevy
   - Avoid deeply nested hierarchies (hamster is 8-level deep max)

3. **Animation Updates**
   - Breathing/idle motion use lightweight math (sin, cos, noise)
   - Blinking uses simple timer (no per-frame calculations)
   - Corruption changes happen infrequently (on state change, not every frame)

4. **Texture Atlasing**
   - Pack all hamster parts into single 512×512 or 1024×1024 atlas
   - Reuse textures for symmetric parts (left/right eyes, ears)
   - Use indexed color mode to reduce file size

5. **Offscreen Rendering**
   - 320×240 internal target is lightweight
   - Single upscaling pass with nearest-neighbor shader
   - Post-processing happens on upscaled result (minimal overhead)

### Profiling Points

- System execution time: `bevy_diagnostic::SystemStepDiagnosticsPlugin`
- Sprite batch count: Renderer diagnostics
- Memory usage: Asset server stats
- Frame time: Target 16.67 ms (60 FPS)

---

## Testing & Validation

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breathing_amplitude() {
        let breathing = BreathingAnimation::default();
        let phase = 0.0;
        let scale = 1.0 + breathing.amplitude * phase.sin();
        assert!(scale >= 0.95 && scale <= 1.05);
    }

    #[test]
    fn test_corruption_palette_index() {
        let mut effect = CorruptionEffect::default();
        effect.corruption_level = 50.0;
        effect.palette_shift_index = (effect.corruption_level / 25.0).floor() as u32;
        assert_eq!(effect.palette_shift_index, 2);
    }

    #[test]
    fn test_blink_interval_range() {
        let interval = 3.5;
        assert!(interval >= 3.0 && interval <= 7.0);
    }
}
```

### Integration Tests

- [ ] Spawn hamster on startup
- [ ] Verify all parts are children of hamster
- [ ] Test animation systems update without crashes
- [ ] Verify Lua script can set corruption & expression
- [ ] Test expression switching (all variants render)
- [ ] Verify CRT shader applies correctly

### Visual Regression Testing

- [ ] Capture reference screenshots at key states (neutral, corrupted, blinking)
- [ ] Compare pixel output against target image aesthetic
- [ ] Verify no texture bleeding or misalignment
- [ ] Test on multiple screen resolutions

### Performance Benchmarks

- [ ] Hamster alone: 60+ FPS
- [ ] Hamster + CRT shader: 60+ FPS
- [ ] Memory usage: < 50 MB for all assets
- [ ] Asset load time: < 500 ms

---

## File Organization (Crate Layout)

```
games/dev/doomexe/src/
├── main.rs
├── lib.rs
├── hamster/
│   ├── mod.rs                 # public interface
│   ├── components.rs          # HamsterNarrator, HamsterPart, etc.
│   ├── systems.rs             # All animation & corruption systems
│   ├── assembly.rs            # assemble_hamster() function
│   ├── shader.rs              # CRT post-processing shader wrapper
│   └── animation.rs           # Animation utilities (breathing, blinking)
├── rendering/
│   ├── mod.rs
│   ├── camera.rs              # Offscreen render setup
│   ├── layers.rs              # RenderLayers organization
│   └── upscaling.rs           # Nearest-neighbor upscaling
├── assets/
│   ├── mod.rs
│   └── loader.rs              # JSON metadata loading
└── scripting/
    ├── mod.rs
    └── hamster_api.rs         # set_corruption(), set_expression() for Lua
```

---

## References & Dependencies

### Required Crates

```toml
[dependencies]
bevy = { version = "0.15", features = ["dynamic_linking"] }
mlua = { version = "0.9", features = ["lua54", "serialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"
noise = "0.9"  # Optional: for Perlin noise in idle motion
```

### Key Bevy Concepts

- **ECS**: `Component`, `System`, `Query`, `Commands`
- **Rendering**: `Sprite`, `Transform`, `GlobalTransform`, `RenderLayers`, `Camera`
- **Assets**: `AssetServer`, `Handle<T>`, `Assets<T>`
- **Hierarchy**: `Parent`, `Children`, `GlobalTransform` (Bevy 0.15+)
- **Scheduling**: `Startup`, `Update`, `Fixed`, `PostUpdate`

### External References

- [Bevy Official Docs](https://docs.rs/bevy/latest/bevy/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Extreme Bevy Series](https://johanhelsing.studio/posts/extreme-bevy-6) (sprite animations)
- [Aseprite Manual](https://www.aseprite.org/docs/)

---

## Next Steps

1. **Export hamster parts** from Aseprite with JSON metadata
2. **Create initial component structure** in Rust
3. **Implement asset loader** for sprites + metadata
4. **Build assembly function** to spawn hamster
5. **Add breathing system** as first animation
6. **Integrate with Lua** for state changes
7. **Implement CRT shader** for final aesthetic
8. **Benchmark & optimize** for 60 FPS

---

## Questions & Decisions

### Q: Should we use a texture atlas or individual textures per part?

**A**: **Start with individual textures** (one per part variant).
- Easier to manage in Aseprite
- More flexibility for swapping expressions
- Performance is fine for 8–10 sprites (single draw call per part)
- Can optimize to unified atlas later if needed

### Q: How do we handle left/right symmetry (eyes, ears)?

**A**: **Use separate entities with different sprites**, positioned symmetrically.
- Easier to animate independently (e.g., winking one eye)
- Supports future asymmetric corruption effects
- Clean ECS design (each part is independent entity)

### Q: What about camera setup for internal 320×240 render?

**A**: **Use Bevy's `RenderLayers` + offscreen texture**.
- Internal camera renders only `RenderLayers` layer 0
- External camera renders upscaled texture + UI on layer 1
- Shader handles nearest-neighbor upscaling

### Q: How do we avoid Z-order fights between parent and children?

**A**: **Use absolute Z in world space, not relative to parent**.
- Each part gets explicit Z index (100, 101, 102, etc.)
- Bevy batches sprites by Z automatically
- No parent-child Z interference

---

## Success Metrics (Milestone 1)

| Metric | Target | How Measured |
|--------|--------|--------------|
| **Visual Fidelity** | Matches target image | Pixel comparison screenshot |
| **Animation Smoothness** | No stuttering | Frame time < 16.67ms consistently |
| **Performance** | 60+ FPS | FPS counter in debug UI |
| **Memory** | < 50 MB | Profiler data |
| **Lua Integration** | Expression/corruption changes in-game | Manual testing with script |
| **Build Time** | < 30 seconds (incremental) | Time command |
| **Asset Load** | < 500 ms | Time from startup to render |

---

**Status**: Ready for implementation  
**Last Updated**: 2026-01-20  
**Owner**: @you (Technical Architect)  
**Feedback**: Open GitHub issues in `dj_engine` repo