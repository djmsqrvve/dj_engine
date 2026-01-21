# DJ Engine – Complete Project Plan
**Date**: 2026-01-20  
**Status**: Planning Phase  
**Next Phase**: Milestone 1 Implementation Sprint

---

## Executive Summary

The DJ Engine project is well-conceived at the architecture level. Your AI captured the vision accurately:
- Cargo workspace with shared engine crate + game projects
- Milestone 1 (Hamster Narrator) is a solid first vertical slice
- Clear separation between engine infrastructure and game-specific code

**However, there are critical scaffolding gaps** that will cause pain during implementation if not addressed now:

1. **No concrete module structure inside `engine/`** – currently just an empty `lib.rs`
2. **No asset pipeline defined** – Aseprite → runtime flow is vague
3. **Lua FFI boundary unclear** – what types/functions get exposed to Lua?
4. **Shader handling not specified** – where do `.wgsl` files live? How are they loaded?
5. **No build/CI strategy** – cargo features, platform targets, test structure
6. **Dependencies not pinned** – Bevy version lock is critical for workspace stability
7. **No error handling strategy** – panic vs. Result types?
8. **Milestone 1 scope creep risk** – animation, shaders, Lua, rendering is a LOT for one milestone

**This plan resolves those gaps and provides a step-by-step roadmap.**

---

## Part 1: Current State (Verified)

### Repository Structure
```
dj_engine/
├── Cargo.toml                 # Workspace manifest
├── Cargo.lock                 # (Should be committed)
├── README.md                  # Top-level overview
├── PROJECT_PLAN.md            # This file
├── engine/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs             # Empty; needs sub-modules
├── tools/                     # (Empty, planned)
├── games/
│   └── dev/
│       └── doomexe/
│           ├── Cargo.toml
│           ├── README.md
│           ├── assets/        # (Mostly planned)
│           │   ├── scripts/
│           │   ├── shaders/
│           │   ├── sprites/
│           │   └── palettes/
│           └── src/
│               ├── main.rs
│               ├── hamster/   # (Module structure needed)
│               ├── scripting/ # (Module structure needed)
│               └── assets/    # (Asset loading utilities)
└── docs/                      # (New) Design documents
```

### Crates & Dependencies (Target)

**Engine Dependencies** (shared across games):
```toml
[dependencies]
bevy = "0.14"              # Must pin exact version for workspace
mlua = { version = "0.9", features = ["lua54", "serialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
nalgebra = "0.32"          # For noise/procedural curves
```

**Game Dependencies** (doomexe-specific):
```toml
[dependencies]
dj_engine = { path = "../../engine" }
bevy = "0.14"              # Must match workspace
mlua = "0.9"
anyhow = "1.0"             # Error handling
tracing = "0.1"            # Logging
```

### Current Issues Identified
- Workspace is set up, but `engine/lib.rs` is empty → no public API yet
- Game crate links to engine but no actual engine code to link against
- No `.gitignore` entries for Bevy build artifacts, cached shaders, etc.
- Cargo.lock should be in repo for reproducible builds

---

## Part 2: Engineering Scaffolding (Action Required Now)

### 2.1 Engine Module Structure

Create this skeleton structure in `engine/src/` to establish the public API **before** implementation:

```
engine/src/
├── lib.rs                      # Module declarations + pub use re-exports
├── rendering/
│   ├── mod.rs                  # Bevy plugin registration
│   ├── camera.rs               # Offscreen render target setup
│   ├── palette.rs              # Palette texture management
│   └── postprocessing.rs        # CRT shader + passes
├── animation/
│   ├── mod.rs                  # Plugin + public API
│   ├── components.rs            # Transform hierarchy, animation state
│   ├── systems.rs               # Breathing, blinking, idle motion
│   └── easing.rs                # Procedural curves (sine, noise, easing)
├── scripting/
│   ├── mod.rs                  # Lua integration plugin
│   ├── ffi.rs                  # Lua-exposed Rust functions
│   └── hot_reload.rs            # Script reload utilities
├── assets/
│   ├── mod.rs                  # Asset loading plugin
│   ├── loaders.rs               # JSON/sprite loaders
│   └── definitions.rs           # Aseprite metadata structs
└── types.rs                     # Shared data types across all modules
```

**Key principle:** Each module is a Bevy Plugin that can be added to the app independently.

### 2.2 Cargo Workspace Configuration

**`dj_engine/Cargo.toml`** (workspace root):
```toml
[workspace]
members = ["engine", "games/dev/doomexe"]
resolver = "2"

[workspace.package]
version = "0.0.1"
edition = "2021"
authors = ["Your Name"]

[workspace.dependencies]
bevy = "0.14"
mlua = { version = "0.9", features = ["lua54", "serialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

This ensures all crates use consistent dependency versions.

### 2.3 `.gitignore` Entries

Add to root `.gitignore`:
```
# Rust
target/
Cargo.lock              # Controversial, but recommended for workspaces
*.swp
*.swo
*~

