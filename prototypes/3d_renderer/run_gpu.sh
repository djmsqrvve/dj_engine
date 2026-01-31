#!/bin/bash
# Run Bevy 3D Renderer with NVIDIA GPU acceleration via WSL2 D3D12

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     BEVY 3D RENDERER - NVIDIA GPU ACCELERATED                 ║"
echo "║     GPU: RTX 2080 Ti via WSL2 D3D12                           ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Set up Vulkan to use D3D12 driver (enables NVIDIA GPU in WSL2)
export VK_ICD_FILENAMES="$HOME/.local/share/vulkan/icd.d/d3d12_icd.x86_64.json"

# Additional performance optimizations
export MESA_D3D12_DEFAULT_ADAPTER_NAME="NVIDIA"
export WSL_GFX_ENABLE_GPU=1

# Optional: Disable software fallback to force GPU
# export LIBGL_ALWAYS_SOFTWARE=0

echo "Vulkan ICD set to: $VK_ICD_FILENAMES"
echo ""

# Verify GPU detection
echo "Detecting GPU..."
vulkaninfo --summary 2>/dev/null | grep -E "(deviceName|deviceType)" || echo "vulkaninfo not available, continuing anyway"
echo ""

# Run the application
echo "Starting Bevy 3D Renderer with GPU acceleration..."
echo ""

cargo run --release 2>&1 | tee gpu_run.log

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                     RUN COMPLETE                              ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Check gpu_run.log for details"
echo ""
echo "To verify GPU was used, look for:"
echo "  'deviceType: DiscreteGpu' or 'NVIDIA' in the logs"
