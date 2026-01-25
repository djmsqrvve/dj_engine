#!/bin/bash
# Debug script for black screen issue - separate console + diagnostics

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║          BEVY 3D RENDERER - RENDER TROUBLESHOOTING            ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# 1. Force unbuffered output and separate console
export RUST_LOG=info,bevy_3d_renderer=error,wgpu=warn
export RUST_BACKTRACE=1

# 2. Run with wayland backend override (better logging in WSL)
export WINIT_UNIX_BACKEND=wayland

# 3. Create a separate console for output
echo "Starting application with forced diagnostics..."
echo "Logs will be written to: render_debug.log"
echo ""

# Run in background with separate console handle
cargo run --release 2>&1 | tee render_debug.log &
APP_PID=$!

echo "Application PID: $APP_PID"
echo "Waiting 5 seconds for startup..."
echo ""

sleep 5

echo "Checking render_debug.log for diagnostic messages..."
echo ""

# Look for specific diagnostic messages
if [ -f render_debug.log ]; then
    echo "═══════════════════════════════════════════════════════════════"
    echo "DIAGNOSTIC MESSAGES FOUND IN LOG:"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    
    # Show diagnostic errors (should be there with ERROR level)
    grep "DIAGNOSTIC SCENE\|UNLIT\|should be VERY visible" render_debug.log | head -20
    
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "ENTITY SPAWN MESSAGES:"
    echo "═══════════════════════════════════════════════════════════════"
    grep -A1 "Spawning LARGE\|DIAGNOSTIC ground plane" render_debug.log | head -15
    
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "MATERIAL & LIGHT INFO:"
    echo "═══════════════════════════════════════════════════════════════"
    grep "Materials loaded:\|Camera:" render_debug.log | head -10
    
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "RENDER RESOURCES:"
    echo "═══════════════════════════════════════════════════════════════"
    grep "Render Resources:\|Meshes:\|Materials:" render_debug.log | head -10
    
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "To stop application: kill $APP_PID"
    echo "To view full logs: tail -f render_debug.log"
    echo "═══════════════════════════════════════════════════════════════"
    
else
    echo "ERROR: Log file not created!"
fi
