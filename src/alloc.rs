#[cfg(any(feature = "alloc", feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_alloc {
  (
    $name:ident[$n:expr]
  ) => {
    impl $name {
      /// Converts to colon-separated format string.
      pub fn to_colon_separated(&self) -> $crate::__private::String {
        let buf = self.to_colon_separated_array();
        // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
        unsafe { $crate::__private::ToString::to_string(::core::str::from_utf8_unchecked(&buf)) }
      }

      /// Converts to hyphen-separated format string.
      pub fn to_hyphen_separated(&self) -> $crate::__private::String {
        let buf = self.to_hyphen_separated_array();
        // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
        unsafe { $crate::__private::ToString::to_string(::core::str::from_utf8_unchecked(&buf)) }
      }

      /// Converts to dot-separated format string.
      pub fn to_dot_separated(&self) -> $crate::__private::String {
        let buf = self.to_dot_separated_array();
        // SAFETY: The buffer is always valid UTF-8 as it only contains ASCII characters.
        unsafe { $crate::__private::ToString::to_string(::core::str::from_utf8_unchecked(&buf)) }
      }
    }
  };
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_alloc {
  (
    $name:ident[$n:expr]
  ) => {};
}
