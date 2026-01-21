# Architecture Overview – DJ Engine

**Version**: 1.0  
**Last Updated**: 2026-01-20  
**Audience**: Developers, architects

---

## System Overview

DJ Engine is organized into **5 core systems**, each with clear responsibilities and interfaces.

```
┌─────────────────────────────────────────────────────────────────┐
│                     Bevy Game Loop (60 FPS)                     │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │   Input      │  │  Animation   │  │  Rendering (Offscreen) │  │
│  │  - Keys      │→→│ - Breathing  │→→│  - Hamster parts      │  │
│  │  - Events    │  │ - Blinking   │  │  - Palette swap       │  │
│  └──────────────┘  │ - Idle motion│  │  - CRT effects        │  │
│                    └──────────────┘  └──────────────────────┘  │
│                         ↓                      ↓                │
│                    ┌──────────────────────────────────┐         │
│                    │   Scripting (Lua)               │         │
│                    │ - FFI boundary (Rust ↔ Lua)    │         │
│                    │ - Hot-reload on file change    │         │
│                    └──────────────────────────────────┘         │
│                         ↑                                        │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │         Shared Engine State (Resources)                   │  │
│  │  - HamsterNarrator (corruption, expression)              │  │
│  │  - ShaderHandles (palette_swap, crt_postprocess)         │  │
│  │  - HamsterPartLibrary (sprite definitions + handles)     │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## 1. Core Systems

### 1.1 Rendering System (`engine/rendering/`)

**Responsibility**: Output graphics at target framerate.

**Components**:
- **`camera.rs`** – Offscreen render target (320×240), camera setup
- **`palette.rs`** – Palette texture management, swapping
- **`postprocessing.rs`** – CRT shader pipeline

**Key Resources**:
- `ShaderHandles` – Compiled shader references
- `PaletteTexture` – GPU texture for color lookup
- `RenderTarget` – Offscreen texture handle

**Public API** (exposed to other systems):
```rust
pub fn setup_offscreen_render_target(commands: &mut Commands);
pub fn apply_palette_swap(corruption: f32) -> ShaderUniform;
pub fn apply_crt_postprocessing(intensity: f32) -> Material;
```

**Performance Targets**:
- 60 FPS sustained
- < 1ms per frame for rendering
- Minimal VRAM usage (offscreen target only)

---

### 1.2 Animation System (`engine/animation/`)

**Responsibility**: Procedural motion and state transitions.

**Components**:
- **`components.rs`** – `BreathingAnimation`, `BlinkingAnimation`, `IdleMotion` components
- **`systems.rs`** – Update systems for each animation type
- **`easing.rs`** – Procedural curves (sine, noise, envelope)

**Key Types**:
```rust
pub struct BreathingAnimation {
    pub amplitude: f32,
    pub frequency: f32,
}

pub struct BlinkingAnimation {
    pub blink_duration: f32,
    pub interval_min: f32,
    pub interval_max: f32,
}

pub struct IdleMotion {
    pub noise_scale: f32,
    pub speed: f32,
}
```

**Public API**:
```rust
pub fn breathing_system(query: Query<(&BreathingAnimation, &mut Transform)>);
pub fn blinking_system(query: Query<(&mut BlinkingAnimation, &mut Sprite)>);
pub fn idle_motion_system(query: Query<(&IdleMotion, &mut Transform)>, time: Res<Time>);
```

**Performance Targets**:
- No allocations per frame
- Trigonometric functions cached when possible
- Noise functions pre-computed

---

### 1.3 Scripting System (`engine/scripting/`)

**Responsibility**: Lua integration and event dispatch.

**Components**:
- **`ffi.rs`** – Lua-exposed Rust functions (the boundary)
- **`hot_reload.rs`** – File watcher, script reloading
- **`context.rs`** – Lua VM lifecycle

**Key Resource**:
```rust
pub struct LuaContext {
    vm: mlua::Lua,
    script_path: PathBuf,
    last_modified: SystemTime,
}
```

**Exposed Functions** (Lua → Rust):
- `set_corruption(f32)`
- `get_corruption() → f32`
- `set_expression(string) → bool`
- `get_expression() → string`
- `log(string)`

**Callbacks** (Rust → Lua):
- `init()`
- `on_key_press(key: string)`
- `on_dialogue_event(name: string, data?: table)`

**Public API**:
```rust
pub fn setup_lua_context(commands: &mut Commands, script_path: &Path);
pub fn reload_lua_script(context: &mut LuaContext) -> Result<()>;
pub fn call_lua_callback(context: &LuaContext, name: &str) -> Result<()>;
```

---

### 1.4 Asset System (`engine/assets/`)

**Responsibility**: Loading and managing game assets.

**Components**:
- **`definitions.rs`** – Rust data structures for assets
- **`loaders.rs`** – Bevy `AssetLoader` implementations
- **`mod.rs`** – Asset plugin registration

**Key Types**:
```rust
pub struct HamsterPartDefinition {
    pub part_name: String,
    pub sprite_file: String,
    pub sprite_size: IVec2,
    pub original_offset: IVec2,
    pub layer_index: u32,
    pub pivot: Vec2,
}

