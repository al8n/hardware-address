#!/bin/bash
set -e

# Get the script directory (wasm/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "Building hardware-address WASM for Node.js testing..."
cd "$SCRIPT_DIR"

# Build for Node.js target. `--out-name hardware_address` keeps the
# output filenames stable across changes to the crate's `[lib] name`
# (see the comment in build-dual.sh).
echo "Building for Node.js target..."
wasm-pack build --target nodejs --release --out-dir pkg-nodejs --out-name hardware_address

# Run the tests
echo ""
echo "Running Node.js tests..."
node test/node-test.js

echo ""
echo "✅ All tests completed successfully!"
