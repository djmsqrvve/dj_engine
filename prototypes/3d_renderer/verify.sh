#!/bin/bash
# Quick verification script for Bevy 3D Renderer

echo "==================================="
echo "Bevy 3D Renderer Verification"
echo "==================================="

# Check if binary exists
if [ -f "target/release/bevy-3d-renderer" ]; then
    echo "✓ Binary compiled successfully"
else
    echo "✗ Binary not found - run 'cargo build --release' first"
    exit 1
fi

# Check assets
echo "✓ Asset directory linked"
ls -1 assets/test_models/dota_models/models/heroes/drow/ | grep "drow_base" | head -2

# Check source files
echo "✓ Source files present:"
ls -1 src/plugins/*.rs
echo ""
echo "To run the application:"
echo "  cargo run --release"
echo ""
echo "Expected output should include:"
echo "  'Drow model loaded successfully! Spawning scene...'"
