# Coding Standards – DJ Engine

**Version**: 1.0  
**Language**: Rust  
**Framework**: Bevy ECS  
**Last Updated**: 2026-01-20

---

## Overview

These standards ensure consistent, readable, and maintainable code across the DJ Engine. Follow them for all contributions.

---

## 1. General Principles

1. **Readability over cleverness** – Write code for the next person reading it, not to show off
2. **Explicit is better than implicit** – Use clear variable names, type annotations where helpful
3. **Fail loudly** – Use `Result` types and panic! strategically, not silently
4. **Document assumptions** – Comment why, not what
5. **Test behavior, not implementation** – Tests should verify results, not implementation details

---

## 2. Formatting & Structure

### Formatting (Non-Negotiable)

All code **must** pass `cargo fmt`:

```bash
cargo fmt --all
```

- 4-space indentation (standard Rust)
- Max line length: 100 characters (soft limit for readability)
- Always use trailing commas in lists
- Space before opening brace: `if x {` not `if x{`

### File Organization

```rust
// 1. Imports (grouped: std, external crates, internal)
use std::collections::HashMap;
use bevy::prelude::*;
use crate::rendering::RenderingPlugin;

// 2. Module declarations
pub mod components;
pub mod systems;
mod internal_helper;

// 3. Re-exports for convenience
pub use components::HamsterNarrator;

// 4. Constants
const DEFAULT_CORRUPTION: f32 = 0.0;

// 5. Type aliases
type Result<T> = std::result::Result<T, AnimationError>;

// 6. Structs/Enums/Traits
#[derive(Component)]
pub struct BreathingAnimation {
    // ...
}

// 7. Implementation blocks (pub impl, then private)
impl BreathingAnimation {
    // ...
}

// 8. Tests (at the end)
#[cfg(test)]
mod tests {
    // ...
}
```

### Struct Field Ordering

1. Public fields first (often components)
2. Private fields (often data)
3. Methods group by public/private

```rust
#[derive(Component)]
pub struct HamsterNarrator {
    // Public
    pub corruption: f32,
    pub expression: Expression,

    // Private
    animation_time: f32,
    blink_timer: f32,
}
```

---

## 3. Naming Conventions

### Crates & Modules
- **Crate names**: Snake case, meaningful (`dj_engine`, `doomexe`)
- **Module names**: Snake case, plural when possible (`systems`, `components`, `assets`)

### Constants
- **All caps with underscores**: `const MAX_CORRUPTION: f32 = 100.0;`
- **Descriptive names**: `BREATHING_FREQUENCY_HZ` not `FREQ`

### Functions & Methods
- **Snake case**: `apply_breathing_animation()`
- **Verb-first for actions**: `update_`, `apply_`, `spawn_`
- **Question-first for predicates**: `is_breathing()`, `has_loaded()`
- **Avoid single-letter names** except in math contexts (x, y, t)

### Variables
- **Snake case**: `hamster_corruption`, `frame_count`
- **Descriptive**: `blink_timer_sec` not `bt` (units in name if relevant)
- **Loop variables**: `for (entity, transform) in query.iter()` (meaningful tuple destructuring)

### Types (Struct/Enum/Trait)
- **PascalCase**: `HamsterNarrator`, `CorruptionEffect`
- **Avoid Hungarian notation** (no `s_str` for String, no `f_value` for f32)

### Enums
```rust
// Values: PascalCase
pub enum Expression {
    Happy,
    Angry,
    Corrupted,
}

// Avoid all-caps for variants unless it's an acronym
pub enum Codec {
    Json,      // ✅ good
    Json,      // ✅ good
    JSON,      // ❌ avoid unless acronym with > 2 letters
}
```

---

## 4. Bevy-Specific Conventions

### Components

```rust
// Marker components (empty): Small, indicate capability
#[derive(Component)]
pub struct BreathingMarker;

// Data components: Hold state
#[derive(Component)]
pub struct BreathingAnimation {
    pub amplitude: f32,
    pub frequency: f32,
}

// Systems always take queries, commands, or resources
fn breathing_system(
    mut query: Query<(&BreathingAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    // ...
}
```

