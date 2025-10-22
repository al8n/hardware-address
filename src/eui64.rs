const EUI64_ADDRESS_SIZE: usize = 8;

addr_ty!(
  /// Represents a physical EUI-64 format address.
  Eui64Addr[EUI64_ADDRESS_SIZE]
);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{ParseError, TestCase};

  use std::{string::ToString, vec, vec::Vec};

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
  fn formatted() {
    let addr = Eui64Addr::try_from("02:00:5e:10:00:00:00:01").unwrap();
    assert_eq!(addr.to_string(), "02:00:5e:10:00:00:00:01");

    let dot = addr.to_dot_seperated_array();
    let dot_str = core::str::from_utf8(&dot).unwrap();
    assert_eq!(dot_str, "0200.5e10.0000.0001");

    let dashed = addr.to_hyphen_seperated_array();
    let dashed_str = core::str::from_utf8(&dashed).unwrap();
    assert_eq!(dashed_str, "02-00-5e-10-00-00-00-01");
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
}