# Bevy
bevy_assets/
*.wgpu_cache/

# IDE
.vscode/
.idea/
*.iml

# OS
.DS_Store
Thumbs.db

# Generated
/dist/
```

### 2.4 Public API Surface

**`engine/src/lib.rs`** sketch:
```rust
pub mod rendering;
pub mod animation;
pub mod scripting;
pub mod assets;
pub mod types;

pub mod prelude {
    pub use crate::rendering::RenderingPlugin;
    pub use crate::animation::AnimationPlugin;
    pub use crate::scripting::ScriptingPlugin;
    pub use crate::assets::AssetPlugin;
}

pub fn engine_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
```

This establishes the public interface **before** adding implementation.

---

## Part 3: Asset Pipeline Design

### 3.1 Aseprite → Runtime Flow

**File Structure** (in `games/dev/doomexe/assets/`):
```
assets/
├── sprites/
│   └── hamster_parts/
│       ├── body.aseprite      # Single layer per part
│       ├── body_body.png
│       ├── body_meta.json     # Generated by Aseprite script
│       ├── head.aseprite
│       ├── head_head.png
│       ├── head_meta.json
│       ├── eye_open.aseprite
│       ├── eye_open_eye.png
│       ├── eye_open_meta.json
│       ├── eye_closed.aseprite
│       ├── eye_closed_eye.png
│       └── eye_closed_meta.json
├── palettes/
│   ├── default.json           # Palette definition (RGB values)
│   └── corrupted.json
├── shaders/
│   ├── palette_swap.wgsl
│   └── crt_postprocess.wgsl
└── scripts/
    └── hamster_dialogue.lua
```

### 3.2 Aseprite Export Format

**Aseprite JSON Metadata** (`body_meta.json`):
```json
{
  "part_name": "body",
  "sprite_file": "body_body.png",
  "sprite_size": { "w": 128, "h": 96 },
  "original_offset": { "x": 0, "y": 0 },
  "layer_index": 0,
  "pivot": { "x": 64, "y": 48 },
  "trim_rect": { "x": 4, "y": 8, "w": 120, "h": 88 }
}
```

This metadata is **manually created or exported via Aseprite scripting**, then parsed at runtime.

### 3.3 Asset Loading Pipeline (Runtime)

**Sequence**:
1. Game startup → `AssetPlugin::setup()`
2. Load all `.json` files in `assets/sprites/hamster_parts/`
3. Parse into `HamsterPartDefinition` structs
4. Load corresponding `.png` images as Bevy `Image` assets
5. Store in a `HamsterPartLibrary` resource
6. On hamster spawn, look up parts by name from library

**Rust struct** (in `engine/assets/definitions.rs`):
```rust
#[derive(serde::Deserialize)]
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
```

---

## Part 4: Lua FFI Specification

### 4.1 Rust Functions Exposed to Lua

| Function | Signature | Purpose |
|----------|-----------|---------|
| `set_corruption` | `fn(f32) → ()` | Update hamster corruption level |
| `set_expression` | `fn(String) → ()` | Change expression (e.g., "happy", "angry", "corrupted") |
| `get_corruption` | `fn() → f32` | Read current corruption |
| `log` | `fn(String) → ()` | Debug output |
| `wait_frames` | `fn(u32) → async` | Yield for N frames (async in Lua) |
| `play_sound` | `fn(String) → ()` | Trigger sound effect |

### 4.2 Lua Script Interface

**`assets/scripts/hamster_dialogue.lua`** (example):
```lua
function init()
    print("Hamster initialized")
end

function on_key_press(key)
    if key == "A" then
        -- Nice choice
        set_corruption(math.max(0, get_corruption() - 10))
        set_expression("happy")
    elseif key == "D" then
        -- Mean choice
        set_corruption(math.min(100, get_corruption() + 10))
        set_expression("angry")
    end
