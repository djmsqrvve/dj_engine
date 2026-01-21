# Asset Pipeline – DJ Engine

**Version**: 1.0  
**Last Updated**: 2026-01-20  
**Tool**: Aseprite → JSON → Rust → Bevy

---

## Overview

This document specifies how assets (sprites, palettes, shaders, scripts) flow from creation through runtime. It's the "glue" between artist tools (Aseprite) and game engine (Bevy).

---

## 1. Asset Directory Structure

### Physical Layout

```
games/dev/doomexe/
├── assets/                    # Root asset directory (Bevy searches here)
│   ├── sprites/
│   │   └── hamster_parts/     # Hamster sprite parts
│   │       ├── body/
│   │       │   ├── body.aseprite       # Source file
│   │       │   ├── body.png            # Exported sprite (256x256)
│   │       │   └── body_meta.json      # Metadata (created manually)
│   │       ├── head/
│   │       ├── eye_open/
│   │       ├── eye_closed/
│   │       ├── mouth_smile/
│   │       ├── mouth_angry/
│   │       ├── mouth_neutral/
│   │       ├── ear_left/
│   │       └── ear_right/
│   │
│   ├── palettes/
│   │   ├── default.json       # Base hamster colors
│   │   ├── corrupted_1.json   # Corruption variant 1 (0-33%)
│   │   ├── corrupted_2.json   # Corruption variant 2 (34-66%)
│   │   └── corrupted_3.json   # Corruption variant 3 (67-100%)
│   │
│   ├── shaders/
│   │   ├── palette_swap.wgsl      # Palette index lookup
│   │   ├── crt_postprocess.wgsl   # Scanlines + vignette
│   │   └── chromatic_aberration.wgsl
│   │
│   └── scripts/
│       └── hamster_dialogue.lua    # Main script
│
└── src/
    └── assets/
        ├── mod.rs              # Asset loading plugin
        ├── loaders.rs          # JSON/sprite loaders
        └── definitions.rs      # Rust type definitions
```

### Asset Types

| Type | Format | Loader | Use |
|------|--------|--------|-----|
| Sprite | PNG (8-bit indexed, 256x256 max) | `SpriteLoader` | Hamster parts, UI |
| Palette | JSON (RGB triplets) | `PaletteLoader` | Color swaps |
| Shader | WGSL (WebGPU Shader Language) | `ShaderLoader` (Bevy native) | Post-processing |
| Script | Lua 5.4 | `ScriptLoader` | Game logic |

---

## 2. Sprite Export Process (Aseprite)

### Step 1: Create in Aseprite

1. **Open Aseprite**
2. **New image**: 256×256 pixels, indexed color (256 colors max)
3. **Create one layer per part** (e.g., "body", "head", "mouth")
4. **Draw hamster part** on single layer (transparent background)
5. **Save as**: `body.aseprite`

**Important**: Aseprite indexed-color mode is critical—palettes are referenced, not baked.

### Step 2: Export PNG

1. **File → Export As**
2. **Choose**: `body.png`
3. **Settings**:
   - Color mode: **Indexed** (preserve palette)
   - Output: **PNG** (supports transparency)
4. **Export**

Result: `body.png` (256×256, indexed color, transparent background)

### Step 3: Create Metadata JSON

**File**: `body_meta.json` (same directory as `body.png`)

```json
{
  "part_name": "body",
  "sprite_file": "body.png",
  "sprite_size": {
    "w": 256,
    "h": 256
  },
  "original_offset": {
    "x": 0,
    "y": 0
  },
  "layer_index": 0,
  "pivot": {
    "x": 128,
    "y": 128
  },
  "trim_rect": {
    "x": 32,
    "y": 24,
    "w": 192,
    "h": 208
  }
}
```

**Field Explanations**:

- **`part_name`** (string): Identifier used in Rust (must match in code)
- **`sprite_file`** (string): Relative path to PNG (from same directory)
- **`sprite_size`** (object): Full PNG dimensions in pixels
- **`original_offset`** (object): Position in composite hamster image (0, 0 = top-left)
- **`layer_index`** (integer): Z-order (0 = back, higher = front). Typically set to 0 for single-part files.
- **`pivot`** (object): Rotation/scale center (usually center of image)
- **`trim_rect`** (object): Bounding box of actual drawn content (for optimization)

### Step 4: Add to Asset Manifest

**File**: `assets/sprites/hamster_parts/manifest.json`

```json
{
  "parts": [
    {
      "part_name": "body",
      "directory": "body",
      "metadata_file": "body_meta.json"
    },
    {
      "part_name": "head",
      "directory": "head",
      "metadata_file": "head_meta.json"
    },
    {
      "part_name": "eye_open",
      "directory": "eye_open",
      "metadata_file": "eye_open_meta.json"
    }
  ]
}
```

This file tells the engine which parts to load at startup.

---

## 3. Palette Format

### Default Palette (`default.json`)

