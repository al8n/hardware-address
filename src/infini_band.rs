addr_ty!(
  /// Represents a physical 20-octet InfiniBand format address.
  InfiniBandAddr[20]
);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::TestCase;

  use std::{string::ToString, vec, vec::Vec};

  const INFINI_BAND_ADDRESS_SIZE: usize = 20;

  fn test_cases() -> Vec<TestCase<INFINI_BAND_ADDRESS_SIZE>> {
    vec![
      // RFC 4391, Section 9.1.1
      TestCase {
        input: "00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01",
        output: Some(vec![
          0x00, 0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x5e,
          0x10, 0x00, 0x00, 0x00, 0x01,
        ]),
        err: None,
      },
      TestCase {
        input: "00-00-00-00-fe-80-00-00-00-00-00-00-02-00-5e-10-00-00-00-01",
        output: Some(vec![
          0x00, 0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x5e,
          0x10, 0x00, 0x00, 0x00, 0x01,
        ]),
        err: None,
      },
      TestCase {
        input: "0000.0000.fe80.0000.0000.0000.0200.5e10.0000.0001",
        output: Some(vec![
          0x00, 0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x5e,
          0x10, 0x00, 0x00, 0x00, 0x01,
        ]),
        err: None,
      },
    ]
  }

  #[test]
  fn parse() {
    let cases = test_cases();
    for (i, test) in cases.iter().enumerate() {
      let result = InfiniBandAddr::try_from(test.input);

      match (result, &test.output) {
        (Ok(out), Some(expected)) => {
          assert_eq!(
            out.as_ref(),
            expected.as_slice(),
            "Test case {}: InfiniBandAddr::parse({}) output mismatch",
            i,
            test.input
          );

          // Test round-trip if this was a valid case
          if test.err.is_none() {
            let formatted = out.to_string();
            let round_trip = InfiniBandAddr::try_from(formatted.as_str());
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
    let addr = InfiniBandAddr::default();
    assert_eq!(addr.octets(), [0; INFINI_BAND_ADDRESS_SIZE]);
  }

  #[test]
  fn formatted() {
    let addr =
      InfiniBandAddr::try_from("00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01")
        .unwrap();
    assert_eq!(
      addr.to_string(),
      "00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01"
    );
    assert_eq!(
      addr.to_colon_separated(),
      "00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01"
    );

    let dot = addr.to_dot_separated_array();
    let dot_str = core::str::from_utf8(&dot).unwrap();
    assert_eq!(dot_str, "0000.0000.fe80.0000.0000.0000.0200.5e10.0000.0001");
    assert_eq!(
      addr.to_dot_separated(),
      "0000.0000.fe80.0000.0000.0000.0200.5e10.0000.0001"
    );

    let dashed = addr.to_hyphen_separated_array();
    let dashed_str = core::str::from_utf8(&dashed).unwrap();
    assert_eq!(
      dashed_str,
      "00-00-00-00-fe-80-00-00-00-00-00-00-02-00-5e-10-00-00-00-01"
    );
    assert_eq!(
      addr.to_hyphen_separated(),
      "00-00-00-00-fe-80-00-00-00-00-00-00-02-00-5e-10-00-00-00-01"
    );
  }

  #[cfg(feature = "serde")]
  #[test]
  fn serde_human_readable() {
    let addr =
      InfiniBandAddr::try_from("00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01")
        .unwrap();
    let json = serde_json::to_string(&addr).unwrap();
    assert_eq!(
      json,
      "\"00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01\""
    );

    let addr2: InfiniBandAddr = serde_json::from_str(&json).unwrap();
    assert_eq!(addr, addr2);
  }

  #[cfg(feature = "serde")]
  #[test]
  fn serde_human_unreadable() {
    let addr =
      InfiniBandAddr::try_from("00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01")
        .unwrap();
    let encoded = bincode::serde::encode_to_vec(addr, bincode::config::standard()).unwrap();
    assert_eq!(
      encoded,
      [
        0x00, 0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x5e,
        0x10, 0x00, 0x00, 0x00, 0x01,
      ]
    );
    assert_eq!(addr.octets(), encoded.as_slice());

    let addr2: InfiniBandAddr =
      bincode::serde::decode_from_slice(&encoded, bincode::config::standard())
        .unwrap()
        .0;
    assert_eq!(addr, addr2);
    let addr3 = InfiniBandAddr::from_raw([
      0x00, 0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x5e,
      0x10, 0x00, 0x00, 0x00, 0x01,
    ]);
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

    let data = [0xAA; 64];
    let a =
      InfiniBandAddr::arbitrary(&mut Unstructured::new(&data)).expect("arbitrary should succeed");
    let b =
      InfiniBandAddr::arbitrary(&mut Unstructured::new(&data)).expect("arbitrary should succeed");
    assert_eq!(a, b, "arbitrary should be deterministic for a fixed input");
  }

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_size_hint_matches_byte_array() {
    use arbitrary::Arbitrary;

    let hint = InfiniBandAddr::size_hint(0);
    let expected = <[u8; INFINI_BAND_ADDRESS_SIZE] as Arbitrary>::size_hint(0);
    assert_eq!(hint, expected);
  }

  #[cfg(feature = "arbitrary")]
  #[test]
  fn arbitrary_consumes_expected_bytes() {
    use arbitrary::{Arbitrary, Unstructured};

    // Two 20-byte draws fit in a 40-byte buffer.
    let data = [0xC3u8; 40];
    let mut u = Unstructured::new(&data);
    let _first = InfiniBandAddr::arbitrary(&mut u).unwrap();
    let _second = InfiniBandAddr::arbitrary(&mut u).unwrap();
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_arbitrary_roundtrips_through_string() {
    use quickcheck::{Arbitrary, Gen};

    let mut g = Gen::new(32);
    for _ in 0..128 {
      let addr = InfiniBandAddr::arbitrary(&mut g);
      let parsed = InfiniBandAddr::try_from(addr.to_string().as_str())
        .expect("to_string() output must parse back");
      assert_eq!(addr, parsed);
    }
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_shrink_terminates_and_preserves_length() {
    use quickcheck::Arbitrary;

    let addr = InfiniBandAddr::from_raw([0xFF; INFINI_BAND_ADDRESS_SIZE]);
    // 20 bytes × per-byte shrink chain can produce many candidates;
    // the `take` keeps this test fast without affecting what we check.
    let shrinks: Vec<_> = addr.shrink().take(4096).collect();
    assert!(!shrinks.is_empty(), "non-zero address should yield shrinks");
    for s in &shrinks {
      assert_eq!(s.octets().len(), INFINI_BAND_ADDRESS_SIZE);
    }
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn quickcheck_shrink_zero_is_empty() {
    use quickcheck::Arbitrary;

    let zero = InfiniBandAddr::from_raw([0; INFINI_BAND_ADDRESS_SIZE]);
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
    fn prop(addr: InfiniBandAddr) -> bool {
      InfiniBandAddr::try_from(addr.to_string().as_str())
        .map(|p| p == addr)
        .unwrap_or(false)
    }
    quickcheck::quickcheck(prop as fn(InfiniBandAddr) -> bool);
  }
}
