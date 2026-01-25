#!/bin/bash
# Quick test runner for Bevy 3D Renderer

echo "====================================="
echo "Bevy 3D Renderer - Test Runner"
echo "====================================="

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if tarpaulin is installed
check_tarpaulin() {
    if command -v cargo-tarpaulin &> /dev/null; then
        echo -e "${GREEN}✓${NC} tarpaulin detected"
        return 0
    else
        echo -e "${YELLOW}!${NC} tarpaulin not found (install with: cargo install cargo-tarpaulin)"
        return 1
    fi
}

# Run basic tests
echo ""
echo "1. Running unit tests..."
cargo test --lib --quiet

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Unit tests passed"
else
    echo -e "${YELLOW}!${NC} Some unit tests failed"
    exit 1
fi

# Run integration tests
echo ""
echo "2. Running integration tests..."
cargo test --test integration_test --quiet

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Integration tests passed"
else
    echo -e "${YELLOW}!${NC} Some integration tests failed"
    exit 1
fi

# Check for tarpaulin and run coverage if available
echo ""
echo "3. Checking for coverage tools..."
if check_tarpaulin; then
    echo ""
    echo "4. Running coverage analysis..."
    echo "   (This may take a few minutes on first run)"
    cargo tarpaulin --out Stdout --quiet
else
    echo ""
    echo -e "${YELLOW}Note:${NC} Install tarpaulin for coverage reports:"
    echo "      cargo install cargo-tarpaulin"
fi

# Summary
echo ""
echo "====================================="
echo -e "${GREEN}All tests completed successfully!${NC}"
echo "====================================="
echo ""
echo "Test Coverage Tips:"
echo "  • View full testing guide: cat TESTING.md"
echo "  • Generate HTML coverage: ./coverage.sh"
echo "  • Run specific test: cargo test test_name"
echo "  • Watch mode: cargo watch -x test"
echo ""
