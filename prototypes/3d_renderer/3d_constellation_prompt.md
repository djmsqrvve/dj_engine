# Prompt: Bevy 3D "Branch Constellation" Dashboard

"I want to build a high-fidelity 3D visualization of a branching feature graph for a game engine, inspired by a celestial neuro-network constellation. This will be a 'Parallel Feature Engine' dashboard.

### Core Requirements:
1. **Engine:** Bevy (latest), Rust.
2. **Visual Components:**
    - **Nodes:** Glowy instanced spheres representing 'Feature Commits'.
    - **Connections:** Dynamic line renderers or ribbon meshes connecting parent/child nodes.
    - **Pulsing Effects:** Nodes should have a 'pulse' animation using a custom shader or material property.
    - **Aesthetic:** Deep space background with a 'Mint Cyberpunk' (Mints/Greens) and 'Pale Rose' (Pinks/Whites) color palette for the nodes.
3. **Core Logic:**
    - **Procedural Constellation:** A system to spawn a central 'CORE' node and several branching 'SPURS' (feature branches) that spiral outward in 3D space.
    - **Health Indicators:** Status-based coloring:
        - **Green/Mint:** Tests Passing.
        - **Red:** Tests Failing.
        - **Yellow/Blue:** Pending/Compiling.
4. **Interaction:**
    - **Orbit Camera:** Interactive 3D camera that can zoom and rotate around the constellation.
    - **Selectable Nodes:** Clicking a node should log its ID to the console.
5. **Aesthetics:** High-intensity Bloom and HDR to make the 'celestial network' feel alive.

Please generate the `Cargo.toml` and a `main.rs` that implements this celestial 3D dashboard."
