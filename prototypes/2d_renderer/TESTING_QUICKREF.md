# Testing Quick Reference Card

## 🚀 Quick Commands

```bash
# Run everything
./test.sh all

# Unit tests only
./test.sh unit

# Integration tests only
./test.sh integration

# Coverage report (HTML)
./test.sh coverage

# Code quality (lint + format)
./test.sh lint
./test.sh format
```

## 🧪 Test Structure

```
src/
├── components.rs     # 3 tests ✅
├── resources.rs      # 4 tests ✅
├── state.rs          # 5 tests ✅
tests/
└── systems_test.rs   # 8 tests ✅
```

**Total: 20 tests**

## 📊 Coverage

```bash
# Generate HTML coverage report
./test.sh coverage

# View report
target/coverage/index.html
```

Target: **85%+** overall coverage

## 🔍 Common Issues

**Tests won't compile?**
```bash
cargo clean
cargo test --no-run  # Compile first
cargo test           # Then run
```

**Install tarpaulin (for coverage)**
```bash
cargo install cargo-tarpaulin
```

**Ubuntu/Debian dependencies**
```bash
sudo apt-get install libssl-dev pkg-config
```

## 📝 Example Test

```rust
#[test]
fn test_feature_x() {
    // Arrange
    let input = 10.0;
    
    // Act
    let result = function_to_test(input);
    
    // Assert
    assert_eq!(result, expected_value);
}
```

## 📚 Full Docs

See [TESTING.md](TESTING.md) for complete testing guide.

---

**Keep this card handy!** 🎴
