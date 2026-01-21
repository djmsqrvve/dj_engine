# CHANGELOG

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Initial project scaffolding and documentation
- Cargo workspace configuration with engine + game crates
- Development workflow guide (WORKFLOW.md)
- Coding standards (CODING_STANDARDS.md)
- Lua FFI specification (LUA_FFI.md)
- Asset pipeline specification (ASSET_PIPELINE.md)

### Changed
- (None yet)

### Deprecated
- (None yet)

### Removed
- (None yet)

### Fixed
- (None yet)

### Security
- (None yet)

---

## [0.1.0] – Hamster Narrator MVP

**Release Date**: TBD (Milestone 1 completion)

### Added (Planned for Milestone 1)

#### Rendering System
- Offscreen render target (320×240 resolution)
- Nearest-neighbor upscaling to window resolution
- CRT post-processing shader (scanlines, vignette)
- Palette swap shader with corruption parameter

#### Animation System
- Breathing animation (sine-wave body scale with area preservation)
- Blinking animation (eye sprite transitions)
- Idle motion (Perlin noise-based jitter)
- Easing curve support for smooth transitions

#### Hamster Assembly
- Procedural hamster rig from sprite parts
- Hierarchical entity structure (parent-child transforms)
- Part definition loading from JSON metadata
- Support for 7+ sprite parts (body, head, eyes, mouth, ears)

#### Corruption System
- Corruption state variable (0.0–100.0)
- Palette shifting based on corruption level
- CRT intensity scaling with corruption
- Visual feedback on expression changes

#### Lua Integration
- Basic Lua FFI with 4 core functions:
  - `set_corruption(f32)`
  - `get_corruption() → f32`
  - `set_expression(string) → bool`
  - `log(string)`
- Callback system: `init()`, `on_key_press(key)`
- Hot-reload support for scripts
- Script error handling and logging

#### Asset Pipeline
- Aseprite export to JSON metadata workflow
- Palette loading from JSON definitions
- Shader loading and compilation
- Script file watching and reload

### Performance
- Sustained 60 FPS on target hardware
- Efficient sprite batching for hamster parts
- Minimal shader overhead

### Documentation
- Complete project plan (PROJECT_PLAN.md)
- Development workflow guide
- Coding standards and conventions
- API documentation (doc comments)

### Testing
- Unit tests for core systems
- Animation timing tests
- Corruption boundary tests

---

## Future Versions (Milestone 2+)

### Planned for v0.2.0 (Dialogue System)
- [ ] Full branching dialogue trees
- [ ] Choice UI system
- [ ] Dialogue state persistence
- [ ] Character emotion expression library

### Planned for v0.3.0 (Extended Animation)
- [ ] Paw animations
- [ ] Head rotation
- [ ] Particle effects
- [ ] Idle motion randomization

### Planned for v0.4.0 (Audio)
- [ ] Background music system
- [ ] Sound effect playback
- [ ] Audio corruption effects
- [ ] Voice line support (TTS or pre-recorded)

### Planned for v1.0.0 (Full Game Release)
- [ ] Complete story implementation
- [ ] Save/load system
- [ ] Multiple scenes and chapters
- [ ] Menus and UI polish
- [ ] Performance optimization
- [ ] Cross-platform testing

---

## Version History

### Development Branches

| Branch | Purpose | Status |
|--------|---------|--------|
| `main` | Stable releases only | 🟢 Ready |
| `develop` | Integration branch for features | 🟢 Ready |
| `feature/*` | Individual feature work | 🔄 In Progress |

**Workflow**: `feature/` → PR → `develop` → release PR → `main` + tag

---

## How to Read This Changelog

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Now-removed features
- **Fixed**: Bug fixes
- **Security**: Security vulnerability fixes

Each version entry should include:
1. Version number (semantic versioning)
2. Release date or "Unreleased"
3. Summary of changes by category

---

## How to Update This Changelog

### When Merging a PR

1. Add entry under `[Unreleased]` section
2. Use category (Added, Fixed, Changed, etc.)
3. Write short, clear descriptions
4. Reference issue numbers if applicable

**Format**:
```markdown
### Added
- Brief description of feature [#123](link)
- Another feature description

### Fixed
- Description of bug fix [#456](link)
```

### When Creating a Release

1. Copy `[Unreleased]` section to new version
2. Add version number and date: `## [0.2.0] – 2026-02-15`
3. Clear `[Unreleased]` section, leaving headers
4. Create git tag: `git tag v0.2.0`
5. Push tag: `git push origin v0.2.0`

---

## Template for New Entry

Copy this into `[Unreleased]` when adding a feature:

```markdown
### Added
- [Brief description] (#issue_number)

### Fixed
- [Brief description] (#issue_number)

### Changed
- [Brief description] (#issue_number)
```

---

## Release Checklist

Before releasing a new version:

- [ ] All tests passing (`cargo test --workspace`)
- [ ] No clippy warnings (`cargo clippy --workspace`)
- [ ] Code formatted (`cargo fmt --all`)
- [ ] Version bumped in `Cargo.toml`
- [ ] CHANGELOG.md updated with all changes
- [ ] Documentation up-to-date
- [ ] Performance metrics collected (if applicable)
- [ ] Changelog tagged and pushed

---

**Last Updated**: 2026-01-20  
**Maintained By**: DJ Engine Team
