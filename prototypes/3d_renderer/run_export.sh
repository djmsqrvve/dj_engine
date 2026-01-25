#!/bin/bash
# Run Bevy 3D Renderer and export renders to PNG files

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║          BEVY 3D RENDERER - EXPORT TO PNG                     ║"
echo "║  Renders will be saved to: ./renders/                         ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Create renders directory
mkdir -p renders

# Run with frame capture
echo "Running with screenshot capture enabled..."
echo ""
echo "Press SPACE to capture additional frames"
echo "Or wait for automatic captures"
echo ""

export RUST_LOG=info,bevy_3d_renderer=error,capture=info
export RUST_BACKTRACE=1

# This will:
# 1. Start the application
# 2. Capture startup.png immediately
# 3. Capture additional frames every 5 seconds
# 4. Save all to ./renders/ directory

cargo run --release 2>&1 | tee export_session.log

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                     EXPORT COMPLETE                           ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Check ./renders/ directory for PNG files:"
ls -lh renders/ 2>/dev/null || echo "  (no renders captured - check logs)"
echo ""
echo "Search logs for captures:"
echo "  grep 'Captured' export_session.log"
echo "  grep 'EXPORTED' export_session.log"
echo ""
echo "View the renders:"
echo "  open renders/*.png  # macOS"
echo "  xdg-open renders/*.png  # Linux"
echo ""
