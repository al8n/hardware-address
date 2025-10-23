# Hardware Address Python Package

Python bindings for the `hardware-address` Rust library, providing support for IEEE 802 MAC-48, EUI-48, EUI-64, and InfiniBand hardware addresses.

## Installation

```bash
pip install hardware-address
```

## Usage

### Basic Examples

```python
from hardware_address import MacAddr, Eui64Addr, InfiniBandAddr

# Parse MAC address from string
mac = MacAddr.parse("00:11:22:33:44:55")
print(mac)  # 00:11:22:33:44:55

# Create from bytes
mac_bytes = bytes([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF])
mac2 = MacAddr.from_bytes(mac_bytes)
print(mac2)  # aa:bb:cc:dd:ee:ff

# Format conversions
print(mac2.to_colon_separated())   # aa:bb:cc:dd:ee:ff
print(mac2.to_hyphen_separated())  # aa-bb-cc-dd-ee-ff
print(mac2.to_dot_separated())     # aabb.ccdd.eeff

# Convert to bytes
mac_bytes = bytes(mac2)
print(len(mac_bytes))  # 6
```

### EUI-64 Addresses

```python
from hardware_address import Eui64Addr

# Parse EUI-64 address
eui64 = Eui64Addr.parse("00:11:22:33:44:55:66:77")
print(eui64)  # 00:11:22:33:44:55:66:77

# Create from bytes
eui64_bytes = bytes([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF])
eui64 = Eui64Addr.from_bytes(eui64_bytes)
print(eui64.to_colon_separated())  # 01:23:45:67:89:ab:cd:ef
```

### InfiniBand Addresses

```python
from hardware_address import InfiniBandAddr

# Create from 20 bytes
ib_bytes = bytes(range(20))
ib = InfiniBandAddr.from_bytes(ib_bytes)
print(ib)

# Convert back to bytes
result = bytes(ib)
print(len(result))  # 20
```

### Parsing Different Formats

All address types support multiple input formats:

```python
from hardware_address import MacAddr

# Colon-separated
mac1 = MacAddr.parse("00:11:22:33:44:55")

# Hyphen-separated
mac2 = MacAddr.parse("00-11-22-33-44-55")

# Dot-separated (Cisco format)
mac3 = MacAddr.parse("0011.2233.4455")

# All produce the same result
assert mac1 == mac2 == mac3
```

### Comparison and Hashing

Addresses support equality comparison and can be used in sets/dicts:

```python
from hardware_address import MacAddr

mac1 = MacAddr.parse("00:11:22:33:44:55")
mac2 = MacAddr.parse("00:11:22:33:44:55")
mac3 = MacAddr.parse("aa:bb:cc:dd:ee:ff")

# Equality
assert mac1 == mac2
assert mac1 != mac3

# Can be used in sets
mac_set = {mac1, mac2, mac3}
print(len(mac_set))  # 2

# Can be used as dict keys
mac_dict = {mac1: "Device 1", mac3: "Device 2"}
```

## API Reference

### MacAddr (6-byte MAC-48/EUI-48)

- `MacAddr.parse(s: str) -> MacAddr` - Parse from string (colon, hyphen, or dot separated)
- `MacAddr.from_bytes(bytes: bytes) -> MacAddr` - Create from 6-byte sequence
- `str(mac)` - Convert to string (colon-separated format)
- `bytes(mac)` - Convert to bytes
- `mac.to_colon_separated() -> str` - Format as "aa:bb:cc:dd:ee:ff"
- `mac.to_hyphen_separated() -> str` - Format as "aa-bb-cc-dd-ee-ff"
- `mac.to_dot_separated() -> str` - Format as "aabb.ccdd.eeff"
- `mac == other` - Equality comparison
- `hash(mac)` - Hash for use in sets/dicts

### Eui64Addr (8-byte EUI-64)

- `Eui64Addr.parse(s: str) -> Eui64Addr` - Parse from string
- `Eui64Addr.from_bytes(bytes: bytes) -> Eui64Addr` - Create from 8-byte sequence
- `str(eui64)` - Convert to string
- `bytes(eui64)` - Convert to bytes
- `eui64.to_colon_separated() -> str` - Colon-separated format
- `eui64.to_hyphen_separated() -> str` - Hyphen-separated format
- `eui64 == other` - Equality comparison
- `hash(eui64)` - Hash for use in sets/dicts

### InfiniBandAddr (20-byte)

- `InfiniBandAddr.from_bytes(bytes: bytes) -> InfiniBandAddr` - Create from 20-byte sequence
- `str(ib)` - Convert to string
- `bytes(ib)` - Convert to bytes
- `ib == other` - Equality comparison
- `hash(ib)` - Hash for use in sets/dicts

## Development

### Prerequisites

- Rust (latest stable)
- Python 3.8+
- maturin (`pip install maturin`)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/al8n/hardware-address
cd hardware-address/python

# Build in development mode
maturin develop --release

# Or build wheels
maturin build --release
```

### Running Tests

```bash
# Install test dependencies
pip install pytest

# Run tests
./test.sh

# Or manually
maturin develop --release
pytest tests/ -v
```

### Project Structure

```
python/
├── tests/
│   ├── __init__.py
│   └── test_hardware_address.py   # Unit tests
├── pyproject.toml                 # Package configuration
├── build-python.sh                # Build script
├── test.sh                        # Test script
└── README.md                      # This file
```

## CI/CD

GitHub Actions workflow (`.github/workflows/python.yml`) automatically:
- Runs unit tests
- Builds wheels for all platforms (Linux, macOS, Windows)
- Builds for multiple architectures (x86_64, aarch64, etc.)
- Publishes to PyPI on tagged releases

## License

MIT OR Apache-2.0

## Links

- [GitHub Repository](https://github.com/al8n/hardware-address)
- [Documentation](https://docs.rs/hardware-address)
- [PyPI Package](https://pypi.org/project/hardware-address/)
- [Issue Tracker](https://github.com/al8n/hardware-address/issues)
