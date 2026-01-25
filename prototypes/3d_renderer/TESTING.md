# Testing Guide - Bevy 3D Renderer

## Running Tests

### Run All Tests
```bash
# Run all tests (library + integration)
cargo test

# Run only library tests
cargo test --lib

# Run only integration tests
cargo test --test integration_test

# Run with output visible
cargo test -- --nocapture

# Run specific test
cargo test test_camera_plugin_builds
```

### Run Tests in Release Mode (Faster)
```bash
# Build tests in release mode
cargo test --release

# Run tests without recompiling (if already built)
cargo test --release --no-run
```

## Test Coverage

### Install Coverage Tools

#### Option 1: tarpaulin (Recommended for Linux)
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html

# View the report
open tarpaulin-report.html
```

#### Option 2: grcov (Cross-platform)
```bash
# Install grcov and llvm-tools
cargo install grcov
rustup component add llvm-tools-preview

# Set environment variables
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="coverage/your_name-%p-%m.profraw"

# Run tests
cargo test

# Generate coverage report
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch \
  --ignore-not-existing --ignore "/*" -o coverage/

# View the report
open coverage/index.html
```

#### Option 3: cargo-llvm-cov
```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Run tests with coverage
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Generate HTML report
cargo llvm-cov --all-features --workspace --html

# View the report
open target/llvm-cov/html/index.html
```

### Quick Coverage Script

Create `coverage.sh`:

```bash
#!/bin/bash
set -e

echo "Running tests with coverage..."

# Clean previous coverage
cargo clean
rm -rf coverage/

# Set coverage flags
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off"
export LLVM_PROFILE_FILE="coverage/your_name-%p-%m.profraw"

# Run tests
cargo test --all-features

# Generate report
grcov . --binary-path ./target/debug/deps/ \
  --source-dir . \
  --output-type html \
  --output-path coverage/ \
  --ignore "target/*" \
  --ignore "tests/*" \
  --ignore "test_models/*" \
  --ignore ".cargo/*"

echo "Coverage report generated at coverage/index.html"
open coverage/index.html
```

Make it executable:
```bash
chmod +x coverage.sh
./coverage.sh
```

## CI/CD Integration

### GitHub Actions

Create `.github/workflows/test.yml`:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Run clippy
        run: cargo clippy -- -D warnings
      
      - name: Check formatting
        run: cargo fmt -- --check

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      
      - name: Generate coverage
        run: cargo tarpaulin --out Xml --all-features --workspace
      
      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          file: cobertura.xml
          fail_ci_if_error: true
```

## Test Organization

### Unit Tests (in `src/`)

```rust
// In src/plugins/camera.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_camera_creation() {
        let camera = OrbitCamera::default();
        assert_eq!(camera.radius, 10.0);
        assert_eq!(camera.yaw, 0.0);
    }

    #[test]
    fn test_camera_transform_calculation() {
        let camera = OrbitCamera {
            center: Vec3::ZERO,
            radius: 5.0,
            yaw: 0.0,
            pitch: 0.5,
        };
        
        // Test transform calculation
        // ...
    }
}
```

### Integration Tests (in `tests/`)

```rust
// In tests/plugins_test.rs
use bevy::prelude::*;

#[test]
fn test_full_rendering_pipeline() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins)
       .add_plugins(CameraPlugin)
       .add_plugins(LightingPlugin)
       .add_plugins(ModelPlugin);
    
    app.update();
    
    // Verify scene has entities
    let entity_count = app.world().entities().len();
    assert!(entity_count > 0);
}
```

### Benchmark Tests

```rust
// benches/rendering_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bevy::prelude::*;

fn benchmark_scene_setup(c: &mut Criterion) {
    c.bench_function("scene_setup", |b| {
        b.iter(|| {
            let mut app = App::new();
            app.add_plugins((
                MinimalPlugins,
                CameraPlugin,
                LightingPlugin,
            ));
            
            black_box(app);
        })
    });
}

criterion_group!(benches, benchmark_scene_setup);
criterion_main!(benches);
```

## Continuous Testing

### Watch Mode

```bash
# Install cargo-watch
cargo install cargo-watch

# Run tests on file changes
cargo watch -x test

# Run specific tests on change
cargo watch -x "test test_camera_plugin_builds"

# Run with coverage on change
cargo watch -x "tarpaulin --out Html"
```

### Pre-commit Hooks

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e

echo "Running tests..."
cargo test --all-features

echo "Running clippy..."
cargo clippy -- -D warnings

echo "Checking formatting..."
cargo fmt -- --check

echo "All checks passed!"
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Current Test Status

### Existing Tests

**Unit Tests:**
- `test_camera_plugin_builds` - Verifies CameraPlugin compiles
- `test_lighting_plugin_builds` - Verifies LightingPlugin compiles  
- `test_model_plugin_builds` - Verifies ModelPlugin compiles

**Integration Tests:**
- `test_full_app_initialization` - Tests full app startup
- `test_scene_setup` - Tests basic scene creation

### Running Existing Tests

```bash
# Run all existing tests
cargo test

# Expected output (approximate):
#     test tests::test_camera_plugin_builds ... ok
#     test tests::test_lighting_plugin_builds ... ok
#     test tests::test_model_plugin_builds ... ok
#     test integration_test::test_full_app_initialization ... ok
#     test integration_test::test_scene_setup ... ok
```

## Coverage Targets

### Minimum Coverage Requirements

- **Line Coverage:** 70% minimum
- **Branch Coverage:** 60% minimum
- **Plugin Coverage:** 80% minimum (critical business logic)

### Current Coverage Estimate

Based on current code structure:

- **src/plugins/camera.rs:** 40% (orbit system not tested)
- **src/plugins/lighting.rs:** 30% (animation not tested)
- **src/plugins/models.rs:** 25% (GLTF loading partially tested)
- **src/main.rs:** 10% (mostly integration code)

**Overall Estimated Coverage:** ~25-30%

### To Improve Coverage

1. Test camera orbit calculations
2. Test light animation systems
3. Test GLTF loading states
4. Test material creation
5. Add rendering integration tests

## Documentation

### Generate Documentation

```bash
# Generate and open documentation
cargo doc --open --no-deps

# Generate documentation with coverage
cargo doc --document-private-items
```

### Test Documentation

```bash
# Test code examples in documentation
cargo test --doc
```

## Performance Testing

### Frame Rate Monitoring

Add to `src/main.rs`:

```rust
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            // ... other plugins
        ))
        // ...
        .run();
}
```

### Run with Performance Logging

```bash
# Run with diagnostics
cargo run --release 2>&1 | grep -E "(fps|frame|ms)"
```

## Troubleshooting

### Tests Won't Compile

```bash
# Clean and rebuild
cargo clean
cargo test --no-run

# Check specific errors
cargo check --tests
```

### Coverage Not Working

```bash
# Ensure llvm-tools are installed
rustup component add llvm-tools-preview

# Check grcov version
grcov --version

# Use cargo-llvm-cov as alternative
cargo llvm-cov --html
```

### Tests Timeout

```bash
# Increase test timeout
cargo test -- --test-threads=1 --timeout=120

# Run specific test first
cargo test test_camera_plugin_builds
```

## Summary Commands

```bash
# Complete test workflow
cargo clean
cargo test --all-features

# Coverage with tarpaulin (recommended)
cargo tarpaulin --out Html --ignore-tests
open tarpaulin-report.html

# Quick check
cargo check --tests
cargo clippy --all-targets

# Format check
cargo fmt -- --check

# Documentation tests
cargo test --doc
```
