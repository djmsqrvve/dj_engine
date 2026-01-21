# DJ Engine Sprite System – Visual Architecture & Best Practices

**Date**: 2026-01-20  
**Scope**: Hamster Narrator Sprite System Design  
**Audience**: Technical Team (Developers)  

---

## 1. Rendering Pipeline Architecture

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                         BEVY RENDERING PIPELINE                             │
└──────────────────────────────────────────────────────────────────────────────┘

                    GAME WORLD (ECS)
                            ▲
                            │
              ┌─────────────┴────────────────┐
              │                              │
    ┌─────────▼──────────┐         ┌────────▼─────────┐
    │  Camera (Internal) │         │  Camera (Main)   │
    │  320×240 render    │         │  Window output   │
    │  RenderLayers: 0   │         │  RenderLayers: 1 │
    └─────────┬──────────┘         └────────▲─────────┘
              │                             │
              │ Renders to:                 │ Samples from:
              ▼                             │
    ┌─────────────────────────────────────────────────────┐
    │  OFFSCREEN TEXTURE (320×240)                       │
    │  ┌──────────────────────────────────────────────┐  │
    │  │ Hamster Entity                              │  │
    │  │ ├─ Body (z=100)                            │  │
    │  │ ├─ Head (z=101)                            │  │
    │  │ ├─ Ears (z=102)                            │  │
    │  │ ├─ Eyes (z=104)                            │  │
    │  │ └─ Mouth (z=105)                           │  │
    │  │                                             │  │
    │  │ All rendered with local transforms         │  │
    │  └──────────────────────────────────────────────┘  │
    └──────────────┬──────────────────────────────────────┘
                   │
         Shader Processing:
    ┌──────────────▼──────────────────────┐
    │  1. NEAREST-NEIGHBOR UPSCALING      │
    │     320×240 → window resolution     │
    │                                      │
    │  2. CRT POST-PROCESSING             │
    │     • Scanlines                     │
    │     • Vignette                      │
    │     • Raster jitter (corruption)    │
    │     • Chromatic aberration (corr.)  │
    │                                      │
    │  3. OUTPUT                          │
    │     Final pixel-perfect image       │
    └──────────────┬──────────────────────┘
                   │
                   ▼
            WINDOW DISPLAY
            (Pixel-Perfect)
```

---

## 2. Entity Hierarchy & Z-Ordering

```
WORLD SPACE (Absolute Z)
───────────────────────

z = 99:   [Background/UI - Not shown here]

z = 100:  HamsterNarrator (root)
          ├─ Transform: (0, 0, 100)
          ├─ GlobalTransform: calculated
          └─ Visibility: computed

z = 100:  Body (child entity)
          ├─ HamsterPart { layer: 0 }
          ├─ BreathingAnimation (scale: 1.0 ± 0.05)
          ├─ CorruptionEffect
          └─ Transform relative to parent
             BUT absolute Z = 100 in world space

z = 101:  Head (child entity)
          ├─ HamsterPart { layer: 1 }
          ├─ IdleMotion
          └─ Z = 101 (rendered ON TOP of body)

z = 102:  LeftEar & RightEar (children)
          └─ Z = 102 (rendered ON TOP of head)

z = 104:  LeftEye & RightEye (children)
          ├─ BlinkingAnimation { state: Open/Closed }
          └─ Z = 104 (rendered ON TOP of ears)

z = 105:  Mouth (child entity)
          └─ Z = 105 (rendered ON TOP of everything)


KEY INSIGHT:
────────────
Child entities have INDEPENDENT z-indices. The parent's transform
does NOT affect the child's z in world space.

Each part gets its own Transform with explicit z position:
  Parent (z=100) + Child (z=101) ≠ Child rendered at z=101 relative to parent
  
Instead:
  Parent (z=100) and Child (z=101) are both rendered in absolute z-order
  Bevy's batching handles depth ordering across all entities
