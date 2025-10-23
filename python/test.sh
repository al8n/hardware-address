#!/bin/bash
set -e

# Get the script directory (python/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "Building and testing hardware-address Python package..."
cd "$SCRIPT_DIR"

# Build the package (pyproject.toml already has manifest-path configured)
echo "Step 1: Building wheel..."
maturin build --release --out dist

# Find the built wheel
WHEEL=$(ls -t dist/*.whl 2>/dev/null | head -n1)

if [ -z "$WHEEL" ]; then
  echo "❌ No wheel found in dist/"
  exit 1
fi

echo "Found wheel: $WHEEL"

# Install the wheel
echo ""
echo "Step 2: Installing wheel..."
pip install --force-reinstall "$WHEEL"

# Run tests
echo ""
echo "Step 3: Running unit tests..."
python -m pytest tests/ -v

echo ""
echo "✅ All Python tests passed!"
