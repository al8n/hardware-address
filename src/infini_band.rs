const INFINI_BAND_ADDRESS_SIZE: usize = 20;

addr_ty!(
  /// Represents a physical 20-octet InfiniBand format address.
  InfiniBandAddr[INFINI_BAND_ADDRESS_SIZE]
);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::TestCase;

  use std::{string::ToString, vec, vec::Vec};

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
  fn formatted() {
    let addr =
      InfiniBandAddr::try_from("00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01")
        .unwrap();
    assert_eq!(
      addr.to_string(),
      "00:00:00:00:fe:80:00:00:00:00:00:00:02:00:5e:10:00:00:00:01"
    );

    let dot = addr.to_dot_seperated_array();
    let dot_str = core::str::from_utf8(&dot).unwrap();
    assert_eq!(dot_str, "0000.0000.fe80.0000.0000.0000.0200.5e10.0000.0001");

    let dashed = addr.to_hyphen_seperated_array();
    let dashed_str = core::str::from_utf8(&dashed).unwrap();
    assert_eq!(
      dashed_str,
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
    let addr3 = InfiniBandAddr::new([
      0x00, 0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x5e,
      0x10, 0x00, 0x00, 0x00, 0x01,
    ]);
    assert_eq!(addr, addr3);
    println!("{:?}", addr);
  }
}
