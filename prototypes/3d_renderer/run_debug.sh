#!/bin/bash
# Run Bevy 3D Renderer with debug logging

echo "====================================="
echo "Bevy 3D Renderer - Debug Run"
echo "====================================="
echo "Log Level: INFO + bevy_3d_renderer debug"
echo "====================================="
echo ""

# Enable debug logging for our crate
export RUST_LOG=info,bevy_3d_renderer=debug

# Run the application
cargo run --release 2>&1 | tee app_output.log

echo ""
echo "====================================="
echo "Application stopped"
echo "====================================="
echo ""
echo "Check app_output.log for full details"
