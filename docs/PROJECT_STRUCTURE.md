# DJ Engine Project Structure

Complete reference for the repository layout.

## Root Directory

```
dj_engine/
├── .github/             # GitHub configuration
├── archive/             # Archived documentation
├── assets/              # Shared assets
├── docs/                # Documentation
├── engine/              # Core engine library
├── games/               # Game projects
├── tools/               # Utilities
├── .gitignore           # Git ignore rules
├── Cargo.lock           # Dependency lock file
├── Cargo.toml           # Workspace manifest
├── CONTRIBUTING.md      # Contributor guide
├── dj                   # CLI helper script
├── GEMINI.md           # AI assistant context
├── LICENSE              # MIT license
├── MAINTAINERS.md       # Project owner guide
└── README.md            # Project overview
```

---

## Engine (`engine/`)

The core library that games depend on.

```
engine/
├── Cargo.toml           # Engine crate manifest
├── examples/            # Example JSON data files
│   ├── database.json    # Items, NPCs, enemies
│   ├── jrpg_scene.json  # JRPG scene example
│   ├── story_graph.json # Dialogue graph example
│   └── td_scene.json    # Tower defense example
├── src/
│   ├── lib.rs           # Library root, prelude
│   ├── core/            # Core plugin and setup
│   │   └── mod.rs
│   ├── data/            # Serializable data types
│   │   ├── mod.rs       # Module exports
│   │   ├── assets.rs    # Asset references
│   │   ├── components.rs # Component data
│   │   ├── database.rs  # Game databases
│   │   ├── loader.rs    # Load/save functions
│   │   ├── project.rs   # Project settings
│   │   ├── scene.rs     # Scene structure
│   │   ├── spawner.rs   # Entity spawning
│   │   └── story.rs     # Story graph data
│   ├── diagnostics/     # Debug and console tools
│   │   ├── mod.rs
│   │   └── console.rs
│   ├── editor/          # Visual editor
│   │   ├── mod.rs       # Main editor UI
│   │   └── validation.rs
│   ├── input/           # Input handling
│   │   └── mod.rs
│   ├── scripting/       # Lua integration
│   │   └── mod.rs
│   └── story_graph/     # Narrative system
│       └── mod.rs
└── tests/               # Integration tests
    ├── editor_integrity.rs
    └── integration_tests.rs
```

---

## Games (`games/`)

Individual game projects that use the engine.

```
games/
└── dev/                 # Development games
    └── doomexe/         # Main game project
        ├── Cargo.toml   # Game crate manifest
        ├── assets/      # Game-specific assets
        │   ├── music/
        │   ├── scripts/
        │   └── sprites/
        ├── src/
        │   ├── main.rs  # Game entry point
        │   └── hamster/ # Hamster narrator feature
        │       ├── mod.rs
        │       ├── components.rs
        │       └── tests/
        ├── scenes/      # Scene JSON files (saved)
        └── story_graphs/ # Story graph JSON files
```

---

## Documentation (`docs/`)

```
docs/
├── README.md                   # Documentation index
├── GETTING_STARTED.md          # First-time setup
├── ARCHITECTURE.md             # System design
├── CODE_STYLE.md               # Coding standards
├── TESTING.md                  # Test guide
├── PROJECT_STRUCTURE.md        # This file
├── Architecture_Specification.json  # Canonical spec
├── Game_Engine_Technical_Roadmap.md # Development plan
└── complete-detailed-docs.md   # Implementation guides
```

---

## GitHub (`.github/`)

```
.github/
├── ISSUE_TEMPLATE/
│   ├── bug_report.md
│   └── feature_request.md
├── PULL_REQUEST_TEMPLATE.md
└── workflows/           # CI/CD (future)
```

---

## Tools (`tools/`)

```
tools/
└── asset_generator/     # Asset processing utilities
```

---

## Key Files Explained

| File | Purpose |
|------|---------|
| `Cargo.toml` (root) | Defines workspace members |
| `dj` | Shell script for common commands |
| `GEMINI.md` | Context for AI coding assistants |
| `Cargo.lock` | Exact dependency versions (committed for reproducibility) |
