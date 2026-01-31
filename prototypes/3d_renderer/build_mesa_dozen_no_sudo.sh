#!/bin/bash
# Build Mesa with Dozen driver WITHOUT sudo
# Uses pre-installed dependencies

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     BUILDING MESA WITH DOZEN DRIVER (No Sudo Mode)            ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Check if we have the required dependencies
echo "[1/4] Checking dependencies..."
echo "--------------------------------------------------"

# Check for meson
if ! command -v meson &> /dev/null; then
    echo "ERROR: meson not found. Install with: pip3 install --user meson"
    exit 1
fi

# Check for necessary libraries
for lib in libdrm libelf libexpat libzstd; do
    if ! pkg-config --exists $lib 2>/dev/null; then
        echo "WARNING: $lib not found via pkg-config"
    fi
done

MESA_VERSION="24.0.7"
BUILD_DIR="$HOME/mesa-build"
INSTALL_PREFIX="$HOME/.local/mesa"

echo ""
echo "[2/4] Downloading Mesa ${MESA_VERSION}..."
echo "--------------------------------------------------"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ ! -f "mesa-${MESA_VERSION}.tar.xz" ]; then
    wget "https://archive.mesa3d.org/mesa-${MESA_VERSION}.tar.xz" || \
    wget "https://mesa.freedesktop.org/archive/mesa-${MESA_VERSION}.tar.xz"
fi

if [ ! -d "mesa-${MESA_VERSION}" ]; then
    tar -xf "mesa-${MESA_VERSION}.tar.xz"
fi

cd "mesa-${MESA_VERSION}"

echo ""
echo "[3/4] Configuring Mesa build..."
echo "--------------------------------------------------"

# Minimal configuration focusing only on Dozen driver
meson setup build \
    --prefix="$INSTALL_PREFIX" \
    --libdir="lib" \
    -Dbuildtype=release \
    -Dvulkan-drivers=dzn \
    -Dgallium-drivers=[] \
    -Ddri-drivers=[] \
    -Dmicrosoft-experimental=true \
    -Dplatforms=[] \
    -Degl=disabled \
    -Dgles1=disabled \
    -Dgles2=disabled \
    -Dglvnd=disabled \
    -Dshared-glapi=disabled \
    -Dllvm=disabled \
    -Dvalgrind=disabled \
    -Dlibunwind=disabled \
    -Dlmsensors=disabled \
    -Dperfetto=disabled \
    -Dtools=[] \
    -Dshader-cache=enabled \
    -Dvulkan-icd-dir="$HOME/.local/share/vulkan/icd.d" \
    --wipe 2>/dev/null || true

echo ""
echo "[4/4] Building Mesa (this will take 30-60 minutes)..."
echo "--------------------------------------------------"
ninja -C build -j$(nproc)

echo "Installing..."
ninja -C build install

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║              BUILD COMPLETE!                                  ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "ICD file location: $HOME/.local/share/vulkan/icd.d/"
ls -la "$HOME/.local/share/vulkan/icd.d/" 2>/dev/null || echo "ICD dir not found"
echo ""
echo "To test:"
echo "  VK_ICD_FILENAMES=$HOME/.local/share/vulkan/icd.d/<icd_file> cargo run --release"
