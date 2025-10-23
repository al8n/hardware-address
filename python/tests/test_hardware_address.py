"""Unit tests for hardware_address Python bindings."""
import unittest
from hardware_address import MacAddr, Eui64Addr, InfiniBandAddr


class TestMacAddr(unittest.TestCase):
    """Test MacAddr (6-byte MAC-48/EUI-48 addresses)."""

    def test_parse_and_to_string(self):
        """Test parsing and converting to string."""
        mac = MacAddr.parse("00:11:22:33:44:55")
        self.assertEqual(str(mac), "00:11:22:33:44:55")

    def test_from_bytes(self):
        """Test creating from bytes."""
        mac_bytes = bytes([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF])
        mac = MacAddr.from_bytes(mac_bytes)
        self.assertEqual(str(mac), "aa:bb:cc:dd:ee:ff")

    def test_to_bytes(self):
        """Test converting to bytes."""
        mac = MacAddr.parse("aa:bb:cc:dd:ee:ff")
        mac_bytes = bytes(mac)
        self.assertEqual(mac_bytes, bytes([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]))
        self.assertEqual(len(mac_bytes), 6)

    def test_format_conversions(self):
        """Test different format conversions."""
        mac = MacAddr.parse("aa:bb:cc:dd:ee:ff")
        self.assertEqual(mac.to_colon_separated(), "aa:bb:cc:dd:ee:ff")
        self.assertEqual(mac.to_hyphen_separated(), "aa-bb-cc-dd-ee-ff")
        self.assertEqual(mac.to_dot_separated(), "aabb.ccdd.eeff")

    def test_parse_different_formats(self):
        """Test parsing different input formats."""
        # Colon-separated
        mac1 = MacAddr.parse("00:11:22:33:44:55")
        self.assertEqual(str(mac1), "00:11:22:33:44:55")

        # Hyphen-separated
        mac2 = MacAddr.parse("00-11-22-33-44-55")
        self.assertEqual(str(mac2), "00:11:22:33:44:55")

        # Dot-separated
        mac3 = MacAddr.parse("0011.2233.4455")
        self.assertEqual(str(mac3), "00:11:22:33:44:55")

    def test_equality(self):
        """Test equality comparison."""
        mac1 = MacAddr.parse("00:11:22:33:44:55")
        mac2 = MacAddr.parse("00:11:22:33:44:55")
        mac3 = MacAddr.parse("aa:bb:cc:dd:ee:ff")

        self.assertEqual(mac1, mac2)
        self.assertNotEqual(mac1, mac3)

    def test_hash(self):
        """Test that addresses are hashable."""
        mac1 = MacAddr.parse("00:11:22:33:44:55")
        mac2 = MacAddr.parse("00:11:22:33:44:55")
        mac3 = MacAddr.parse("aa:bb:cc:dd:ee:ff")

        # Same addresses should have same hash
        self.assertEqual(hash(mac1), hash(mac2))

        # Can be used in sets/dicts
        mac_set = {mac1, mac2, mac3}
        self.assertEqual(len(mac_set), 2)

    def test_invalid_parse(self):
        """Test parsing invalid input raises error."""
        with self.assertRaises(ValueError):
            MacAddr.parse("invalid")

        with self.assertRaises(ValueError):
            MacAddr.parse("00:11:22:33:44")  # Too short

    def test_invalid_bytes_length(self):
        """Test creating from invalid byte length raises error."""
        with self.assertRaises(ValueError):
            MacAddr.from_bytes(bytes([0x00, 0x11]))  # Too short


class TestEui64Addr(unittest.TestCase):
    """Test Eui64Addr (8-byte EUI-64 addresses)."""

    def test_parse_and_to_string(self):
        """Test parsing and converting to string."""
        eui64 = Eui64Addr.parse("00:11:22:33:44:55:66:77")
        self.assertEqual(str(eui64), "00:11:22:33:44:55:66:77")

    def test_from_bytes(self):
        """Test creating from bytes."""
        eui64_bytes = bytes([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF])
        eui64 = Eui64Addr.from_bytes(eui64_bytes)
        self.assertEqual(str(eui64), "01:23:45:67:89:ab:cd:ef")

    def test_to_bytes(self):
        """Test converting to bytes."""
        eui64 = Eui64Addr.parse("01:23:45:67:89:ab:cd:ef")
        eui64_bytes = bytes(eui64)
        self.assertEqual(eui64_bytes, bytes([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]))
        self.assertEqual(len(eui64_bytes), 8)

    def test_format_conversions(self):
        """Test different format conversions."""
        eui64 = Eui64Addr.parse("01:23:45:67:89:ab:cd:ef")
        self.assertEqual(eui64.to_colon_separated(), "01:23:45:67:89:ab:cd:ef")
        self.assertEqual(eui64.to_hyphen_separated(), "01-23-45-67-89-ab-cd-ef")

    def test_equality(self):
        """Test equality comparison."""
        eui64_1 = Eui64Addr.parse("00:11:22:33:44:55:66:77")
        eui64_2 = Eui64Addr.parse("00:11:22:33:44:55:66:77")
        eui64_3 = Eui64Addr.parse("01:23:45:67:89:ab:cd:ef")

        self.assertEqual(eui64_1, eui64_2)
        self.assertNotEqual(eui64_1, eui64_3)

    def test_invalid_parse(self):
        """Test parsing invalid input raises error."""
        with self.assertRaises(ValueError):
            Eui64Addr.parse("invalid")


class TestInfiniBandAddr(unittest.TestCase):
    """Test InfiniBandAddr (20-byte addresses)."""

    def test_from_bytes(self):
        """Test creating from bytes."""
        ib_bytes = bytes(range(20))
        ib = InfiniBandAddr.from_bytes(ib_bytes)
        self.assertIsNotNone(ib)

    def test_to_bytes(self):
        """Test converting to bytes."""
        ib_bytes = bytes(range(20))
        ib = InfiniBandAddr.from_bytes(ib_bytes)
        result_bytes = bytes(ib)
        self.assertEqual(result_bytes, ib_bytes)
        self.assertEqual(len(result_bytes), 20)

    def test_to_string(self):
        """Test converting to string."""
        ib_bytes = bytes(range(20))
        ib = InfiniBandAddr.from_bytes(ib_bytes)
        ib_str = str(ib)
        # Should contain colons for hex format
        self.assertIn(":", ib_str)

    def test_equality(self):
        """Test equality comparison."""
        ib_bytes1 = bytes(range(20))
        ib_bytes2 = bytes(range(20))
        ib_bytes3 = bytes(range(20, 40))

        ib1 = InfiniBandAddr.from_bytes(ib_bytes1)
        ib2 = InfiniBandAddr.from_bytes(ib_bytes2)
        ib3 = InfiniBandAddr.from_bytes(ib_bytes3)

        self.assertEqual(ib1, ib2)
        self.assertNotEqual(ib1, ib3)

    def test_invalid_bytes_length(self):
        """Test creating from invalid byte length raises error."""
        with self.assertRaises(ValueError):
            InfiniBandAddr.from_bytes(bytes([0x00, 0x11]))  # Too short


if __name__ == "__main__":
    unittest.main()
