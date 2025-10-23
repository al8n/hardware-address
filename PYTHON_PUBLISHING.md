# Publishing to PyPI (Python Package Index)

This guide shows how to publish the Python bindings for `hardware-address` to PyPI.

## Prerequisites

1. Install required tools:
```bash
pip install maturin twine
```

2. Create accounts:
   - [PyPI account](https://pypi.org/account/register/)
   - [TestPyPI account](https://test.pypi.org/account/register/) (for testing)

3. Configure API tokens:
   - Go to https://pypi.org/manage/account/token/
   - Create a new API token
   - Save it securely

## Project Setup

1. Create `pyproject.toml` in the project root:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "hardware-address"
version = "0.1.3"
description = "IEEE 802 MAC-48, EUI-48, EUI-64, or a 20-octet IP over InfiniBand link-layer addresses"
readme = "README.md"
requires-python = ">=3.8"
license = {text = "MIT OR Apache-2.0"}
keywords = ["mac-address", "eui64", "eui48", "hardware-address", "network"]
authors = [
    {name = "Al Liu", email = "scygliu1@gmail.com"}
]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "License :: OSI Approved :: Apache Software License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Programming Language :: Rust",
    "Topic :: Software Development :: Libraries",
    "Topic :: System :: Networking",
]

[project.urls]
Homepage = "https://github.com/al8n/hardware-address"
Repository = "https://github.com/al8n/hardware-address"
Documentation = "https://docs.rs/hardware-address"
"Bug Tracker" = "https://github.com/al8n/hardware-address/issues"

[tool.maturin]
features = ["pyo3"]
python-source = "python"
module-name = "hardware_address"
```

2. Create `Cargo.toml` additions for Python:

```toml
[lib]
name = "hardware_address"
crate-type = ["cdylib", "rlib"]

[features]
default = ["std"]
# ... existing features ...
python = ["pyo3"]
```

## Building

### Build for development:
```bash
maturin develop --features pyo3
```

### Build wheel for distribution:
```bash
# Build for current platform
maturin build --release --features pyo3

# Build for multiple Python versions
maturin build --release --features pyo3 -i python3.8 -i python3.9 -i python3.10 -i python3.11 -i python3.12
```

### Build for all platforms (using CI/CD):

Create `.github/workflows/python-release.yml`:

```yaml
name: Python Release

on:
  release:
    types: [published]
  workflow_dispatch:

jobs:
  linux:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [x86_64, i686, aarch64, armv7]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist --features pyo3
          manylinux: auto
      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-linux-${{ matrix.target }}
          path: dist

  windows:
    runs-on: windows-latest
    strategy:
      matrix:
        target: [x64, x86]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'
          architecture: ${{ matrix.target }}
      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist --features pyo3
      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-windows-${{ matrix.target }}
          path: dist

  macos:
    runs-on: macos-latest
    strategy:
      matrix:
        target: [x86_64, aarch64]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist --features pyo3
      - name: Upload wheels
        uses: actions/upload-artifact@v4
        with:
          name: wheels-macos-${{ matrix.target }}
          path: dist

  release:
    name: Release
    runs-on: ubuntu-latest
    needs: [linux, windows, macos]
    steps:
      - uses: actions/download-artifact@v4
        with:
          pattern: wheels-*
          merge-multiple: true
          path: dist
      - name: Publish to PyPI
        uses: PyO3/maturin-action@v1
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_API_TOKEN }}
        with:
          command: upload
          args: --skip-existing dist/*
```

## Testing

### Test on TestPyPI first:
```bash
# Build
maturin build --release --features pyo3

# Upload to TestPyPI
twine upload --repository testpypi dist/*

# Install and test
pip install --index-url https://test.pypi.org/simple/ hardware-address
```

### Test the package:
```python
from hardware_address import MacAddr

# Test basic functionality
addr = MacAddr.from_str("00:00:5e:00:53:01")
print(addr)  # Should print: 00:00:5e:00:53:01

# Test bytes
data = bytes(addr)
assert len(data) == 6

# Test comparison
addr2 = MacAddr.from_str("00:00:5e:00:53:01")
assert addr == addr2

# Test hash
assert hash(addr) == hash(addr2)
```

## Publishing to PyPI

### Manual publishing:
```bash
# Build for all platforms (or use CI/CD artifacts)
maturin build --release --features pyo3

# Upload to PyPI
maturin upload
# Or using twine:
twine upload dist/*
```

### Using GitHub Actions (Recommended):

1. Add `PYPI_API_TOKEN` to repository secrets:
   - Go to repository Settings → Secrets and variables → Actions
   - Add new repository secret: `PYPI_API_TOKEN`
   - Paste your PyPI API token

2. Create a GitHub release:
   - Tag the version (e.g., `v0.1.3`)
   - Publish the release
   - GitHub Actions will automatically build and publish to PyPI

## Post-Publishing

1. Verify the package on PyPI: https://pypi.org/project/hardware-address/
2. Test installation:
   ```bash
   pip install hardware-address
   ```
3. Update documentation with installation instructions

## Troubleshooting

### Build fails on specific platform:
- Check Rust toolchain compatibility
- Ensure pyo3 version supports the target platform

### Import error after installation:
- Verify the wheel was built with the correct features: `--features pyo3`
- Check Python version compatibility

### Publishing fails:
- Verify API token is correct
- Check if version already exists on PyPI
- Ensure package name is not taken

## Resources

- [Maturin User Guide](https://www.maturin.rs/)
- [PyO3 Documentation](https://pyo3.rs/)
- [PyPI Publishing Guide](https://packaging.python.org/tutorials/packaging-projects/)
