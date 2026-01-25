#!/bin/bash
# Use the diagnostic main.rs to debug black screen - keeps console separate

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║         BEVY 3D RENDERER - DIAGNOSTIC RUN                     ║"
echo "║  This will load src/simple_main.rs with forced visible objects║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Backup original main.rs if it exists
if [ -f "src/main.rs" ] && [ ! -f "src/main_original.rs" ]; then
    cp src/main.rs src/main_original.rs
    echo "✓ Backed up original main.rs to src/main_original.rs"
fi

# Use the diagnostic version
if [ -f "src/simple_main.rs" ]; then
    cp src/simple_main.rs src/main.rs
    echo "✓ Using diagnostic main.rs (spawns unlit cubes)"
else
    echo "✗ ERROR: src/simple_main.rs not found!"
    exit 1
fi

echo ""
echo "Building with diagnostic scene..."
echo ""

# Run with stderr redirected to both console and file
export RUST_LOG=error,bevy_3d_renderer=error
export RUST_BACKTRACE=1

# Run the application (stderr goes to console, stdout goes to diagnostic.log)
cargo run --release 2>&1 | tee diagnostic.log

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                   DIAGNOSTIC RUN COMPLETE                     ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Search diagnostic.log for:"
echo "  • 'DIAGNOSTIC SCENE' - shows what was spawned"
echo "  • 'RED cube\|GREEN cube\|BLUE cube\|YELLOW cube' - colors spawned"
echo "  • 'ENTITY COUNT' - confirms entities exist"
echo "  • 'Camera:' - camera position and clear color"
echo ""
echo "To restore original: cp src/main_original.rs src/main.rs"