### Systems

**Naming**: `<verb>_<noun>_system`

```rust
fn update_breathing_animation_system() { }
fn apply_corruption_effects_system() { }
fn reload_lua_scripts_system() { }
```

**Organization**: 
- Early systems (input, gameplay logic)
- Late systems (rendering, post-processing)
- Use `.before()` / `.after()` for dependencies if needed

### Plugins

```rust
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_breathing_system,
                update_blinking_system,
                apply_idle_motion_system,
            ))
            .register_type::<BreathingAnimation>();
    }
}
```

---

## 5. Error Handling

### Use `Result<T>` for Fallible Operations

```rust
// ✅ Good
fn load_asset(path: &str) -> Result<Image> {
    let data = std::fs::read(path)?;
    Ok(Image::from_bytes(&data))
}

// ❌ Bad: silently returns None
fn load_asset(path: &str) -> Option<Image> {
    std::fs::read(path).ok()?.into()
}

// ❌ Bad: panics on any error
fn load_asset(path: &str) -> Image {
    let data = std::fs::read(path).expect("file not found");
    Image::from_bytes(&data)
}
```

### Define Clear Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    #[error("Invalid animation curve: {0}")]
    InvalidCurve(String),

    #[error("Animation not found: {0}")]
    NotFound(String),

    #[error("Animation system error: {0}")]
    SystemError(#[from] anyhow::Error),
}

type Result<T> = std::result::Result<T, AnimationError>;
```

### When to Panic

- **Panic for programmer errors**: Invalid enum state, assertion failures
- **Result for user/runtime errors**: File not found, invalid input, network issues

```rust
// ✅ Panic: programming error
fn set_expression(&mut self, expr: Expression) {
    debug_assert!(matches!(expr, Expression::Happy | Expression::Angry));
    self.expression = expr;
}

// ✅ Result: runtime error
fn load_config(path: &str) -> Result<Config> {
    let json = std::fs::read_to_string(path)?;
    serde_json::from_str(&json)
        .map_err(|e| AnimationError::ConfigError(e.to_string()))
}
```

---

## 6. Documentation

### Required Documentation

**Every public module** needs a doc comment:

```rust
//! Procedural animation systems for hamster movement.
//!
//! This module provides:
//! - Sine-wave breathing with area preservation
//! - Perlin noise-based idle motion
//! - Customizable easing curves
//!
//! # Example
//!
//! ```ignore
//! let hamster = HamsterNarrator::default();
//! let breathing = BreathingAnimation {
//!     amplitude: 0.1,
//!     frequency: 2.0,
//! };
//! ```
pub mod animation {
    // ...
}
```

**Every public struct/enum/function**:

```rust
/// Applies a breathing animation to an entity.
///
/// The body scales up and down sinusoidally while maintaining
/// approximate area preservation (squash & stretch effect).
///
/// # Arguments
///
/// * `amplitude` - Maximum scale deviation (0.0 = no breathing)
/// * `frequency` - Cycles per second (Hz)
///
/// # Panics
///
/// Panics if `frequency` is 0 or negative.
///
/// # Example
///
/// ```ignore
/// let breathing = BreathingAnimation::new(0.1, 2.0);
/// ```
pub fn new(amplitude: f32, frequency: f32) -> Self {
    assert!(frequency > 0.0, "frequency must be positive");
    Self { amplitude, frequency }
}
```

### Comment Guidelines

**DON'T** comment the obvious:

```rust
// ❌ Bad: comment states what code does
let corruption = corruption + 10.0;  // Add 10 to corruption

