# DJ Engine Sprite System V2

**Version**: 2.0  
**Date**: 2026-01-20  
**Status**: Active Development

---

## Overview

The sprite system enables procedural character assembly from individual parts with component-driven animation. Characters are built from a hierarchy of sprite entities that animate independently via ECS systems.

### Core Principles

1. **Entity per Part**: Each sprite part (body, head, eyes) is a separate entity
2. **Parent-Child Hierarchy**: Parts are children of a root entity for transform inheritance
3. **Component-Driven Animation**: Animation behaviors defined by marker components
4. **Data-Driven Assembly**: Part definitions loaded from manifest files
5. **Absolute Z-Ordering**: Z-indices are world-space, not parent-relative

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      ENTITY HIERARCHY                       │
├─────────────────────────────────────────────────────────────┤
│  CharacterRoot (HamsterNarrator)                            │
│    ├─ Body      (z=100, BreathingAnimation)                 │
│    ├─ Head      (z=101, IdleMotion)                         │
│    ├─ LeftEye   (z=104, BlinkingAnimation)                  │
│    ├─ RightEye  (z=104, BlinkingAnimation)                  │
│    ├─ Mouth     (z=105)                                     │
│    ├─ LeftPaw   (z=102)                                     │
│    └─ RightPaw  (z=102)                                     │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
Manifest (TOML) → Assembly System → Entity Hierarchy → Animation Systems → Renderer
```

---

## Components

### CharacterRoot

Marker for the root entity. Holds character-wide state.

```rust
#[derive(Component, Default)]
pub struct CharacterRoot {
    pub corruption: f32,        // 0.0 - 100.0
    pub expression: Expression,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    #[default]
    Neutral,
    Happy,
    Angry,
}
```

### SpritePart

Marker for child sprite entities.

```rust
#[derive(Component)]
pub struct SpritePart {
    pub kind: PartKind,
    pub z_layer: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartKind {
    Body, Head, LeftEye, RightEye, Mouth, LeftPaw, RightPaw, LeftFoot, RightFoot,
}
```

### Animation Components

```rust
/// Smooth breathing scale oscillation
#[derive(Component)]
pub struct BreathingAnimation {
    pub amplitude: f32,   // Default: 0.03
    pub frequency: f32,   // Default: 0.5 Hz
}

/// Random eye blinking
#[derive(Component)]
pub struct BlinkingAnimation {
    pub timer: Timer,
    pub is_closed: bool,
    pub interval_range: (f32, f32), // Default: (3.0, 7.0)
}

/// Subtle head movement
#[derive(Component)]
pub struct IdleMotion {
    pub amplitude: Vec2,  // Default: (3.0, 1.5)
    pub frequency: f32,   // Default: 0.15 Hz
}
```

---

## Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `breathing_system` | Update | Scale body with sine wave |
| `blinking_system` | Update | Toggle eye visibility on timer |
| `idle_motion_system` | Update | Offset head position |
| `expression_system` | Update | Swap head sprite on expression change |

### System Execution Order

```rust
app.add_systems(Update, (
    breathing_system,
    blinking_system,
    idle_motion_system,
    expression_system,
).chain());
```

---

## Asset Pipeline

### Directory Structure

```
assets/sprites/hamster/
├── manifest.toml        # Part definitions
├── body/
│   └── body.png
├── head/
│   ├── neutral.png
│   ├── happy.png
│   └── angry.png
├── eyes/
│   ├── open.png
│   └── closed.png
└── paws/
    ├── left.png
    └── right.png
```

### Manifest Format

```toml
[character]
name = "hamster"
base_z = 100

[[parts]]
kind = "Body"
sprite = "body/body.png"
offset = [0, 0]
z_offset = 0
animations = ["breathing"]

[[parts]]
kind = "Head"
sprite = "head/neutral.png"
offset = [0, 26]
z_offset = 1
animations = ["idle_motion"]

[[parts]]
kind = "LeftEye"
sprite = "eyes/open.png"
offset = [-8, 32]
z_offset = 4
animations = ["blinking"]
```

---

## Assembly

The `assemble_character` function reads the manifest and spawns all entities:

```rust
pub fn assemble_character(
    commands: &mut Commands,
    asset_server: &AssetServer,
    manifest: &CharacterManifest,
) -> Entity {
    let root = commands.spawn((
        CharacterRoot::default(),
        Transform::from_xyz(0.0, 0.0, manifest.base_z as f32),
        Visibility::default(),
    )).id();

    for part in &manifest.parts {
        let z = manifest.base_z + part.z_offset;
        let child = commands.spawn((
            Sprite {
                image: asset_server.load(&format!("sprites/{}/{}", manifest.name, part.sprite)),
                ..default()
            },
            Transform::from_xyz(part.offset.x, part.offset.y, z as f32),
            SpritePart { kind: part.kind, z_layer: z },
        )).id();
        
        // Add animation components based on manifest
        add_animations(commands, child, &part.animations);
        
        commands.entity(root).add_child(child);
    }
    
    root
}
```

---

## Rendering Notes

### Z-Ordering

> [!IMPORTANT]
> Z-indices are **absolute world-space values**, not relative to parent.

Bevy batches sprites by Z across all entities. Use explicit Z values:
- Body: 100
- Head: 101  
- Paws: 102
- Eyes: 104
- Mouth: 105

### Pixel-Perfect Rendering

```rust
app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
```

For internal 320×240 resolution with upscaling, use a render-to-texture camera.

---

## Performance

| Metric | Target | Typical |
|--------|--------|---------|
| Entity count | <20 | 8 |
| Draw calls | <10 | 3 (batched) |
| Frame time | <16.67ms | <1ms |
| Memory | <50MB | ~5MB |

---

## File Structure

```
src/hamster/
├── mod.rs          # Plugin definition, re-exports
├── components.rs   # All component structs
├── assembly.rs     # assemble_character function
├── systems.rs      # Animation systems
└── manifest.rs     # TOML manifest loading
```

---

## Quick Reference

### Spawn a Character

```rust
let manifest = load_manifest("assets/sprites/hamster/manifest.toml");
let hamster = assemble_character(&mut commands, &asset_server, &manifest);
```

### Change Expression

```rust
fn change_expression(mut query: Query<&mut CharacterRoot>) {
    for mut root in &mut query {
        root.expression = Expression::Happy;
    }
}
```

### Add Corruption Effect

```rust
fn apply_corruption(mut query: Query<&mut CharacterRoot>) {
    for mut root in &mut query {
        root.corruption = (root.corruption + 0.5).min(100.0);
    }
}
```
