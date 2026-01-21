# DJ Engine Development Workflow

**Version**: 1.0  
**Last Updated**: 2026-01-20  
**Audience**: DJ Engine contributors

---

## Overview

This document defines the development workflow for DJ Engine—from planning through code review to deployment. It's designed to be lightweight (Gemini in VS Code, no CI/CD overkill) while maintaining quality and preventing common mistakes.

---

## 1. Development Phases

### Phase 1: Planning (Before Code)
1. **Create GitHub Issue** describing the feature/fix
2. **Link to Milestone** (Milestone 1, 2, etc.)
3. **Assign to self** and set priority label
4. **Wait for feedback** before starting implementation

### Phase 2: Implementation (Local Development)
1. **Create feature branch** from `main`: `git checkout -b feature/hamster-breathing`
2. **Follow code standards** (see CODING_STANDARDS.md)
3. **Commit frequently** with clear messages
4. **Test locally** before pushing
5. **Push to GitHub** when ready for review

### Phase 3: Review (Before Merging)
1. **Open Pull Request** linking to the issue
2. **Self-review** your code (catch obvious errors)
3. **Request review** from team members
4. **Address feedback** in follow-up commits
5. **Squash or rebase** before merge if requested

### Phase 4: Integration (After Merge)
1. **Verify main branch builds** locally
2. **Close related issues**
3. **Update CHANGELOG.md**
4. **Tag release** if applicable (v0.1.0, v0.2.0, etc.)

---

## 2. Branch Naming Convention

| Type | Pattern | Example |
|------|---------|---------|
| Feature | `feature/<feature-name>` | `feature/hamster-breathing` |
| Bug fix | `fix/<bug-name>` | `fix/palette-swap-corruption` |
| Refactor | `refactor/<system-name>` | `refactor/animation-components` |
| Documentation | `docs/<topic>` | `docs/lua-ffi-guide` |
| Experiment | `exp/<experiment-name>` | `exp/gpu-instancing` |

**Naming Rules**:
- Use lowercase only
- Use hyphens (not underscores)
- Be specific and descriptive
- Max 50 characters

---

## 3. Commit Message Standard

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code refactor (no feature change)
- `docs`: Documentation only
- `test`: Test additions/fixes
- `chore`: Build, dependencies, tooling
- `perf`: Performance improvement

### Scope
- `engine`: Core engine code
- `rendering`: Rendering system
- `animation`: Animation system
- `scripting`: Lua FFI
- `assets`: Asset loading/pipeline
- `doomexe`: Game-specific code
- `workspace`: Cargo.toml, CI, etc.

### Subject
- Imperative mood ("add" not "added")
- No period at end
- Max 50 characters
- Lowercase except proper nouns

### Body (Optional)
- Explain **why**, not what
- Wrap at 72 characters
- Reference issue numbers: `Fixes #123`

### Examples

**Good**:
```
feat(animation): add breathing animation for hamster body

Implement sine-wave scale animation with area preservation.
Body scales up/down while maintaining visual consistency.
Easing curve smoothed with cubic interpolation.

Fixes #42
```

**Good**:
```
fix(rendering): correct palette swap shader sampler bounds

Palette texture was sampling outside bounds at high corruption.
Clamp texture coordinates to [0.0, 1.0] in shader.

Fixes #87
```

**Bad**:
```
update code

fixed some stuff

Fixes the palette
```

---

## 4. Code Review Process

### For Authors (Asking for Review)

1. **Self-review first**
   - Read through your entire PR
   - Check for obvious errors, typos, logic flaws
   - Ensure code follows CODING_STANDARDS.md
   - Verify tests are passing

2. **Write a clear PR description**
   ```markdown
   ## Description
   Brief summary of changes.

   ## Type of Change
   - [ ] New feature
   - [ ] Bug fix
   - [ ] Breaking change
   - [ ] Documentation update

   ## Related Issues
   Fixes #123

   ## Testing
   How was this tested? What scenarios?

   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Self-review completed
   - [ ] Comments added for complex logic
   - [ ] Documentation updated
   - [ ] Tests pass locally
   ```

3. **Request specific reviewers**
   - For rendering changes → ask reviewer familiar with Bevy
   - For Lua changes → ask reviewer familiar with mlua
   - For systems/architecture → ask reviewer who understands ECS patterns

