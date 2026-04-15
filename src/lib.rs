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
    paste::paste! {
      pub use [< __ $name:snake __ >]::{$name, [< Parse $name Error >]};

      #[doc(hidden)]
      #[allow(unused)]
      mod [< __ $name:snake __ >] {
        #[cfg(feature = "pyo3")]
        use $crate::__private::pyo3 as __pyo3;

        #[cfg(feature = "wasm-bindgen")]
        use $crate::__private::wasm_bindgen as __wasm_bindgen;

        #[doc = "Represents an error that occurred while parsing `" $name "`."]
        pub type [< Parse $name Error >] = $crate::ParseError<$n>;

        $(#[$attr])*
        #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::Eq, ::core::cmp::PartialEq, ::core::cmp::Ord, ::core::cmp::PartialOrd, ::core::hash::Hash)]
        // `from_py_object` explicitly opts in to the `FromPyObject`
        // derive that pyo3 used to generate automatically for `Clone`
        // types. Address types are tiny (`[u8; N]` with `N` in {6, 8, 20}),
        // so the Clone-based conversion is effectively free and lets them
        // be passed as arguments to `#[pyfunction]` / `#[pymethods]`.
        #[cfg_attr(feature = "pyo3", $crate::__private::pyo3::pyclass(crate = "__pyo3", from_py_object))]
        #[cfg_attr(feature = "wasm-bindgen", $crate::__private::wasm_bindgen::prelude::wasm_bindgen(wasm_bindgen = __wasm_bindgen))]
        #[repr(transparent)]
        pub struct $name(pub(crate) [::core::primitive::u8; $n]);
      }
    }

    #[allow(unexpected_cfgs)]
    const _: () = {
      impl ::core::default::Default for $name {
        #[inline]
        fn default() -> Self {
          $name::new()
        }
      }

      impl $name {
        /// The size of the address in bytes.
        pub const SIZE: ::core::primitive::usize = $n;

        /// Creates a zeroed address.
        #[inline]
        pub const fn new() -> Self {
          $name([0; $n])
        }

        /// Creates from raw byte array address.
        #[inline]
        pub const fn from_raw(addr: [::core::primitive::u8; $n]) -> Self {
          $name(addr)
        }

        /// Returns the address as a byte slice.
        #[inline]
        pub const fn as_bytes(&self) -> &[::core::primitive::u8] {
          &self.0
        }

        /// Returns the octets of the address.
        #[inline]
        pub const fn octets(&self) -> [::core::primitive::u8; $n] {
          self.0
        }

        /// Returns an array contains a colon formatted address.
        ///
        /// The returned array can be used to directly convert to `str`
        /// by using [`core::str::from_utf8(&array).unwrap( )`](core::str::from_utf8).
        #[inline]
        pub const fn to_colon_separated_array(&self) -> [::core::primitive::u8; $n * 3 - 1] {
          let mut buf = [0u8; $n * 3 - 1];
          let mut i = 0;

          while i < $n {
            if i > 0 {
              buf[i * 3 - 1] = b':';
            }

            buf[i * 3] = $crate::__private::HEX_DIGITS[(self.0[i] >> 4) as ::core::primitive::usize];
            buf[i * 3 + 1] = $crate::__private::HEX_DIGITS[(self.0[i] & 0xF) as ::core::primitive::usize];
            i += 1;
          }

          buf
        }

        /// Returns an array contains a hyphen formatted address.
        ///
        /// The returned array can be used to directly convert to `str`
        /// by using [`core::str::from_utf8(&array).unwrap( )`](core::str::from_utf8).
        #[inline]
        pub const fn to_hyphen_separated_array(&self) -> [::core::primitive::u8; $n * 3 - 1] {
          let mut buf = [0u8; $n * 3 - 1];
          let mut i = 0;

          while i < $n {
            if i > 0 {
              buf[i * 3 - 1] = b'-';
            }

            buf[i * 3] = $crate::__private::HEX_DIGITS[(self.0[i] >> 4) as ::core::primitive::usize];
            buf[i * 3 + 1] = $crate::__private::HEX_DIGITS[(self.0[i] & 0xF) as ::core::primitive::usize];
            i += 1;
          }

          buf
        }

        /// Returns an array contains a dot formatted address.
        ///
        /// The returned array can be used to directly convert to `str`
        /// by using [`core::str::from_utf8(&array).unwrap( )`](core::str::from_utf8).
        #[inline]
        pub const fn to_dot_separated_array(&self) -> [::core::primitive::u8; $n * 2 + ($n / 2 - 1)] {
          let mut buf = [0u8; $n * 2 + ($n / 2 - 1)];
          let mut i = 0;

          while i < $n {
            // Convert first nibble to hex char
            buf[i * 2 + i / 2] = $crate::__private::HEX_DIGITS[(self.0[i] >> 4) as ::core::primitive::usize];
            // Convert second nibble to hex char
            buf[i * 2 + 1 + i / 2] = $crate::__private::HEX_DIGITS[(self.0[i] & 0xF) as ::core::primitive::usize];

            // Add dot every 2 bytes except for the last group
            if i % 2 == 1 && i != $n - 1 {
              buf[i * 2 + 2 + i / 2] = b'.';
            }
            i += 1;
          }

          buf
        }

        /// Converts to colon-separated format string.
        #[cfg(any(feature = "alloc", feature = "std"))]
        #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
        pub fn to_colon_separated(&self) -> $crate::__private::String {
          let buf = self.to_colon_separated_array();
          // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
          unsafe { $crate::__private::ToString::to_string(::core::str::from_utf8_unchecked(&buf)) }
        }

        /// Converts to hyphen-separated format string.
        #[cfg(any(feature = "alloc", feature = "std"))]
        #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
        pub fn to_hyphen_separated(&self) -> $crate::__private::String {
          let buf = self.to_hyphen_separated_array();
          // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
          unsafe { $crate::__private::ToString::to_string(::core::str::from_utf8_unchecked(&buf)) }
        }

        /// Converts to dot-separated format string.
        #[cfg(any(feature = "alloc", feature = "std"))]
        #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
        pub fn to_dot_separated(&self) -> $crate::__private::String {
          let buf = self.to_dot_separated_array();
          // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
          unsafe { $crate::__private::ToString::to_string(::core::str::from_utf8_unchecked(&buf)) }
        }
      }

      impl ::core::str::FromStr for $name {
        type Err = $crate::__private::paste::paste! { [< Parse $name Error >] };

        #[inline]
        fn from_str(src: &str) -> ::core::result::Result<Self, Self::Err> {
          $crate::parse::<$n>(src.as_bytes()).map(Self)
        }
      }

      impl ::core::cmp::PartialEq<[::core::primitive::u8]> for $name {
        #[inline]
        fn eq(&self, other: &[::core::primitive::u8]) -> bool {
          self.0.eq(other)
        }
      }

      impl ::core::cmp::PartialEq<$name> for [::core::primitive::u8] {
        #[inline]
        fn eq(&self, other: &$name) -> bool {
          other.eq(self)
        }
      }

      impl ::core::cmp::PartialEq<&[::core::primitive::u8]> for $name {
        #[inline]
        fn eq(&self, other: &&[::core::primitive::u8]) -> bool {
          ::core::cmp::PartialEq::eq(self, *other)
        }
      }

      impl ::core::cmp::PartialEq<$name> for &[::core::primitive::u8] {
        #[inline]
        fn eq(&self, other: &$name) -> bool {
          ::core::cmp::PartialEq::eq(*self, other)
        }
      }

      impl ::core::borrow::Borrow<[::core::primitive::u8]> for $name {
        #[inline]
        fn borrow(&self) -> &[::core::primitive::u8] {
          self
        }
      }

      impl ::core::ops::Deref for $name {
        type Target = [::core::primitive::u8];

        #[inline]
        fn deref(&self) -> &Self::Target {
          self.as_bytes()
        }
      }

      impl ::core::convert::AsRef<[::core::primitive::u8]> for $name {
        #[inline]
        fn as_ref(&self) -> &[::core::primitive::u8] {
          ::core::borrow::Borrow::borrow(self)
        }
      }

      impl ::core::convert::From<[::core::primitive::u8; $n]> for $name {
        #[inline]
        fn from(addr: [::core::primitive::u8; $n]) -> Self {
          $name(addr)
        }
      }

      impl ::core::convert::From<$name> for [::core::primitive::u8; $n] {
        #[inline]
        #[allow(unexpected_cfgs)]
        fn from(addr: $name) -> Self {
          addr.0
        }
      }

      impl ::core::convert::TryFrom<&str> for $name {
        type Error = $crate::__private::paste::paste! { [< Parse $name Error >] };

        #[inline]
        fn try_from(src: &str) -> ::core::result::Result<Self, Self::Error> {
          <$name as ::core::str::FromStr>::from_str(src)
        }
      }

      impl ::core::fmt::Debug for $name {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          ::core::fmt::Display::fmt(self, f)
        }
      }

      impl core::fmt::Display for $name {
        #[inline]
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          let buf = self.to_colon_separated_array();
          write!(
            f,
            "{}",
            // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
            unsafe { ::core::str::from_utf8_unchecked(&buf) },
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
            let buf = self.to_colon_separated_array();
            // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
            serializer.serialize_str(unsafe { ::core::str::from_utf8_unchecked(&buf) })
          } else {
            <[::core::primitive::u8; $n] as $crate::__private::serde::Serialize>::serialize(&self.0, serializer)
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
            let bytes = <[::core::primitive::u8; $n] as $crate::__private::serde::Deserialize>::deserialize(deserializer)?;
            ::core::result::Result::Ok($name(bytes))
          }
        }
      }
    };

    #[cfg(feature = "arbitrary")]
    $crate::__addr_ty_arbitrary! { $name[$n] }

    #[cfg(feature = "quickcheck")]
    $crate::__addr_ty_quickcheck! { $name[$n] }

    #[cfg(feature = "pyo3")]
    $crate::__addr_ty_pyo3! { $name[$n] }

    #[cfg(feature = "wasm-bindgen")]
    $crate::__addr_ty_wasm_bindgen! { $name[$n] }
  }
}

