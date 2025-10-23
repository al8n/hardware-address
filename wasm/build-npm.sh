#!/bin/bash
set -e

# Get the script directory (wasm/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Get the project root (parent of wasm/)
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "Building hardware-address for npm..."
echo "Project root: $PROJECT_ROOT"

# Build from project root
cd "$PROJECT_ROOT"

# Build for bundler (default for npm)
echo "Building for bundler target..."
wasm-pack build --target bundler --release --out-dir wasm/pkg -- --features wasm-bindgen,std

# Copy the npm-specific README to pkg directory
echo "Copying README..."
cp "$SCRIPT_DIR/README.npm.md" "$SCRIPT_DIR/pkg/README.md"

echo "Build complete! Package ready in ./wasm/pkg directory"
echo ""
echo "To test locally:"
echo "  cd wasm/pkg && npm link"
echo ""
echo "To publish:"
echo "  cd wasm/pkg && npm publish"
