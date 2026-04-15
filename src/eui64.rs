addr_ty!(
  /// Represents a physical EUI-64 format address.
  Eui64Addr[8]
);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{ParseError, TestCase};

  use std::{string::ToString, vec, vec::Vec};

  const EUI64_ADDRESS_SIZE: usize = 8;

  fn test_cases() -> Vec<TestCase<EUI64_ADDRESS_SIZE>> {
    vec![
      // RFC 7042, Section 2.2.2
      TestCase {
        input: "02:00:5e:10:00:00:00:01",
        output: Some(vec![0x02, 0x00, 0x5e, 0x10, 0x00, 0x00, 0x00, 0x01]),
        err: None,
      },
      TestCase {
        input: "02-00-5e-10-00-00-00-01",
        output: Some(vec![0x02, 0x00, 0x5e, 0x10, 0x00, 0x00, 0x00, 0x01]),
        err: None,
      },
      TestCase {
        input: "0200.5e10.0000.0001",
        output: Some(vec![0x02, 0x00, 0x5e, 0x10, 0x00, 0x00, 0x00, 0x01]),
        err: None,
      },
      TestCase {
        input: "ab:cd:ef:AB:CD:EF:ab:cd",
        output: Some(vec![0xab, 0xcd, 0xef, 0xab, 0xcd, 0xef, 0xab, 0xcd]),
        err: None,
      },
      TestCase {
        input: "0200-5e10.0000.0001",
        output: None,
        err: Some(ParseError::UnexpectedSeparator {
          expected: b'.',
          actual: b'-',
        }),
      },
      TestCase {
        input: "xx00.5e10.0000.0001",
        output: None,
        err: Some(ParseError::InvalidHexDigit([b'x', b'x'])),
      },
      TestCase {
        input: "00xx.5e10.0000.0001",
        output: None,
        err: Some(ParseError::InvalidHexDigit([b'x', b'x'])),
      },
    ]
  }

  #[test]
  fn parse() {
    let cases = test_cases();
    for (i, test) in cases.iter().enumerate() {
      let result = Eui64Addr::try_from(test.input);

      match (result, &test.output) {
        (Ok(out), Some(expected)) => {
          assert_eq!(
            out,
            expected.as_slice(),
            "Test case {}: Eui64Addr::parse({}) output mismatch",
            i,
            test.input
          );

          // Test round-trip if this was a valid case
          if test.err.is_none() {
            let formatted = out.to_string();
            let round_trip = Eui64Addr::try_from(formatted.as_str());
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
    let addr = Eui64Addr::default();
    assert_eq!(addr.octets(), [0; EUI64_ADDRESS_SIZE]);
  }

  #[test]
  fn formatted() {
    let addr = Eui64Addr::try_from("02:00:5e:10:00:00:00:01").unwrap();
    assert_eq!(addr.to_string(), "02:00:5e:10:00:00:00:01");
    assert_eq!(addr.to_colon_separated(), "02:00:5e:10:00:00:00:01");

    let dot = addr.to_dot_separated_array();
    let dot_str = core::str::from_utf8(&dot).unwrap();
    assert_eq!(dot_str, "0200.5e10.0000.0001");
    assert_eq!(addr.to_dot_separated(), "0200.5e10.0000.0001");

    let dashed = addr.to_hyphen_separated_array();
    let dashed_str = core::str::from_utf8(&dashed).unwrap();
    assert_eq!(dashed_str, "02-00-5e-10-00-00-00-01");
    assert_eq!(addr.to_hyphen_separated(), "02-00-5e-10-00-00-00-01");
  }

  #[cfg(feature = "serde")]
  #[test]
  fn serde_human_readable() {
    let addr = Eui64Addr::try_from("02:00:5e:10:00:00:00:01").unwrap();
    let json = serde_json::to_string(&addr).unwrap();
    assert_eq!(json, "\"02:00:5e:10:00:00:00:01\"");

    let addr2: Eui64Addr = serde_json::from_str(&json).unwrap();
    assert_eq!(addr, addr2);
  }

  #[cfg(feature = "serde")]
  #[test]
  fn serde_human_unreadable() {
    let addr = Eui64Addr::try_from("02:00:5e:10:00:00:00:01").unwrap();
    let encoded = bincode::serde::encode_to_vec(addr, bincode::config::standard()).unwrap();
    assert_eq!(encoded, [2, 0, 94, 16, 0, 0, 0, 1]);
    assert_eq!(addr.octets(), [2, 0, 94, 16, 0, 0, 0, 1]);

    let addr2: Eui64Addr = bincode::serde::decode_from_slice(&encoded, bincode::config::standard())
      .unwrap()
      .0;
    assert_eq!(addr, addr2);

    let addr3 = Eui64Addr::from([2, 0, 94, 16, 0, 0, 0, 1]);
    assert_eq!(addr, addr3);

    let octets: [u8; EUI64_ADDRESS_SIZE] = addr3.into();
    assert_eq!(octets, addr3.octets());
    println!("{:?}", addr);
  }

  // ------------------------------------------------------------------
  // `arbitrary` / `quickcheck` Arbitrary impls
  // ------------------------------------------------------------------

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_is_deterministic() {
    use arbitrary::{Arbitrary, Unstructured};

    let data = [0xAA; 32];
    let a = Eui64Addr::arbitrary(&mut Unstructured::new(&data)).expect("arbitrary should succeed");
    let b = Eui64Addr::arbitrary(&mut Unstructured::new(&data)).expect("arbitrary should succeed");
    assert_eq!(a, b, "arbitrary should be deterministic for a fixed input");
  }

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_size_hint_matches_byte_array() {
    use arbitrary::Arbitrary;

    let hint = Eui64Addr::size_hint(0);
    let expected = <[u8; EUI64_ADDRESS_SIZE] as Arbitrary>::size_hint(0);
    assert_eq!(hint, expected);
  }

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_consumes_expected_bytes() {
    use arbitrary::{Arbitrary, Unstructured};

    // Two 8-byte draws should fit in a 16-byte buffer.
    let data = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut u = Unstructured::new(&data);
    let _first = Eui64Addr::arbitrary(&mut u).unwrap();
    let _second = Eui64Addr::arbitrary(&mut u).unwrap();
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_arbitrary_roundtrips_through_string() {
    use quickcheck::{Arbitrary, Gen};

    let mut g = Gen::new(32);
    for _ in 0..128 {
      let addr = Eui64Addr::arbitrary(&mut g);
      let parsed =
        Eui64Addr::try_from(addr.to_string().as_str()).expect("to_string() output must parse back");
      assert_eq!(addr, parsed);
    }
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_shrink_terminates_and_preserves_length() {
    use quickcheck::Arbitrary;

    let addr = Eui64Addr::from_raw([0xFF; EUI64_ADDRESS_SIZE]);
    let shrinks: Vec<_> = addr.shrink().take(4096).collect();
    assert!(!shrinks.is_empty(), "non-zero address should yield shrinks");
    for s in &shrinks {
      assert_eq!(s.octets().len(), EUI64_ADDRESS_SIZE);
    }
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_shrink_zero_is_empty() {
    use quickcheck::Arbitrary;

    let zero = Eui64Addr::from_raw([0; EUI64_ADDRESS_SIZE]);
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
    fn prop(addr: Eui64Addr) -> bool {
      Eui64Addr::try_from(addr.to_string().as_str())
        .map(|p| p == addr)
        .unwrap_or(false)
    }
    quickcheck::quickcheck(prop as fn(Eui64Addr) -> bool);
  }
}