mod mac;
pub use mac::*;

mod eui64;
pub use eui64::*;

mod infini_band;
pub use infini_band::*;

#[cfg(feature = "pyo3")]
mod py;
#[cfg(feature = "wasm-bindgen")]
mod wasm;

#[cfg(feature = "arbitrary")]
mod arbitrary;

#[cfg(feature = "quickcheck")]
mod quickcheck;

#[doc(hidden)]
pub mod __private {
  /// Lowercase ASCII hex digits for formatting.
  pub const HEX_DIGITS: [::core::primitive::u8; 16] = *b"0123456789abcdef";

  /// Lookup table: ASCII byte → nibble value (`0..=15`), or `0xFF` for
  /// anything that isn't a valid hex digit. Branch-free alternative to
  /// chained `match` arms.
  pub const HEX_VAL: [::core::primitive::u8; 256] = {
    let mut t = [0xFFu8; 256];
    let mut i = 0;
    while i < 10 {
      t[b'0' as usize + i] = i as u8;
      i += 1;
    }
    let mut i = 0;
    while i < 6 {
      t[b'a' as usize + i] = (i + 10) as u8;
      t[b'A' as usize + i] = (i + 10) as u8;
      i += 1;
    }
    t
  };

  /// Parse two ASCII hex characters into a single byte.
  ///
  /// Returns `None` if either character isn't a valid hex digit. Used
  /// by [`crate::parse`] and (internally) by [`crate::xtoi2`].
  #[inline]
  pub const fn hex_byte(
    hi: ::core::primitive::u8,
    lo: ::core::primitive::u8,
  ) -> ::core::option::Option<::core::primitive::u8> {
    let hi_val = HEX_VAL[hi as usize];
    if hi_val == 0xFF {
      return ::core::option::Option::None;
    }
    let lo_val = HEX_VAL[lo as usize];
    if lo_val == 0xFF {
      return ::core::option::Option::None;
    }
    ::core::option::Option::Some((hi_val << 4) | lo_val)
  }

