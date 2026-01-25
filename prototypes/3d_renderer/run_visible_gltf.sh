#!/bin/bash
# Run with model visibility verification

export RUST_LOG=info,bevy_3d_renderer=error

echo "Running Bevy 3D Renderer with GLTF visibility verification..."
echo "Looking for: 'Drow model spawned' and 'Model bounds:'"
echo ""

cargo run --release 2>&1 | tee gltf_visibility.log

echo ""
echo "Search gltf_visibility.log for model info:"
echo "  grep 'Drow model' gltf_visibility.log"
echo "  grep 'Model bounds' gltf_visibility.log"
echo "  grep 'Camera positioned' gltf_visibility.log"
