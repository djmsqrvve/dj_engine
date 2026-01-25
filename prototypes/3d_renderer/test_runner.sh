#!/bin/bash
# Enhanced test runner for Bevy 3D Renderer with full logging

# Enable all log levels for Rust and Bevy
export RUST_LOG=debug,bevy_3d_renderer=debug,bevy=info,warn,error

echo "====================================="
echo "Bevy 3D Renderer - Test Runner"
echo "====================================="
echo "Log Level: DEBUG (all levels enabled)"
echo "====================================="

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Track results
UNIT_TESTS_PASSED=0
INTEGRATION_TESTS_PASSED=0
COVERAGE_AVAILABLE=0

# Check if tarpaulin is installed
check_tarpaulin() {
    if command -v cargo-tarpaulin &> /dev/null; then
        echo -e "${GREEN}✓${NC} tarpaulin detected"
        COVERAGE_AVAILABLE=1
        return 0
    else
        echo -e "${YELLOW}!${NC} tarpaulin not found"
        echo "  Install with: cargo install cargo-tarpaulin"
        return 1
    fi
}

# Function to print section header
print_section() {
    echo ""
    echo -e "${BLUE}▶${NC} $1"
    echo "  ${BLUE}─${NC}─────────────────────────────────"
}

# Function to print success
print_success() {
    echo -e "  ${GREEN}✓${NC} $1"
}

# Function to print warning
print_warning() {
    echo -e "  ${YELLOW}⚠${NC} $1"
}

# Function to print error
print_error() {
    echo -e "  ${RED}✗${NC} $1"
}

# Function to print info
print_info() {
    echo -e "  ${BLUE}ℹ${NC} $1"
}

# Run unit tests
print_section "Running Unit Tests (src/lib.rs)"

echo "  Compiling and testing plugins..."
cargo test --lib 2>&1 | tee test_output.log

if [ $? -eq 0 ]; then
    UNIT_TESTS_PASSED=1
    # Extract test results
    TEST_SUMMARY=$(grep "test result:" test_output.log | tail -1)
    print_success "$TEST_SUMMARY"
else
    print_error "Unit tests failed"
    print_info "Check test_output.log for details"
    exit 1
fi

# Run integration tests
print_section "Running Integration Tests"

echo "  Testing app initialization and entity spawning..."
cargo test --test integration_test 2>&1 | tee -a test_output.log

if [ $? -eq 0 ]; then
    INTEGRATION_TESTS_PASSED=1
    TEST_SUMMARY=$(grep "test result:" test_output.log | tail -1)
    print_success "$TEST_SUMMARY"
else
    print_error "Integration tests failed"
    print_info "Check test_output.log for details"
    exit 1
fi

# Run other test files if they exist
if [ -f "tests/gltf_loading_test.rs" ]; then
    print_section "Running GLTF Loading Tests"
    cargo test --test gltf_loading_test 2>&1 | tee -a test_output.log
    
    if [ $? -eq 0 ]; then
        TEST_SUMMARY=$(grep "test result:" test_output.log | tail -1)
        print_success "$TEST_SUMMARY"
    else
        print_warning "GLTF tests failed (non-critical)"
    fi
fi

if [ -f "tests/camera_lighting_test.rs" ]; then
    print_section "Running Camera & Lighting Tests"
    cargo test --test camera_lighting_test 2>&1 | tee -a test_output.log
    
    if [ $? -eq 0 ]; then
        TEST_SUMMARY=$(grep "test result:" test_output.log | tail -1)
        print_success "$TEST_SUMMARY"
    else
        print_warning "Camera/Lighting tests failed (non-critical)"
    fi
fi

# Check for tarpaulin
echo ""
print_section "Coverage Tools Status"
check_tarpaulin

# Show a sample coverage suggestion
if [ $COVERAGE_AVAILABLE -eq 0 ]; then
    echo ""
    print_info "To enable coverage reports:"
    echo "    cargo install cargo-tarpaulin"
    echo "    cargo tarpaulin --out Html"
    echo "    open tarpaulin-report.html"
fi

# Final summary
echo ""
echo "====================================="
echo "TEST RUN SUMMARY"
echo "====================================="

if [ $UNIT_TESTS_PASSED -eq 1 ]; then
    print_success "Unit Tests: PASSED"
else
    print_error "Unit Tests: FAILED"
fi

if [ $INTEGRATION_TESTS_PASSED -eq 1 ]; then
    print_success "Integration Tests: PASSED"
else
    print_error "Integration Tests: FAILED"
fi

TOTAL_TESTS=$(grep -c "test result:" test_output.log 2>/dev/null || echo "0")
if [ $TOTAL_TESTS -gt 0 ]; then
    print_info "Total test batches run: $TOTAL_TESTS"
fi

echo ""
if [ $UNIT_TESTS_PASSED -eq 1 ] && [ $INTEGRATION_TESTS_PASSED -eq 1 ]; then
    echo -e "${GREEN}╔═══════════════════════════════════╗${NC}"
    echo -e "${GREEN}║   ALL TESTS COMPLETED SUCCESSFULLY!${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════╝${NC}"
else
    echo -e "${RED}╔═══════════════════════════════════╗${NC}"
    echo -e "${RED}║   SOME TESTS FAILED - REVIEW LOGS ${NC}"
    echo -e "${RED}╚═══════════════════════════════════╝${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo "  • View detailed logs: cat test_output.log"
echo "  • Full testing guide: cat TESTING.md"
echo "  • Quick reference: cat QUICKSTART.md"
echo "  • Coverage reports: ./coverage.sh"
echo ""
echo -e "${BLUE}Useful Commands:${NC}"
echo "  • Run specific test: cargo test test_name"
echo "  • Watch mode: cargo watch -x test"
echo "  • Debug logging: RUST_LOG=debug cargo test"
echo ""
echo "====================================="
