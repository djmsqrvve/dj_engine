# DJ Engine

**A modular game engine for narrative-heavy JRPGs and visual novels**

![Rust](https://img.shields.io/badge/Rust-1.92-orange?style=flat-square)
![Bevy](https://img.shields.io/badge/Bevy-0.18-green?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)

## ✨ Features

- **Story Graph System**: Node-based narrative engine for complex dialogue branching.
- **Procedural Animation**: Built-in generic systems for breathing, blinking, and idle motion.
- **Lua Scripting**: Full scripting support via `mlua` to control engine systems at runtime.
- **Egui Editor**: Integrated tooling for scene editing, story graph management, and debugging.
- **RPG Systems**: Modular inventory, quest logging, and combat stats.
- **Navigation**: Grid-based A* pathfinding.
- **Logging**: Automatic persistent file mapping to `~/.dj_engine/logs/`.

## 🚀 Quick Start

### Prerequisites

- Rust 1.92+
- Bevy 0.18 compatible environments (Vulkan/Metal/DX12)

### Running the CLI RPG Demo

The `cli_rpg` tool demonstrates the story graph engine running in a terminal environment without the graphical overhead.

```bash
cargo run -p cli_rpg
```

### Running Tests

```bash
# Run generic tests
cargo test -p dj_engine

# Run all workspace tests (some integration tests may be heavy)
cargo test --workspace
```

## 📁 Repository Structure

```text
dj_engine/
├── engine/              # Core library crate
│   ├── src/
│   │   ├── animation/   # Generic procedural animation
│   │   ├── story_graph/ # Narrative engine
│   │   ├── editor/      # Egui-based tools
│   │   ├── lua_scripting/ # Scripting bridge
│   │   └── game/        # RPG systems (Inventory, Quests)
│   └── tests/           # Integration tests
├── tools/
│   └── cli_rpg/         # Terminal-based story runner
└── games/               # Active game projects
```

> **Note**: Historical reference code and the `bridge` prototype have been moved to `../dj_engine_archive` to keep this repository clean and focused on the core engine.

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.
