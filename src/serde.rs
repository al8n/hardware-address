#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_serde {
  (
    $name:ident[$n:expr]
  ) => {
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
            <[::core::primitive::u8; $n] as $crate::__private::serde::Serialize>::serialize(
              &self.0, serializer,
            )
          }
        }
      }

      impl<'a> $crate::__private::serde::Deserialize<'a> for $name {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
        where
          D: $crate::__private::serde::Deserializer<'a>,
        {
          if deserializer.is_human_readable() {
            struct AddressVisitor;

            impl<'de> $crate::__private::serde::de::Visitor<'de> for AddressVisitor {
              type Value = $name;

              fn expecting(
                &self,
                formatter: &mut ::core::fmt::Formatter<'_>,
              ) -> ::core::fmt::Result {
                ::core::write!(formatter, "a colon-, hyphen-, or dot-separated address")
              }

              fn visit_borrowed_str<E>(
                self,
                value: &'de str,
              ) -> ::core::result::Result<Self::Value, E>
              where
                E: $crate::__private::serde::de::Error,
              {
                <$name as ::core::str::FromStr>::from_str(value)
                  .map_err($crate::__private::serde::de::Error::custom)
              }

              fn visit_str<E>(self, value: &str) -> ::core::result::Result<Self::Value, E>
              where
                E: $crate::__private::serde::de::Error,
              {
                <$name as ::core::str::FromStr>::from_str(value)
                  .map_err($crate::__private::serde::de::Error::custom)
              }
            }

            deserializer.deserialize_str(AddressVisitor)
          } else {
            let bytes =
              <[::core::primitive::u8; $n] as $crate::__private::serde::Deserialize>::deserialize(
                deserializer,
              )?;
            ::core::result::Result::Ok($name(bytes))
          }
        }
      }
    };
  };
}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_serde {
  (
    $name:ident[$n:expr]
  ) => {};
}