```

---

## 3. Animation State Machine

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                    ANIMATION & STATE UPDATES                                 │
└──────────────────────────────────────────────────────────────────────────────┘

                      HamsterNarrator Component
                      ┌────────────────────────┐
                      │ corruption: f32        │  ←─── Lua Script Updates
                      │ expression: Expression │      (Every frame or on event)
                      │ animation_time: f32    │
                      │ mood: Option<Mood>     │
                      └────────┬───────────────┘
                               │
                    ┌──────────┴──────────────┐
                    │                         │
        ┌───────────▼────────────┐  ┌────────▼──────────────┐
        │ BREATHING SYSTEM       │  │ BLINKING SYSTEM      │
        ├───────────────────────┤  ├───────────────────────┤
        │ Input:                │  │ Input:                │
        │ • BreathingAnimation  │  │ • BlinkingAnimation   │
        │ • time.elapsed_secs() │  │ • time.delta()        │
        │                        │  │ • timer.tick()        │
        │ Output:                │  │                        │
        │ • Transform.scale      │  │ Output:                │
        │   Y = 1.0 ± 0.05       │  │ • Sprite.color.alpha  │
        │   X = 1.0 / Y          │  │   → 1.0 (open)        │
        │                        │  │   → 0.0 (closed)      │
        └────────────────────────┘  └────────────────────────┘
                    │                         │
        ┌───────────▼────────────┐  ┌────────▼──────────────┐
        │ IDLE MOTION SYSTEM     │  │ CORRUPTION SYSTEM    │
        ├───────────────────────┤  ├───────────────────────┤
        │ Input:                │  │ Input:                │
        │ • IdleMotion          │  │ • HamsterNarrator     │
        │ • time.elapsed_secs() │  │   .corruption         │
        │                        │  │                        │
        │ Output:                │  │ Output:                │
        │ • Transform.translation│  │ • CorruptionEffect    │
        │   Offset by sine/cos   │  │ • Updates palette idx │
        │   (~5px max)           │  │ • Updates CRT params  │
        └────────────────────────┘  └────────────────────────┘
                    │                         │
                    │                         │
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  BEVY RENDERING        │
                    │  ├─ Collect all Sprites│
                    │  ├─ Sort by Z-index    │
                    │  ├─ Batch by texture   │
                    │  └─ Render in order    │
                    └───────────┬────────────┘
                                │
                                ▼
                        ┌───────────────────┐
                        │ OFFSCREEN BUFFER  │
                        │ (320×240)         │
                        └───────┬───────────┘
                                │
                                ▼
                        ┌───────────────────┐
                        │ CRT SHADER        │
                        │ (Upscale + Effects)
                        └───────┬───────────┘
                                │
                                ▼
                        WINDOW DISPLAY
```

---

## 4. Component Relationship Diagram

```
┌────────────────────────────────────────────────────────────────────────┐
│                      ENTITY COMPONENT STRUCTURE                        │
└────────────────────────────────────────────────────────────────────────┘

HAMSTER NARRATOR (Root Entity)
├─ Transform                    ← Position/rotation/scale in world
├─ GlobalTransform              ← Computed world transform (updated by Bevy)
├─ Visibility                   ← Is entity visible?
├─ InheritedVisibility          ← Inherited from parent
├─ HamsterNarrator {            ← State marker
│    corruption: 0.0,
│    expression: Neutral,
│    animation_time: 0.0,
│    mood: None
│  }
└─ Children                     ← List of child entity IDs


BODY (Child Entity 1)
├─ Transform                    ← Position relative to parent + abs Z
├─ GlobalTransform              ← Updated by Bevy's hierarchy system
├─ Sprite {                     ← Render component
│    image: Handle<Image>,
│    custom_size: None,
│    ...
│  }
├─ HamsterPart {                ← Part type marker
│    part_type: Body,
│    offset: (0, 0),
│    layer: 0,
│    base_rotation: 0.0
│  }
├─ BreathingAnimation {         ← Animation state
│    amplitude: 0.05,
│    frequency: 0.5,
│    phase_offset: 0.0
│  }
├─ CorruptionEffect {           ← Visual state
│    corruption_level: 0.0,
│    palette_shift_index: 0,
│    raster_jitter_amplitude: 0.0,
│    chromatic_aberration: 0.0
│  }
├─ Parent(HamsterNarrator)      ← Link to parent
└─ GlobalTransform             ← Recalculated automatically


HEAD (Child Entity 2)
├─ Transform                    ← Position + abs Z = 101
├─ GlobalTransform
├─ Sprite
├─ HamsterPart { layer: 1 }
├─ IdleMotion {                 ← Subtle position animation
│    position_amplitude: (5, 2),
│    frequency: 0.2,
│    phase_offset: 0.0
│  }
├─ CorruptionEffect
├─ Parent(HamsterNarrator)
└─ GlobalTransform


LEFT_EYE (Child Entity 3)
├─ Transform                    ← Position + abs Z = 104
├─ GlobalTransform
├─ Sprite
├─ HamsterPart { layer: 4 }
├─ BlinkingAnimation {          ← Eye open/close state
│    timer: Timer,
│    blink_duration: 100ms,
│    interval_range: (3, 7),
│    state: Open,
│    is_left_eye: true
│  }
├─ CorruptionEffect
├─ Parent(HamsterNarrator)
└─ GlobalTransform


[... Similar for RIGHT_EYE, MOUTH, EARS, etc. ...]


KEY OBSERVATIONS:
─────────────────
1. Each part is a separate entity (ECS principle)
2. Parent-child relationship established via Parent component + GlobalTransform
3. Each component has independent Transform with explicit Z
4. Sprites rendered by Bevy's sprite batching system (automatic)
5. Animations updated by independent systems querying specific components
6. Corruption affects all parts (CorruptionEffect on each)
```

