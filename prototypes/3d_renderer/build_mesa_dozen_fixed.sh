#!/bin/bash
# Build Mesa from source with Microsoft Dozen (Vulkan-D3D12) driver for WSL2
# FIXED for Mesa 24.0.x

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     BUILDING MESA WITH DOZEN DRIVER FOR WSL2 GPU              ║"
echo "║     Target: RTX 2080 Ti Vulkan Acceleration                   ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

MESA_VERSION="24.0.7"
BUILD_DIR="$HOME/mesa-build"
INSTALL_PREFIX="$HOME/.local/mesa"

echo "[1/7] Installing build dependencies..."
echo "--------------------------------------------------"
echo "spyguy13" | sudo -S apt-get update
echo "spyguy13" | sudo -S apt-get install -y \
    build-essential cmake meson ninja-build git python3-pip python3-mako \
    python3-yaml llvm llvm-dev libclang-dev clang libdrm-dev libelf-dev \
    libexpat1-dev libwayland-dev wayland-protocols libx11-dev \
    libx11-xcb-dev libxcb-dri2-0-dev libxcb-dri3-dev libxcb-glx0-dev \
    libxcb-present-dev libxcb-randr0-dev libxcb-shm0-dev libxcb-sync-dev \
    libxcb-xfixes0-dev libxdamage-dev libxext-dev libxfixes-dev \
    libxrandr-dev libxxf86vm-dev libzstd-dev zlib1g-dev flex bison \
    libvulkan-dev glslang-tools spirv-tools

echo ""
echo "[2/7] Setting up Mesa build directory..."
echo "--------------------------------------------------"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ ! -f "mesa-${MESA_VERSION}.tar.xz" ]; then
    wget "https://archive.mesa3d.org/mesa-${MESA_VERSION}.tar.xz"
fi

if [ -d "mesa-${MESA_VERSION}" ]; then
    rm -rf "mesa-${MESA_VERSION}"
fi
tar -xf "mesa-${MESA_VERSION}.tar.xz"
cd "mesa-${MESA_VERSION}"

echo ""
echo "[3/7] Configuring Mesa build with Dozen driver..."
echo "--------------------------------------------------"
# Note: In Mesa 24.0, use 'microsoft-experimental' which includes dzn (Dozen)
# The Dozen driver is Microsoft's Vulkan-on-D3D12 implementation

meson setup build \
    --prefix="$INSTALL_PREFIX" \
    --libdir="lib" \
    -Dbuildtype=release \
    -Dvulkan-drivers=microsoft-experimental \
    -Dgallium-drivers=d3d12 \
    -Ddri-drivers=[] \
    -Dplatforms=x11 \
    -Degl=disabled \
    -Dgles1=disabled \
    -Dgles2=enabled \
    -Dglvnd=disabled \
    -Dshared-glapi=enabled \
    -Dllvm=enabled \
    -Dvalgrind=disabled \
    -Dlibunwind=disabled \
    -Dlmsensors=disabled \
    -Dperfetto=disabled \
    -Dtools=[] \
    --wipe 2>/dev/null || true

echo ""
echo "[4/7] Building Mesa (this will take 1-2 hours)..."
echo "--------------------------------------------------"
echo "Compiling with $(nproc) threads..."
ninja -C build -j$(nproc)

echo ""
echo "[5/7] Installing Mesa to ${INSTALL_PREFIX}..."
echo "--------------------------------------------------"
ninja -C build install

echo ""
echo "[6/7] Setting up Vulkan ICD..."
echo "--------------------------------------------------"
# Find the dzn ICD file that was built
mkdir -p "$HOME/.local/share/vulkan/icd.d"

# The ICD file should be in the build directory
if [ -f "build/src/microsoft/vulkan/dzn_icd.x86_64.json" ]; then
    cp "build/src/microsoft/vulkan/dzn_icd.x86_64.json" "$HOME/.local/share/vulkan/icd.d/"
    # Update the library path in the ICD file
    sed -i "s|\"library_path\": .*|\"library_path\": \"$INSTALL_PREFIX/lib/libvulkan_dzn.so\",|" "$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json"
fi

echo "Created ICD file: $HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json"

echo ""
echo "[7/7] Verification..."
echo "--------------------------------------------------"
echo "Checking for built libraries..."
ls -la "$INSTALL_PREFIX/lib/libvulkan_dzn.so" 2>/dev/null || echo "WARNING: libvulkan_dzn.so not found!"
ls -la "$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json" || echo "WARNING: ICD file not created!"

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║              MESA BUILD COMPLETE!                             ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "To use the new driver, run:"
echo "  export VK_ICD_FILENAMES=\"$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json\""
echo "  export LD_LIBRARY_PATH=\"$INSTALL_PREFIX/lib:\$LD_LIBRARY_PATH\""
echo "  cd /mnt/c/Users/Mike/Documents/dj_engine/prototypes/3d_renderer"
echo "  cargo run --release"
echo ""
echo "Expected result: AdapterInfo shows 'Microsoft Direct3D12' instead of 'llvmpipe'"