// ✅ Good: comment explains why
let corruption = corruption + 10.0;  // Intensify corruption effect for player choice
```

**DO** comment non-obvious decisions:

```rust
// ✅ Good: explains a performance choice
// Cache palette textures to avoid repeated allocation on every render.
// Profile shows 12ms savings when rendering 100+ hamsters.
let palettes = self.palette_cache.get_or_insert_with(|| { ... });
```

**DO** use `TODO` / `FIXME` with issue links:

```rust
// TODO: Replace with GPU instancing (see issue #123)
// Current approach serializes 100+ transforms per frame.
for hamster in hamsters.iter() {
    render_hamster(hamster);
}
```

---

## 7. Safety & Unwrap Usage

### Minimize `unwrap()` and `expect()`

```rust
// ❌ Bad: Panics on invalid enum
let expr = match string {
    "happy" => Expression::Happy,
    "angry" => Expression::Angry,
    _ => panic!("Invalid expression"),  // Caller can't handle this
};

// ✅ Good: Returns error, caller decides what to do
fn parse_expression(s: &str) -> Result<Expression> {
    match s {
        "happy" => Ok(Expression::Happy),
        "angry" => Ok(Expression::Angry),
        other => Err(AnimationError::InvalidExpression(other.to_string())),
    }
}
```

### Safe Unwrap Patterns

Only use `unwrap()` when **impossible to fail**:

```rust
// ✅ Acceptable: regex pattern is compile-time checked
let pattern = regex::Regex::new(r"^\d+$").unwrap();

// ✅ Acceptable: Lua function is guaranteed to exist
let init = lua.globals().get::<_, mlua::Function>("init").unwrap();

// ❌ Never: User could provide invalid data
let num: u32 = user_input.parse().unwrap();  // Should use ? or match
```

---

## 8. Testing

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Arrange-Act-Assert pattern
    #[test]
    fn test_breathing_animation_scales_correctly() {
        // Arrange
        let mut anim = BreathingAnimation::new(0.1, 1.0);
        let initial_scale = 1.0;

        // Act
        let scaled = anim.update_and_get_scale(0.25);  // 1/4 cycle

        // Assert
        assert!(scaled > initial_scale);
        assert!(scaled < initial_scale + 0.2);
    }

    // Test error cases
    #[test]
    fn test_corruption_bounds() {
        let mut narrator = HamsterNarrator::default();

        narrator.set_corruption(-50.0);
        assert_eq!(narrator.corruption, 0.0);

        narrator.set_corruption(150.0);
        assert_eq!(narrator.corruption, 100.0);
    }
}
```

### Naming Tests

```rust
#[test]
fn test_<function>_<scenario>_<expected_outcome>() { }

// Examples
#[test]
fn test_corruption_at_100_applies_maximum_distortion() { }

#[test]
fn test_breathing_with_zero_frequency_panics() { }

#[test]
fn test_load_animation_with_missing_file_returns_error() { }
```

---

## 9. Performance Considerations

### Avoid Allocations in Hot Loops

```rust
// ❌ Bad: Allocates new Vec every frame
fn render_system(query: Query<&Transform>) {
    let transforms = query.iter().collect::<Vec<_>>();  // Allocation!
    for t in transforms {
        render(t);
    }
}

// ✅ Good: No allocation
fn render_system(query: Query<&Transform>) {
    for t in query.iter() {
        render(t);
    }
}
```

### Cache Expensive Computations

```rust
// ✅ Good: Compute once, reuse
fn breathing_system(mut query: Query<&mut Transform, With<BreathingMarker>>, time: Res<Time>) {
    let time_sin = (time.elapsed_secs() * 2.0 * PI).sin();  // Compute once
    for mut transform in query.iter_mut() {
        transform.scale.y = 1.0 + time_sin * 0.1;
    }
}
```

### Use Queries Efficiently

```rust
// ❌ Bad: Iterates all entities twice
for (hamster, _) in query1.iter() {
    for (_, transform) in query2.iter() {
        // ...
    }
}

// ✅ Good: Single query join
for (hamster, transform) in query.iter() {
    // ...
}
```

---

## 10. Dependency Management

### Prefer Workspace Dependencies

Always reference `Cargo.toml` workspace definitions:

