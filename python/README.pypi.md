# hardware-address

IEEE 802 MAC-48, EUI-48, EUI-64, and InfiniBand hardware addresses for Python.

[![PyPI version](https://img.shields.io/pypi/v/hardware-address.svg)](https://pypi.org/project/hardware-address/)
[![License](https://img.shields.io/badge/License-Apache%202.0%2FMIT-blue.svg)](https://github.com/al8n/hardware-address)
[![Python versions](https://img.shields.io/pypi/pyversions/hardware-address.svg)](https://pypi.org/project/hardware-address/)

A fast, memory-safe library for working with hardware addresses, powered by Rust.

## Installation

```bash
pip install hardware_address
```

## Features

- 🚀 **Fast**: Native Rust implementation with Python bindings
- 🔒 **Memory Safe**: Written in Rust for guaranteed memory safety
- 🎯 **Type Safe**: Full type hints and IDE autocomplete support
- 📦 **Zero Dependencies**: No external dependencies
- 🔄 **Multiple Formats**: Parse and format addresses in colon, hyphen, or dot-separated formats

## Quick Start

```python
from hardware_address import MacAddr

# Parse from string
addr = MacAddr.parse("00:00:5e:00:53:01")

# Create from bytes
addr = MacAddr.from_bytes(b"\x00\x00\x5e\x00\x53\x01")

# String representation
print(str(addr))   # "00:00:5e:00:53:01"
print(repr(addr))  # MacAddr("00:00:5e:00:53:01")

# Get bytes
data = bytes(addr)

# Comparison and hashing
if addr1 == addr2:
    print("Addresses match!")

# Use as dictionary key
devices = {addr: "Server 1"}
```

## Supported Address Types

### MacAddr (6 bytes)

IEEE 802 MAC-48/EUI-48 addresses:

```python
from hardware_address import MacAddr

addr = MacAddr.parse("00:00:5e:00:53:01")
print(addr)  # 00:00:5e:00:53:01
```

### Eui64Addr (8 bytes)

EUI-64 addresses used in IPv6 and other protocols:

```python
from hardware_address import Eui64Addr

addr = Eui64Addr.parse("02:00:5e:10:00:00:00:01")
print(addr)  # 02:00:5e:10:00:00:00:01
```

### InfiniBandAddr (20 bytes)

IP over InfiniBand link-layer addresses:

```python
from hardware_address import InfiniBandAddr

addr = InfiniBandAddr.parse(
    "00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01"
)
```

## Parsing Formats

All address types support three parsing formats:

```python
from hardware_address import MacAddr

# Colon-separated (standard)
addr1 = MacAddr.parse("00:00:5e:00:53:01")

# Hyphen-separated (Windows style)
addr2 = MacAddr.parse("00-00-5e-00-53-01")

# Dot-separated (Cisco style)
addr3 = MacAddr.parse("0000.5e00.5301")

# All produce the same address
assert addr1 == addr2 == addr3
```

## API Reference

### Creating Addresses

```python
# From string
addr = MacAddr.parse("00:00:5e:00:53:01")

# From bytes
addr = MacAddr.from_bytes(b"\x00\x00\x5e\x00\x53\x01")
```

### Converting Addresses

```python
# To string (default format)
s = str(addr)  # "00:00:5e:00:53:01"

# To bytes
data = bytes(addr)  # b'\x00\x00\x5e\x00\x53\x01'

# Representation for debugging
r = repr(addr)  # MacAddr("00:00:5e:00:53:01")
```

### Comparison Operations

```python
# Equality
if addr1 == addr2:
    print("Equal")

# Ordering
if addr1 < addr2:
    print("addr1 is less than addr2")

# Hashing (use as dict key)
devices = {addr: "Device 1"}
```

### Error Handling

```python
from hardware_address import MacAddr

try:
    addr = MacAddr.parse("invalid")
except ValueError as e:
    print(f"Parse error: {e}")

try:
    addr = MacAddr.from_bytes(b"\x00\x00")  # Wrong length
except ValueError as e:
    print(f"Invalid length: {e}")
```

## Performance

This library is implemented in Rust and compiled to native code, providing:

- **Parsing**: ~10-50x faster than pure Python implementations
- **Memory**: Zero-copy operations where possible
- **Safety**: No buffer overflows or memory leaks

## Type Hints

Full type hints are included for IDE support:

```python
from hardware_address import MacAddr
from typing import Dict

def process_address(addr: MacAddr) -> bytes:
    return bytes(addr)

devices: Dict[MacAddr, str] = {}
devices[MacAddr.parse("00:00:5e:00:53:01")] = "Server"
```

## Platform Support

Pre-built wheels are available for:

- **Linux**: x86_64, i686, aarch64, armv7
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x64, x86
- **Python**: 3.8, 3.9, 3.10, 3.11, 3.12

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/al8n/hardware-address/blob/main/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/al8n/hardware-address/blob/main/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Links

- [Documentation](https://docs.rs/hardware-address)
- [Source Code](https://github.com/al8n/hardware-address)
- [Issue Tracker](https://github.com/al8n/hardware-address/issues)
- [Rust Crate](https://crates.io/crates/hardware-address)

## Contributing

Contributions are welcome! See the [repository](https://github.com/al8n/hardware-address) for details.
