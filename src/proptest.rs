#[cfg(feature = "proptest")]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_proptest {
  (
    $name:ident[$n:expr]
  ) => {
    const _: () = {
      impl $crate::__private::proptest::arbitrary::Arbitrary for $name {
        type Parameters = ();
        type Strategy = $crate::__private::proptest::strategy::BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
          use $crate::__private::proptest::strategy::Strategy;

          $crate::__private::proptest::arbitrary::any::<[::core::primitive::u8; $n]>()
            .prop_map($name)
            .boxed()
        }
      }
    };
  };
}

#[cfg(not(feature = "proptest"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_proptest {
  (
    $name:ident[$n:expr]
  ) => {};
}
