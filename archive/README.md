
***

## 3. `engine/README.md`

```markdown
# DJ Engine – Core

This directory will eventually host the reusable engine crates that doomexe and future games share.

---

## Planned Crates / Modules

### `engine/rendering`

- Wgpu/Bevy‑oriented helpers for:
  - Fixed‑resolution offscreen targets (e.g., 320×240).
  - Palette‑indexed image formats and palette texture management.
  - CRT / glitch / scanline post‑processing passes.
- APIs to:
  - Register post‑processing shaders.
  - Feed corruption/intensity parameters to shaders in a generic way.

### `engine/animation`

- Components and systems for:
  - Hierarchical transforms (attach parts to parents).
  - Procedural animation curves (sine, noise, envelopes).
  - Squash & stretch with area preservation.
  - Idle motion driven by Perlin/simplex noise.

Designed so doomexe can define a “hamster rig” but other games can define different rigs with the same tools.

### `engine/scripting`

- Wrapper around `mlua` for:
  - Loading and sandboxing Lua scripts.
  - Registering Rust functions and types into Lua.
  - Hot‑reload of scripts while preserving Rust‑side game state.
- Utilities for:
  - Script error reporting and diagnostics.
  - Linking scripts to entities (e.g., “this NPC uses script X”).

### `engine/assets`

- Asset loading/pipeline utilities:
  - Aseprite → JSON parsing and conversion to Rust structs.
  - Palette definitions (JSON) → GPU textures.
  - Config/metadata parsing for procedural rigs, easing curves, etc.
- Build‑time support:
  - Optional `build.rs` helpers to pre‑bake atlases or look‑up tables.

### `engine/rpg` (later)

- JRPG‑flavored systems:
  - Stats, status effects, damage formulas.
  - Inventory & equipment.
  - Turn‑based combat flow.
- Designed to be independent of doomexe’s specific story.

### `engine/net` (much later / RTS phase)

- ECS‑friendly networking patterns.
- Entity replication, client prediction, rollback simulation.

---

## Design Constraints

- No direct doomexe dependencies here; only generic data structures and logic.
- Prefer **config‑driven** APIs so games can define behavior in data/Lua rather than Rust enums hardcoded in the engine.
- Keep crates focused and composable—games may not use every module.

During early development, expect many systems to live inside `games/dev/doomexe` and be migrated here once they stabilize.
