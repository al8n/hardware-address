# Hardware Address WASM Package

WebAssembly bindings for the `hardware-address` Rust library, providing support for IEEE 802 MAC-48, EUI-48, EUI-64, and InfiniBand hardware addresses in JavaScript/TypeScript.

## Package Structure

This directory contains a standalone WASM wrapper that re-exports the parent `hardware-address` crate with WASM bindings enabled.

## Build Options

### Dual Package (Recommended for npm)

Builds a package that supports both ESM (import) and CommonJS (require):

```bash
./build-dual.sh
```

This creates `pkg-dual/` with:
- **ESM** (import): For modern bundlers (webpack, vite, rollup) and browsers
- **CommonJS** (require): For Node.js without bundler

**Package exports:**
```json
{
  "exports": {
    ".": {
      "import": "./hardware_address.js",
      "require": "./node/hardware_address.js"
    }
  }
}
```

**Publish:**
```bash
cd pkg-dual
npm publish
```

### Bundler-Only Package

Builds ESM-only package (smaller, simpler):

```bash
./build-npm.sh
```

This creates `pkg/` with ESM support only. Good for projects that exclusively use bundlers.

### Node.js Testing

Build and test for Node.js:

```bash
./test.sh           # Tests with Node.js target
./test-dual.sh      # Tests dual package (both ESM and CommonJS)
```

## Usage

### Installation

```bash
npm install hardware-address
```

### ESM (Modern JavaScript/TypeScript)

```javascript
import { MacAddr, Eui64Addr, InfiniBandAddr } from 'hardware-address';

// Parse MAC address
const mac = MacAddr.parse("00:11:22:33:44:55");
console.log(mac.toString());  // "00:11:22:33:44:55"

// Create from bytes
const bytes = new Uint8Array([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
const mac2 = MacAddr.fromBytes(bytes);

// Format conversions
console.log(mac2.toColonSeparated());   // "aa:bb:cc:dd:ee:ff"
console.log(mac2.toHyphenSeparated());  // "aa-bb-cc-dd-ee-ff"
console.log(mac2.toDotSeparated());     // "aabb.ccdd.eeff"
```

### CommonJS (Node.js)

```javascript
const { MacAddr, Eui64Addr, InfiniBandAddr } = require('hardware-address');

const mac = MacAddr.parse("00:11:22:33:44:55");
console.log(mac.toString());
```

### TypeScript

TypeScript definitions are automatically included:

```typescript
import { MacAddr, Eui64Addr, InfiniBandAddr } from 'hardware-address';

const mac: MacAddr = MacAddr.parse("00:11:22:33:44:55");
const bytes: Uint8Array = mac.toBytes();
```

## API

All three address types (`MacAddr`, `Eui64Addr`, `InfiniBandAddr`) support:

### Static Methods

- `parse(s: string): Address` - Parse from string (colon, hyphen, or dot separated)
- `fromBytes(bytes: Uint8Array): Address` - Create from byte array
- `new()` - Create zeroed address (constructor)

### Instance Methods

- `toString(): string` - Convert to default string format (colon-separated)
- `toBytes(): Uint8Array` - Get raw bytes
- `toColonSeparated(): string` - Format as "aa:bb:cc:dd:ee:ff"
- `toHyphenSeparated(): string` - Format as "aa-bb-cc-dd-ee-ff"
- `toDotSeparated(): string` - Format as "aabb.ccdd.eeff"

### Address Types

- **MacAddr**: 6-byte MAC-48/EUI-48 address
- **Eui64Addr**: 8-byte EUI-64 address
- **InfiniBandAddr**: 20-byte InfiniBand address

## Development

### Prerequisites

- Rust (latest stable)
- wasm-pack (`cargo install wasm-pack`)
- Node.js 18+

### Building

```bash
# Dual package (ESM + CommonJS)
./build-dual.sh

# ESM only
./build-npm.sh

# Development/testing build
wasm-pack build --target nodejs --dev
```

### Testing

```bash
# Test dual package
./test-dual.sh

# Test Node.js build only
./test.sh
```

### Project Structure

```
wasm/
├── src/
│   └── lib.rs                  # Re-exports hardware-address with WASM bindings
├── Cargo.toml                  # WASM-specific dependencies
├── build-dual.sh               # Build dual ESM/CommonJS package
├── build-npm.sh                # Build ESM-only package
├── test.sh                     # Test Node.js build
├── test-dual.sh                # Test dual package
├── test/
│   ├── node-test.js            # Node.js CommonJS tests
│   ├── test-commonjs.js        # CommonJS import tests
│   └── test-esm.mjs            # ESM import tests
└── README.md                   # This file
```

## CI/CD

GitHub Actions workflow (`.github/workflows/wasm.yml`) automatically:
- Builds and tests the dual package
- Verifies both ESM and CommonJS work correctly
- Uploads build artifacts
- Builds for multiple targets (bundler, nodejs, web)

## License

MIT OR Apache-2.0
