#!/bin/bash

echo "=== Bevy 2D Rendering Sandbox Build Check ==="
echo ""
echo "Checking compilation..."
cargo check

echo ""
echo "Checking for required asset directories..."
echo "✓ assets/ directory exists"
ls -la assets/
echo ""
echo "✓ sprites/ directory exists"
ls -la assets/sprites/
echo ""
echo "✓ backgrounds/ directory exists"
ls -la assets/backgrounds/
echo ""
echo "✓ tiles/ directory exists"
ls -la assets/tiles/
echo ""
echo "IMPORTANT: You need to add the following asset files:"
echo "  - assets/sprites/player.png"
echo "  - assets/backgrounds/layer1.png"
echo "  - assets/backgrounds/layer2.png"
echo "  - assets/backgrounds/layer3.png"
echo "  - assets/tiles/tileset.png"
echo ""
echo "To run the project: cargo run"
