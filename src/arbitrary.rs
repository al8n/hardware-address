
#[macro_export]
#[doc(hidden)]
macro_rules! __addr_ty_arbitrary {
  (
    $name:ident[$n:expr]
  ) => {
    #[cfg(feature = "arbitrary")]
    const _: () = {
      impl<'a> $crate::__private::arbitrary::Arbitrary<'a> for $name {
        fn arbitrary(u: &mut $crate::__private::arbitrary::Unstructured<'a>) -> $crate::__private::arbitrary::Result<Self> {
          <[::core::primitive::u8; $n] as $crate::__private::arbitrary::Arbitrary>::arbitrary(u).map($name)
        }

        fn size_hint(depth: ::core::primitive::usize) -> (::core::primitive::usize, ::core::option::Option<::core::primitive::usize>) {
          <[::core::primitive::u8; $n] as $crate::__private::arbitrary::Arbitrary>::size_hint(depth)
        }
      }
    };
  }
}
