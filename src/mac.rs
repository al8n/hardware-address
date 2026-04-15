addr_ty!(
  /// Represents a physical hardware address (MAC address).
  #[doc(alias = "Eui48Addr")]
  MacAddr[6]
);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{ParseError, TestCase};

  use std::{string::ToString, vec, vec::Vec};

  const MAC_ADDRESS_SIZE: usize = 6;

  fn test_cases() -> Vec<TestCase<MAC_ADDRESS_SIZE>> {
    vec![
      // RFC 7042, Section 2.1.1
      TestCase {
        input: "00:00:5e:00:53:01",
        output: Some(vec![0x00, 0x00, 0x5e, 0x00, 0x53, 0x01]),
        err: None,
      },
      TestCase {
        input: "00-00-5e-00-53-01",
        output: Some(vec![0x00, 0x00, 0x5e, 0x00, 0x53, 0x01]),
        err: None,
      },
      TestCase {
        input: "0000.5e00.5301",
        output: Some(vec![0x00, 0x00, 0x5e, 0x00, 0x53, 0x01]),
        err: None,
      },
      TestCase {
        input: "ab:cd:ef:AB:CD:EF",
        output: Some(vec![0xab, 0xcd, 0xef, 0xab, 0xcd, 0xef]),
        err: None,
      },
      // Invalid MAC-48 cases
      TestCase {
        input: "01.02.03.04.05.06",
        output: None,
        err: Some(ParseError::InvalidSeparator(b'.')),
      },
      TestCase {
        input: "01:02:03:04:05:06:",
        output: None,
        err: Some(ParseError::InvalidLength(18)),
      },
      TestCase {
        input: "x1:02:03:04:05:06",
        output: None,
        err: Some(ParseError::InvalidHexDigit([b'x', b'1'])),
      },
      TestCase {
        input: "01-02:03:04:05:06",
        output: None,
        err: Some(ParseError::UnexpectedSeparator {
          expected: b'-',
          actual: b':',
        }),
      },
    ]
  }

  #[test]
  fn parse() {
    let cases = test_cases();
    for (i, test) in cases.iter().enumerate() {
      let result = MacAddr::try_from(test.input);

      match (result, &test.output) {
        (Ok(out), Some(expected)) => {
          assert_eq!(
            expected.as_slice(),
            out,
            "Test case {}: MacAddr::parse({}) output mismatch",
            i,
            test.input
          );

          // Test round-trip if this was a valid case
          if test.err.is_none() {
            let formatted = out.to_string();
            let round_trip = MacAddr::try_from(formatted.as_str());
            assert!(
              round_trip.is_ok(),
              "Test case {}: Round-trip parse failed for {}",
              i,
              formatted
            );
            assert_eq!(
              round_trip.unwrap(),
              out,
              "Test case {}: Round-trip value mismatch",
              i
            );
          }
        }
        (Err(err), None) => {
          assert_eq!(
            Some(&err),
            test.err.as_ref(),
            "Test case {}: Expected error containing '{:?}', got '{:?}'",
            i,
            test.err,
            err
          );
        }
        (Ok(out), None) => {
          panic!(
            "Test case {}: Expected error '{:?}', got success: {:?}",
            i, test.err, out
          );
        }
        (Err(err), Some(expected)) => {
          panic!(
            "Test case {}: Expected {:?}, got error: {:?}",
            i, expected, err
          );
        }
      }
    }
  }

  #[test]
  fn test_default() {
    let addr = MacAddr::default();
    assert_eq!(addr.octets(), [0, 0, 0, 0, 0, 0]);
  }

  #[test]
  fn formatted() {
    let addr = MacAddr::try_from("00:00:5e:00:53:01").unwrap();
    assert_eq!(addr.to_string(), "00:00:5e:00:53:01");
    assert_eq!(addr.to_colon_separated(), "00:00:5e:00:53:01");

    let dot = addr.to_dot_separated_array();
    let dot_str = core::str::from_utf8(&dot).unwrap();
    assert_eq!(dot_str, "0000.5e00.5301");
    assert_eq!(addr.to_dot_separated(), "0000.5e00.5301");

    let dashed = addr.to_hyphen_separated_array();
    let dashed_str = core::str::from_utf8(&dashed).unwrap();
    assert_eq!(dashed_str, "00-00-5e-00-53-01");
    assert_eq!(addr.to_hyphen_separated(), "00-00-5e-00-53-01");
  }

  #[cfg(feature = "serde")]
  #[test]
  fn serde_human_readable() {
    let addr = MacAddr::try_from("00:00:5e:00:53:01").unwrap();
    let json = serde_json::to_string(&addr).unwrap();
    assert_eq!(json, "\"00:00:5e:00:53:01\"");

    let addr2: MacAddr = serde_json::from_str(&json).unwrap();
    assert_eq!(addr, addr2);
  }

  #[cfg(feature = "serde")]
  #[test]
  fn serde_human_unreadable() {
    let addr = MacAddr::try_from("00:00:5e:00:53:01").unwrap();
    let json = bincode::serde::encode_to_vec(addr, bincode::config::standard()).unwrap();
    assert_eq!(json, [0, 0, 94, 0, 83, 1]);
    assert_eq!(addr.octets(), [0, 0, 94, 0, 83, 1]);

    let addr2: MacAddr = bincode::serde::decode_from_slice(&json, bincode::config::standard())
      .unwrap()
      .0;
    assert_eq!(addr, addr2);

    let addr3 = MacAddr::from_raw([0, 0, 94, 0, 83, 1]);
    assert_eq!(addr, addr3);

    println!("{:?}", addr);
  }

  // ------------------------------------------------------------------
  // `arbitrary` / `quickcheck` Arbitrary impls
  // ------------------------------------------------------------------

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_is_deterministic() {
    use arbitrary::{Arbitrary, Unstructured};

    // The generated impl delegates to `<[u8; 6]>::arbitrary`, so the
    // same input bytes must produce the same output every time.
    let data = [0xAA; 32];
    let a = MacAddr::arbitrary(&mut Unstructured::new(&data)).expect("arbitrary should succeed");
    let b = MacAddr::arbitrary(&mut Unstructured::new(&data)).expect("arbitrary should succeed");
    assert_eq!(a, b, "arbitrary should be deterministic for a fixed input");
  }

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_size_hint_matches_byte_array() {
    use arbitrary::Arbitrary;

    // The generated impl must forward `size_hint` to the byte array's
    // — otherwise fuzzers will over- or under-budget the input.
    let hint = MacAddr::size_hint(0);
    let expected = <[u8; MAC_ADDRESS_SIZE] as Arbitrary>::size_hint(0);
    assert_eq!(hint, expected);
  }

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_consumes_expected_bytes() {
    use arbitrary::{Arbitrary, Unstructured};

    // A MacAddr is 6 bytes; pulling one from an Unstructured should
    // leave `buf.len() - 6` bytes available for the next draw.
    let data = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut u = Unstructured::new(&data);
    let _first = MacAddr::arbitrary(&mut u).unwrap();
    let _second = MacAddr::arbitrary(&mut u).unwrap();
    // Two 6-byte draws from 12 bytes must succeed; a third draw
    // against an empty buffer would not, but we don't assert on the
    // exact error kind (that's an `arbitrary` implementation detail).
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_arbitrary_roundtrips_through_string() {
    use quickcheck::{Arbitrary, Gen};

    let mut g = Gen::new(32);
    for _ in 0..128 {
      let addr = MacAddr::arbitrary(&mut g);
      let parsed =
        MacAddr::try_from(addr.to_string().as_str()).expect("to_string() output must parse back");
      assert_eq!(addr, parsed);
    }
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_shrink_terminates_and_preserves_length() {
    use quickcheck::Arbitrary;

    let addr = MacAddr::from_raw([0xFF; MAC_ADDRESS_SIZE]);
    // Cap the collect so a hypothetical upstream change that made
    // shrink lazy-infinite can't hang the test suite.
    let shrinks: Vec<_> = addr.shrink().take(4096).collect();
    assert!(!shrinks.is_empty(), "non-zero address should yield shrinks");
    // The impl filters `Vec<u8>::shrink` down to length-N vecs before
    // rebuilding the tuple struct — every yielded item is therefore a
    // well-formed MacAddr (implicit by type).
    for s in &shrinks {
      assert_eq!(s.octets().len(), MAC_ADDRESS_SIZE);
    }
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_shrink_zero_is_empty() {
    use quickcheck::Arbitrary;

    // A zero MacAddr is already minimal: each byte's u8::shrink is
    // empty, and the length-reducing shrinks are filtered out by our
    // `.filter_map` guard, so the overall iterator must be empty.
    let zero = MacAddr::from_raw([0; MAC_ADDRESS_SIZE]);
    let shrinks: Vec<_> = zero.shrink().collect();
    assert!(
      shrinks.is_empty(),
      "zero address should yield no shrinks, got {:?}",
      shrinks
    );
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_roundtrip_property() {
    // Smoke-test that the `quickcheck` harness accepts our Arbitrary
    // impl (the full library integration, not just the trait in
    // isolation). The property below is the same string-roundtrip as
    // `quickcheck_arbitrary_roundtrips_through_string`, but driven by
    // `quickcheck::quickcheck` rather than a hand-rolled loop.
    fn prop(addr: MacAddr) -> bool {
      MacAddr::try_from(addr.to_string().as_str())
        .map(|p| p == addr)
        .unwrap_or(false)
    }
    quickcheck::quickcheck(prop as fn(MacAddr) -> bool);
  }
}
