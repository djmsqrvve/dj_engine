# DJ Engine

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/Bevy-0.15-green)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A modular game engine for **narrative-heavy JRPGs** and **visual novels**, built with Rust and Bevy.

## ✨ Features

- **Story Graph System** - Node-based dialogue and narrative branching
- **Egui Editor** - Visual editor for scenes and story graphs  
- **Lua Scripting** - Runtime scripting via mlua
- **Data-Driven** - JSON-serializable scenes, databases, and graphs
- **Procedural Animation** - Palette-driven corruption effects

## 🚀 Quick Start

```bash
# Clone
git clone https://github.com/djmsqrvve/dj_engine.git
cd dj_engine

# Run Editor
./dj e

# Run Tests  
./dj test

# Build Release
cargo build --release
```

## 📁 Project Structure

```
dj_engine/
├── engine/          # Core engine library
│   ├── src/         # Rust source (data, editor, story_graph, scripting)
│   └── examples/    # Example JSON files
├── games/dev/       # Game projects (doomexe)
├── docs/            # Documentation
└── tools/           # Asset utilities
```

## 📖 Documentation

- [Architecture](docs/Architecture_Specification.json) - Core design spec
- [Roadmap](docs/Game_Engine_Technical_Roadmap.md) - 20-week development plan
- [Complete Docs](docs/complete-detailed-docs.md) - Implementation guides

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT © [djmsqrvve](https://github.com/djmsqrvve)