4. **Respond to feedback promptly**
   - Don't take criticism personally
   - Ask for clarification if feedback is unclear
   - Push fixes in new commits (don't force-push)
   - Mark conversations as resolved when addressed

### For Reviewers (Reviewing Code)

**Goal**: Help maintainers write better code. Be kind, specific, and constructive.

**What to Check**:
1. ✅ Does it solve the stated problem?
2. ✅ Does it follow CODING_STANDARDS.md?
3. ✅ Is the code readable and maintainable?
4. ✅ Are error cases handled?
5. ✅ Does it integrate cleanly with existing code?
6. ✅ Are there performance concerns?
7. ✅ Is documentation complete?

**Comment Types**:
- 🟢 **Suggestion** (non-blocking): "Consider using X instead of Y"
- 🟡 **Note** (should address): "This could cause issue Z in scenario W"
- 🔴 **Blocking** (must fix): "This breaks API contract / violates safety"

**Example Review Comment**:
```
🟡 Note: The `HashMap` lookup in `render()` happens every frame.
Consider caching the result in a resource to avoid repeated allocation.
See PERFORMANCE.md for profiling guidance.

Suggestion: Add a test for the corruption bounds check?
Currently only edge cases 0 and 100 are tested.
```

---

## 5. Testing Standards

### Unit Tests (In Module)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corruption_bounds_clamp() {
        let mut hamster = HamsterNarrator::default();
        hamster.set_corruption(-50.0);
        assert_eq!(hamster.corruption, 0.0);
        
        hamster.set_corruption(150.0);
        assert_eq!(hamster.corruption, 100.0);
    }
}
```

### Integration Tests

Located in `tests/` at crate root:

```
tests/
├── common/
│   └── mod.rs          # Shared test utilities
├── hamster_assembly.rs # Tests for hamster part loading
└── lua_ffi.rs          # Tests for Lua integration
```

### Test Naming Convention

- `test_<function>_<scenario>`
- `test_<function>_<scenario>_<expected_result>`

**Examples**:
- `test_breathing_animation_scales_correctly()`
- `test_corruption_at_max_triggers_distortion()`
- `test_lua_set_expression_invalid_returns_error()`

### Running Tests Locally

```bash
# Test everything
cargo test --workspace

# Test specific crate
cargo test -p dj_engine

# Test with logging
RUST_LOG=debug cargo test --workspace -- --nocapture

# Test single test
cargo test test_breathing_animation_scales_correctly
```

---

## 6. Documentation Requirements

### Every Public Module Needs

- **Module doc comment** (in `mod.rs`)
  ```rust
  //! Breathing and idle motion animation systems.
  //!
  //! This module implements procedural animation curves for organic motion:
  //! - Sine-wave breathing with area preservation
  //! - Perlin noise-based idle jitter
  //! - Customizable easing functions
  ```

- **Public struct/function doc comments**
  ```rust
  /// Applies a breathing animation to the hamster's body.
  ///
  /// # Arguments
  /// * `amplitude` - Max scale deviation (0.0 = no breathing)
  /// * `frequency` - Cycles per second (Hz)
  ///
  /// # Panics
  /// Panics if frequency is 0 or negative.
  ///
  /// # Example
  /// ```ignore
  /// let breathing = BreathingAnimation::new(0.1, 2.0);
  /// ```
  pub fn apply_breathing(amplitude: f32, frequency: f32) { }
  ```

- **Example code** in doc comments (use `/// #[ignore]` for untestable examples)

### README.md in Every Module

Optional but recommended for complex systems:

```markdown
# Animation System

## Components
- `BreathingAnimation` - Body scale animation
- `BlinkingAnimation` - Eye state transitions
- `IdleMotion` - Noise-driven jitter

## Systems
- `apply_breathing_system()`
- `update_blink_timer_system()`
- `apply_idle_motion_system()`

## Usage Example
[Code example here]
```

---

## 7. Local Development Setup

### Initial Setup

```bash
# Clone repository
git clone https://github.com/yourusername/dj_engine.git
cd dj_engine

# Create local branch for your work
git checkout -b feature/your-feature

# Verify build works
cargo check --workspace
cargo test --workspace
```

### Before Committing

```bash
# Format code (required)
cargo fmt --all

# Check for issues
cargo clippy --workspace -- -D warnings

# Run tests
cargo test --workspace

# Verify it compiles
cargo check --workspace

# Then commit
git add .
git commit -m "feat(scope): description"
```

### Useful Local Commands

```bash
# Watch for changes and rebuild
cargo watch -x check -x test

# Build release binary
cargo build --release -p doomexe

# Run with debug logging
RUST_LOG=debug cargo run -p doomexe

# Generate documentation
cargo doc --no-deps --open
```

---

## 8. Pull Request Checklist

Before marking a PR as "Ready for Review":

