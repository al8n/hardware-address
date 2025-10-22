// The code is inspired by https://github.com/golang/go/blob/master/src/net/mac.go

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
#[allow(unused_extern_crates)]
extern crate alloc as std;

/// A macro for defining address types.
#[macro_export]
macro_rules! addr_ty {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {

    $crate::__private::paste::paste! {
      #[doc = "Represents an error that occurred while parsing `" $name "`."]
      pub type [< Parse $name Error >] = $crate::ParseError<$n>;
    }

    $(#[$attr])*
    #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::Eq, ::core::cmp::PartialEq, ::core::cmp::Ord, ::core::cmp::PartialOrd, ::core::hash::Hash)]
    #[repr(transparent)]
    pub struct $name([u8; $n]);

    #[allow(unexpected_cfgs)]
    const _: () = {
      impl $name {
        /// Creates from a byte array.
        #[cfg_attr(not(tarpaulin), inline(always))]
        pub const fn new(addr: [u8; $n]) -> Self {
          $name(addr)
        }

        /// Returns the address as a byte slice.
        #[cfg_attr(not(tarpaulin), inline(always))]
        pub const fn as_bytes(&self) -> &[u8] {
          &self.0
        }

        /// Returns the octets of the address.
        #[cfg_attr(not(tarpaulin), inline(always))]
        pub const fn octets(&self) -> [u8; $n] {
          self.0
        }

        /// Returns an array contains a colon formatted address.
        ///
        /// The returned array can be used to directly convert to `str`
        /// by using [`core::str::from_utf8(&array).unwrap( )`](core::str::from_utf8).
        #[cfg_attr(not(tarpaulin), inline(always))]
        pub const fn to_colon_seperated_array(&self) -> [u8; $n * 3 - 1] {
          let mut buf = [0u8; $n * 3 - 1];
          let mut i = 0;

          while i < $n {
            if i > 0 {
              buf[i * 3 - 1] = b':';
            }

            buf[i * 3] = $crate::__private::HEX_DIGITS[(self.0[i] >> 4) as usize];
            buf[i * 3 + 1] = $crate::__private::HEX_DIGITS[(self.0[i] & 0xF) as usize];
            i += 1;
          }

          buf
        }

        /// Returns an array contains a hyphen formatted address.
        ///
        /// The returned array can be used to directly convert to `str`
        /// by using [`core::str::from_utf8(&array).unwrap( )`](core::str::from_utf8).
        #[cfg_attr(not(tarpaulin), inline(always))]
        pub const fn to_hyphen_seperated_array(&self) -> [u8; $n * 3 - 1] {
          let mut buf = [0u8; $n * 3 - 1];
          let mut i = 0;

          while i < $n {
            if i > 0 {
              buf[i * 3 - 1] = b'-';
            }

            buf[i * 3] = $crate::__private::HEX_DIGITS[(self.0[i] >> 4) as usize];
            buf[i * 3 + 1] = $crate::__private::HEX_DIGITS[(self.0[i] & 0xF) as usize];
            i += 1;
          }

          buf
        }

        /// Returns an array contains a dot formatted address.
        ///
        /// The returned array can be used to directly convert to `str`
        /// by using [`core::str::from_utf8(&array).unwrap( )`](core::str::from_utf8).
        #[cfg_attr(not(tarpaulin), inline(always))]
        pub const fn to_dot_seperated_array(&self) -> [u8; $n * 2 + ($n / 2 - 1)] {
          let mut buf = [0u8; $n * 2 + ($n / 2 - 1)];
          let mut i = 0;

          while i < $n {
            // Convert first nibble to hex char
            buf[i * 2 + i / 2] = $crate::__private::HEX_DIGITS[(self.0[i] >> 4) as usize];
            // Convert second nibble to hex char
            buf[i * 2 + 1 + i / 2] = $crate::__private::HEX_DIGITS[(self.0[i] & 0xF) as usize];

            // Add dot every 2 bytes except for the last group
            if i % 2 == 1 && i != $n - 1 {
              buf[i * 2 + 2 + i / 2] = b'.';
            }
            i += 1;
          }

          buf
        }
      }

      impl ::core::str::FromStr for $name {
        type Err = $crate::__private::paste::paste! { [< Parse $name Error >] };

        #[cfg_attr(not(tarpaulin), inline(always))]
        fn from_str(src: &str) -> ::core::result::Result<Self, Self::Err> {
          $crate::parse::<$n>(src).map(Self)
        }
      }

      impl ::core::cmp::PartialEq<[u8]> for $name {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn eq(&self, other: &[u8]) -> bool {
          self.0.eq(other)
        }
      }

      impl ::core::cmp::PartialEq<$name> for [u8] {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn eq(&self, other: &$name) -> bool {
          other.eq(self)
        }
      }

      impl ::core::cmp::PartialEq<&[u8]> for $name {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn eq(&self, other: &&[u8]) -> bool {
          ::core::cmp::PartialEq::eq(self, *other)
        }
      }

      impl ::core::cmp::PartialEq<$name> for &[u8] {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn eq(&self, other: &$name) -> bool {
          ::core::cmp::PartialEq::eq(*self, other)
        }
      }

      impl ::core::borrow::Borrow<[u8]> for $name {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn borrow(&self) -> &[u8] {
          self
        }
      }

      impl ::core::ops::Deref for $name {
        type Target = [u8];

        #[cfg_attr(not(tarpaulin), inline(always))]
        fn deref(&self) -> &Self::Target {
          self.as_bytes()
        }
      }

      impl ::core::convert::AsRef<[u8]> for $name {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn as_ref(&self) -> &[u8] {
          ::core::borrow::Borrow::borrow(self)
        }
      }

      impl ::core::convert::From<[u8; $n]> for $name {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn from(addr: [u8; $n]) -> Self {
          $name(addr)
        }
      }

      impl ::core::convert::From<$name> for [u8; $n] {
        #[cfg_attr(not(tarpaulin), inline(always))]
        #[allow(unexpected_cfgs)]
        fn from(addr: $name) -> Self {
          addr.0
        }
      }

      impl ::core::convert::TryFrom<&str> for $name {
        type Error = $crate::__private::paste::paste! { [< Parse $name Error >] };

        #[cfg_attr(not(tarpaulin), inline(always))]
        fn try_from(src: &str) -> ::core::result::Result<Self, Self::Error> {
          <$name as ::core::str::FromStr>::from_str(src)
        }
      }

      impl ::core::fmt::Debug for $name {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          ::core::fmt::Display::fmt(self, f)
        }
      }

      impl core::fmt::Display for $name {
        #[cfg_attr(not(tarpaulin), inline(always))]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          let buf = self.to_colon_seperated_array();
          write!(
            f,
            "{}",
            core::str::from_utf8(&buf).unwrap(),
          )
        }
      }
    };

    #[cfg(feature = "serde")]
    const _: () = {
      impl $crate::__private::serde::Serialize for $name {
        fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
        where
          S: $crate::__private::serde::Serializer,
        {
          if serializer.is_human_readable() {
            let buf = self.to_colon_seperated_array();
            serializer.serialize_str(::core::str::from_utf8(&buf).unwrap())
          } else {
            <[u8; $n] as $crate::__private::serde::Serialize>::serialize(&self.0, serializer)
          }
        }
      }

      impl<'a> $crate::__private::serde::Deserialize<'a> for $name {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
        where
          D: $crate::__private::serde::Deserializer<'a>,
        {
          if deserializer.is_human_readable() {
            let s = <&str as $crate::__private::serde::Deserialize>::deserialize(deserializer)?;
            <$name as ::core::str::FromStr>::from_str(s).map_err($crate::__private::serde::de::Error::custom)
          } else {
            let bytes = <[u8; $n] as $crate::__private::serde::Deserialize>::deserialize(deserializer)?;
            Ok($name(bytes))
          }
        }
      }
    };
  }
}

