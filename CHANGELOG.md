# UNRELEASED

## 0.2.0 (23rd Oct, 2025)

### Features

#### Python Bindings
- **Added Python package support** via PyO3
  - Available on PyPI as `hardware-address`
  - Supports Python 3.8+
  - Full API coverage for `MacAddr`, `Eui64Addr`, and `InfiniBandAddr`
  - Cross-platform wheels for Linux, macOS, Windows (x86_64, aarch64, armv7, etc.)
  - Added comprehensive test suite with pytest
  - Added CI/CD workflow for automated testing and PyPI publishing

#### WebAssembly Bindings

- **Added WebAssembly/JavaScript package support** via wasm-bindgen
  - Available on npm as `hardware-address`
  - Dual package supporting both ESM (import) and CommonJS (require)
  - TypeScript definitions auto-generated
  - Full API coverage for `MacAddr`, `Eui64Addr`, and `InfiniBandAddr`
  - Supports bundlers (webpack, vite, rollup) and Node.js
  - Added comprehensive test suite for Node.js
  - Added CI/CD workflow for automated testing and npm publishing

#### Property-Based Testing

- **Added `arbitrary::Arbitrary` trait** implementation
  - Enables fuzzing and property-based testing with cargo-fuzz
  - Generates random valid hardware addresses for testing
- **Added `quickcheck::Arbitrary` trait** implementation
  - Enables property-based testing with QuickCheck
  - Includes shrinking support for better error reporting

### Build & Infrastructure

- **Reorganized project structure**
  - Created `python/` directory for Python package
  - Created `wasm/` directory for WebAssembly package
  - Each subdirectory has its own build scripts and documentation
- **Added comprehensive CI/CD**
  - Python workflow: tests, multi-platform builds, PyPI releases
  - WASM workflow: tests, multi-target builds, npm releases
  - Automated release publishing on git tags
- **Added build scripts**
  - `python/build-python.sh` - Build Python wheels
  - `python/test.sh` - Run Python tests
  - `wasm/build-dual.sh` - Build dual ESM/CommonJS package
  - `wasm/build-npm.sh` - Build ESM-only package
  - `wasm/test.sh` - Run WASM tests for Node.js
  - `wasm/test-dual.sh` - Test dual package

### Documentation

- **Added comprehensive README files**
  - `python/README.md` - Python package documentation
  - `python/README.pypi.md` - PyPI-specific documentation
  - `wasm/README.md` - WASM package documentation
  - `wasm/README.npm.md` - npm-specific documentation
- **Added publishing guides**
  - Python publishing guide with maturin instructions
  - WASM publishing guide with wasm-pack instructions

### Dependencies

- Added `pyo3` 0.27 (optional, for Python bindings)
  - Enabled `macros`, `extension-module`, and `abi3-py37` features
- Added `wasm-bindgen` 0.2 (optional, for WebAssembly bindings)
- Added `arbitrary` 1.0 (optional, for fuzzing)
- Added `quickcheck` 1.0 (optional, for property testing)

### Internal Changes

- Added `[lib] crate-type = ["lib", "cdylib"]` to support Python extensions
- Added `#[pymodule]` function for Python module initialization
- Added wasm-bindgen implementations for JavaScript interop
- Updated `.gitignore` to exclude build artifacts

## 0.1.0 (January 5th, 2025)

### Features