---

## 5. System Execution Order

```
┌──────────────────────────────────────────────────────────────────────────┐
│                     BEVY SCHEDULE (Per Frame)                            │
└──────────────────────────────────────────────────────────────────────────┘

SCHEDULE: Startup
  ├─ setup_hamster()
  │  ├─ Spawn Camera2d
  │  └─ Call assemble_hamster()
  │     ├─ Spawn HamsterNarrator root
  │     ├─ Spawn 7-8 HamsterPart child entities
  │     ├─ Add animation components (breathing, blinking)
  │     └─ Add to scene tree
  └─ Finished initialization


SCHEDULE: Update (every frame, ~16.67ms @ 60 FPS)
  │
  ├─ INPUT HANDLING (optional, debug only)
  │  └─ debug_input_system()
  │     └─ Update HamsterNarrator { corruption, expression }
  │
  ├─ ANIMATION UPDATES (in parallel where possible)
  │  ├─ breathing_system()
  │  │  └─ Query: (Transform, BreathingAnimation) with HamsterPart
  │  │     └─ Update Transform.scale
  │  │
  │  ├─ blinking_system()
  │  │  └─ Query: (BlinkingAnimation, Sprite) with HamsterPart
  │  │     └─ Update Sprite.color.alpha & timer
  │  │
  │  ├─ idle_motion_system()
  │  │  └─ Query: (Transform, IdleMotion) with Head part
  │  │     └─ Update Transform.translation
  │  │
  │  └─ corruption_system()
  │     └─ Query: (HamsterNarrator, CorruptionEffect)
  │        └─ Update palette indices, CRT parameters
  │
  ├─ BEVY INTERNAL
  │  ├─ Update GlobalTransform for all entities
  │  ├─ Compute visibility
  │  ├─ Prepare render data
  │  └─ Cull off-screen objects
  │
  ├─ RENDERING
  │  ├─ Internal Camera (320×240 target)
  │  │  └─ Render sprites to offscreen texture
  │  │
  │  ├─ Main Camera (window output)
  │  │  ├─ Sample offscreen texture
  │  │  └─ Apply CRT shader
  │  │
  │  └─ Output to window
  │
  └─ Frame complete


TOTAL TIME PER FRAME:
  Game logic: ~1ms (animations are cheap)
  Rendering: ~14ms (sprite batching is efficient)
  Margin: ~1.67ms (headroom for future features)
  
  Total: ~16.67ms target (60 FPS)
```

---

## 6. Data Flow: Lua → State → Visual

```
┌─────────────────────────────────────────────────────────────────────────┐
│               LUA SCRIPT → BEVY STATE → VISUAL OUTPUT                  │
└─────────────────────────────────────────────────────────────────────────┘

                        LUA SCRIPT
                   (dialogue control)
                            │
                            ▼
              set_corruption(50) ─┐
              set_expression(1)   │
              log("message")      │
                            │    │
                    ┌───────┴────┴──────┐
                    │                   │
                    ▼                   ▼
            Resource Channel         Resource
            (Command sender)    (State holder)
                    │                   │
                    └───────┬───────────┘
                            │
                    ┌───────▼──────────┐
                    │ UPDATE SYSTEM    │
                    │ (Rust side)      │
                    │                  │
                    │ Receives channel │
                    │ messages & updates
                    │ HamsterNarrator  │
                    │ { corruption,    │
                    │   expression }   │
                    └───────┬──────────┘
                            │
             ┌──────────────┴──────────────┐
             │                             │
        ┌────▼─────────┐          ┌────────▼─────┐
        │ CORRUPTION   │          │ EXPRESSION   │
        │ SYSTEM       │          │ UPDATE       │
        │              │          │              │
        │ Reads: 50%   │          │ Reads: 1     │
        │              │          │ (Happy)      │
        │ Writes to:   │          │              │
        │ • Palette idx│          │ Writes:      │
        │ • CRT params │          │ • Mouth      │
        │ • Jitter amt │          │   sprite     │
        │              │          │ • Color tint │
        └────┬─────────┘          └────────┬─────┘
             │                             │
             └──────────────┬──────────────┘
                            │
                    ┌───────▼──────────┐
                    │ ALL PARTS        │
                    │ Update with:     │
                    │ • CorruptionEff. │
                    │ • Sprite changes │
                    │ • Color updates  │
                    └───────┬──────────┘
                            │
                    ┌───────▼──────────┐
                    │ RENDERING        │
                    │ Visual changes:  │
                    │ • Mouth shape    │
                    │ • Color palette  │
                    │ • CRT intensity  │
                    │ • Chromatic abbr.│
                    └───────┬──────────┘
                            │
                            ▼
                    PLAYER SEES:
                    • Different mouth
                    • Color tint
                    • Stronger glitch
```

