#!/bin/bash
# Run Bevy 3D Renderer with NVIDIA GPU acceleration via Dozen (Vulkan-D3D12)
# This script enables the custom Mesa build with WSL2 GPU support

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     BEVY 3D RENDERER - NVIDIA RTX 2080 Ti ACCELERATED         ║"
echo "║     Backend: Vulkan → Dozen → D3D12 → GPU                    ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Set up Mesa Dozen driver
export VK_ICD_FILENAMES="$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json"
export LD_LIBRARY_PATH="/usr/lib/wsl/lib:$HOME/.local/mesa/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH"

# Force X11 (Wayland not supported by Dozen driver)
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY

# Allow non-compliant Vulkan adapter (Dozen is not fully conformant yet)
export WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER=1

# Prefer NVIDIA GPU
export MESA_D3D12_DEFAULT_ADAPTER_NAME="NVIDIA"

# Run with GPU
echo "Starting with GPU acceleration..."
cargo run --release --bin bevy-3d-renderer 2>&1 | tee gpu_run.log

echo ""
echo "Run complete! Check gpu_run.log for details"
