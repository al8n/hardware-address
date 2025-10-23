#[macro_export]
#[doc(hidden)]
macro_rules! __addr_ty_quickcheck {
  (
    $name:ident[$n:expr]
  ) => {
    #[cfg(feature = "quickcheck")]
    const _: () = {
      impl $crate::__private::quickcheck::Arbitrary for $name {
        fn arbitrary(g: &mut $crate::__private::quickcheck::Gen) -> Self {
          let mut bytes = [0u8; $n];
          for byte in &mut bytes {
            *byte = <u8 as $crate::__private::quickcheck::Arbitrary>::arbitrary(g);
          }
          $name(bytes)
        }

        fn shrink(&self) -> $crate::__private::Box<dyn ::core::iter::Iterator<Item = Self>> {
          let bytes = self.0.to_vec();
          $crate::__private::Box::new(bytes.shrink().filter_map(|v| {
            if v.len() == $n {
              let mut arr = [0u8; $n];
              arr.copy_from_slice(&v);
              ::core::option::Option::Some($name(arr))
            } else {
              ::core::option::Option::None
            }
          }))
        }
      }
    };
  };
}