```json
{
  "palette_name": "default",
  "colors": [
    { "index": 0, "r": 0, "g": 0, "b": 0 },      // Color 0
    { "index": 1, "r": 240, "g": 100, "b": 50 }, // Hamster fur
    { "index": 2, "r": 255, "g": 255, "b": 255 },// White (eyes)
    { "index": 3, "r": 0, "g": 0, "b": 0 },      // Black (pupils)
    // ... up to 256 colors (can skip unused indices)
  ]
}
```

**Constraints**:
- **Exactly 256 entries** or sparse (only defined indices are required)
- **RGB only** (no alpha; transparency is in sprite layer)
- **Indices must be unique**

### Corruption Variants

Create separate palette files for corruption stages:

**`corrupted_1.json`** (0-33% corruption):
```json
{
  "palette_name": "corrupted_1",
  "colors": [
    { "index": 1, "r": 220, "g": 80, "b": 30 }   // Slightly darker
  ]
}
```

**`corrupted_2.json`** (34-66% corruption):
```json
{
  "palette_name": "corrupted_2",
  "colors": [
    { "index": 1, "r": 180, "g": 40, "b": 10 }   // Much darker, reddish
  ]
}
```

**`corrupted_3.json`** (67-100% corruption):
```json
{
  "palette_name": "corrupted_3",
  "colors": [
    { "index": 1, "r": 120, "g": 10, "b": 10 }   // Very dark red
  ]
}
```

---

## 4. Rust Loaders & Definitions

### Data Structures (`engine/assets/definitions.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HamsterPartDefinition {
    pub part_name: String,
    pub sprite_file: String,
    pub sprite_size: IVec2,
    pub original_offset: IVec2,
    pub layer_index: u32,
    pub pivot: Vec2,
    pub trim_rect: Option<Rect>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PaletteDefinition {
    pub palette_name: String,
    pub colors: Vec<ColorEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorEntry {
    pub index: u32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorEntry {
    pub fn to_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }
}
```

### Loaders (`engine/assets/loaders.rs`)

```rust
use bevy::asset::{AssetLoader, LoadContext};
use anyhow::Result;
use serde_json;

pub struct HamsterPartLoader;

impl AssetLoader for HamsterPartLoader {
    type Asset = HamsterPartDefinition;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load<'a>(
        &'a self,
        reader: &mut bevy::asset::io::Reader<'a>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes)?;
        serde_json::from_str(&text).map_err(Into::into)
    }

    fn extensions(&self) -> &[&str] {
        &["hamster_meta.json"]
    }
}

pub struct PaletteLoader;

impl AssetLoader for PaletteLoader {
    type Asset = PaletteDefinition;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load<'a>(
        &'a self,
        reader: &mut bevy::asset::io::Reader<'a>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let text = String::from_utf8(bytes)?;
        serde_json::from_str(&text).map_err(Into::into)
    }

    fn extensions(&self) -> &[&str] {
        &["palette.json"]
    }
}
```

### Asset Plugin (`engine/assets/mod.rs`)

```rust
use bevy::app::{App, Plugin};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        // Register custom loaders
        app.register_asset_loader(HamsterPartLoader);
        app.register_asset_loader(PaletteLoader);
    }
}
```

---

## 5. Runtime Asset Loading

### Load Flow

```
Game Start
    ↓
AssetPlugin::setup()
    ├─ Initialize AssetServer
    └─ Register loaders
        ↓
Main Scene Startup
    ├─ Load hamster manifest
    ├─ For each part in manifest:
    │   ├─ Load JSON metadata
    │   ├─ Load PNG sprite
    │   └─ Store in HamsterPartLibrary
    ├─ Load palette JSONs
    ├─ Load shaders
    └─ Load script
        ↓
Spawn Hamster
    ├─ Look up parts from library
    ├─ Create entity hierarchy
    └─ Attach sprites
        ↓
Run Game
```

### Example: Spawn Hamster System

```rust
#[derive(Resource)]
pub struct HamsterPartLibrary {
    parts: HashMap<String, (HamsterPartDefinition, Handle<Image>)>,
}

fn spawn_hamster_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    library: Res<HamsterPartLibrary>,
) {
    // Create root entity
    let hamster = commands.spawn(HamsterNarrator::default()).id();

    // Spawn each part as child
    for (name, (definition, image_handle)) in &library.parts {
        let part = commands
            .spawn((
                HamsterPart {
                    part_type: name.clone(),
                    offset: definition.original_offset.as_vec2(),
                    layer: definition.layer_index,
                },
                Sprite {
                    custom_size: Some(Vec2::new(
                        definition.sprite_size.x as f32,
                        definition.sprite_size.y as f32,
                    )),
                    ..default()
                },
                Texture(image_handle.clone()),
                Transform {
                    translation: definition.original_offset.as_vec3(),
                    ..default()
                },
            ))
            .id();

        commands.entity(hamster).add_child(part);
    }
}
```

---

## 6. Hot-Reloading Assets

### File Watching

Bevy's `AssetServer` automatically watches `assets/` directory:

1. **Script file changes** → Reloads Lua VM, calls `init()` again
2. **Palette changes** → Reapplies palette texture
3. **Shader changes** → Recompiles shader
4. **Sprite changes** → Reloads texture

**No manual intervention needed** – just edit and save.

### Debounce

Rapid file changes (< 500ms apart) are debounced to one reload. This prevents 10 reloads from a single multi-file edit.

---

## 7. Validation & Error Handling

### Validation Checklist

Before shipping assets:

- [ ] All sprite PNGs are indexed color (not RGB)
- [ ] All metadata JSON files are valid JSON (use `jq` to check)
- [ ] All `part_name` values in JSON match Rust enum variants
- [ ] All referenced files exist (check `sprite_file` paths)
- [ ] All palette indices are 0–255
- [ ] All RGB values are 0–255
- [ ] No duplicate `part_name` values
- [ ] All sprites have transparent background (PNG saved with transparency)

### Validation Script

Create `scripts/validate_assets.py`:

```python
#!/usr/bin/env python3
import json
import os

