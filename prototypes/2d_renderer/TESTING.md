# Testing Guide - Bevy 2D Renderer

This document describes how to run tests and check code coverage for the Bevy 2D Renderer project.

## 📋 Overview

The project includes:
- **Unit Tests**: Tests for individual modules (components, resources, state)
- **Integration Tests**: Tests for system interactions
- **Coverage Reports**: Code coverage analysis with tarpaulin

## 🚀 Quick Start

### Run All Tests
```bash
cd /mnt/c/Users/Mike/Documents/dj_engine/prototypes/2d_renderer
./test.sh
```

Or use cargo directly:
```bash
cargo test
```

## 🔧 Test Types

### 1. Unit Tests (Library Tests)

Located in the source files as `#[cfg(test)]` modules:
- `src/components.rs` - Component creation and validation
- `src/resources.rs` - Resource default values and custom values
- `src/state.rs` - State enum validation

**Run unit tests only:**
```bash
./test.sh unit
```

**Run with verbose output:**
```bash
./test.sh unit true
```

### 2. Integration Tests

Located in `tests/` directory:
- `tests/systems_test.rs` - System interaction tests

**Run integration tests only:**
```bash
./test.sh integration
```

### 3. Documentation Tests

Tests in doc comments (none currently, but can be added):

**Run doc tests:**
```bash
./test.sh doc
```

### 4. Code Coverage

Generate HTML coverage reports using [cargo-tarpaulin](https://github.com/xd009642/tarpaulin):

**Generate coverage report:**
```bash
./test.sh coverage
```

This will:
1. Install `cargo-tarpaulin` if not present
2. Generate coverage report in `target/coverage/`
3. Attempt to open the HTML report in your browser

**Coverage report location:**
```
target/coverage/index.html
```

### 5. Code Quality Checks

#### Clippy (Linting)
```bash
./test.sh lint
```

#### Format Checking
```bash
./test.sh format
```

Format code if needed:
```bash
cargo fmt
```

## 📊 Coverage Thresholds

Target coverage goals:
- **Components**: 100% (simple structs with clear behavior)
- **Resources**: 100% (default implementations tested)
- **State**: 100% (enum variants tested)
- **Systems**: 80%+ (complex Bevy system logic)
- **Overall**: 85%+

## 🧪 Writing New Tests

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_x() {
        // Arrange
        let input = 10.0;
        
        // Act
        let result = function_to_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Bevy System Test Example

```rust
use bevy::prelude::*;

#[test]
fn test_bevy_system() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<MousePosition>();
    
    app.update();
    
    assert!(app.world().contains_resource::<MousePosition>());
}
```

## 🎯 Continuous Integration

For CI/CD pipelines, use:

```bash
# Full test suite
./test.sh all

# Or individual steps
cargo test --lib
cargo test --test '*'
cargo clippy -- -D warnings
cargo fmt -- --check

# Generate coverage for CI
 cargo tarpaulin --out Xml --output-dir ./coverage/
```

## 🐛 Troubleshooting

### Tests Won't Compile

**Problem**: Tests timeout or take too long
**Solution**: Run with `--no-run` first to compile, then run tests
```bash
cargo test --no-run
cargo test
```

### Bevy System Tests Fail

**Problem**: "Resource not found" errors
**Solution**: Ensure you're adding all necessary plugins and resources
```rust
app.add_plugins(MinimalPlugins)
    .init_resource::<YourResource>();
```

### Coverage Report Won't Generate

**Problem**: `cargo-tarpaulin` installation fails
**Solution**: Install required system packages:

**Ubuntu/Debian:**
```bash
sudo apt-get install libssl-dev pkg-config
```

**Fedora/RHEL:**
```bash
sudo dnf install openssl-devel pkg-config
```

**Arch Linux:**
```bash
sudo pacman -S openssl pkg-config
```

### WSL2 Audio Warnings

**Problem**: ALSA warnings when running tests
**Solution**: These are harmless and can be ignored. To suppress:
```bash
export ALSA_CARD=Generic
./test.sh
```

## 📈 Current Test Coverage

### Test Files
✅ `src/components.rs` - 3 tests  
✅ `src/resources.rs` - 4 tests  
✅ `src/state.rs` - 5 tests  
✅ `tests/systems_test.rs` - 8 tests  

**Total: 20 tests**

Run `./test.sh` to see current test results.

## 🎓 Best Practices

### 1. Test Naming
Use descriptive test names: `test_feature_expected_behavior`

### 2. Arrange-Act-Assert
Structure tests clearly:
```rust
// Arrange - setup
// Act - execute
// Assert - verify
```

### 3. Test Independence
Each test should be independent and not rely on other tests.

### 4. Edge Cases
Test edge cases and error conditions:
- Zero values
- Maximum values
- Invalid inputs

### 5. Bevy Specific
- Use `MinimalPlugins` for fast tests
- Test systems in isolation when possible
- Use `app.update()` to run schedules

## 🔍 Example Test Output

```bash
$ ./test.sh unit

==========================================
Bevy 2D Renderer - Test Suite
==========================================

[INFO] Running test suite with type: unit

[INFO] Building project...
   Compiling bevy-2d-renderer v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 12.34s

[INFO] Running Unit Tests (Modules)...
-----------------------------------

running 12 tests
test components::tests::test_animation_timer_creation ... ok
test components::tests::test_parallax_layer_creation ... ok
test resources::tests::test_camera_settings_custom ... ok
test resources::tests::test_camera_settings_default ... ok
test state::tests::test_app_state_variants ... ok
test state::tests::test_game_state_default ... ok
...

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured

[SUCCESS] Unit tests passed!

==========================================
[SUCCESS] Test suite completed successfully!
==========================================
```

## 📚 Additional Resources

- [Bevy Testing Examples](https://github.com/bevyengine/bevy/tree/main/examples#testing)
- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo-tarpaulin README](https://github.com/xd009642/tarpaulin)
- [Bevy Cheat Sheet](https://bevy-cheatbook.github.io/testing.html)

## 🆘 Getting Help

If tests are failing:
1. Check that all dependencies are installed
2. Run `./check.sh` to verify asset setup
3. Check Rust version: `rustc --version` (should be 1.75+)
4. Try `cargo clean` and rebuild
5. Open an issue with test output

---

**Happy Testing!** 🎉
