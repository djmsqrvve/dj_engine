# DJ Engine - AI Summary

## Project Overview
**DJ Engine** is a custom game framework in Rust (using Bevy) for building "cursed" narrative text adventures.
**Doomexe** is the first game being built on this engine (in `games/dev/doomexe`).

## Current State (2026-01-20)
- **Repository**: Configured as a Cargo Workspace.
- **Engine**: `engine/` crate initialized (currently empty lib).
- **Game**: `games/dev/doomexe/` project initialized.
- **Legacy**: Old web prototype moved to `games/dev/doomexe/legacy_web_prototype`.

## Directory Structure
```
/dj_engine
├── Cargo.toml          # Workspace root
├── engine/             # Core shared library
├── tools/              # (Planned) Asset tools
└── games/dev/doomexe/  # The Game
    ├── Cargo.toml      # Game-specific dependencies (Bevy, Mlua)
    ├── assets/         # Shaders, Scripts, Sprites
    └── src/
        ├── main.rs     # Entry point
        ├── hamster/    # (Planned) Procedural hamster logic
        └── scripting/  # (Planned) Lua integration
```

## Immediate Goals (Milestone 1)
The current focus is **Milestone 1: Hamster Narrator**.
1.  **Procedural Assembly**: Assemble hamster faces from sprite parts.
2.  **Rendering**: Low-res offscreen render + CRT shader.
3.  **Lua Scripting**: Basic dialogue flow.

## Build Status
- `cargo check` currently failing/passing (See terminal output).
- Ensure `bevy` and `mlua` dependencies compile correctly on the target environment.