end
```

### 4.3 Hot-Reload Mechanism

Script hot-reload flow:
1. Watch `assets/scripts/` directory for changes
2. On file write, reload script into Lua VM
3. Call `init()` function again to re-initialize state
4. **Preserve** Rust-side game state (corruption, expression, etc.) across reload

**Implementation location**: `engine/scripting/hot_reload.rs`

---

## Part 5: Shader Strategy

### 5.1 Shader Files & Location

All shaders in `games/dev/doomexe/assets/shaders/`:

**`palette_swap.wgsl`**: Palette index lookup shader
- Input: sprite texture + palette texture + corruption level
- Output: sampled color with palette shift based on corruption

**`crt_postprocess.wgsl`**: Post-processing pass
- Inputs: scene texture, corruption level, time
- Effects: scanlines, vignette, chromatic aberration intensity scales with corruption

### 5.2 Shader Uniforms

```rust
pub struct CorruptionUniforms {
    pub corruption: f32,    // 0.0 – 1.0 (normalized)
    pub time: f32,          // For animated effects
    pub palette_shift: i32,  // Which palette variant (0 = default, 1 = corrupted)
}
```

### 5.3 Loading & Caching

- Shaders should be loaded by Bevy's `AssetServer` at startup
- Create `Bevy` materials (`StandardMaterial` or custom) that reference loaded shader handles
- Store shader handles in a resource: `pub struct ShaderHandles { crt: Handle<Shader>, palette: Handle<Shader> }`

---

## Part 6: Error Handling Strategy

### 6.1 Error Types

Create `engine/src/error.rs`:
```rust
pub type Result<T> = std::result::Result<T, DJEngineError>;

#[derive(Debug, thiserror::Error)]
pub enum DJEngineError {
    #[error("Asset loading failed: {0}")]
    AssetLoadError(String),
    
