# Bevy 2D Rendering Sandbox

A lightweight 2D rendering prototype using the Bevy game engine with a focus on high-quality 2D aesthetics and performance.

## Features

- **Animated Sprites** - A simple sprite sheet animator for a protagonist character
- **Parallax Background** - 3 layers of scrolling background to create depth
- **2D Lighting** - Point light following the mouse cursor
- **Tilemap Support** - Using `bevy_ecs_tilemap` to render basic terrain
- **Camera Control** - Smooth camera follow with zoom capabilities
- **Visual Style** - "Mint Cyberpunk" aesthetic with neon greens, deep purples, and subtle glows

## Project Structure

This project follows the "One Concern per File" principle:

```
src/
├── main.rs              # Entry point and Bevy app setup
├── state.rs             # Game state management (AppState, GameState)
├── components.rs        # ECS components (Player, Camera, Light, etc.)
├── resources.rs         # Shared resources (GameAssets, MousePosition, etc.)
├── systems.rs           # Game systems (camera, lighting, animation, etc.)
└── ui/
    ├── mod.rs          # UI module declaration
    └── hud.rs          # HUD components and systems
```

## Prerequisites

- Rust 1.75 or later
- Cargo

## Running the Project

1. Clone or download this project
2. Navigate to the project directory
3. Run with cargo:

```bash
cargo run
```

For release build with optimizations:

```bash
cargo run --release
```

## Controls

- **Mouse Movement** - Moves the point light
- **+ (Equal Key)** - Zoom out
- **- (Minus Key)** - Zoom in

## Required Assets

You need to add the following assets to the `assets/` directory:

### Sprites
- `assets/sprites/player.png` - Player sprite sheet (4 frames, 32x32 each)

### Backgrounds (for parallax effect)
- `assets/backgrounds/layer1.png` - Far background (slowest scroll)
- `assets/backgrounds/layer2.png` - Mid background
- `assets/backgrounds/layer3.png` - Near background (fastest scroll)

### Tiles
- `assets/tiles/tileset.png` - Tilemap texture

## Configuration

### Visual Style

The "Mint Cyberpunk" aesthetic is configured with:
- Clear color: Deep dark blue (rgb: 0.05, 0.05, 0.1)
- Lighting: Neon green/cyan (rgb: 0.0, 1.0, 0.5)
- Text: Neon green

### Camera Settings

Camera behavior can be adjusted in `resources.rs`:
- `follow_speed` - How quickly the camera follows the player (default: 5.0)
- `zoom_speed` - Zoom in/out speed (default: 0.1)
- `current_zoom` - Initial zoom level (default: 1.0)

## Dependencies

- `bevy` - Game engine (version 0.14)
- `bevy_ecs_tilemap` - Tilemap rendering (version 0.14)
- `bevy_trickfilm` - Animation support (version 0.7)

## Development

### Adding New Systems

To add new systems:

1. Define any new components in `components.rs`
2. Define any new resources in `resources.rs`
3. Implement the system logic in `systems.rs`
4. Add the system to the appropriate schedule in `main.rs`

### Adding New States

To add new game states:

1. Add the state to the enum in `state.rs`
2. Use the state for conditional system execution in `main.rs`

## Performance

This project uses optimized dev profiles for faster compilation:
- All dependencies use opt-level 3
- Your code uses opt-level 1

For maximum performance, use `--release` flag.

## License

This is a prototype project created for demonstration purposes.
