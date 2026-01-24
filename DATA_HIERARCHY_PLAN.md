# Data Hierarchy Implementation Plan

This document outlines the technical plan for restructuring the DJ Engine data model into a 5-layer hierarchy. It focuses on the **Rust implementation** specifics—how we use Rust's type system, memory safety, and serialization features to build a robust "World Editor" architecture.

---

## 1. Game (`Project`)
**Concept:** The top-level container (e.g., "Warcraft 3").
**Rust Implementation:**
We keep the existing `Project` struct but refine it to be the "Root of Trust" for global data.

```rust
// engine/src/data/project.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Game Root. Serialization uses Serde to convert this struct <-> JSON.
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Project {
    /// Unique ID (UUIDv4) to prevent collisions if projects are merged
    pub id: String,
    
    /// Display name ("Warcraft 3")
    pub name: String,
    
    /// Global settings (Resolution, Inputs)
    pub settings: ProjectSettings,
    
    /// Registry of all Maps in the project.
    /// We use a HashMap for O(1) lookups by ID.
    /// Key: Map ID, Value: Relative path to map file
    #[serde(default)]
    pub maps: HashMap<String, String>, 
    
    /// Registry of all Game Modes.
    #[serde(default)]
    pub modes: HashMap<String, String>,
}
```
**Rust Note:** `HashMap<String, String>` is used here as a "registry". Rust's strong typing ensures we don't accidentally treat a Map ID as a Mode ID if we wrap them in "Newtypes" (tuple structs) later, but for now `String` is flexible.

---

## 2. Map (`MapAsset`)
**Concept:** Static Environment (Terrain, Trees, Buildings).
**Rust Implementation:**
This is a standard Rust struct that holds "heavy" data like large arrays for tiles.

```rust
// engine/src/data/map.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapAsset {
    pub id: String,
    pub name: String, // "Twisted Meadows"
    
    /// The physical dimensions (width, height).
    /// Using a struct ensures we can't mix up x/y.
    pub size: TileSize, 
    
    /// The visual background (color or texture path).
    pub background: ColorData,
    
    /// Layers of static tiles.
    /// Vec<Layer> is a dynamic array (growable list). 
    /// Ordered by index (0 = bottom, 1 = top).
    pub layers: Vec<MapLayer>,
    
    /// Navigation mesh data for pathfinding.
    /// Encapsulated in its own struct to keep MapAsset clean.
    #[serde(default)] 
    pub nav_mesh: NavGrid, 
}

/// A specific layer of the map (e.g., "Ground", "Cliffs").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayer {
    pub id: String,
    pub visible: bool,
    /// Flattened 1D array of tile IDs for performance (cache locality).
    /// Access via: y * width + x
    pub tiles: Vec<u32>, 
}
```
**Why Rust?**
- `Vec<u32>` for tiles: Rust vectors are contiguous in memory. Iterating over them for rendering is extremely fast (CPU cache friendly).
- splitting `Map` from `Scenario` avoids duplicating this heavy data.

---

## 3. Mode (`GameMode`)
**Concept:** The Ruleset (Survival, CTF, RPG).
**Rust Implementation:**
This is lightweight, mostly configuration and script references.

```rust
// engine/src/data/mode.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMode {
    pub id: String,
    pub name: String, // "Survival Mode"
    
    /// Path to the Lua script that defines the rules.
    /// e.g., "scripts/modes/survival.lua"
    pub script_path: String,
    
    /// Global constants for this mode exposed to Lua.
    /// HashMap<String, f32> allows flexible tuning variables 
    /// (e.g., "gold_per_sec": 1.5) without recompiling Rust code.
    #[serde(default)]
    pub constants: HashMap<String, f32>,
}
```

---

## 4. Scenario (`ScenarioData`)
**Concept:** The Content/Mission (Navigating Map + Mode + specific Units).
**Rust Implementation:**
This struct acts as the "Linker". It borrows a Map, applies a Mode, and adds dynamic entities.

```rust
// engine/src/data/scenario.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioData {
    pub id: String,
    pub name: String, // "Mission 1: The Defense"
    
    /// Reference to the Map ID.
    /// NOTE: We store the ID, not the MapAsset itself. 
    /// We load the MapAsset at runtime. This mimics "Composition over Inheritance".
    pub map_id: String,
    
    /// Reference to the GameMode ID.
    pub mode_id: String,
    
    /// Dynamic Entities specific to this scenario (Peons, Spawners).
    /// Unlike Maps (tiles), these are "Object" data.
    #[serde(default)]
    pub entities: Vec<ScenarioEntity>,
    
    /// Objective scripts specific to this mission.
    pub script_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioEntity {
    pub id: String,
    pub prefab_id: Option<String>, // e.g., "grunt_lvl1"
    pub position: Vec3Data,
    // Scenario specific overrides (e.g., this grunt has 2x HP)
    pub components: EntityComponents, 
}
```
**Rust Note:** `Option<String>` is Rust's safe way of handling nulls. If `script_path` is `None`, the compiler forces us to handle that case (e.g., run default logic), preventing runtime crashes.

---

## 5. Scene (Dynamic)
**Concept:** A runtime slice (e.g., Cutscene).
**Rust Implementation:**
This is less of a saved asset and more of a runtime state or a very small asset.

```rust
// engine/src/data/scene.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CutsceneData {
    pub id: String,
    /// A sequence of actions (timeline).
    pub instructions: Vec<CutsceneInstruction>,
}

/// Enum for different instruction types.
/// Rust Enums are "Algebraic Data Types" - they can hold data!
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")] // serializes as {"type": "CameraMove", ...}
pub enum CutsceneInstruction {
    CameraMove { target: Vec3Data, duration: f32 },
    Dialogue { speaker: String, text: String },
    Wait { seconds: f32 },
    SpawnUnit { unit_id: String, at: Vec3Data },
}
```
**Why Rust Enums?** `CutsceneInstruction` allows us to have a strictly typed list of *different* actions. You can't put a "Banana" in this list; it MUST be a valid instruction type defined at compile time.

---

## Plan of Action

1.  **Shared Types**: Verify `types.rs` has all primitive types (`Vec3Data`, `ColorData`).
2.  **Define Structs**: Create the files in `engine/src/data/`.
3.  **Serialization**: Implement `loader.rs` functions for each new type (`load_map`, `save_scenario`).
4.  **Editor Refactor**:
    *   **Map Editor**: Only edits `MapAsset` (Tiles, collision).
    *   **Scenario Editor**: Loads a `Map`, locks it (read-only), allowing you to place `ScenarioEntities`.