mod mac;
pub use mac::*;

mod eui64;
pub use eui64::*;

mod infini_band;
pub use infini_band::*;

#[doc(hidden)]
pub mod __private {
  pub const HEX_DIGITS: &[u8] = b"0123456789abcdef";

  #[cfg(feature = "serde")]
  pub use serde;

  pub use paste;
}

/// Maximum value to prevent overflow
const BIG: i32 = 0x7fffffff;

/// Converts a hexadecimal slice to an integer.
/// Returns a tuple containing:
/// - The parsed number
/// - Number of bytes consumed
#[inline]
pub const fn xtoi(bytes: &[u8]) -> Option<(i32, usize)> {
  let mut n: i32 = 0;

  let mut idx = 0;
  let num_bytes = bytes.len();

  while idx < num_bytes {
    let c = bytes[idx];
    match c {
      b'0'..=b'9' => {
        n *= 16;
        n += (c - b'0') as i32;
      }
      b'a'..=b'f' => {
        n *= 16;
        n += (c - b'a') as i32 + 10;
      }
      b'A'..=b'F' => {
        n *= 16;
        n += (c - b'A') as i32 + 10;
      }
      _ => break,
    }

    if n == BIG {
      return None;
    }

    idx += 1;
  }

  if idx == 0 {
    return None;
  }

  Some((n, idx))
}

/// Converts the next two hex digits of s into a byte.
/// If s is longer than 2 bytes then the third byte must match e.
#[inline]
pub const fn xtoi2(s: &str, e: u8) -> Option<u8> {
  // Take first two characters and parse them
  let bytes = s.as_bytes();
  let num_bytes = bytes.len();

  // Check if string is longer than 2 chars and third char matches e
  if num_bytes > 2 && bytes[2] != e {
    return None;
  }

  let res = if num_bytes >= 2 {
    let buf = [bytes[0], bytes[1]];
    xtoi(&buf)
  } else {
    xtoi(bytes)
  };

  match res {
    Some((n, 2)) => Some(n as u8),
    _ => None,
  }
}

#[inline]
const fn dot_seperated_format_len<const N: usize>() -> usize {
  N * 2 + (N / 2 - 1)
}

#[inline]
const fn colon_seperated_format_len<const N: usize>() -> usize {
  N * 3 - 1
}

