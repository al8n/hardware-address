// Test CommonJS (require) support
const { MacAddr, Eui64Addr, InfiniBandAddr } = require('../pkg-dual/node/hardware_address.js');

console.log('Testing CommonJS (require) support...\n');

function assert(condition, message) {
  if (!condition) {
    throw new Error('Assertion failed: ' + message);
  }
  console.log('✓ ' + message);
}

// Test MacAddr
const mac = MacAddr.parse("00:11:22:33:44:55");
assert(mac.toString() === '00:11:22:33:44:55', 'MacAddr parse and toString');

const bytes = new Uint8Array([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
const mac2 = MacAddr.fromBytes(bytes);
assert(mac2.toString() === 'aa:bb:cc:dd:ee:ff', 'MacAddr from bytes');

// Test Eui64Addr
const eui64 = Eui64Addr.parse("00:11:22:33:44:55:66:77");
assert(eui64.toString() === '00:11:22:33:44:55:66:77', 'Eui64Addr parse and toString');

// Test InfiniBandAddr
const ib_bytes = new Uint8Array(20);
for (let i = 0; i < 20; i++) {
  ib_bytes[i] = i;
}
const ib = InfiniBandAddr.fromBytes(ib_bytes);
assert(ib.toBytes().length === 20, 'InfiniBandAddr length');

console.log('\n✅ CommonJS tests passed!');
