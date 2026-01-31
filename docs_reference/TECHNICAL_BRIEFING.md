# DJ Engine Technical Briefing

**Version:** 0.1.0 (Dev)
**Date:** 2026-01-24

## 1. Project Overview
DJ Engine is a custom game engine built in Rust using the `Bevy` game engine framework. It features a comprehensive editor (`./dj e`) built with `bevy_egui`. The engine is designed to support role-playing games with complex branching narratives (Story Graphs), 2D grid-based maps (Feature Grid), and campaign management.

## 2. Architecture

### Core Stack
- **Languages:** Rust
- **Framework:** Bevy 0.15+ (ECS architecture)
- **UI:** `bevy_egui` (Immediate Mode GUI)
- **Serialization:** `serde` + `serde_json`

### Key Directories
- **`engine/src/data/`**: The Data Layer. All serializable data structures live here.
    - `project.rs`: Top-level project and settings definitions (`EngineSettings`, `EditorPreferences`).
    - `story.rs`: Story Graph data models (`StoryNodeData`, `StoryGraphData`).
    - `loader.rs`: Utility functions for loading/saving JSON assets.
- **`engine/src/editor/`**: The Editor Layer.
    - `state.rs`: Runtime state resources (`EditorUiState`, `ActiveStoryGraph`).
    - `ui/views.rs`: Main UI drawing logic (Settings, Story Graph editor).
    - `systems.rs`: Bevy systems handling logic updates (window resizing, input).
- **`engine/tests/`**: Integration Tests.
    - `headless_tests.rs`: Minimal/Headless tests for CI/Verification.

## 3. Key Components & Implementation Details

### A. Settings Persistence
*   **Location:** `~/.dj_engine/settings.json` (Linux/Mac) or `%USERPROFILE%/.dj_engine/settings.json` (Windows).
*   **Implementation:** 
    *   Defined in `src/data/project.rs` as `EngineSettings`.
    *   Loaded on startup in `editor/mod.rs`.
    *   Auto-saved on modification in `src/editor/ui/views.rs`.
    *   System `apply_window_settings_system` applies changes (resolution, mode) to the window logic.

### B. Story Graph System
*   **Data Model:** Nodes (`StoryNodeData`) connected by IDs. Supports `Start`, `Dialogue`, `Choice`, `SubGraph`, etc.
*   **Validation:** Graphs can be validated for broken links, dead ends, and missing entities.
*   **Serialization:** Saved as JSON. `StoryNodeVariant` uses internally tagged enum (`type` field).
*   **Editor View:** `draw_story_graph` in `src/editor/ui/views.rs`.

### C. Headless Testing
*   **Goal:** Verify game logic (graph loading, grid population) without a GPU/Window.
*   **Strategy:** 
    *   Use `MinimalPlugins` + `ScheduleRunnerPlugin`.
    *   Avoid adding `EditorPlugin` or `EguiPlugin` directly as they require renderer resources.
    *   Manually initialize required resources (`EditorUiState`, `FeatureGrid`) in the test.
*   **File:** `engine/tests/headless_tests.rs`.

## 4. Current Status & Deliverables

### Branching Dialogue (New Horizon)
*   **Test Game:** `games/dev/new_horizon/story_graphs/test_game.json`
*   **Content:** A branching dialogue script verifying `Start` -> `SubGraph` -> `Choice` flow.
*   **Verification:**
    *   **Manual:** In Editor -> Story Graph View -> Click "📂 Load Test Game" toolbar button.
    *   **Automated:** `cargo test --test headless_tests`

### Known Issues / Notes
1.  **CLI Usage**: The engine is launched via `./dj e`. Commands like `--view story` are arguments to the binary, not REPL commands.
    *   Correct: `./dj e --view story` (if implemented in `main.rs`/`editor/mod.rs`)
    *   Incorrect: Typing `--view story` into a running `dj>` prompt (this prompt is likely from a test runner or custom shell wrapper).
2.  **"Initial Node Only"**: If the Story Graph view only shows a start node, it means the graph is empty or load failed. Check the console log for "Failed to load" errors. Ensure the "Load Test Game" button was essentially clicked.

## 5. Instructions for Next Agent
1.  **Verify Graph Loading**: The user reported seeing only the initial node. Debug `draw_story_graph` or `loader` pathing if "Load Test Game" fails.
    *   Check if `std::path::PathBuf` in `views.rs` is correct relative to the binary execution path.
2.  **Improve CLI**: If `--view` is confusing, implementing a proper CLI argument parser in `main.rs` using `clap` is recommended to select the initial view state directly.
3.  **Visual Confirmation**: Ensure the `ActiveStoryGraph` resource update triggers a UI refresh properly.

## 6. How to Run
*   **Editor:** `./dj e`
*   **Tests:** `cargo test`
*   **Headless Tests:** `cargo test --test headless_tests`

*   **Headless Tests:** `cargo test --test headless_tests`

*   **Headless Tests:** `cargo test --test headless_tests`

## 7. Product Ecosystem & Vision
Detailed specifications are available in the `docs/` suite.

### A. Design Philosophy: Parallel Feature Engine (`docs/VISION_CORE.md`)
DJ Engine pivots the workflow from "building one game" to **"testing multiple futures"**. The core innovation is the **Branching Dashboard**, where designers create "Test Planes" to verify feature branches (e.g., "Branch 5: New Physics") in parallel.

### B. Visual Branching (`docs/SYSTEM_VISUAL_VERSIONING.md`)
The primary interface is the **Node Bus**.
*   **CORE:** The stable heart.
*   **Test Planes:** Isolated sandboxes for experimentation.
*   **Main Bridge:** The merge interface for committing verified features.
*   **BOM:** Live data stream manifests.

### C. Supporting Systems
*   **Visual Logic (`docs/SYSTEM_STORY_GRAPH.md`):** State machine graphs for narrative flow.
*   **Spatial Logic (`docs/SYSTEM_FEATURE_GRID.md`):** Layered logical topology.

## 8. Development Roadmap
1.  **Refine CLI:** Implement strict argument parsing.
2.  **Branching UI:** Implement the visual graph (CORE + Nodes) as seen in the mockups.
3.  **Headless Runner:** Ensure `cargo test` can target specific "Test Planes" via ID.
