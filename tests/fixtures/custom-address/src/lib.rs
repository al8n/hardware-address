#![cfg_attr(not(feature = "std"), no_std)]
#![deny(warnings)]

#[cfg(test)]
extern crate std;

hardware_address::addr_ty!(MyAddr[12]);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn custom_address_uses_the_public_macro() {
    let address = MyAddr::from_raw([
      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
    ]);
    assert_eq!(
      address.octets(),
      [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
      ]
    );

    let parsed = MyAddr::try_from("00:01:02:03:04:05:06:07:08:09:0a:0b").unwrap();
    assert_eq!(parsed, address);
    assert_eq!(
      std::format!("{}", parsed),
      "00:01:02:03:04:05:06:07:08:09:0a:0b"
    );

    let colon = parsed.to_colon_separated_array();
    assert_eq!(
      core::str::from_utf8(&colon).unwrap(),
      "00:01:02:03:04:05:06:07:08:09:0a:0b"
    );

    let err: ParseMyAddrError = "not an address".parse::<MyAddr>().unwrap_err();
    assert!(matches!(err, hardware_address::ParseError::InvalidLength(_)));
  }

  #[cfg(feature = "serde")]
  #[test]
  fn custom_address_implements_serde() {
    fn assert_impl<T: serde::Serialize + for<'de> serde::Deserialize<'de>>() {}
    assert_impl::<MyAddr>();
  }

  #[cfg(feature = "arbitrary")]
  #[test]
  fn custom_address_implements_arbitrary() {
    fn assert_impl<T: arbitrary::Arbitrary<'static>>() {}
    assert_impl::<MyAddr>();
  }

  #[cfg(feature = "quickcheck")]
  #[test]
  fn custom_address_implements_quickcheck() {
    fn assert_impl<T: quickcheck::Arbitrary>() {}
    assert_impl::<MyAddr>();
  }
}
