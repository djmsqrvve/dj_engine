#!/bin/bash

# Bevy 2D Renderer - Test Suite Runner
# This script runs unit tests, integration tests, and coverage reports

set -e

echo "=========================================="
echo "Bevy 2D Renderer - Test Suite"
echo "=========================================="
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo is not installed. Please install Rust and Cargo."
    exit 1
fi

# Parse command line arguments
TEST_TYPE="${1:-all}"
VERBOSE="${2:-false}"

print_status "Running test suite with type: $TEST_TYPE"
echo ""

# Build the project first
print_status "Building project..."
cargo build --lib

# Run unit tests in library
case $TEST_TYPE in
    unit|all)
        echo ""
        print_status "Running Unit Tests (Modules)..."
        echo "-----------------------------------"
        
        if [ "$VERBOSE" = "true" ]; then
            cargo test --lib -- --nocapture
        else
            cargo test --lib
        fi
        
        if [ $? -eq 0 ]; then
            print_success "Unit tests passed!"
        else
            print_error "Unit tests failed!"
            exit 1
        fi
        ;;
esac

# Run integration tests
case $TEST_TYPE in
    integration|all)
        echo ""
        print_status "Running Integration Tests..."
        echo "-----------------------------------"
        
        if [ "$VERBOSE" = "true" ]; then
            cargo test --test '*' -- --nocapture
        else
            cargo test --test '*'
        fi
        
        if [ $? -eq 0 ]; then
            print_success "Integration tests passed!"
        else
            print_error "Integration tests failed!"
            exit 1
        fi
        ;;
esac

# Run doctests
case $TEST_TYPE in
    doc|all)
        echo ""
        print_status "Running Documentation Tests..."
        echo "-----------------------------------"
        
        cargo test --doc
        
        if [ $? -eq 0 ]; then
            print_success "Doc tests passed!"
        else
            print_error "Doc tests failed!"
            exit 1
        fi
        ;;
esac

# Run all tests together
case $TEST_TYPE in
    all)
        echo ""
        print_status "Running All Tests Together..."
        echo "-----------------------------------"
        
        cargo test
        
        if [ $? -eq 0 ]; then
            print_success "All tests passed!"
        else
            print_error "Some tests failed!"
            exit 1
        fi
        ;;
esac

# Run test coverage with tarpaulin (if installed)
case $TEST_TYPE in
    coverage)
        echo ""
        print_status "Running Test Coverage with Tarpaulin..."
        echo "-----------------------------------"
        
        if command -v cargo-tarpaulin &> /dev/null; then
            cargo tarpaulin --out Html --output-dir ./target/coverage/
            
            if [ $? -eq 0 ]; then
                print_success "Coverage report generated at target/coverage/index.html"
                
                # Try to open the coverage report
                if command -v xdg-open &> /dev/null; then
                    xdg-open ./target/coverage/index.html
                elif command -v open &> /dev/null; then
                    open ./target/coverage/index.html
                else
                    print_warning "Could not open coverage report automatically."
                    print_status "Please open target/coverage/index.html manually."
                fi
            else
                print_error "Coverage generation failed!"
                exit 1
            fi
        else
            print_warning "cargo-tarpaulin is not installed."
            print_status "Installing cargo-tarpaulin..."
            
            cargo install cargo-tarpaulin
            
            if [ $? -eq 0 ]; then
                print_success "cargo-tarpaulin installed successfully."
                print_status "Re-running coverage..."
                cargo tarpaulin --out Html --output-dir ./target/coverage/
            else
                print_error "Failed to install cargo-tarpaulin."
                print_warning "You may need to install SSL development packages:"
                print_warning "  Ubuntu/Debian: sudo apt-get install libssl-dev pkg-config"
                print_warning "  Fedora/RHEL: sudo dnf install openssl-devel pkg-config"
                print_warning "  Arch: sudo pacman -S openssl pkg-config"
                exit 1
            fi
        fi
        ;;
esac

# Run clippy lints
case $TEST_TYPE in
    lint|all)
        echo ""
        print_status "Running Clippy Lints..."
        echo "-----------------------------------"
        
        cargo clippy -- -D warnings
        
        if [ $? -eq 0 ]; then
            print_success "Clippy lints passed!"
        else
            print_warning "Clippy found warnings. Please fix them."
        fi
        ;;
esac

# Run formatting check
case $TEST_TYPE in
    format|all)
        echo ""
        print_status "Checking Code Formatting..."
        echo "-----------------------------------"
        
        cargo fmt -- --check
        
        if [ $? -eq 0 ]; then
            print_success "Code is properly formatted!"
        else
            print_error "Code is not formatted properly. Run 'cargo fmt' to fix."
            exit 1
        fi
        ;;
esac

echo ""
echo "=========================================="
print_success "Test suite completed successfully!"
echo "=========================================="
echo ""
echo "Test Types Available:"
echo "  ./test.sh unit       - Run unit tests only"
echo "  ./test.sh integration - Run integration tests only"
echo "  ./test.sh doc        - Run documentation tests"
echo "  ./test.sh coverage   - Generate coverage report"
echo "  ./test.sh lint       - Run clippy lints"
echo "  ./test.sh format     - Check formatting"
echo "  ./test.sh all        - Run everything (default)"
echo ""
echo "Add 'true' as second argument for verbose output:"
echo "  ./test.sh unit true  - Verbose unit tests"
echo ""