pub struct HamsterPartLibrary {
    parts: HashMap<String, (HamsterPartDefinition, Handle<Image>)>,
}

pub struct PaletteDefinition {
    pub palette_name: String,
    pub colors: Vec<ColorEntry>,
}
```

**Public API**:
```rust
pub async fn load_hamster_parts(path: &Path) -> Result<HamsterPartLibrary>;
pub async fn load_palette(path: &Path) -> Result<PaletteDefinition>;
pub fn create_palette_texture(palette: &PaletteDefinition) -> Image;
```

---

### 1.5 Type System (`engine/types.rs`)

**Responsibility**: Shared data structures used across all systems.

**Core Types**:
```rust
/// The main hamster character component.
#[derive(Component, Resource)]
pub struct HamsterNarrator {
    pub corruption: f32,        // 0.0–100.0
    pub expression: Expression,
    pub animation_time: f32,
    pub mood: Mood,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    Neutral,
    Happy,
    Angry,
    Sad,
    Corrupted,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Normal,
    Excited,
    Melancholy,
}

/// Represents a hamster sprite part (child entity).
#[derive(Component)]
pub struct HamsterPart {
    pub part_type: String,
    pub offset: Vec2,
    pub layer: u32,
}

/// Shader uniform for corruption effects.
pub struct CorruptionUniforms {
    pub corruption: f32,      // 0.0–1.0 (normalized)
    pub time: f32,            // For animated effects
    pub palette_shift: i32,   // Which palette variant (0, 1, 2, 3)
}
```

**Error Types**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum DJEngineError {
    #[error("Asset loading failed: {0}")]
    AssetLoadError(String),
    
    #[error("Lua error: {0}")]
    LuaError(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderError(String),
}

pub type Result<T> = std::result::Result<T, DJEngineError>;
```

---

## 2. Data Flow Diagrams

### 2.1 Hamster Initialization

```
Game Start
    ↓
AssetPlugin::setup()
    ├─ Load all sprite metadata (*.meta.json)
    ├─ Load all PNG images
    └─ Create HamsterPartLibrary
        ↓
spawn_hamster_system()
    ├─ Create root HamsterNarrator entity
    └─ For each part in library:
        ├─ Create child entity with HamsterPart component
        ├─ Attach sprite image
        └─ Set transform offset
            ↓
Hamster Ready
```

### 2.2 Animation Update Loop

```
Bevy Update Phase (every frame)
    ↓
breathing_system()
    ├─ For each entity with BreathingAnimation:
    │   ├─ Calculate scale: sin(time * frequency)
    │   ├─ Apply to transform
    │   └─ Update all children (parts follow)
    ↓
blinking_system()
    ├─ Decrement blink_timer
    ├─ If timer < 0: pick new interval, toggle eye sprite
    ↓
idle_motion_system()
    ├─ Sample Perlin noise at time * speed
    ├─ Apply jitter to head position
    ↓
rendering_system()
    ├─ Render all entities to offscreen target (320×240)
    ├─ Apply palette swap based on corruption
    ├─ Apply CRT post-processing
    └─ Upscale to window with nearest-neighbor
```

### 2.3 Corruption State Change

```
on_key_press("A") [nice choice]
    ↓
lua_script::on_key_press("A")
    ├─ Calls Rust: set_corruption(math.max(0, current - 10))
    └─ Calls Rust: set_expression("happy")
        ↓
Rust FFI updates HamsterNarrator resource
    ├─ corruption = 35.0 (decreased)
    ├─ expression = Happy
        ↓
Next frame:
    ├─ rendering_system() reads new corruption
    ├─ Calculates palette_shift based on corruption level
    ├─ Applies palette swap shader
    └─ Output shows new colors + expression
```

### 2.4 Lua Hot-Reload

```
Lua script file edited and saved
    ↓
File watcher detects change (debounce 500ms)
    ↓
scripting_system() calls reload_lua_script()
    ├─ Read file from disk
    ├─ Clear old Lua VM
    ├─ Create fresh Lua context
    ├─ Register FFI functions
    ├─ Load new script text
    ├─ Call init() callback
    └─ **Preserve Rust-side state (corruption, expression)**
        ↓
Game continues with new script, same state
```

---

## 3. Plugin Architecture

Each system is a Bevy `Plugin` that can be added independently:

```rust
// In main.rs
use dj_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenderingPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(ScriptingPlugin)
        .add_plugins(AssetPlugin)
        .add_systems(Startup, spawn_hamster_system)
        .run();
}
```

**Benefits**:
- Each system is independent and testable
- Can add/remove features without touching other systems
- Easy to disable expensive systems during development

---

## 4. Module Dependencies

```
                    dj_engine (engine/)
                           │
       ┌───────────────────┼───────────────────┬────────────┐
       │                   │                   │            │
   rendering/          animation/         scripting/    assets/
       │                   │                   │            │
       └───────────────────┼───────────────────┴────────────┘
                           │
                       types.rs
                           │
                    (shared by all)


doomexe (games/dev/doomexe/)
    │
    ├── Imports: dj_engine crate
    │
    ├── Game-specific code:
    │   ├── main.rs (app setup)
    │   ├── hamster/ (game-specific hamster logic)
    │   ├── scripting/ (Lua script management)
    │   └── assets/ (local asset loading)
```

