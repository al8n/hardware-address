# hardware-address

IEEE 802 MAC-48, EUI-48, EUI-64, and InfiniBand hardware addresses for WebAssembly.

[![npm version](https://img.shields.io/npm/v/hardware-address.svg)](https://www.npmjs.com/package/hardware-address)
[![License](https://img.shields.io/badge/License-Apache%202.0%2FMIT-blue.svg)](https://github.com/al8n/hardware-address)

## Installation

```bash
npm install hardware-address
```

## Usage

### ES Modules (Browser/Bundler)

```javascript
import { MacAddr, Eui64Addr, InfiniBandAddr } from 'hardware-address';

// Parse from string
const addr = MacAddr.parse('00:00:5e:00:53:01');

// Create from bytes
const bytes = new Uint8Array([0x00, 0x00, 0x5e, 0x00, 0x53, 0x01]);
const addr2 = MacAddr.fromBytes(bytes);

// Format conversions
console.log(addr.toString());             // "00:00:5e:00:53:01"
console.log(addr.toHyphenSeparated());    // "00-00-5e-00-53-01"
console.log(addr.toDotSeparated());       // "0000.5e00.5301"

// Get bytes
const rawBytes = addr.toBytes();
```

### Node.js (CommonJS)

```javascript
const { MacAddr } = require('hardware-address');

const addr = MacAddr.parse('00:00:5e:00:53:01');
console.log(addr.toString());
```

### TypeScript

```typescript
import { MacAddr, Eui64Addr, InfiniBandAddr } from 'hardware-address';

const addr: MacAddr = MacAddr.parse('00:00:5e:00:53:01');
const bytes: Uint8Array = addr.toBytes();

// Type-safe API
const formatted: string = addr.toColonSeparated();
```

## API Reference

### MacAddr (6-byte IEEE 802 MAC-48/EUI-48)

```typescript
class MacAddr {
  // Constructors
  constructor();                              // Creates zeroed address
  static fromBytes(bytes: Uint8Array): MacAddr;
  static parse(s: string): MacAddr;

  // Conversions
  toString(): string;                         // Colon-separated format
  toBytes(): Uint8Array;
  toColonSeparated(): string;                 // "00:00:5e:00:53:01"
  toHyphenSeparated(): string;                // "00-00-5e-00-53-01"
  toDotSeparated(): string;                   // "0000.5e00.5301"

  // Cleanup
  free(): void;
}
```

### Eui64Addr (8-byte EUI-64)

```typescript
class Eui64Addr {
  constructor();
  static fromBytes(bytes: Uint8Array): Eui64Addr;
  static parse(s: string): Eui64Addr;
  toString(): string;
  toBytes(): Uint8Array;
  toColonSeparated(): string;
  toHyphenSeparated(): string;
  toDotSeparated(): string;
  free(): void;
}
```

### InfiniBandAddr (20-byte IP over InfiniBand)

```typescript
class InfiniBandAddr {
  constructor();
  static fromBytes(bytes: Uint8Array): InfiniBandAddr;
  static parse(s: string): InfiniBandAddr;
  toString(): string;
  toBytes(): Uint8Array;
  toColonSeparated(): string;
  toHyphenSeparated(): string;
  toDotSeparated(): string;
  free(): void;
}
```

## Supported Formats

All address types support parsing and formatting in three standard formats:

| Format | Example |
|--------|---------|
| Colon-separated | `00:00:5e:00:53:01` |
| Hyphen-separated | `00-00-5e-00-53-01` |
| Dot-separated | `0000.5e00.5301` |

## Examples

### Parsing Multiple Formats

```javascript
import { MacAddr } from 'hardware-address';

// All of these parse to the same address
const addr1 = MacAddr.parse('00:00:5e:00:53:01');
const addr2 = MacAddr.parse('00-00-5e-00-53-01');
const addr3 = MacAddr.parse('0000.5e00.5301');

console.log(addr1.toString() === addr2.toString()); // true
console.log(addr2.toString() === addr3.toString()); // true
```

### Working with Bytes

```javascript
import { MacAddr } from 'hardware-address';

// Create from raw bytes
const bytes = new Uint8Array([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
const addr = MacAddr.fromBytes(bytes);

console.log(addr.toString()); // "aa:bb:cc:dd:ee:ff"

// Convert back to bytes
const rawBytes = addr.toBytes();
console.log(rawBytes); // Uint8Array [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]
```

### EUI-64 Addresses

```javascript
import { Eui64Addr } from 'hardware-address';

const addr = Eui64Addr.parse('02:00:5e:10:00:00:00:01');
console.log(addr.toHyphenSeparated()); // "02-00-5e-10-00-00-00-01"
console.log(addr.toDotSeparated());    // "0200.5e10.0000.0001"
```

### InfiniBand Addresses

```javascript
import { InfiniBandAddr } from 'hardware-address';

const addr = InfiniBandAddr.parse(
  '00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01'
);
console.log(addr.toString());
```

## Error Handling

All parsing and creation methods throw errors on invalid input:

```javascript
import { MacAddr } from 'hardware-address';

try {
  // Invalid: wrong length
  const addr = MacAddr.parse('00:00:5e:00:53');
} catch (error) {
  console.error('Parse error:', error.message);
}

try {
  // Invalid: wrong byte count
  const bytes = new Uint8Array([0x00, 0x00]);
  const addr = MacAddr.fromBytes(bytes);
} catch (error) {
  console.error('Invalid length:', error.message);
}
```

## Browser Compatibility

This package uses WebAssembly and requires:
- Modern browsers with WebAssembly support
- Node.js 12.0 or higher

## Performance

Hardware Address is compiled to WebAssembly from Rust, providing:
- ⚡ Near-native performance
- 🔒 Memory safety
- 📦 Small bundle size (~20KB gzipped)
- 🚀 Zero runtime dependencies

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/al8n/hardware-address/blob/main/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/al8n/hardware-address/blob/main/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see the [main repository](https://github.com/al8n/hardware-address) for contribution guidelines.

## Links

- [Documentation](https://docs.rs/hardware-address)
- [Source Code](https://github.com/al8n/hardware-address)
- [Issue Tracker](https://github.com/al8n/hardware-address/issues)
- [Rust Crate](https://crates.io/crates/hardware-address)
