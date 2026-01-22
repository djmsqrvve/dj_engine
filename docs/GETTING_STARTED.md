# Getting Started with DJ Engine

This guide walks you through setting up DJ Engine for the first time.

## Prerequisites

- **Rust 1.75+** - [Install via rustup](https://rustup.rs/)
- **Git** - For cloning the repository
- **A terminal** - bash, PowerShell, or similar

### Platform Notes

| Platform | Status | Notes |
|----------|--------|-------|
| Linux | ✅ Best | Recommended for development |
| WSL2 | ✅ Good | Use with VcXsrv or WSLg |
| Windows | ⚠️ Works | May have graphics quirks |
| macOS | ⚠️ Untested | Should work |

---

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/djmsqrvve/dj_engine.git
cd dj_engine
```

### 2. Build the Project

```bash
cargo build --workspace
```

This compiles the engine and all game projects. First build takes 2-5 minutes.

### 3. Verify Installation

```bash
./dj test
```

You should see all tests passing:
```
test result: ok. 26 passed; 0 failed
```

---

## Running the Editor

```bash
./dj e
```

This opens the Egui-based visual editor where you can:
- Edit scenes (Level Editor view)
- Create story graphs (Story Graph view)
- Inspect entities and resources

### All CLI Commands

```bash
# Development
./dj e          # Run editor
./dj d          # Run DoomExe game
./dj m          # Run minimal test

# Testing & Quality
./dj t          # Run all tests
./dj c          # Check code compiles
./dj fmt        # Format code
./dj lint       # Run Clippy linter

# Build
./dj b          # Build release
./dj doc        # Generate documentation
./dj clean      # Clean build artifacts
```

---

## Project Structure Overview

```
dj_engine/
├── engine/          # The core engine (what you're building on)
├── games/dev/       # Your game projects go here
├── docs/            # Documentation
└── dj               # CLI helper script
```

---

## Creating Your First Project

1. Copy the template:
```bash
cp -r games/dev/doomexe games/dev/my_game
```

2. Update `games/dev/my_game/Cargo.toml`:
```toml
[package]
name = "my_game"
```

3. Add to workspace in root `Cargo.toml`:
```toml
members = [
    "engine",
    "games/dev/doomexe",
    "games/dev/my_game",  # Add this
]
```

4. Run your game:
```bash
cargo run -p my_game
```

---

## Next Steps

- Read the [Architecture Guide](ARCHITECTURE.md) to understand the system
- Check out [Code Style](CODE_STYLE.md) before contributing
- Explore [example JSON files](../engine/examples/) for data formats

---

## Troubleshooting

### Build fails with graphics errors

On WSL2, you may need to set up X11 forwarding:
```bash
export DISPLAY=:0
```

### Tests fail

Make sure you have the latest Rust:
```bash
rustup update
```

### Editor crashes on start

Check the console output. Common issues:
- Missing assets folder
- Invalid JSON in project files
