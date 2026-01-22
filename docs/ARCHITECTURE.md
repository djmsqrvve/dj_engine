# DJ Engine Architecture

This document explains the core design and architecture of DJ Engine.

## Overview

DJ Engine is a **modular, plugin-based game engine** built on Bevy ECS. It separates concerns into distinct layers:

```
┌─────────────────────────────────────────────────┐
│                   Games                          │
│            (doomexe, your game)                  │
├─────────────────────────────────────────────────┤
│                 DJ Engine                        │
│  ┌─────────┬─────────┬─────────┬─────────────┐  │
│  │ Editor  │ Story   │ Script  │ Diagnostics │  │
│  │ Plugin  │ Graph   │ Plugin  │   Plugin    │  │
│  ├─────────┴─────────┴─────────┴─────────────┤  │
│  │              Data Module                   │  │
│  │    (Serializable types, loaders)           │  │
│  ├───────────────────────────────────────────┤  │
│  │              Core Plugin                   │  │
│  │    (App setup, registrations)              │  │
│  └───────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│                    Bevy                          │
│        (ECS, Rendering, Audio, etc.)             │
└─────────────────────────────────────────────────┘
```

---

## Core Principles

### 1. Data-Driven Design

All game content is defined in JSON files, not hardcoded:
- Scenes → `scenes/*.json`
- Story graphs → `story_graphs/*.json`
- Databases → `databases/*.json`

**Benefit**: Content can be edited without recompiling.

### 2. Separation of Data and Runtime

```rust
// ❌ Bad: Mixing serialization with runtime logic
#[derive(Component, Serialize)]
struct Player { hp: i32, bevy_handle: Handle<Image> }

// ✅ Good: Separate data and runtime
// Data (for JSON)
#[derive(Serialize, Deserialize)]
struct PlayerData { hp: i32, sprite_id: String }

// Runtime (for ECS)
#[derive(Component)]
struct Player { hp: i32 }
```

### 3. Plugin Architecture

Each feature is a Bevy plugin that can be enabled/disabled:

```rust
app.add_plugins((
    DJEnginePlugin,     // Core
    EditorPlugin,       // Visual editor
    ScriptingPlugin,    // Lua support
    DiagnosticsPlugin,  // Debug tools
));
```

---

## Module Reference

### Core (`engine/src/core/`)
- `DJEnginePlugin` - Main engine initialization
- Registers all data types for reflection

### Data (`engine/src/data/`)
| File | Purpose |
|------|---------|
| `mod.rs` | Module exports, DataPlugin |
| `project.rs` | Project settings |
| `scene.rs` | Scene, Layer, Entity |
| `components.rs` | Component data structs |
| `story.rs` | Story graph data |
| `database.rs` | Items, NPCs, enemies |
| `assets.rs` | Asset references |
| `loader.rs` | Load/save functions |
| `spawner.rs` | Entity spawning systems |

### Story Graph (`engine/src/story_graph/`)
- `StoryGraph` - Runtime graph resource
- `GraphExecutor` - Processes nodes
- `StoryNode` - Dialogue, Choice, Branch, etc.

### Editor (`engine/src/editor/`)
- `EditorPlugin` - Egui-based visual editor
- Level editor view
- Story graph editor view
- Inspector panel

### Scripting (`engine/src/scripting/`)
- `ScriptingPlugin` - Lua integration via mlua
- `LuaContext` - Thread-safe Lua state
- Script loading and execution

---

## Data Flow

```
JSON Files                    Runtime
    │                           │
    ▼                           │
load_scene()              ┌─────┴─────┐
load_story_graph()   ──►  │  Bevy     │
load_database()           │  World    │
    │                     │  (ECS)    │
    │                     └─────┬─────┘
    │                           │
    ▼                           ▼
Data Structs              Components
(Serialize)               (Runtime)
```

---

## Key Types

### StoryNode (Runtime)
```rust
enum StoryNode {
    Dialogue { speaker, text, next },
    Choice { prompt, options },
    Branch { condition, if_true, if_false },
    End,
}
```

### StoryNodeData (Serialized)
```rust
struct StoryNodeData {
    id: String,
    position: Vec3Data,  // Editor position
    data: StoryNodeVariant,
}
```

---

## Extension Points

1. **New Component Types** - Add to `data/components.rs`
2. **New Node Types** - Add to `story_graph/mod.rs` and `data/story.rs`
3. **New Editor Views** - Add to `editor/mod.rs`
4. **Game Plugins** - Create in `games/dev/your_game/`