  #[cfg(feature = "serde")]
  pub use serde;

  #[cfg(feature = "arbitrary")]
  pub use arbitrary;

  #[cfg(feature = "quickcheck")]
  pub use quickcheck;

  #[cfg(feature = "pyo3")]
  pub use pyo3;

  #[cfg(all(feature = "pyo3", feature = "std"))]
  pub use std::hash::DefaultHasher;
  #[cfg(all(feature = "pyo3", not(feature = "std")))]
  pub type DefaultHasher = ::core::hash::BuildHasherDefault<::core::hash::SipHasher>;

  #[cfg(feature = "wasm-bindgen")]
  pub use wasm_bindgen;

  #[cfg(any(feature = "alloc", feature = "std"))]
  pub use std::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
  };

  pub use paste;
}

/// Converts a hexadecimal slice to an integer.
///
/// Reads as many leading ASCII hex digits from `bytes` as fit in an
/// `i32` and stops at the first non-hex byte.
///
/// Returns a tuple of `(parsed, consumed)` — the value and the number
/// of bytes consumed — or `None` if:
///
/// - `bytes` is empty,
/// - the first byte isn't a valid hex digit, or
/// - the accumulated value would overflow `i32` (i.e. exceed
///   `i32::MAX`).
#[inline]
pub const fn xtoi(bytes: &[::core::primitive::u8]) -> Option<(i32, ::core::primitive::usize)> {
  // Use `u32` internally so `n * 16 + digit` never panics on overflow
  // in debug builds. We still cap at `i32::MAX` to preserve the
  // public signature's non-negative `i32` contract.
  let mut n: u32 = 0;
  let mut idx = 0;
  let num_bytes = bytes.len();

  while idx < num_bytes {
    let digit = __private::HEX_VAL[bytes[idx] as usize];
    if digit == 0xFF {
      break;
    }

    n = match n.checked_mul(16) {
      Some(v) => v,
      None => return None,
    };
    n = match n.checked_add(digit as u32) {
      Some(v) => v,
      None => return None,
    };
    if n > i32::MAX as u32 {
      return None;
    }

    idx += 1;
  }

  if idx == 0 {
    return None;
  }

  Some((n as i32, idx))
}