---

## 7. Performance Characteristics

```
┌──────────────────────────────────────────────────────────────────────────┐
│                      PERFORMANCE ANALYSIS                               │
└──────────────────────────────────────────────────────────────────────────┘

ENTITY OVERHEAD (per frame):
  HamsterNarrator:          ~10 bytes (state struct)
  × 8 HamsterPart children: ~30 bytes each = 240 bytes
  Transform data:           ~50 bytes per entity
  GlobalTransform calc:     ~1 arithmetic ops per entity
  Total state memory:       ~500 bytes (negligible)

RENDERING OVERHEAD:
  Sprite batching:          1 draw call per unique texture
  Our case:                 ~8 parts, 1 call (if on same atlas)
  vs typical UI scene:      ~50-100 draw calls
  Our overhead:             <1% of total

ANIMATION SYSTEM COST (per frame):
  breathing_system:         8 sin/cos calls → ~0.001ms
  blinking_system:          8 timer.tick() → ~0.0001ms
  idle_motion_system:       8 sin/cos calls → ~0.001ms
  corruption_system:        8 math ops → ~0.0001ms
  Total animation:          ~0.003ms (negligible)

MEMORY USAGE:
  Assets on disk:           ~200 KB (sprites)
  Loaded in RAM:            ~2 MB (textures)
  Scene entities:           ~5 KB
  Total footprint:          ~2 MB (tiny)

FRAME TIME BUDGET (60 FPS = 16.67ms):
  Game logic (ours):        ~0.01ms
  Rendering (GPU):          ~14.00ms
  Margin:                   ~2.66ms
  Utilization:              ~85% (comfortable)

SCALING:
  Add 1 more hamster:       +0.01ms logic, +1 draw call
  Add 10 more hamsters:     +0.1ms logic, +10 draw calls (still <1ms)
  Still well under 60 FPS
```

---

## 8. Best Practices Checklist

### Component Design
- [x] Each part is its own entity (ECS principle)
- [x] Components are small and focused
- [x] No heavy computation in components
- [x] State is data, behavior is in systems

### Rendering
- [x] Use absolute Z-indices (not relative to parent)
- [x] Explicit layer ordering (100, 101, 102...)
- [x] Keep sprites on same texture atlas when possible
- [x] Use nearest-neighbor filtering for pixel art
- [x] Offscreen render at low resolution, upscale with shader