    #[error("Lua error: {0}")]
    LuaError(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderError(String),
    
    #[error("Animation error: {0}")]
    AnimationError(String),
}
```

### 6.2 Policy

- **Engine code**: Use `Result<T>` for recoverable errors
- **Game code**: Can panic on unrecoverable startup errors
- **Lua calls**: Wrap Lua errors, log, and gracefully degrade
- **Asset loading**: If a non-critical asset fails, log warning and continue; if critical, error and exit

---

## Part 7: Milestone 1 Scope Definition

### 7.1 Deliverables (MVP)

**By end of Milestone 1:**

✅ Hamster appears on screen, assembled from 5+ sprite parts  
✅ Breathing animation (sine-wave body scale)  
✅ Blinking eye animation (timer-driven)  
✅ Idle head motion (noise-based jitter)  
✅ Corruption slider (key-driven) changes palette + CRT intensity  
✅ Lua script can change expression and corruption without restart  
✅ CRT post-processing visible (scanlines, vignette)  
✅ Runs at 60 FPS  

### 7.2 Out of Scope (Milestone 1)

❌ Full dialogue system (just basic input → state mapping)  
❌ Save/load state  
❌ Multiple scenes  
❌ Paws animation  
❌ Particle effects  
❌ Sound/music  
❌ UI / menu system  
❌ Build system optimization  

### 7.3 Implementation Order

1. **Week 1: Scaffolding**
   - Set up all module skeletons in `engine/src/`
   - Create basic `AssetPlugin` with JSON loading
   - Verify Cargo workspace compiles

2. **Week 2: Rendering Foundation**
   - Implement offscreen render target (320×240)
   - Set up upscaling to window (nearest-neighbor)
   - Load and apply CRT shader

3. **Week 3: Hamster Assembly & Animation**
   - Load hamster parts from JSON metadata
   - Spawn hierarchical entity structure
   - Implement breathing (scale) + blinking + idle motion systems
   - Verify all animations blend smoothly

4. **Week 4: Corruption & Lua**
   - Implement palette swap shader with corruption uniform
   - Hook up Lua `set_corruption()` / `set_expression()` FFI
   - Basic input handling (keys A/D to trigger state changes)
   - Hot-reload script on file change

5. **Week 5: Polish & Stability**
   - Fix edge cases, optimize perf
   - Document all modules
   - Create example Lua script
   - Verify 60 FPS on target hardware

---

## Part 8: Dependency Versions & Stability

### 8.1 Pinned Versions (as of 2026-01-20)

```toml
[workspace.dependencies]
bevy = "0.14"              # Latest stable; DO NOT upgrade mid-milestone
mlua = { version = "0.9", features = ["lua54", "serialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
nalgebra = "0.32"          # Math/noise
noise = "0.8"              # Perlin noise for idle motion
```

### 8.2 Bevy Considerations

- Bevy 0.14 is heavy; initial builds take 2–3 minutes on modest hardware
- Consider separating `bevy` feature flags to reduce compile time during development:
  - Use `bevy/dynamic_linking` for debug builds
  - Use `bevy/render` selectively if headless mode is tested

---

## Part 9: Testing & Build Automation

### 9.1 Test Structure

```
engine/src/
  lib.rs
  rendering/
    mod.rs
    #[cfg(test)] tests.rs
  animation/
    mod.rs
    #[cfg(test)] tests.rs
games/dev/doomexe/src/
  main.rs
  #[cfg(test)] integration_tests.rs
```

### 9.2 CI Pipeline (Recommended)

Create `.github/workflows/build.yml`:
```yaml
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo check --workspace
      - run: cargo test --workspace
      - run: cargo clippy --workspace -- -D warnings
```

### 9.3 Local Development Commands

```bash
# Full rebuild
cargo build --release -p doomexe

# Run with logging
RUST_LOG=debug cargo run -p doomexe

# Check all crates without building
cargo check --workspace

# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --all -- --check
```

---

## Part 10: Documentation Standards

### 10.1 Module Documentation

Every public module should have a doc comment:

```rust
//! Hamster-specific rendering and assembly logic.
//!
//! This module provides the `HamsterNarrator` component and systems
//! for procedurally assembling hamster rigs from sprite parts, applying
//! transformations, and rendering to offscreen textures.
//!
//! # Example
//!
//! ```ignore
//! let mut world = World::new();
//! world.init_resource::<HamsterPartLibrary>();
//! spawn_hamster(&mut world, "assets/sprites/hamster_parts/");
//! ```
```

### 10.2 Design Docs Location

Create `docs/` directory at repo root:
```
docs/
├── ARCHITECTURE.md          # High-level system design
├── RENDERING_PIPELINE.md    # Low-level rendering flow
├── LUA_FFI.md               # Lua ↔ Rust boundary
├── ASSET_PIPELINE.md        # How assets flow from Aseprite to game
└── MILESTONE_1.md           # This milestone's spec (expanded)
```

---

## Part 11: Next Steps (Immediate Actions)

### Before Writing Any Game Code:

1. **✅ Create engine module skeleton**
   - Add `rendering/`, `animation/`, `scripting/`, `assets/` directories
   - Write empty `mod.rs` files with public trait definitions
   - This takes 1–2 hours and prevents major refactors later

2. **✅ Pin workspace Cargo.toml**
   - Ensure all crates reference workspace-level dependencies
   - Commit `Cargo.lock` to repo

3. **✅ Set up CI/testing scaffold**
   - Create GitHub Actions workflow (or local test script)
   - Ensure `cargo check --workspace` passes

4. **✅ Write Asset Pipeline spec** (expanded from Part 3)
   - Define exactly how Aseprite JSON metadata looks
   - Create example files for testing

5. **✅ Prototype Lua FFI minimal implementation**
   - Standalone test: load Lua file, call Rust function from Lua
   - Verify `mlua` compiles with chosen features

6. **❌ DO NOT YET:**
   - Implement full animation system
   - Write shaders
   - Build hamster spawn logic
   - These come after the scaffolding is solid

---

## Part 12: Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Bevy version mismatch between engine + game | Build failure | Pin workspace deps, test early |
| Lua ↔ Rust boundary undefined | Refactor churn | Spec FFI now, implement incrementally |
| Asset pipeline too complex | Timeline slip | Start with dummy JSON, iterate |
| Shader compilation errors on different hardware | Release blocker | Test shaders early, provide fallback |
| Performance degradation from corruption effects | Unplayable at high corruption | Profile at 60 FPS target early |
| Hot-reload breaks game state | Frustrating debugging | Preserve state on reload, test extensively |

---

## Part 13: Success Metrics

**Milestone 1 is complete when:**

| Criterion | Measure | Pass/Fail |
|-----------|---------|-----------|
| Compilation | `cargo build -p doomexe` succeeds | ✅ |
| Frame rate | 60 FPS sustained on reference hardware | ✅ |
| Hamster visibility | 5+ sprite parts visible, correctly positioned | ✅ |
| Animation | Breathing, blinking, idle motion all smooth | ✅ |
| Corruption effect | Visual change noticeable at 0%, 50%, 100% corruption | ✅ |
| Lua integration | Script can modify hamster state from keypresses | ✅ |
| Code quality | No clippy warnings, >80% doc coverage on public APIs | ✅ |
| Deliverable | Single-scene prototype, runs standalone | ✅ |

---

## Conclusion

**Your AI's vision is solid.** The project is well-scoped and the first game (doomexe) is an excellent vertical slice to build the engine around. However, **execution requires concrete scaffolding**—especially around module boundaries, asset pipelines, and Lua FFI.

**The next 1–2 weeks should focus entirely on Part 1–9 of this plan:**
- Set up module structure
- Finalize asset pipeline spec
- Prototype Lua FFI
- Establish CI/testing

**Only then should implementation sprints begin.** This upfront investment will save 10+ hours of refactoring down the road.

---

**Questions to discuss:**
1. Are you okay deferring full dialogue until Milestone 2?
2. Do you have reference hardware specs for performance targets?
3. Should save/load state be in Milestone 2 or later?
4. Will doomexe have multiple games on the engine, or is this a one-game prototype for now?
