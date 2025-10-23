#!/bin/bash
set -e

# Get the script directory (python/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "Building and testing hardware-address Python package..."
cd "$SCRIPT_DIR"

# Build the package in development mode
echo "Step 1: Building package in development mode..."
maturin develop --release

# Run tests
echo ""
echo "Step 2: Running unit tests..."
python -m pytest tests/ -v

echo ""
echo "✅ All Python tests passed!"
