#[cfg(feature = "wasm-bindgen")]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_wasm_bindgen {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {
    const _: () = {
      use $crate::__private::wasm_bindgen::prelude::*;
      use $crate::__private::ToString as __ToString;

      #[wasm_bindgen(wasm_bindgen = $crate::__private::wasm_bindgen)]
      impl $name {
        /// Creates a zeroed address.
        #[wasm_bindgen(constructor)]
        #[doc(hidden)]
        pub fn __new_js() -> Self {
          $name::new()
        }

        /// Create an address from bytes.
        #[wasm_bindgen(js_name = "fromBytes")]
        #[doc(hidden)]
        pub fn __from_bytes_js(
          bytes: &[::core::primitive::u8],
        ) -> ::core::result::Result<$name, $crate::__private::wasm_bindgen::JsError> {
          if bytes.len() != $n {
            return ::core::result::Result::Err($crate::__private::wasm_bindgen::JsError::new(
              &__ToString::to_string(&::core::format_args!(
                "Expected {} bytes, got {}",
                $n,
                bytes.len()
              )),
            ));
          }
          let mut arr = [0u8; $n];
          arr.copy_from_slice(bytes);
          ::core::result::Result::Ok($name(arr))
        }

        /// Parses an address from a string.
        #[wasm_bindgen(js_name = "parse")]
        #[doc(hidden)]
        pub fn __parse_js(
          s: &::core::primitive::str,
        ) -> ::core::result::Result<$name, $crate::__private::wasm_bindgen::JsError> {
          <$name as ::core::str::FromStr>::from_str(s).map_err(|e| {
            $crate::__private::wasm_bindgen::JsError::new(&__ToString::to_string(
              &::core::format_args!("{}", e),
            ))
          })
        }

        /// Converts to string representation.
        #[wasm_bindgen(js_name = "toString")]
        #[doc(hidden)]
        pub fn __to_string_js(&self) -> $crate::__private::String {
          __ToString::to_string(&::core::format_args!("{}", self))
        }

        /// Returns the address as bytes.
        #[wasm_bindgen(js_name = "toBytes")]
        #[doc(hidden)]
        pub fn __to_bytes_js(&self) -> $crate::__private::Vec<::core::primitive::u8> {
          self.0.to_vec()
        }

        /// Converts to colon-separated format address.
        #[wasm_bindgen(js_name = "toColonSeparated")]
        #[doc(hidden)]
        pub fn __to_colon_separated_js(&self) -> $crate::__private::String {
          self.to_colon_separated()
        }

        /// Converts to hyphen-separated format address.
        #[wasm_bindgen(js_name = "toHyphenSeparated")]
        #[doc(hidden)]
        pub fn __to_hyphen_separated_js(&self) -> $crate::__private::String {
          self.to_hyphen_separated()
        }

        /// Converts to dot-separated format address.
        #[wasm_bindgen(js_name = "toDotSeparated")]
        #[doc(hidden)]
        pub fn __to_dot_separated_js(&self) -> $crate::__private::String {
          self.to_dot_separated()
        }

        /// Converts to colon-separated format bytes.
        #[wasm_bindgen(js_name = "toColonSeparatedBytes")]
        #[doc(hidden)]
        pub fn __to_colon_separated_array_js(
          &self,
        ) -> $crate::__private::Vec<::core::primitive::u8> {
          self.to_colon_separated_array().to_vec()
        }

        /// Converts to hyphen-separated format bytes.
        #[wasm_bindgen(js_name = "toHyphenSeparatedBytes")]
        #[doc(hidden)]
        pub fn __to_hyphen_separated_array_js(
          &self,
        ) -> $crate::__private::Vec<::core::primitive::u8> {
          self.to_hyphen_separated_array().to_vec()
        }

        /// Converts to dot-separated format bytes.
        #[wasm_bindgen(js_name = "toDotSeparatedBytes")]
        #[doc(hidden)]
        pub fn __to_dot_separated_array_js(&self) -> $crate::__private::Vec<::core::primitive::u8> {
          self.to_dot_separated_array().to_vec()
        }
      }
    };
  };
}

#[cfg(not(feature = "wasm-bindgen"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_wasm_bindgen {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {};
}