- [ ] Branch is up-to-date with `main`
- [ ] `cargo fmt --all` run and committed
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo test --workspace` passes
- [ ] All public APIs have doc comments
- [ ] Related issue is linked in PR description
- [ ] Commit messages follow standard (see §3)
- [ ] No debug prints left in code
- [ ] No `TODO` comments without associated issues
- [ ] CHANGELOG.md updated (if applicable)

---

## 9. Handling Large Changes

### For Features > 500 LOC or Complex Architecture

1. **Open a Draft PR early**
   - Describe the approach and design decisions
   - Solicit feedback before writing all the code
   - Prevents wasted effort on wrong direction

2. **Break into smaller commits**
   - Each commit should be a logical, compilable unit
   - Easier to review and understand
   - Easier to revert if needed

3. **Include a migration guide**
   - If this breaks existing APIs, document breaking changes
   - Provide before/after code examples
   - Link to MIGRATION.md if one exists

### Example Draft PR Description

```markdown
## Design: Palette Swap Shader Integration

### Problem
Currently hardcoding palette transitions. Need shader-driven approach.

### Proposed Solution
1. Define `CorruptionUniforms` struct with palette index
2. Create WGSL shader for palette lookup
3. Expose `set_corruption()` to Lua

### Questions for Review
- Should we pre-allocate palette textures or load on-demand?
- Do we need to support > 256-color palettes?
- Should palette be configurable per scene?

### Implementation Plan
- [x] Shader code
- [ ] Rust FFI integration
- [ ] Lua binding
- [ ] Tests and documentation

Feedback welcome before I continue!
```

---

## 10. Merge Strategy

### Squash vs. Rebase

For **Milestone 1 work**:
- **Small PRs (< 5 commits)**: Squash and merge
- **Large PRs (> 5 commits)**: Rebase and merge (preserves history)

Use GitHub's **"Squash and merge"** button unless you have a good reason not to.

### Before Merging

1. **All checks pass** (tests, clippy, formatting)
2. **At least one approval** from another contributor
3. **Branch is up-to-date** with main (GitHub's "Update branch" button)
4. **Conversation resolved** (no outstanding questions)

---

## 11. Release Process

### Versioning Scheme

Use Semantic Versioning: `MAJOR.MINOR.PATCH`

- **MAJOR** (0.X.0): Breaking changes to engine API
- **MINOR** (X.1.0): New features, backwards-compatible
- **PATCH** (X.X.1): Bug fixes only

**Milestone 1**: v0.1.0  
**Milestone 2**: v0.2.0  
**etc.**

### Release Checklist

1. **Update version** in `Cargo.toml` and `Cargo.lock`
2. **Update CHANGELOG.md** with all changes since last release
3. **Tag commit** with version: `git tag v0.1.0`
4. **Push tag** to GitHub: `git push origin v0.1.0`
5. **Create GitHub Release** with changelog summary

---

## 12. Common Gotchas & Solutions

| Problem | Solution |
|---------|----------|
| "My code compiles locally but CI fails" | Ensure you ran `cargo test --workspace` and `cargo clippy` |
| "I committed to main by mistake" | Use `git reset HEAD~1` to undo, then create proper branch |
| "Merge conflict on Cargo.lock" | Don't manually edit; run `cargo update` and commit result |
| "Bevy shader won't load" | Check file path is relative to `assets/` root; use `asset_server.load()` |
| "Lua FFI segfaults" | Wrap FFI calls in `catch_unwind()` or `panic = "abort"` in Cargo.toml |
| "Hot-reload breaks game state" | Read SCRIPTING.md for preservation guidelines |

---

## 13. Escalation Process

### If You're Stuck

1. **Check documentation** first (see Part 10 of PROJECT_PLAN.md)
2. **Search GitHub issues** for similar problems
3. **Ask in Slack** or create a GitHub Discussion (don't DM)
4. **Create a minimal reproducible example** if it's a bug

### If You Have a Disagreement

1. **Talk it through** with the person directly
2. **Reference design docs** to ground discussion in shared understanding
3. **Create a GitHub Issue** if it's a design decision that affects multiple people
4. **Don't merge controversial changes** without discussion

---

## 14. Remote Collaboration Guidelines

Since you're using "Gemini in VS Code" (presumably Google's Gemini API):

### For AI-Assisted Development

- ✅ Use AI for boilerplate, documentation, test generation
- ✅ Use AI for explaining error messages and debugging
- ✅ **Always review AI-generated code** before committing
- ❌ Don't blindly copy-paste large AI completions
- ❌ Don't commit code you don't understand

### Commit Message for AI-Generated Code

```
feat(rendering): add palette texture management

Generated with AI assistance; manually reviewed and tested.
Refs #45
```

---

## Conclusion

This workflow prioritizes:
1. **Clarity** (commit messages, PR descriptions)
2. **Quality** (testing, code review, standards)
3. **Efficiency** (lightweight process, no overkill)
4. **Collaboration** (clear feedback, kind reviews)

Start with this, iterate based on what works for your team.
