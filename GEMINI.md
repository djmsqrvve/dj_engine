# dj_engine Project Context

## Overview
**dj_engine** is a formally specified, ambitious game engine project built on **Rust** and **Bevy 0.15**. It is designed to create "cursed" narrative-heavy games with procedural 2D character animation and palette-driven visual effects.

The project is currently in **Milestone 1 (Hamster Narrator)** of a rigorous **5-Phase Development Plan**. It involves a custom Bevy-based engine (`dj_engine`), a primary game target (`doomexe`), and an extensive suite of asset tools.

## Key Directories

*   **`docs/`**: **THE SOURCE OF TRUTH.** Contains ~15,000 lines of detailed specifications.
    *   `MASTER-DOCUMENTATION-INDEX.md`: The entry point for all documentation.
    *   `complete-detailed-docs.md`: Contains code templates and detailed implementation guides for Phases 0-2.
    *   `Game_Engine_Technical_Roadmap.md`: The comprehensive 20-week execution plan.
    *   `Implementation_Summary.md`: A summary of the current implementation state.
*   **`engine/`**: The core shared library.
    *   `src/animation`: Procedural animation system (breathing, blinking).
    *   `src/scripting`: Lua integration (`mlua`) for game logic.
    *   `src/story_graph`: JSON-serializable node system for narrative branching.
    *   `src/data`: Serialization types (Projects, Scenes, Database).
    *   `src/rendering`: Custom rendering pipeline (offscreen target -> upscaling -> post-processing).
*   **`games/`**:
    *   `dev/doomexe`: The flagship game. A JRPG/Visual Novel hybrid featuring a "Hamster" narrator and corruption mechanics.
*   **`tools/`**:
    *   `asset_generator`: Rust-based tools for asset processing.
*   **`archive/`**: Contains historical context, previous iterations (`AI_SUMMARY.md`), and legacy prototypes.
    *   `games/dev/doomexe/legacy_web_prototype`: A React/Vite prototype of the game concept.

## Architecture & Concepts

### The "Three Pillars"
1.  **Universal Unit/Actor:** A standardized ECS entity structure (`Actor`, `Stats`) designed to work across genres (JRPG & RTS).
2.  **Story Graph:** A JSON-serializable node system for branching narratives, driven by a `StoryDirector` and executed via Lua.
3.  **Lua Scripting:** Extensive FFI layer allowing high-level game logic and cutscenes to be written in Lua without recompiling Rust.

### Visual Style
*   **Procedural Animation:** Characters are assembled from sprite parts (eyes, mouth, body) and animated procedurally (sine waves for breathing, timers for blinking).
*   **Corruption:** Visual effects driven by palette manipulation and shader-based distortion.
*   **Retro Aesthetic:** Low-resolution internal rendering scaled up with CRT effects.

## Development Workflow

### Building & Running
*   **Run the Game (`doomexe`):**
    ```bash
    cargo run -p doomexe
    ```
*   **Run Tests:**
    ```bash
    cargo test --workspace
    ```
*   **Build Release:**
    ```bash
    cargo build --release
    ```

### Code Conventions
*   **Specification-First:** Changes should align with the documentation in `docs/`.
*   **Bevy 0.15:** Uses modern Bevy features like `#[require(Component)]` and Observers.
*   **Workspace:** All code is organized in a Cargo Workspace.

## Important References
*   **`docs/MASTER-DOCUMENTATION-INDEX.md`**: Start here to navigate the massive documentation library.
*   **`docs/complete-detailed-docs.md`**: Contains the "Phase 0-2" implementation details and code templates.
*   **`engine/src/lib.rs`**: Core engine entry point.