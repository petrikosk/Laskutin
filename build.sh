#!/bin/bash

# Build script for Laskutin using Podman on Ubuntu 20.04.6 LTS

set -e

IMAGE_NAME="laskutin-builder"
CONTAINER_NAME="laskutin-build-$(date +%s)"
BUILD_DIR="./build-output"

echo "🏗️  Building Laskutin with Podman..."

# Create output directory
mkdir -p "$BUILD_DIR"

# Check if image exists
if podman image exists "$IMAGE_NAME"; then
    echo "✅ Container image '$IMAGE_NAME' already exists, using existing image"
else
    echo "📦 Building container image '$IMAGE_NAME'..."
    podman build -t "$IMAGE_NAME" .
fi

# Run the container with build commands
echo "🚀 Running build in container..."
podman run --rm --name "$CONTAINER_NAME" \
    -v "$(pwd):/app" \
    -w /app \
    "$IMAGE_NAME" \
    bash -c "
        set -e
        echo '📥 Installing frontend dependencies...'
        pnpm install --frozen-lockfile
        
        echo '🦀 Fetching Rust dependencies...'
        cd src-tauri && cargo fetch && cd ..
        
        echo '🔨 Building application...'
        export SQLX_OFFLINE=true
        pnpm tauri build
        
        echo '📦 Organizing build artifacts...'
        mkdir -p /app/build-output
        
        # Copy DEB packages if they exist
        if ls src-tauri/target/release/bundle/deb/*.deb 1> /dev/null 2>&1; then
            cp src-tauri/target/release/bundle/deb/*.deb /app/build-output/
            echo '✅ DEB package copied'
        fi
        
        # Copy AppImage if it exists
        if ls src-tauri/target/release/bundle/appimage/*.AppImage 1> /dev/null 2>&1; then
            cp src-tauri/target/release/bundle/appimage/*.AppImage /app/build-output/
            echo '✅ AppImage copied'
        fi
        
        # Copy binary if it exists
        if [ -f src-tauri/target/release/laskutin ]; then
            cp src-tauri/target/release/laskutin /app/build-output/
            echo '✅ Binary copied'
        fi
        
        echo '🎯 Build artifacts ready in build-output/'
        ls -la /app/build-output/
    "

echo "✅ Build completed! Artifacts available in: $BUILD_DIR"
echo "📁 Built files:"
ls -la "$BUILD_DIR" 2>/dev/null || echo "No artifacts found"

echo "🎉 Build process finished successfully!"