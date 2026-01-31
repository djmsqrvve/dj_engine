#!/bin/bash
# Simplified Mesa build with Dozen driver

set -e

echo "Building Mesa with Dozen driver..."

cd ~/mesa-build/mesa-24.0.7

# Clean build
rm -rf build

# Simplified configuration - minimal options
meson setup build \
    --prefix="$HOME/.local/mesa" \
    -Dbuildtype=release \
    -Dvulkan-drivers=microsoft-experimental \
    -Dgallium-drivers=d3d12 \
    -Ddri-drivers=[] \
    -Dplatforms=x11

echo "Configuration complete. Building..."
ninja -C build -j$(nproc)

echo "Installing..."
ninja -C build install

echo "Setting up ICD..."
mkdir -p "$HOME/.local/share/vulkan/icd.d"

# Find and copy the ICD file
if [ -f "build/src/microsoft/vulkan/dzn_icd.x86_64.json" ]; then
    cp "build/src/microsoft/vulkan/dzn_icd.x86_64.json" "$HOME/.local/share/vulkan/icd.d/"
    echo "ICD file installed!"
else
    echo "ICD file not found in expected location"
    find build -name "*.json" -type f 2>/dev/null | head -10
fi

echo "Build complete!"
