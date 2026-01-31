#!/bin/bash
# Build script for Bevy WASM game

set -e

echo "Building Themathar Game for WebAssembly..."

# Install wasm-pack if not installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh
fi

# Build WASM
echo "Compiling WASM..."
wasm-pack build --target web --release --out-dir web/pkg

# Copy index.html if it doesn't exist
if [ ! -f "web/index.html" ]; then
    echo "Warning: web/index.html not found"
fi

echo ""
echo "=== Build Complete ==="
echo "WASM artifacts are in: web/pkg/"
echo ""
echo "To run a local server:"
echo "  cd web"
echo "  python3 -m http.server 8080"
echo ""
echo "Then visit: http://localhost:8080"
