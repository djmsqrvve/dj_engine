# Prompt: Bevy 3D Rendering Sandbox

"I want to build a high-fidelity 3D rendering prototype using the Bevy game engine. This should be a sandbox for testing PBR materials and lighting.

Our goal is to render this preexisting model accurately in our engine. dj_engine\prototypes\3d_renderer\test_models\dota_models\models\heroes\drow

### Core Requirements:
1. **Tech Stack:** Bevy (latest), Rust.
2. **Features to Implement:**
    - **GLTF Loading:** Auto-load all .glb/.gltf models from an `assets/models` folder.
    - **PBR Materials:** A system to spawn primitive shapes (cubes, spheres) with varying roughness/metalness to test the lighting model.
    - **Dynamic Lighting:** A sun (DirectionalLight) and a few moving PointLights with shadows enabled.
    - **Post-Processing:** Bloom, Tonemapping, and Depth of Field.
    - **Orbit Camera:** An interactive camera that rotates around a central point (like in Blender/Unity).
3. **Visual Style:** 'Pale Rose' aesthetic—soft pinks, whites, and elegant shadows.
4. **Structure:** Modular plugin-based architecture (LightingPlugin, ModelPlugin, CameraPlugin).

Please generate the `Cargo.toml` and a basic `main.rs` that sets up this environment."