/// Converts the next two hex digits of `s` into a byte.
///
/// If `s` is longer than 2 bytes then the third byte must match `e`.
/// Returns `None` if either of the first two bytes isn't a valid hex
/// digit, or if `s.len() > 2` and `s[2] != e`.
#[inline]
pub const fn xtoi2(s: &[u8], e: u8) -> Option<::core::primitive::u8> {
  if s.len() < 2 {
    return None;
  }
  if s.len() > 2 && s[2] != e {
    return None;
  }
  __private::hex_byte(s[0], s[1])
}

#[inline]
const fn dot_separated_format_len<const N: ::core::primitive::usize>() -> ::core::primitive::usize {
  N * 2 + (N / 2 - 1)
}

#[inline]
const fn colon_separated_format_len<const N: ::core::primitive::usize>() -> ::core::primitive::usize
{
  N * 3 - 1
}

/// ParseError represents an error that occurred while parsing hex address.
#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ParseError<const N: ::core::primitive::usize> {
  /// Returned when the input string has a invalid length.
  #[error("invalid length: colon or hyphen separated format requires {ch_len} bytes, dot separated format requires {dlen} bytes, but got {0} bytes", ch_len = colon_separated_format_len::<N>(), dlen = dot_separated_format_len::<N>())]
  InvalidLength(::core::primitive::usize),
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
  InvalidHexDigit([::core::primitive::u8; 2]),
}

impl<const N: ::core::primitive::usize> ParseError<N> {
  /// Returns the length of the address.
  #[inline]
  pub const fn invalid_length(len: ::core::primitive::usize) -> Self {
    Self::InvalidLength(len)
  }

  /// Returns an error for an unexpected separator.
  #[inline]
  pub const fn unexpected_separator(expected: u8, actual: u8) -> Self {
    Self::UnexpectedSeparator { expected, actual }
  }

  /// Returns an error for an invalid separator.
  #[inline]
  pub const fn invalid_separator(sep: u8) -> Self {
    Self::InvalidSeparator(sep)
  }

