#!/bin/bash
set -e

# Get the script directory (python/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "Building and testing hardware-address Python package..."
cd "$SCRIPT_DIR"

# Check if we're in a virtual environment, if not create one
if [ -z "$VIRTUAL_ENV" ] && [ -z "$CONDA_PREFIX" ]; then
  echo "Step 1: Creating virtual environment..."
  python -m venv .venv
  source .venv/bin/activate
  echo "✓ Virtual environment activated"
else
  echo "Step 1: Using existing virtual environment"
fi

# Install test dependencies
echo ""
echo "Step 2: Installing test dependencies..."
pip install pytest

# Build and install in development mode
echo ""
echo "Step 3: Building and installing package..."
pip install maturin
maturin develop --release

# Run tests
echo ""
echo "Step 4: Running unit tests..."
python -m pytest tests/ -v

echo ""
echo "✅ All Python tests passed!"

# Deactivate if we created the venv
if [ -f ".venv/bin/activate" ]; then
  deactivate || true
fi