**Rule**: `engine/` never imports from `games/`. Games import from `engine/`.

---

## 5. Thread Safety & Concurrency

### Current Model (Milestone 1)

- **Single-threaded** render thread (Bevy default)
- **Lua VM**: Not thread-safe in Bevy; all calls from main thread
- **Asset loading**: Bevy's `AssetServer` handles async on separate thread

### Future Considerations

- **GPU instancing** (future): Potential for parallel animation updates
- **Networking** (future): Separate thread for server communication
- **Asset streaming** (future): Progressive loading on background thread

---

## 6. Performance Characteristics

| Component | Time Budget | Current | Headroom |
|-----------|------------|---------|----------|
| **Animation systems** | 2ms | ~0.5ms | 75% |
| **Rendering (offscreen)** | 3ms | ~1ms | 67% |
| **CRT post-processing** | 1ms | ~0.3ms | 70% |
| **Lua FFI overhead** | 1ms | ~0.1ms | 90% |
| **Asset loading** | N/A (async) | Varies | N/A |
| **Total budget** | 16.67ms | ~2ms | 88% |

**Headroom**: Room for features and optimization.

---

## 7. Error Handling Strategy

### At Each Boundary

| Boundary | Error Type | Handler |
|----------|-----------|---------|
| **Asset load fails** | `AssetLoadError` | Log warning, skip asset; critical = exit |
| **Lua script error** | `LuaError` | Wrapped in Bevy error, logged, game continues |
| **Shader compile fails** | `ShaderError` | Logged; fallback material used |
| **Animation NaN/Inf** | Panic in debug, clamp in release | Guard with `is_finite()` checks |

### Policy

- **Development**: Panic on programmer errors (assertions, unwrap)
- **Release**: Graceful degradation; log errors, continue game
- **User input**: Never panic; return errors

---

## 8. Testing Strategy

### Unit Tests

Each module has local `#[cfg(test)] mod tests`:
- Test component creation and state changes
- Test error boundary conditions
- Test parsing/loading logic

### Integration Tests

`games/dev/doomexe/tests/`:
- Hamster spawning and assembly
- Animation timing
- Lua script loading and execution
- Corruption state transitions

### Performance Tests

Benchmarks for hot paths:
- Animation system updates (1000 entities)
- Lua FFI calls per frame
- Shader application overhead

---

## 9. Extensibility Points

### For Future Games (Beyond doomexe)

1. **Animation System**: Add new animation types by implementing new `Component + System` pairs
2. **Scripting**: Extend FFI API without breaking existing Lua scripts
3. **Rendering**: Add new post-processing effects as shader passes
4. **Asset System**: New asset types via `AssetLoader` trait

### Example: Adding a "TalkingAnimation" System

```rust
// In engine/animation/
#[derive(Component)]
pub struct TalkingAnimation {
    pub mouth_sprites: Vec<Handle<Image>>,
    pub playback_rate: f32,
}

fn talking_animation_system(
    mut query: Query<(&TalkingAnimation, &mut Sprite)>,
) {
    for (talking, mut sprite) in query.iter_mut() {
        // Update mouth sprite based on time
    }
}

// In game (doomexe):
app.add_systems(Update, talking_animation_system);
```

---

## 10. Known Limitations

### Milestone 1

- **Single hamster** (no multiple characters on screen)
- **No dialogue branching** (just input → state mapping)
- **No save/load** (game state reset on restart)
- **No audio** (visual-only)
- **No mobile support** (desktop only)

### Future

- Multiple characters will require entity pooling
- Dialogue trees need state machine in Lua
- Save system needs serialization strategy
- Audio requires integration with Bevy's audio plugin

---

## 11. Development Roadmap

### Phase 1: Foundation (Weeks 1–5, Milestone 1)
- ✅ Set up module structure
- ✅ Rendering foundation
- ✅ Animation system
- ✅ Lua FFI minimal
- ✅ Asset pipeline
- ✅ Integration testing

### Phase 2: Story (Weeks 6–10, Milestone 2)
- [ ] Full dialogue system
- [ ] Branching storylines
- [ ] Save/load state
- [ ] Scene transitions

### Phase 3: Polish (Weeks 11–15, Milestone 3)
- [ ] Extended animation library
- [ ] Audio integration
- [ ] Visual effects (particles, distortion)
- [ ] Performance optimization

### Phase 4: Release (Week 16+)
- [ ] Cross-platform testing
- [ ] Build automation
- [ ] Public release prep

---

## Conclusion

DJ Engine is designed for **clarity and extensibility**. Each system has a clear responsibility, a stable interface, and room to grow. The Lua boundary keeps designers happy while Rust keeps performance predictable.

Start with the core systems (rendering, animation, scripting), test thoroughly, and iterate.

Questions? See the other documentation or open a GitHub discussion.
