#!/bin/bash
set -e

# Get the script directory (wasm/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "Building hardware-address WASM for Node.js testing..."
cd "$SCRIPT_DIR"

# Build for Node.js target
echo "Building for Node.js target..."
wasm-pack build --target nodejs --release --out-dir pkg-nodejs

# Run the tests
echo ""
echo "Running Node.js tests..."
node test/node-test.js

echo ""
echo "✅ All tests completed successfully!"