/// ParseError represents an error that occurred while parsing hex address.
#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ParseError<const N: usize> {
  /// Returned when the input string has a invalid length.
  #[error("invalid length: colon or hyphen separated format requires {ch_len} bytes, dot separated format requires {dlen} bytes, but got {0} bytes", ch_len = colon_seperated_format_len::<N>(), dlen = dot_seperated_format_len::<N>())]
  InvalidLength(usize),
  /// Returned when the input string has an invalid seperator.
  #[error("unexpected separator: expected {expected}, but got {actual}")]
  UnexpectedSeparator {
    /// The expected separator.
    expected: u8,
    /// The actual separator.
    actual: u8,
  },
  /// Returned when the input string has an unexpected separator.
  #[error("invalid separator: {0}")]
  InvalidSeparator(u8),
  /// Invalid digit.
  #[error("invalid digit: {0:?}")]
  InvalidHexDigit([u8; 2]),
}

impl<const N: usize> ParseError<N> {
  /// Returns the length of the address.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn invalid_length(len: usize) -> Self {
    Self::InvalidLength(len)
  }

  /// Returns an error for an unexpected separator.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn unexpected_separator(expected: u8, actual: u8) -> Self {
    Self::UnexpectedSeparator { expected, actual }
  }

  /// Returns an error for an invalid separator.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn invalid_separator(sep: u8) -> Self {
    Self::InvalidSeparator(sep)
  }

  /// Returns an error for an invalid hex digit.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn invalid_hex_digit(digit: [u8; 2]) -> Self {
    Self::InvalidHexDigit(digit)
  }
}

/// Parses s as an IEEE 802 MAC-48, EUI-48, EUI-64, or a 20-octet
/// IP over InfiniBand link-layer address and etc using one of the following formats:
///
/// - Colon-separated:
///   - `00:00:5e:00:53:01`
///   - `02:00:5e:10:00:00:00:01`
///
/// - Hyphen-separated:
///   - `00-00-5e-00-53-01`
///   - `02-00-5e-10-00-00-00-01`
///
/// - Dot-separated:
///   - `0000.5e00.5301`
///   - `0200.5e10.0000.0001`
pub fn parse<const N: usize>(src: &str) -> Result<[u8; N], ParseError<N>> {
  let dot_seperated_len = dot_seperated_format_len::<N>();
  let colon_seperated_len = colon_seperated_format_len::<N>();
  let len = src.len();

  let bytes = src.as_bytes();
  match () {
    () if len == dot_seperated_len => {
      let mut hw = [0; N];
      let mut x = 0;

      for i in (0..N).step_by(2) {
        if x + 4 != len && bytes[x + 4] != b'.' {
          return Err(ParseError::unexpected_separator(b'.', bytes[x + 4]));
        }

        match xtoi2(&src[x..x + 2], 0) {
          Some(byte) => hw[i] = byte,
          None => return Err(ParseError::invalid_hex_digit([bytes[x], bytes[x + 1]])),
        }
        match xtoi2(&src[x + 2..], b'.') {
          Some(byte) => hw[i + 1] = byte,
          None => return Err(ParseError::invalid_hex_digit([bytes[x + 2], bytes[x + 3]])),
        }

        x += 5;
      }

      Ok(hw)
    }
    () if len == colon_seperated_len => {
      let mut hw = [0; N];
      let mut x = 0;

      let sep = bytes[2];
      if !(sep == b':' || sep == b'-') {
        return Err(ParseError::invalid_separator(sep));
      }

      #[allow(clippy::needless_range_loop)]
      for i in 0..N {
        if x + 2 != len {
          let csep = bytes[x + 2];
          if csep != sep {
            return Err(ParseError::unexpected_separator(sep, csep));
          }
        }

        match xtoi2(&src[x..], sep) {
          Some(byte) => hw[i] = byte,
          None => return Err(ParseError::invalid_hex_digit([bytes[x], bytes[x + 1]])),
        }
        x += 3;
      }

      Ok(hw)
    }
    _ => Err(ParseError::invalid_length(len)),
  }
}

#[cfg(test)]
struct TestCase<const N: usize> {
  input: &'static str,
  output: Option<std::vec::Vec<u8>>,
  err: Option<ParseError<N>>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_xtoi() {
    assert_eq!(xtoi(b""), None);
    assert_eq!(xtoi(b"0"), Some((0, 1)));
    assert_eq!(xtoi(b"12"), Some((0x12, 2)));
    assert_eq!(xtoi(b"1a"), Some((0x1a, 2)));
    assert_eq!(xtoi(b"1A"), Some((0x1a, 2)));
    assert_eq!(xtoi(b"12x"), Some((0x12, 2)));
    assert_eq!(xtoi(b"x12"), None);
  }

  #[test]
  fn test_xtoi2() {
    assert_eq!(xtoi2("12", b'\0'), Some(0x12));
    assert_eq!(xtoi2("12x", b'x'), Some(0x12));
    assert_eq!(xtoi2("12y", b'x'), None);
    assert_eq!(xtoi2("1", b'\0'), None);
    assert_eq!(xtoi2("xy", b'\0'), None);
  }
}
