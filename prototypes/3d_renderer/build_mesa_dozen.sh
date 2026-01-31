#!/bin/bash
# Build Mesa from source with Microsoft Dozen (Vulkan-D3D12) driver for WSL2
# This enables GPU acceleration for Vulkan apps in WSL2

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     BUILDING MESA WITH DOZEN DRIVER FOR WSL2 GPU              ║"
echo "║     Target: RTX 2080 Ti Vulkan Acceleration                   ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Configuration
MESA_VERSION="24.0.7"
BUILD_DIR="$HOME/mesa-build"
INSTALL_PREFIX="$HOME/.local/mesa"

echo "[1/8] Installing build dependencies..."
echo "--------------------------------------------------"
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    cmake \
    meson \
    ninja-build \
    git \
    python3-pip \
    python3-mako \
    python3-yaml \
    llvm \
    llvm-dev \
    libclang-dev \
    clang \
    libdrm-dev \
    libelf-dev \
    libexpat1-dev \
    libwayland-dev \
    wayland-protocols \
    libx11-dev \
    libx11-xcb-dev \
    libxcb-dri2-0-dev \
    libxcb-dri3-dev \
    libxcb-glx0-dev \
    libxcb-present-dev \
    libxcb-randr0-dev \
    libxcb-shm0-dev \
    libxcb-sync-dev \
    libxcb-xfixes0-dev \
    libxdamage-dev \
    libxext-dev \
    libxfixes-dev \
    libxrandr-dev \
    libxxf86vm-dev \
    libzstd-dev \
    zlib1g-dev \
    flex \
    bison \
    libvulkan-dev \
    glslang-tools \
    libglvnd-core-dev

echo ""
echo "[2/8] Downloading Mesa ${MESA_VERSION}..."
echo "--------------------------------------------------"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ ! -f "mesa-${MESA_VERSION}.tar.xz" ]; then
    wget "https://archive.mesa3d.org/mesa-${MESA_VERSION}.tar.xz"
fi

if [ ! -d "mesa-${MESA_VERSION}" ]; then
    tar -xf "mesa-${MESA_VERSION}.tar.xz"
fi

cd "mesa-${MESA_VERSION}"

echo ""
echo "[3/8] Configuring Mesa build with Dozen driver..."
echo "--------------------------------------------------"
# Key flags:
# -Dvulkan-drivers=dzn,swrast : Build Dozen (D3D12 Vulkan) and software drivers
# -Dgallium-drivers=d3d12 : OpenGL over D3D12
# -Dmicrosoft-experimental=true : Enable Microsoft WSL extensions

meson setup build \
    --prefix="$INSTALL_PREFIX" \
    --libdir="$INSTALL_PREFIX/lib" \
    -Dbuildtype=release \
    -Dvulkan-drivers=dzn,swrast \
    -Dgallium-drivers=d3d12 \
    -Ddri-drivers=[] \
    -Dmicrosoft-experimental=true \
    -Dplatforms=x11,wayland \
    -Degl-native-platform=x11 \
    -Dgles1=disabled \
    -Dgles2=enabled \
    -Dglvnd=true \
    -Dshared-glapi=enabled \
    -Dllvm=enabled \
    -Dvalgrind=disabled \
    -Dlibunwind=disabled \
    -Dlmsensors=disabled \
    -Dperfetto=false \
    -Dtools=[] \
    -Dintel-rt=disabled

echo ""
echo "[4/8] Building Mesa (this will take 1-2 hours)..."
echo "--------------------------------------------------"
echo "Go grab coffee! Compiling with $(nproc) threads..."
ninja -C build -j$(nproc)

echo ""
echo "[5/8] Installing Mesa to ${INSTALL_PREFIX}..."
echo "--------------------------------------------------"
ninja -C build install

echo ""
echo "[6/8] Setting up Vulkan ICD..."
echo "--------------------------------------------------"
# Create the ICD file for the Dozen driver
mkdir -p "$HOME/.local/share/vulkan/icd.d"

cat > "$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json" << EOF
{
    "file_format_version" : "1.0.0",
    "ICD": {
        "library_path": "$INSTALL_PREFIX/lib/libvulkan_dzn.so",
        "api_version" : "1.3.0"
    }
}
EOF

echo "Created ICD file: $HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json"

echo ""
echo "[7/8] Setting up environment..."
echo "--------------------------------------------------"
# Create environment setup script
cat > "$HOME/.local/mesa/setup-mesa-env.sh" << EOF
#!/bin/bash
# Source this file to use the custom Mesa build
export VK_ICD_FILENAMES="$HOME/.local/share/vulkan/icd.d/dzn_icd.x86_64.json"
export LD_LIBRARY_PATH="$INSTALL_PREFIX/lib:\$LD_LIBRARY_PATH"
export LIBGL_DRIVERS_PATH="$INSTALL_PREFIX/lib/dri"
export MESA_D3D12_DEFAULT_ADAPTER_NAME="NVIDIA"
echo "Mesa Dozen driver enabled!"
EOF

chmod +x "$HOME/.local/mesa/setup-mesa-env.sh"

echo ""
echo "[8/8] Verification..."
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
echo "  source $HOME/.local/mesa/setup-mesa-env.sh"
echo "  cd /mnt/c/Users/Mike/Documents/dj_engine/prototypes/3d_renderer"
echo "  cargo run --release"
echo ""
echo "Or add to your .bashrc:"
echo "  source $HOME/.local/mesa/setup-mesa-env.sh"
echo ""
echo "Expected result: AdapterInfo shows 'Microsoft Direct3D12' instead of 'llvmpipe'"
