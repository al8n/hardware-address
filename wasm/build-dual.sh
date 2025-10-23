#!/bin/bash
set -e

# Get the script directory (wasm/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "Building hardware-address dual package for npm..."
cd "$SCRIPT_DIR"

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf pkg-dual

# Build for bundler (ESM)
echo "Building for bundler target (ESM)..."
wasm-pack build --target bundler --release --out-dir pkg-dual

# Build for nodejs (CommonJS) into a temporary directory
echo "Building for Node.js target (CommonJS)..."
wasm-pack build --target nodejs --release --out-dir pkg-nodejs-temp

# Copy the Node.js specific files to a node subdirectory
echo "Setting up dual package structure..."
mkdir -p pkg-dual/node

# Copy Node.js CommonJS files
cp pkg-nodejs-temp/hardware_address.js pkg-dual/node/
cp pkg-nodejs-temp/hardware_address_bg.wasm pkg-dual/node/
cp pkg-nodejs-temp/hardware_address.d.ts pkg-dual/node/

# Create a package.json in node/ to mark it as CommonJS
cat > pkg-dual/node/package.json << 'NODEPKG'
{
  "type": "commonjs"
}
NODEPKG

# Clean up temp directory
rm -rf pkg-nodejs-temp

# Create the dual package.json
echo "Creating package.json with exports..."
cat > pkg-dual/package.json << 'EOF'
{
  "name": "hardware-address",
  "version": "0.2.0",
  "description": "IEEE 802 MAC-48, EUI-48, EUI-64, and InfiniBand hardware addresses for WebAssembly",
  "type": "module",
  "main": "./node/hardware_address.js",
  "module": "./hardware_address.js",
  "types": "./hardware_address.d.ts",
  "exports": {
    ".": {
      "types": "./hardware_address.d.ts",
      "import": "./hardware_address.js",
      "require": "./node/hardware_address.js"
    },
    "./package.json": "./package.json"
  },
  "files": [
    "hardware_address_bg.wasm",
    "hardware_address.js",
    "hardware_address_bg.js",
    "hardware_address.d.ts",
    "hardware_address_bg.wasm.d.ts",
    "node/hardware_address.js",
    "node/hardware_address_bg.wasm",
    "node/hardware_address.d.ts",
    "node/package.json",
    "LICENSE-APACHE",
    "LICENSE-MIT",
    "README.md"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/al8n/hardware-address"
  },
  "keywords": [
    "mac-address",
    "eui64",
    "eui48",
    "hardware-address",
    "network",
    "infiniband",
    "ieee802",
    "wasm",
    "webassembly"
  ],
  "author": "Al Liu <scygliu1@gmail.com>",
  "license": "MIT OR Apache-2.0",
  "bugs": {
    "url": "https://github.com/al8n/hardware-address/issues"
  },
  "homepage": "https://github.com/al8n/hardware-address#readme",
  "sideEffects": [
    "./hardware_address.js"
  ]
}
EOF

# Copy README
echo "Copying README..."
if [ -f "$SCRIPT_DIR/README.npm.md" ]; then
  cp "$SCRIPT_DIR/README.npm.md" "$SCRIPT_DIR/pkg-dual/README.md"
else
  echo "Warning: README.npm.md not found"
fi

# Copy license files from parent directory
echo "Copying license files..."
if [ -f "$SCRIPT_DIR/../LICENSE-APACHE" ]; then
  cp "$SCRIPT_DIR/../LICENSE-APACHE" "$SCRIPT_DIR/pkg-dual/"
fi
if [ -f "$SCRIPT_DIR/../LICENSE-MIT" ]; then
  cp "$SCRIPT_DIR/../LICENSE-MIT" "$SCRIPT_DIR/pkg-dual/"
fi

echo ""
echo "✅ Build complete! Dual package ready in ./pkg-dual directory"
echo ""
echo "Package structure:"
echo "  pkg-dual/"
echo "    ├── hardware_address.js          (ESM - for bundlers/import)"
echo "    ├── hardware_address_bg.wasm     (WASM binary for ESM)"
echo "    ├── hardware_address_bg.js       (ESM bindings)"
echo "    ├── hardware_address.d.ts        (TypeScript definitions)"
echo "    ├── node/"
echo "    │   ├── hardware_address.js      (CommonJS - for require())"
echo "    │   └── hardware_address_bg.wasm (WASM binary for CommonJS)"
echo "    └── package.json                 (with dual exports)"
echo ""
echo "Usage:"
echo "  ESM:        import { MacAddr } from 'hardware-address';"
echo "  CommonJS:   const { MacAddr } = require('hardware-address');"
echo ""
echo "To test locally:"
echo "  cd pkg-dual && npm link"
echo ""
echo "To publish:"
echo "  cd pkg-dual && npm publish"