### Animation
- [x] Use time-based animation (not frame counting)
- [x] Use sine/cosine for smooth motion
- [x] Use timers for discrete events (blinking)
- [x] Phase offsets for staggered animations
- [x] Randomize intervals (don't use fixed timing)

### Hierarchy
- [x] Use Bevy's Parent/Children components
- [x] GlobalTransform updates automatically
- [x] Each child has independent Transform
- [x] Don't rely on parent scale for child rendering

### Performance
- [x] Batch spirits by Z and texture
- [x] Cull off-screen entities automatically
- [x] Use components instead of nested structs
- [x] Query only what you need
- [x] Avoid expensive math in hot loops

### Code Organization
- [x] Separate components, systems, assembly into modules
- [x] Use plugins for organizing features
- [x] Clear naming conventions
- [x] Reusable helper functions
- [x] Document assumptions (Z-indexing rules, etc.)

---

## 9. Example: Visual State Changes

```
User Action: Press U (increase corruption)

Before:
  ┌─────────────────────────────┐
  │ HamsterNarrator             │
  │ corruption: 0.0             │
  │ expression: Neutral         │
  └─────────────────────────────┘
           │
           │ Renders as:
           ▼
  ┌─────────────────────────────┐
  │ Color: Normal               │
  │ Mouth: Neutral shape        │
  │ CRT: Low intensity          │
  │ Distortion: None            │
  └─────────────────────────────┘


User Press: U

        ┌──────────────────────────────────┐
        │ debug_input_system() fires       │
        │ narrator.corruption += 0.5       │
        │ → corruption: 0.5                │
        └──────────────────────────────────┘
                      │
                      ▼
        ┌──────────────────────────────────┐
        │ corruption_system() fires        │
        │ Updates all parts' CorruptionEff.│
        │ palette_shift_idx: 0 (no change) │
        │ jitter_amp: 0.0025               │
        └──────────────────────────────────┘
                      │
                      ▼
        ┌──────────────────────────────────┐
        │ Rendering system picks up change │
        │ Applies slight color tint        │
        │ Adds minimal jitter              │
        └──────────────────────────────────┘
                      │
                      ▼
        ┌──────────────────────────────────┐
        │ Screen Update                    │
        │ Subtle visual change             │
        └──────────────────────────────────┘


Continue Pressing: U (multiple times)

After corruption reaches 50%:
  corruption_system() calculates:
    palette_shift_idx = 50 / 25 = 2 (purple tint)
    jitter_amp = 50 / 200 = 0.25
    aberration = 50 / 100 * 2 = 1.0 px

  ┌─────────────────────────────┐
  │ Color: Purple tint          │
  │ Mouth: Could change         │
  │ CRT: Medium intensity       │
  │ Distortion: Noticeable      │
  └─────────────────────────────┘

After corruption reaches 100%:
  corruption_system() calculates:
    palette_shift_idx = 100 / 25 = 4 (red-pink tint)
    jitter_amp = 100 / 200 = 0.5
    aberration = 100 / 100 * 2 = 2.0 px (max)

  ┌─────────────────────────────┐
  │ Color: Red-pink tint        │
  │ Mouth: Corrupted shape      │
  │ CRT: Maximum intensity      │
  │ Distortion: Heavy glitch    │
  └─────────────────────────────┘
```

---

## 10. Troubleshooting Decision Tree

```
┌─────────────────────────────────────────┐
│ HAMSTER NOT APPEARING                   │
└─────────────────────────────────────────┘
        │
        ├─ Camera spawned? ─No─→ Add Camera2d::default()
        │
        ├─ Assets loaded? ─No─→ Check asset paths
        │                       Check ImagePlugin::default_nearest()
        │
        ├─ Z in valid range? ─No─→ Use z: 100+ range
        │                          Avoid negative z
        │
        ├─ Parent-child correct? ─No─→ Use commands.entity(parent).add_child(child)
        │
        └─ Still not visible? ─→ Add debug output to systems
                                Print Transform.z, visibility

┌─────────────────────────────────────────┐
│ ANIMATION JERKY/STUTTERS                │
└─────────────────────────────────────────┘
        │
        ├─ System in Update? ─No─→ Move to Update schedule
        │
        ├─ Frame rate stable? ─No─→ Check for other heavy work
        │                          Use profiler
        │
        ├─ Math in hot loop? ─Yes─→ Cache results if possible
        │                          Use lookup tables
        │
        └─ Still stuttering? ─→ Profile with diagnostic plugin
                               Check GPU stalls

┌─────────────────────────────────────────┐
│ PARTS RENDERING IN WRONG ORDER          │
└─────────────────────────────────────────┘
        │
        ├─ Z-indices correct? ─No─→ Set explicit z: 100, 101, 102...
        │
        ├─ Z in world space? ─No─→ Use absolute z, not relative
        │
        ├─ Transform.z != layer? ─→ Sync these values
        │
        └─ Check Bevy version ─→ May have changed z handling
```

---

## Final Checklist: Ready to Build?

- [x] Understand entity hierarchy (parent/children)
- [x] Know Z-indexing rules (absolute, not relative)
- [x] Have component structure planned
- [x] Know which systems to implement
- [x] Have asset export workflow
- [x] Know performance targets (60 FPS)
- [x] Understand Lua integration path
- [x] Ready to write code!

**Next Step**: Start with `components.rs` + `assembly.rs` to get hamster rendering. Then add animation systems incrementally.

---

**Created**: 2026-01-20  
**Owner**: Technical Team  
**Status**: Ready for Development ✨