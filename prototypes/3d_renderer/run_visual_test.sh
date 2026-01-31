#!/bin/bash
# Quick visual test - GPU accelerated 3D scene

echo "Launching GPU-accelerated 3D scene..."
echo ""
echo "Controls:"
echo "  - Camera auto-rotates around scene"
echo "  - SPACE: Print message"
echo "  - ESC: Exit"
echo ""

export VK_ICD_FILENAMES="$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json"
export LD_LIBRARY_PATH="/usr/lib/wsl/lib:$HOME/.local/mesa/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH"
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY
export WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER=1
export MESA_D3D12_DEFAULT_ADAPTER_NAME="NVIDIA"

cd /mnt/c/Users/Mike/Documents/dj_engine/prototypes/3d_renderer
./target/release/bevy-3d-renderer
