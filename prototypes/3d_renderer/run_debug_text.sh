#!/bin/bash

# Build and run the debug text / XYZ axes test
# This helps verify if rendering is working at all

echo "========================================"
echo "Building Debug Text + XYZ Axes Test..."
echo "========================================"

# Build the debug-text binary
cargo build --bin debug-text 2>&1

if [ $? -eq 0 ]; then
    echo ""
    echo "========================================"
    echo "Build successful! Running debug test..."
    echo "========================================"
    echo ""
    echo "This test will show:"
    echo "  - 2D text at corners of the screen"
    echo "  - XYZ axis lines (Red=+X, Green=+Y, Blue=+Z)"
    echo "  - Colored cubes showing axis directions"
    echo "  - Grid on the ground"
    echo ""
    echo "CONTROLS:"
    echo "  SPACE = Pause/resume camera rotation"
    echo "  ESC   = Exit"
    echo ""
    echo "If you see TEXT but no 3D: 2D works, 3D broken"
    echo "If you see NOTHING: Rendering completely broken"
    echo "If you see EVERYTHING: Rendering works!"
    echo ""
    
    cargo run --bin debug-text 2>&1
else
    echo ""
    echo "========================================"
    echo "Build failed!"
    echo "========================================"
fi
