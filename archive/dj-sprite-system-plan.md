# DJ Engine Sprite System – Full Implementation Plan

**Last Updated:** January 20, 2026  
**Purpose:** Complete phased rollout for integrating a professional sprite authoring, asset management, and runtime system into DJ Engine.

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Requirements & Constraints](#requirements--constraints)
3. [Core Data Model](#core-data-model)
4. [Phase 1: Foundation (Runtime & Data Structures)](#phase-1-foundation-runtime--data-structures)
5. [Phase 2: Asset Pipeline (Import & Build)](#phase-2-asset-pipeline-import--build)
6. [Phase 3: Editor Tooling](#phase-3-editor-tooling)
7. [Phase 4: Advanced Features](#phase-4-advanced-features)
8. [Reference Style Guide](#reference-style-guide)
9. [File Format Specifications](#file-format-specifications)
10. [Testing & Validation](#testing--validation)
11. [Deployment Checklist](#deployment-checklist)

---

## Executive Summary

The **DJ Sprite System** establishes a complete workflow for creating, managing, and rendering sprite-based assets in DJ Engine. It supports:

- **Authoring:** Pixel-perfect sprite slicing, frame definition, and animation composition.
- **Management:** Centralized sprite atlases with deterministic packing and hot-reload support.
- **Runtime:** Efficient rendering, animation playback, and component-based animation control.
- **Tooling:** Integrated editor for real-time preview, metadata editing, and asset validation.

**Target Outcome:** Artists can drop a PNG into a folder, run the sprite editor, define frames and animations, and have game-ready assets in minutes. The same workflow scales to production with thousands of sprites.

**Estimated Timeline:** 12–16 weeks (4 phases, 2–4 weeks each).

---

## Requirements & Constraints

### Functional Requirements

| ID | Requirement | Priority |
|---|---|---|
| FR-1 | Load PNG/sprite sheet textures from project folders | P0 |
| FR-2 | Slice images into sprite frames (grid + manual) | P0 |
| FR-3 | Define animations as frame sequences with duration | P0 |
| FR-4 | Per-frame metadata: pivot, collision boxes, tags, events | P0 |
| FR-5 | Pack sprites into atlases with configurable settings | P0 |
| FR-6 | Export compact metadata (`.djatlas.json`, `.djanims.json`) | P0 |
| FR-7 | Live animation preview in DJ's renderer | P0 |
| FR-8 | Hot-reload sprite/animation data during dev | P1 |
| FR-9 | Sprite search and filtering in editor | P1 |
| FR-10 | Collision/hitbox visualization | P1 |

### Non-Functional Requirements

| ID | Requirement | Priority |
|---|---|---|
| NF-1 | Cross-platform (Windows, Mac, Linux) | P0 |
| NF-2 | No external runtime dependencies beyond DJ core | P0 |
| NF-3 | Scales to 3000+ sprites and 500+ animations | P1 |
| NF-4 | Deterministic builds (same input → same output) | P0 |
| NF-5 | Build time < 5 seconds for typical projects | P1 |
| NF-6 | Editor responsiveness: all interactions < 200ms | P2 |

### Constraints

- **Target Pixel Art Style:** 32×32 to 256×256 sprites, up to 8-bit palette reference.
- **Atlas Size Limit:** 2048×2048 or 4096×4096 depending on target platform.
- **Animation Frame Limit:** 256 frames per animation (soft).
- **Metadata Format:** JSON for human-readability and tooling flexibility.

---

## Core Data Model

### Runtime Structures (in pseudocode)

```cpp
// Sprite definition (immutable, loaded from .djatlas.json)
struct SpriteDef {
    String id;                      // "characters/hero_idle_0"
    AtlasHandle atlas;
    Rect2i rect;                    // in atlas pixels
    Vec2 pivot;                     // origin point (normalized 0–1 or pixel coords)
    Array<String> tags;             // ["hero", "idle"]
    Array<CollisionBox> collision_boxes;
    Map<String, Any> custom_data;   // extensible metadata
};

// Sprite in the world
struct SpriteComponent {
    SpriteHandle sprite;
    Color tint;
    bool flip_x, flip_y;
    int layer;
    Vec2 scale;
};

// Animation definition (loaded from .djanims.json)
struct AnimationDef {
    String id;                      // "characters/hero_idle"
    Array<AnimationFrame> frames;
    PlaybackMode mode;              // Loop, Once, PingPong
    float fps;                       // default playback speed
    Array<AnimationEvent> events;
};

struct AnimationFrame {
    String sprite_id;
    float duration;                 // 0 = use fps; > 0 overrides fps
};

struct AnimationEvent {
    float time;                     // relative to animation start
    String name;
    Map<String, Any> payload;
};

// Animation controller
struct SpriteAnimationComponent {
    AnimationHandle animation;
    float time;                     // elapsed time in current playback
    int current_frame_index;
    bool playing;
    float speed;                    // playback speed multiplier
    
    // Linked reference for rendering
    SpriteComponent* sprite_component;
};

// Atlas (manages loaded sprites)
struct SpriteAtlas {
    String id;                      // "characters", "fx", "ui"
    TextureHandle texture;
    Map<String, SpriteDef> sprites;
};

// Global manager
struct SpriteAtlasManager {
    Map<String, SpriteAtlas> atlases;
    Map<String, AnimationDef> animations;
    
    SpriteHandle get_sprite(const String& id);
    AnimationHandle get_animation(const String& id);
    void load_atlas(const String& path);
    void hot_reload(const String& atlas_id);
};
```

---

## Phase 1: Foundation (Runtime & Data Structures)

**Duration:** 2–3 weeks  
**Goal:** Establish runtime systems so sprites can be loaded, rendered, and animated.

### 1.1 Define Component Architecture

- [ ] Create `SpriteComponent` and `SpriteAnimationComponent` classes.
- [ ] Integrate with DJ's existing entity/component system.
- [ ] Define serialization format (if entities are saved to disk).

**Deliverables:**
- Header files with component definitions.
- Component serialization/deserialization functions.

### 1.2 Implement SpriteAtlasManager

- [ ] Build `SpriteAtlasManager` singleton.
- [ ] Load `.djatlas.json` metadata files.
- [ ] Index sprites by string ID (e.g., `"characters/hero_idle_0"`).
- [ ] Support multiple atlases with collision detection (no ID collisions).

**Deliverables:**
- Functional `SpriteAtlasManager` class.
- Unit tests for sprite lookup and atlas loading.
- Example `.djatlas.json` files for testing.

### 1.3 Implement SpriteRenderSystem

- [ ] Create `SpriteRenderSystem` that iterates `SpriteComponent`s each frame.
- [ ] Render sprites with correct texture, UV coordinates, pivot, scale, tint, and layer sorting.
- [ ] Leverage existing DJ renderer for quad batching and shader support.
- [ ] Support camera transforms, parallax, and depth ordering.

**Deliverables:**
- Functional render system.
- Integration test with a simple scene containing 10+ sprites.

### 1.4 Implement SpriteAnimationSystem

- [ ] Create `SpriteAnimationSystem` to update animation state.
- [ ] Advance `time` by `delta_time * speed`.
- [ ] Compute current frame index based on `time`, `fps`, and `playback_mode`.
- [ ] Trigger animation events at appropriate timestamps.
- [ ] Link `SpriteAnimationComponent` to `SpriteComponent` for rendering.

**Deliverables:**
- Functional animation update system.
- Unit tests for frame advance logic (loop, once, ping-pong).
- Test animation with multiple speeds and speeds multipliers.

### 1.5 Dummy Asset Setup

- [ ] Create hand-crafted `.djatlas.json` and `.djanims.json` files.
- [ ] Use the hamster example sprite (or similar) to populate test data.
- [ ] Verify all Phase 1 systems work with manually-authored metadata.

**Deliverables:**
- Test assets in `assets/sprites/test/`.
- Example files demonstrating all metadata features.
- A playable demo scene with animated sprites.

### 1.6 Documentation

- [ ] Write runtime API documentation (component usage, system lifecycle).
- [ ] Create integration guide for existing DJ systems (renderer, entity manager).

**Deliverables:**
- `RUNTIME_API.md` in repo.

---

## Phase 2: Asset Pipeline (Import & Build)

**Duration:** 3–4 weeks  
**Goal:** Automate sprite import, slicing, and atlas packing so artists don't manually author JSON.

### 2.1 Image Loader & Texture Utilities

- [ ] Implement image decoder (PNG support as minimum).
- [ ] Create texture buffer classes (RGBA, indexed color support).
- [ ] Build flip/scale/rotate utilities for sprite manipulation.

**Deliverables:**
- Image loading library (wrapper around stb_image or similar).
- Unit tests with sample PNGs.

### 2.2 Sprite Slicing Engine

- [ ] **Grid Slicing:** Given cell size, margin, offset, auto-generate frame rects.
- [ ] **Manual Slicing:** Parse user-defined frame boundaries (could be stored in a `.slicing.json` or metadata).
- [ ] **Metadata Extraction:** Support Aseprite JSON export format (optional but recommended).

**Deliverables:**
- Slicing functions with configurable parameters.
- Test cases with sample sheets at different grid sizes.

### 2.3 Atlas Packing Algorithm

- [ ] Implement or integrate bin-packing algorithm (e.g., guillotine or skyline).
- [ ] Support configurable atlas size, padding, power-of-two constraints.
- [ ] Minimize wasted space; prioritize read performance.

**Deliverables:**
- Packing function that takes sprite rects and outputs packed result.
- Utility to visualize packing (for debugging and validation).

### 2.4 Texture Composition

- [ ] Render packed sprites onto atlas texture(s).
- [ ] Handle transparency and edge bleeding (dilate borders or padding).
- [ ] Output final atlas PNG.

**Deliverables:**
- Texture composition and export functions.
- Quality checks (no overlaps, valid UVs).

### 2.5 Metadata Exporter

- [ ] Generate `.djatlas.json` from sliced sprites and packed atlas.
- [ ] Generate `.djanims.json` from animation definitions (initially manual, later from editor).
- [ ] Validate all sprite IDs, animation references, and event handlers.

**Deliverables:**
- Export functions for both formats.
- Validation report (warnings for unused sprites, undefined references).

### 2.6 Build System Integration

- [ ] Add a **Sprite Build Step** to DJ's asset pipeline.
- [ ] Detect changed source images and re-pack only affected atlases.
- [ ] Output to `build/assets/atlases/` and `build/assets/metadata/`.
- [ ] Implement headless CLI mode for CI/CD.

**Deliverables:**
- Build step callable from DJ's main build system.
- CLI tool for command-line sprite compilation.
- Integration test: build demo project, verify output.

### 2.7 Project Configuration

- [ ] Define `.djspriterc.json` or similar for project-level sprite settings.
- [ ] Configure atlas groups, size limits, padding, packing algorithm.
- [ ] Define source folders and corresponding output locations.

**Example Config:**
```json
{
  "version": "1.0",
  "atlases": [
    {
      "id": "characters",
      "sources": ["assets/sprites/characters/"],
      "max_size": [2048, 2048],
      "padding": 2,
      "algorithm": "skyline"
    },
    {
      "id": "fx",
      "sources": ["assets/sprites/fx/"],
      "max_size": [1024, 1024],
      "padding": 1
    }
  ]
}
```

**Deliverables:**
- Configuration schema and loader.
- Default template config.

### 2.8 Testing & Validation

- [ ] Automated tests for slicing, packing, and export accuracy.
- [ ] Test with real-world sprite sheets (e.g., various grid sizes, irregular layouts).
- [ ] Performance benchmarks (build time for 100, 500, 1000+ sprites).

**Deliverables:**
- Test suite with sample assets.
- Performance report.

### 2.9 Documentation

- [ ] Write asset pipeline guide: folder structure, naming conventions, slicing metadata.
- [ ] Provide examples and troubleshooting.

**Deliverables:**
- `ASSET_PIPELINE.md` in repo.

---

## Phase 3: Editor Tooling

**Duration:** 3–4 weeks  
**Goal:** Build a dedicated sprite editor so artists can slice, animate, and preview without leaving DJ.

### 3.1 Editor Application Scaffold

- [ ] Set up main editor window/application.
- [ ] Integrate with DJ's rendering context and input handling.
- [ ] Establish panel/dock system (ImGui, Qt, or your existing UI framework).

**Deliverables:**
- Editor executable that launches and displays an empty window.
- Panel system skeleton.

### 3.2 Project Browser Panel

- [ ] List sprite source folders and sprite groups.
- [ ] Show existing `.djatlas.json` and `.djanims.json` files.
- [ ] Context menu: create new sprite set, re-build atlas, rename, delete.
- [ ] File browser integration.

**Deliverables:**
- Project panel UI with file operations.
- Test with sample project structure.

### 3.3 Texture & Slicing Panel

- [ ] Display source image/sheet at variable zoom.
- [ ] Implement grid slicing tool:
  - Input: cell width, height, margin, offset.
  - Output: frame rects auto-labeled.
- [ ] Manual slicing tool (drag rectangles).
- [ ] Per-frame inspector:
  - Show/edit sprite ID.
  - Adjust pivot (visual widget + numeric input).
  - Define collision boxes (visual rect drawing + list editor).
  - Tag editor.

**Deliverables:**
- Full slicing panel UI.
- Frame inspector with pivot and collision editing.
- Test with sample sprite sheets.

### 3.4 Animation Panel

- [ ] List animations for selected sprite group.
- [ ] Timeline view:
  - Show frame sequence horizontally.
  - Drag frames to reorder.
  - Adjust per-frame duration or global FPS.
- [ ] Playback controls: play, pause, step, speed slider.
- [ ] Event editor: add/remove/edit events (name, time, payload).

**Deliverables:**
- Animation timeline UI.
- Event editor.
- Fully functional animation composition.

### 3.5 Preview Panel

- [ ] Render selected sprite or animation using DJ's renderer.
- [ ] Controls:
  - Play/pause animation.
  - Speed slider.
  - Flip X/Y toggles.
  - Background color picker.
  - Zoom slider.
- [ ] Show collision boxes overlay.
- [ ] FPS and timing display.

**Deliverables:**
- Preview rendering system integrated with Phase 1 systems.
- Full interactive preview panel.

### 3.6 Build & Export from Editor

- [ ] "Build Atlas" button: trigger Phase 2 build pipeline from editor UI.
- [ ] Progress bar and status messages.
- [ ] Display any warnings/errors (e.g., oversized atlas, ID collisions).

**Deliverables:**
- Build trigger and feedback UI.
- Error/warning reporting.

### 3.7 Hot Reload System

- [ ] Monitor asset files for changes.
- [ ] On change, reload `.djatlas.json` and `.djanims.json` into running game.
- [ ] Update `SpriteAtlasManager` and active animations in-place.
- [ ] Optional: visual feedback (flash or notification).

**Deliverables:**
- File watcher integrated with `SpriteAtlasManager`.
- Test with live game update scenario.

### 3.8 Search & Filtering

- [ ] Search bar to find sprites by ID or tag.
- [ ] Filter by atlas group or animation status.
- [ ] Quick-preview on hover.

**Deliverables:**
- Search and filter UI.

### 3.9 Documentation & UX

- [ ] Write editor user guide with screenshots.
- [ ] Provide keyboard shortcuts reference.
- [ ] In-editor tooltips and help text.

**Deliverables:**
- `EDITOR_GUIDE.md`.
- Tooltip system in editor.

---

## Phase 4: Advanced Features

**Duration:** 2–3 weeks  
**Goal:** Expand the system with polish, performance, and advanced authoring capabilities.

### 4.1 Sprite Variants & Skinning

- [ ] Support sprite variants (e.g., hero in different armor colors).
- [ ] Define variant mappings in metadata.
- [ ] Runtime variant selection.

**Deliverables:**
- Variant metadata schema and runtime support.

### 4.2 Advanced Animation Features

- [ ] Weighted layering (multiple sprite layers per animation frame).
- [ ] Sprite blend modes (additive, multiply, etc.).
- [ ] Animation blending (cross-fade between animations).

**Deliverables:**
- Blend mode support in animation system.
- Cross-fade interpolation.

### 4.3 Scriptable Animation Events

- [ ] Define event handlers (C++ lambdas or script callbacks).
- [ ] Fire events during animation playback (e.g., trigger sound, spawn particle).
- [ ] Event queue and callback system.

**Deliverables:**
- Event callback registration and dispatch system.
- Example scripts demonstrating event usage.

### 4.4 Sprite Editor Plugins

- [ ] API for third-party sprite tools (Aseprite plugins, external scripts).
- [ ] Direct export from Aseprite to `.djatlas.json`.
- [ ] JSON schema validation.

**Deliverables:**
- Plugin API documentation.
- Example Aseprite plugin.

### 4.5 Performance Optimization

- [ ] Sprite atlas compression (optional ASTC/BCn support).
- [ ] Adaptive batching: group by atlas, then by blend mode.
- [ ] Profiling integration: measure render time per sprite, per atlas.

**Deliverables:**
- Compression pipeline (optional).
- Profiling hooks.

### 4.6 Animation Graph Editor

- [ ] Visual editor for state machines (blend current animation → next animation based on conditions).
- [ ] Support parameter-driven transitions (e.g., `speed > 0 ? run : idle`).
- [ ] Condition editor (simple DSL or visual nodes).

**Deliverables:**
- Animation graph data structure and editor.
- Runtime state machine interpreter.

### 4.7 Batch Operations

- [ ] Bulk rename sprites/animations.
- [ ] Batch import from folder.
- [ ] Bulk validation and repair.

**Deliverables:**
- Batch operation UI and functions.

### 4.8 Metadata Versioning & Migration

- [ ] Version `.djatlas.json` and `.djanims.json` formats.
- [ ] Auto-migration between versions.
- [ ] Backward compatibility testing.

**Deliverables:**
- Version schema and migration functions.
- Test with multiple version transitions.

### 4.9 Statistics & Reporting

- [ ] Dashboard showing total sprites, animations, memory usage, atlas efficiency.
- [ ] Export reports (CSV, JSON) for asset auditing.

**Deliverables:**
- Statistics collection and reporting UI.

### 4.10 Documentation & Examples

- [ ] Advanced tutorials (variants, events, state machines).
- [ ] Best practices guide (atlas size, memory, performance).
- [ ] Troubleshooting FAQ.

**Deliverables:**
- `ADVANCED_USAGE.md`.
- Example project showcasing all features.

---

## Reference Style Guide

Based on the hamster sprite example provided, the DJ Sprite System should support:

### Art Style
- **Resolution:** 32×32 to 256×256 pixels (configurable per sprite).
- **Color Depth:** 8-bit palette or full RGBA.
- **Anti-aliasing:** Minimal (pixel-art aesthetic).
- **Transparency:** Full alpha support.

### Animation Characteristics
- **Frame Rate:** Variable FPS per animation (e.g., idle at 8 FPS, attack at 12 FPS).
- **Pose Transitions:** Smooth transitions between animation states via blending or state machines.
- **Visual Effects:** Fire/glow overlays, screen shake events, particle triggers.

### Example: Hamster Sprite Breakdown

Given the reference image (demon/hamster character):

```
Sprite ID: "enemies/hamster_boss"
Size: 128×128 pixels
Pivot: (64, 96) [lower-center]
Animations:
  - idle: frames 0–3, 8 FPS, loop
  - attack: frames 4–7, 12 FPS, once
  - hurt: frames 8–9, 10 FPS, once
Events:
  - attack: "firebreath" at t=0.2s
  - hurt: "scream" at t=0.0s
```

This demonstrates the full range of metadata the system should support.

---

## File Format Specifications

### `.djatlas.json` – Sprite Atlas Metadata

```json
{
  "version": "1.0",
  "id": "characters",
  "texture": "atlases/characters.png",
  "texture_width": 2048,
  "texture_height": 2048,
  "sprites": [
    {
      "id": "hero_idle_0",
      "rect": {
        "x": 0,
        "y": 0,
        "width": 32,
        "height": 32
      },
      "pivot": {
        "x": 16,
        "y": 24
      },
      "tags": ["hero", "idle"],
      "collision_boxes": [
        {
          "name": "hurtbox",
          "x": 4,
          "y": 8,
          "width": 24,
          "height": 20
        }
      ],
      "custom_data": {
        "material": "flesh",
        "priority": 1
      }
    }
  ]
}
```

### `.djanims.json` – Animation Definitions

```json
{
  "version": "1.0",
  "atlas_id": "characters",
  "animations": [
    {
      "id": "hero_idle",
      "frames": [
        {
          "sprite_id": "hero_idle_0",
          "duration": 0.125
        },
        {
          "sprite_id": "hero_idle_1",
          "duration": 0.125
        }
      ],
      "playback_mode": "Loop",
      "fps": 8,
      "events": [
        {
          "time": 0.0,
          "name": "on_loop",
          "payload": {}
        }
      ]
    }
  ]
}
```

### `.djspriterc.json` – Project Configuration

See Section 2.7 above for full schema.

---

## Testing & Validation

### Unit Tests

- Runtime component lifecycle and serialization.
- Animation frame advance logic (loop, once, ping-pong).
- Sprite lookup by ID and tag.
- Slicing algorithms (grid, manual).
- Atlas packing and collision detection.
- Metadata format parsing and validation.

### Integration Tests

- Load real assets, render them, advance animations, verify output.
- Multi-atlas scenarios (switching between atlases).
- Hot-reload during gameplay.
- Build pipeline end-to-end.

### Performance Tests

- Benchmark render time with 100, 500, 1000+ sprites.
- Measure memory footprint per sprite/animation.
- Build time for various project sizes.

### Manual Testing

- Artist workflow: import sprite sheet → slice → animate → preview → build.
- Editor responsiveness under load.
- Hotreload stability.

---

## Deployment Checklist

### Before Release

- [ ] All Phase 1–4 features complete and tested.
- [ ] No critical bugs.
- [ ] Performance targets met (< 5s build, < 200ms editor interactions).
- [ ] Cross-platform testing (Windows, macOS, Linux).
- [ ] Code review and documentation complete.
- [ ] Example project demonstrating all features.

### Release Steps

1. **Tag Release** (e.g., `sprite-system-v1.0`).
2. **Update Changelog** with new features, bug fixes, breaking changes.
3. **Deploy to Main Branch.**
4. **Announce** to team with usage guide and examples.
5. **Monitor** for issues and iterate on Phase 4 features based on feedback.

### Post-Release Support

- Bug fixes (patch releases).
- Performance optimizations (minor releases).
- Feature requests → Phase 4 expansion or Phase 5 (future).

---

## Summary Timeline

| Phase | Duration | Key Deliverables |
|---|---|---|
| 1 (Foundation) | 2–3 weeks | Runtime systems, components, basic rendering/animation |
| 2 (Pipeline) | 3–4 weeks | Image import, slicing, packing, metadata export, build integration |
| 3 (Editor) | 3–4 weeks | Full sprite editor UI, hot-reload, search |
| 4 (Advanced) | 2–3 weeks | Variants, animation blending, state machines, plugins, reporting |
| **Total** | **12–16 weeks** | Production-ready sprite system |

---

## Next Steps

1. **Prioritize:** Confirm which features are MVP (likely Phase 1–2) vs. nice-to-have (Phase 4).
2. **Assign Teams:** Designate owners for runtime, pipeline, editor, testing.
3. **Set Milestones:** Break each phase into 1–2 week sprints.
4. **Spike Tasks:** Evaluate external libraries (bin-packing, image loading).
5. **Begin Phase 1:** Start with component architecture and sprite loading.

---

## Appendix: Glossary

- **Atlas:** A large texture containing multiple smaller sprites packed together.
- **Sprite:** A single image or frame within an atlas.
- **Frame:** A single image in an animation sequence.
- **Pivot/Origin:** The point around which a sprite rotates or is positioned.
- **Collision Box:** A rectangular region used for hit detection.
- **Playback Mode:** How an animation progresses (Loop, Once, PingPong).
- **Hot-Reload:** Updating assets in a running game without restarting.
- **Bin-Packing:** Algorithm to arrange rectangles (sprites) into a larger container (atlas) with minimal wasted space.

---

**End of Document**
