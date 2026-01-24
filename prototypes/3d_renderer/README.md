# Bevy 3D Rendering Sandbox

A high-fidelity 3D rendering prototype using the Bevy game engine for testing PBR materials and lighting with a 'Pale Rose' aesthetic.

## Features

- **GLTF Loading**: Automatically loads GLTF/GLB models from the `assets/test_models` directory
- **PBR Material Testing**: Interactive primitive shapes (cubes, spheres) with varying roughness/metalness values
- **Dynamic Lighting**: 
  - Directional light (sun) with shadows
  - Two animated point lights with different colors
- **Post-Processing**: Custom clear color (Bloom/Tonemapping temporarily removed due to Bevy 0.14 API changes)
- **Orbit Camera**: Interactive camera controls
- **Pale Rose Aesthetic**: Soft pinks, whites, and elegant shadows

## Project Structure

```
src/
├── main.rs              # Entry point and scene setup
└── plugins/
    ├── mod.rs           # Plugin exports
    ├── camera.rs        # Orbit camera implementation
    ├── lighting.rs      # Dynamic lighting system
    └── models.rs        # GLTF loading and PBR test objects

assets/
└── test_models/         # Symlink to test_models directory
    └── dota_models/
        └── models/heroes/drow/  # Drow Ranger model files
```

## Running the Project

First, ensure you have Rust installed. Then build and run:

```bash
cargo run --release
```

The initial build may take some time as Bevy and its dependencies are compiled.

## Controls

- **Mouse**: Orbit around the scene center
- **Future**: Camera controls will be enhanced with mouse/keyboard input

## Test Models

The sandbox includes Dota 2's Drow Ranger model for testing:
- `drow_base.gltf` - Main character model

Additional model files are available in the `test_models/dota_models/models/heroes/drow/` directory.

## Customization

### Camera Settings
Edit `src/plugins/camera.rs` to adjust:
- Orbit radius
- Initial yaw/pitch
- Center point

### Lighting
Edit `src/plugins/lighting.rs` to modify:
- Sun color and intensity
- Point light colors and animation
- Shadow settings

### Materials
Edit `src/plugins/models.rs` to change:
- PBR material parameters
- Color palette
- Primitive shapes

## Future Enhancements

- Mouse/keyboard camera controls
- Depth of Field post-processing
- UI for real-time material/parameter adjustment
- More model loading options
- Environment map support
