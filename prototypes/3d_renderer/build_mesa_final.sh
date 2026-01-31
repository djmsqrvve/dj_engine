#!/bin/bash
# Final Mesa build with Dozen driver - simplified and correct

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     BUILDING MESA WITH DOZEN DRIVER                           ║"
echo "╚═══════════════════════════════════════════════════════════════╝"

cd ~/mesa-build/mesa-24.0.7

# Clean build
rm -rf build

echo "[1/3] Configuring..."
meson setup build \
    --prefix="$HOME/.local/mesa" \
    -Dbuildtype=release \
    -Dvulkan-drivers=microsoft-experimental \
    -Dgallium-drivers=d3d12 \
    -Dplatforms=x11

echo ""
echo "[2/3] Building (this will take 30-60 minutes)..."
ninja -C build -j$(nproc)

echo ""
echo "[3/3] Installing..."
ninja -C build install

# Setup ICD
echo ""
echo "Setting up Vulkan ICD..."
mkdir -p "$HOME/.local/share/vulkan/icd.d"

if [ -f "build/src/microsoft/vulkan/dzn_icd.x86_64.json" ]; then
    cp "build/src/microsoft/vulkan/dzn_icd.x86_64.json" "$HOME/.local/share/vulkan/icd.d/"
    echo "✓ ICD file installed to $HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json"
else
    echo "⚠ ICD file not found, searching..."
    find build -name "*icd*.json" -type f 2>/dev/null | head -5
fi

echo ""
echo "Build complete!"
echo "Run: export VK_ICD_FILENAMES=\"$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json\""
