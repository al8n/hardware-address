// For Node.js, we need to use the nodejs target build
// Run: wasm-pack build --target nodejs --out-dir pkg-nodejs

const { MacAddr, Eui64Addr, InfiniBandAddr } = require('../pkg-nodejs/hardware_address.js');

console.log('Testing hardware-address WASM package...\n');

// Helper function for testing
function assert(condition, message) {
  if (!condition) {
    throw new Error('Assertion failed: ' + message);
  }
  console.log('✓ ' + message);
}

// Test MacAddr (MAC-48 / 6 bytes)
console.log('=== Testing MacAddr (MAC-48) ===');
const mac = MacAddr.parse("00:11:22:33:44:55");
assert(mac.toString() === '00:11:22:33:44:55', 'MacAddr parse and toString');

// Test MacAddr from bytes
const bytes_mac = new Uint8Array([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
const mac2 = MacAddr.fromBytes(bytes_mac);
assert(mac2.toString() === 'aa:bb:cc:dd:ee:ff', 'MacAddr from bytes');

// Test MacAddr toBytes
const retrieved_bytes = mac2.toBytes();
assert(retrieved_bytes.length === 6, 'MacAddr toBytes length');
assert(retrieved_bytes[0] === 0xAA, 'MacAddr toBytes content');

// Test format conversions
assert(mac2.toColonSeparated() === 'aa:bb:cc:dd:ee:ff', 'MacAddr colon-separated');
assert(mac2.toHyphenSeparated() === 'aa-bb-cc-dd-ee-ff', 'MacAddr hyphen-separated');
assert(mac2.toDotSeparated() === 'aabb.ccdd.eeff', 'MacAddr dot-separated');

// Test Eui64Addr (8 bytes)
console.log('\n=== Testing Eui64Addr ===');
const eui64 = Eui64Addr.parse("00:11:22:33:44:55:66:77");
assert(eui64.toString() === '00:11:22:33:44:55:66:77', 'Eui64Addr parse and toString');

// Test Eui64Addr from bytes
const bytes_eui64 = new Uint8Array([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
const eui64_2 = Eui64Addr.fromBytes(bytes_eui64);
assert(eui64_2.toString() === '01:23:45:67:89:ab:cd:ef', 'Eui64Addr from bytes');

// Test Eui64Addr format conversions
assert(eui64_2.toColonSeparated() === '01:23:45:67:89:ab:cd:ef', 'Eui64Addr colon-separated');
assert(eui64_2.toHyphenSeparated() === '01-23-45-67-89-ab-cd-ef', 'Eui64Addr hyphen-separated');

// Test InfiniBand
console.log('\n=== Testing InfiniBandAddr ===');
const ib_bytes = new Uint8Array(20);
for (let i = 0; i < 20; i++) {
  ib_bytes[i] = i;
}
const ib = InfiniBandAddr.fromBytes(ib_bytes);
assert(ib.toBytes().length === 20, 'InfiniBand address length');

const ib_hex = ib.toString();
assert(ib_hex.includes(':'), 'InfiniBand toString format');

// Test error handling
console.log('\n=== Testing Error Handling ===');
try {
  MacAddr48.parse("invalid");
  assert(false, 'Should throw error for invalid MAC-48');
} catch (e) {
  assert(true, 'MAC-48 parse error handling');
}

try {
  const invalid_bytes = new Uint8Array([0x00, 0x11]); // Too short
  MacAddr48.fromBytes(invalid_bytes);
  assert(false, 'Should throw error for invalid byte length');
} catch (e) {
  assert(true, 'MAC-48 fromBytes error handling');
}

console.log('\n✅ All tests passed!');
