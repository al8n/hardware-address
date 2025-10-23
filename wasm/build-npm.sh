#!/bin/bash
set -e

# Get the script directory (wasm/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "⚠️  NOTE: This builds bundler-only package (ESM)."
echo "For dual ESM/CommonJS package, use: ./build-dual.sh"
echo ""
echo "Building hardware-address for npm (bundler target only)..."
echo "WASM directory: $SCRIPT_DIR"

# Build from wasm directory
cd "$SCRIPT_DIR"

# Build for bundler (default for npm)
echo "Building for bundler target..."
wasm-pack build --target bundler --release --out-dir pkg

# Copy the npm-specific README to pkg directory
echo "Copying README..."
cp "$SCRIPT_DIR/README.npm.md" "$SCRIPT_DIR/pkg/README.md"

# Copy license files from parent directory
echo "Copying license files..."
if [ -f "$SCRIPT_DIR/../LICENSE-APACHE" ]; then
  cp "$SCRIPT_DIR/../LICENSE-APACHE" "$SCRIPT_DIR/pkg/"
fi
if [ -f "$SCRIPT_DIR/../LICENSE-MIT" ]; then
  cp "$SCRIPT_DIR/../LICENSE-MIT" "$SCRIPT_DIR/pkg/"
fi

echo "Build complete! Package ready in ./wasm/pkg directory"
echo ""
echo "⚠️  This package only supports ESM (import), not CommonJS (require)."
echo "For dual ESM/CommonJS support, use: ./build-dual.sh"
echo ""
echo "To test locally:"
echo "  cd wasm/pkg && npm link"
echo ""
echo "To publish:"
echo "  cd wasm/pkg && npm publish"
