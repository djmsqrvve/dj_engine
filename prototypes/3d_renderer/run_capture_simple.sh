#!/bin/bash
# Simple frame capture for Bevy 0.14 - capture on key press

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║          BEVY 3D RENDERER - MANUAL CAPTURE                    ║"
echo "║  Press SPACE to capture frames to ./captures/                 ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

mkdir -p captures

export RUST_LOG=info,bevy_3d_renderer=error
export RUST_BACKTRACE=1

# Run the diagnostic version and capture on spacebar
cargo run --release 2>&1 | tee capture_session.log

echo ""
echo "Session ended. Check capture_session.log for details."