```toml
# ❌ Bad: Inconsistent versions across crates
[dependencies]
bevy = "0.14"       # In dj_engine
bevy = "0.13"       # In doomexe (MISMATCH!)

# ✅ Good: Central version management
[workspace.dependencies]
bevy = "0.14"

# In dj_engine/Cargo.toml
[dependencies]
bevy = { workspace = true }

# In games/dev/doomexe/Cargo.toml
[dependencies]
bevy = { workspace = true }
```

### Minimize External Dependencies

- Add only if it solves a real problem
- Prefer smaller, focused crates over large frameworks
- Document why a dependency was added in comments

---

## 11. Common Patterns

### Builder Pattern (for Complex Initialization)

```rust
pub struct HamsterRenderer {
    resolution: UVec2,
    palette: Handle<Palette>,
    corruption_intensity: f32,
}

impl HamsterRenderer {
    pub fn new(resolution: UVec2) -> Self {
        Self {
            resolution,
            palette: Handle::default(),
            corruption_intensity: 0.0,
        }
    }

    pub fn with_palette(mut self, palette: Handle<Palette>) -> Self {
        self.palette = palette;
        self
    }

    pub fn with_corruption(mut self, intensity: f32) -> Self {
        self.corruption_intensity = intensity.clamp(0.0, 1.0);
        self
    }
}

// Usage
let renderer = HamsterRenderer::new(UVec2::new(320, 240))
    .with_palette(default_palette)
    .with_corruption(0.5);
```

### Resource Wrapper Pattern (for Game State)

```rust
#[derive(Resource)]
pub struct HamsterState {
    narrator: HamsterNarrator,
}

impl HamsterState {
    pub fn get_corruption(&self) -> f32 {
        self.narrator.corruption
    }

    pub fn set_corruption(&mut self, value: f32) {
        self.narrator.corruption = value.clamp(0.0, 100.0);
    }
}
```

---

## 12. Linting & Clippy

### Clippy Rules (Enforced)

All code **must** pass:

```bash
cargo clippy --workspace -- -D warnings
```

### Common Clippy Violations to Fix

| Warning | Fix |
|---------|-----|
| `clippy::needless_borrow` | Remove unnecessary `&` |
| `clippy::should_implement_trait` | Implement standard trait (e.g., `From`) |
| `clippy::match_like_matches_macro` | Use `matches!()` |
| `clippy::or_fun_call` | Use `.unwrap_or_else()` |
| `clippy::too_many_arguments` | Extract into struct or group logically |

---

## 13. Code Review Checklist (For Reviewers)

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Public APIs are documented
- [ ] Naming follows conventions
- [ ] No obvious performance issues (allocations in hot loops, etc.)
- [ ] Error handling is appropriate (Result vs panic)
- [ ] Complex logic has explanatory comments
- [ ] No debug prints or TODO without issue links
- [ ] Commit messages are clear

---

## 14. Quick Start Template

### New Module

```rust
//! [Brief description of module purpose].
//!
//! # Example
//!
//! ```ignore
//! // Usage example
//! ```

use bevy::prelude::*;

#[derive(Component)]
pub struct MyComponent {
    pub value: f32,
}

impl MyComponent {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, my_system);
    }
}

fn my_system(mut query: Query<&mut MyComponent>) {
    for mut component in query.iter_mut() {
        // Implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let comp = MyComponent::new(42.0);
        assert_eq!(comp.value, 42.0);
    }
}
```

---

## 15. Gradual Enforcement

**Phase 1 (Week 1)**: Guidelines only  
**Phase 2 (Week 2)**: Gentle reminders in code review  
**Phase 3 (Week 3+)**: Enforce in CI (clippy, fmt, tests)

No one's expected to memorize these immediately. Ask questions in PR reviews!

---

## Conclusion

These standards balance **quality** (no footguns) with **pragmatism** (not bureaucratic). They exist to make code:
- Easier to read for future maintainers
- Easier to debug when things break
- Easier to refactor without breaking things

Questions? Ask in the team channel or open a GitHub discussion.
