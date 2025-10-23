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
}
