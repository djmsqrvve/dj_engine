
***

## 2. `games/dev/doomexe/README.md`

```markdown
# doomexe

doomexe is a dark‑fantasy horror JRPG prototype built on **DJ Engine**. It features a corrupted hamster narrator that reacts to player choices, remembers previous runs, and visually degrades as the story darkens.

For now, doomexe is primarily a **sandbox for developing DJ Engine**. It will eventually become a full game.

---

## Milestone 1 – Hamster Narrator

**Goal:** A single animated scene with a procedural hamster like the concept image: a large, candle‑lit, slightly corrupted hamster portrait delivering text.

### Requirements

- **Procedural hamster assembly**
  - Parts: body, head, left/right ear, left/right eye variants, mouth, left/right paw.
  - Each part is a sprite exported from Aseprite with position metadata.
  - At runtime, parts are attached as entities/children and composed visually.

- **Animation**
  - Breathing: squash & stretch on body, with area (volume) roughly preserved.
  - Blinking: eye changes with a timer (closed/open frames).
  - Idle motion: slight head sway and/or jitter using noise.
  - Gesture hooks: expressions for “neutral”, “amused”, “angry”, “corrupted”, etc.

- **Corruption system**
  - Single `corruption: f32` field (0–100).
  - Corruption influences:
    - Palette shift (colors become harsher/“wrong”).
    - Screen jitter / scanline intensity.
    - Possibly slight warping of the portrait.

- **Rendering**
  - Internal resolution: e.g. 320×240.
  - Render to offscreen texture; upscale with nearest neighbor.
  - Post‑processing pass for CRT (scanlines, vignette, chromatic aberration).

- **Lua scripting**
  - Minimal Lua file:
    - Chooses expression and corruption changes when a “choice” is taken.
    - Example: keypress triggers a different script branch.

- **Hot‑reload**
  - Lua scripts in `assets/scripts/` can be edited and reloaded at runtime.
  - Palette configuration in JSON can be reloaded at runtime.

---

## Project Layout (doomexe)

```text
games/dev/doomexe/
  Cargo.toml
  README.md

  assets/
    sprites/
      hamster_parts/      # Aseprite‑exported layers
    palettes/
      hamster_default.json
      hamster_corrupted.json
    shaders/
      hamster_palette.wgsl
      crt_postprocess.wgsl
    scripts/
      hamster_dialogue.lua

  src/
    main.rs

    hamster/
      mod.rs              # Plugin registration
      components.rs       # HamsterNarrator, HamsterPart, etc.
      systems.rs          # Animation / corruption / input
      render.rs           # Offscreen rendering + CRT pass

    scripting/
      mod.rs              # Lua integration via mlua

    assets/
      mod.rs              # Load Aseprite JSON & palettes