  /// Returns an error for an invalid hex digit.
  #[inline]
  pub const fn invalid_hex_digit(digit: [::core::primitive::u8; 2]) -> Self {
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
pub const fn parse<const N: ::core::primitive::usize>(
  src: &[u8],
) -> Result<[::core::primitive::u8; N], ParseError<N>> {
  let dot_separated_len = dot_separated_format_len::<N>();
  let colon_separated_len = colon_separated_format_len::<N>();
  let len = src.len();

  if len == dot_separated_len {
    let mut hw = [0u8; N];
    let mut x = 0usize;
    let mut i = 0usize;

    while i < N {
      // Validate the `.` separator between each 4-hex-digit group,
      // except when we're at the end of the input.
      if x + 4 < len && src[x + 4] != b'.' {
        return Err(ParseError::unexpected_separator(b'.', src[x + 4]));
      }

      match __private::hex_byte(src[x], src[x + 1]) {
        Some(byte) => hw[i] = byte,
        None => return Err(ParseError::invalid_hex_digit([src[x], src[x + 1]])),
      }
      match __private::hex_byte(src[x + 2], src[x + 3]) {
        Some(byte) => hw[i + 1] = byte,
        None => return Err(ParseError::invalid_hex_digit([src[x + 2], src[x + 3]])),
      }

      x += 5;
      i += 2;
    }

    return Ok(hw);
  }

  if len == colon_separated_len {
    let sep = src[2];
    if sep != b':' && sep != b'-' {
      return Err(ParseError::invalid_separator(sep));
    }

    let mut hw = [0u8; N];
    let mut x = 0usize;
    let mut i = 0usize;

    while i < N {
      if x + 2 < len {
        let csep = src[x + 2];
        if csep != sep {
          return Err(ParseError::unexpected_separator(sep, csep));
        }
      }

      match __private::hex_byte(src[x], src[x + 1]) {
        Some(byte) => hw[i] = byte,
        None => return Err(ParseError::invalid_hex_digit([src[x], src[x + 1]])),
      }

      x += 3;
      i += 1;
    }

    return Ok(hw);
  }

  Err(ParseError::invalid_length(len))
}

#[cfg(test)]
struct TestCase<const N: ::core::primitive::usize> {
  input: &'static str,
  output: Option<std::vec::Vec<::core::primitive::u8>>,
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
    assert_eq!(xtoi2(b"12", b'\0'), Some(0x12));
    assert_eq!(xtoi2(b"12x", b'x'), Some(0x12));
    assert_eq!(xtoi2(b"12y", b'x'), None);
    assert_eq!(xtoi2(b"1", b'\0'), None);
    assert_eq!(xtoi2(b"xy", b'\0'), None);
  }

  /// Regression test for the pre-fix overflow bug: `xtoi("FFFFFFFF")`
  /// used to silently return `Some((-1, 8))` in release builds and
  /// panic in debug builds due to the broken `if n == BIG` equality
  /// check being placed *after* multiplication. The fix uses checked
  /// arithmetic and caps at `i32::MAX`.
  #[test]
  fn test_xtoi_overflow_is_detected() {
    // 8 'F's = 0xFFFF_FFFF, exceeds i32::MAX.
    assert_eq!(xtoi(b"FFFFFFFF"), None);
    // 7 'F's = 0x0FFF_FFFF, fits in i32.
    assert_eq!(xtoi(b"FFFFFFF"), Some((0x0FFF_FFFF, 7)));
    // 8 '7's + digits just below i32::MAX also fine.
    assert_eq!(xtoi(b"7FFFFFFF"), Some((0x7FFF_FFFF, 8)));
    // One above i32::MAX must reject.
    assert_eq!(xtoi(b"80000000"), None);
    // Long all-hex string rejects cleanly instead of wrapping/panicking.
    assert_eq!(xtoi(b"0123456789ABCDEF"), None);
  }

  /// `parse` is `const fn`, so we can build `MacAddr`-style constants
  /// at compile time. This test exercises that directly.
  #[test]
  fn test_parse_is_const() {
    const MAC: [u8; 6] = match parse::<6>(b"00:11:22:33:44:55") {
      Ok(v) => v,
      Err(_) => panic!(),
    };
    assert_eq!(MAC, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);

    // Also covers the hyphen-separated form.
    const MAC2: [u8; 6] = match parse::<6>(b"aa-bb-cc-dd-ee-ff") {
      Ok(v) => v,
      Err(_) => panic!(),
    };
    assert_eq!(MAC2, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);

    // And the dot-separated form.
    const MAC3: [u8; 6] = match parse::<6>(b"0000.5e00.5301") {
      Ok(v) => v,
      Err(_) => panic!(),
    };
    assert_eq!(MAC3, [0x00, 0x00, 0x5E, 0x00, 0x53, 0x01]);
  }

  /// Fast-path `hex_byte` sanity: all valid digits, plus a few
  /// invalids at boundary values.
  #[test]
  fn test_hex_byte() {
    use crate::__private::hex_byte;

    assert_eq!(hex_byte(b'0', b'0'), Some(0x00));
    assert_eq!(hex_byte(b'f', b'f'), Some(0xFF));
    assert_eq!(hex_byte(b'F', b'F'), Some(0xFF));
    assert_eq!(hex_byte(b'1', b'a'), Some(0x1A));
    assert_eq!(hex_byte(b'1', b'A'), Some(0x1A));

    // Invalid first nibble.
    assert_eq!(hex_byte(b'g', b'0'), None);
    assert_eq!(hex_byte(b'/', b'0'), None); // '/' = '0' - 1
    assert_eq!(hex_byte(b':', b'0'), None); // ':' = '9' + 1
                                            // Invalid second nibble.
    assert_eq!(hex_byte(b'0', b'g'), None);
    // Both invalid.
    assert_eq!(hex_byte(0, 0), None);
  }
}