ASSETS_DIR = "games/dev/doomexe/assets"

def validate_json(filepath):
    try:
        with open(filepath) as f:
            json.load(f)
        return True
    except Exception as e:
        print(f"❌ Invalid JSON: {filepath}")
        print(f"   {e}")
        return False

def validate_sprites():
    sprites_dir = f"{ASSETS_DIR}/sprites/hamster_parts"
    for part_dir in os.listdir(sprites_dir):
        meta_file = f"{sprites_dir}/{part_dir}/{part_dir}_meta.json"
        if not validate_json(meta_file):
            return False
    print("✅ All sprite metadata valid")
    return True

def validate_palettes():
    palettes_dir = f"{ASSETS_DIR}/palettes"
    for palette_file in os.listdir(palettes_dir):
        if palette_file.endswith(".json"):
            if not validate_json(f"{palettes_dir}/{palette_file}"):
                return False
    print("✅ All palettes valid")
    return True

if __name__ == "__main__":
    validate_sprites()
    validate_palettes()
```

Run before committing:

```bash
python scripts/validate_assets.py
```

---

## 8. Asset Naming Conventions

### Sprite Parts

Use descriptive, lowercase names:
- `body` (main torso)
- `head` (head)
- `ear_left`, `ear_right` (ears)
- `eye_open`, `eye_closed` (eyes)
- `mouth_neutral`, `mouth_smile`, `mouth_angry` (mouths)

### Palettes

- `default.json` (base colors)
- `corrupted_1.json`, `corrupted_2.json`, `corrupted_3.json` (corruption stages)

### Shaders

- `palette_swap.wgsl` (palette index lookup)
- `crt_postprocess.wgsl` (post-processing effects)

### Scripts

- `hamster_dialogue.lua` (main script)
- `hamster_dialogue_debug.lua` (debug/test script)

---

## 9. Limitations & Edge Cases

### PNG Constraints

- **Max dimensions**: 512×512 pixels (larger wastes VRAM)
- **Must be indexed color** (256 colors max)
- **Transparency**: Use alpha channel (PNG supports it)
- **Color space**: sRGB

### Palette Constraints

- **Max colors**: 256
- **Indices**: Must be unique, 0–255
- **No alpha in palette**: Transparency is in sprite layer

### Shader Constraints

- **Language**: WGSL only (Bevy's standard)
- **Texture samplers**: Max 8 (practical limit)
- **Uniforms**: Keep < 256 bytes (typical)

---

## 10. Troubleshooting

| Problem | Cause | Solution |
|---------|-------|----------|
| Sprite not rendering | Not indexed color in Aseprite | Re-export as indexed PNG |
| Asset file not loading | Wrong extension or path | Check `assets/` directory path matches |
| Metadata JSON parse error | Invalid JSON syntax | Run through `jq` validator |
| Palette doesn't apply | Palette indices don't match sprite | Regenerate both in Aseprite |
| Shader compile error | WGSL syntax error | Check `.wgsl` file syntax |
| Hot-reload not working | File watcher not running | Restart game; check `assets/` writable |

---

## 11. Example Workflow

### Create New Hamster Part

1. **Open Aseprite**
2. **New 256×256 indexed image**
3. **Draw paw** on single layer
4. **Save as** `games/dev/doomexe/assets/sprites/hamster_parts/paw_left/paw_left.aseprite`
5. **Export PNG** → `paw_left.png`
6. **Create metadata** → `paw_left_meta.json`:
   ```json
   {
     "part_name": "paw_left",
     "sprite_file": "paw_left.png",
     "sprite_size": { "w": 256, "h": 256 },
     "original_offset": { "x": -50, "y": 120 },
     "layer_index": 2,
     "pivot": { "x": 128, "y": 128 }
   }
   ```
7. **Add to manifest** → `hamster_parts/manifest.json`
8. **Rust code** → Add `paw_left` variant to `PartType` enum
9. **Test** → Run game, should see paw attached to hamster

---

## Conclusion

The asset pipeline balances:
- **Artist workflow** (use familiar Aseprite)
- **Engine performance** (indexed palettes, efficient loading)
- **Designer flexibility** (JSON metadata, hot-reload)

Assets are the visual language of the game. Keep them organized, well-named, and validated.
