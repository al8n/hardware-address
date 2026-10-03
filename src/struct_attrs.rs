#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_struct_base {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {
    $(#[$attr])*
    #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::Eq, ::core::cmp::PartialEq, ::core::cmp::Ord, ::core::cmp::PartialOrd, ::core::hash::Hash)]
    // `from_py_object` explicitly opts in to the `FromPyObject`
    // derive that pyo3 used to generate automatically for `Clone`
    // types. Address types are tiny (`[u8; N]` with `N` in {6, 8, 20}),
    // so the Clone-based conversion is effectively free and lets them
    // be passed as arguments to `#[pyfunction]` / `#[pymethods]`.
    #[repr(transparent)]
    pub struct $name(pub(crate) [::core::primitive::u8; $n]);
  };
}

#[cfg(all(feature = "pyo3", feature = "wasm-bindgen"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_struct {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {
    use $crate::__private::pyo3 as __pyo3;

    $crate::__addr_ty_struct_base! {
      $(#[$attr])*
      #[$crate::__private::pyo3::pyclass(crate = "__pyo3", from_py_object)]
      #[$crate::__private::wasm_bindgen::prelude::wasm_bindgen(wasm_bindgen = $crate::__private::wasm_bindgen)]
      $name[$n]
    }
  };
}

#[cfg(all(feature = "pyo3", not(feature = "wasm-bindgen")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_struct {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {
    use $crate::__private::pyo3 as __pyo3;

    $crate::__addr_ty_struct_base! {
      $(#[$attr])*
      #[$crate::__private::pyo3::pyclass(crate = "__pyo3", from_py_object)]
      $name[$n]
    }
  };
}

#[cfg(all(not(feature = "pyo3"), feature = "wasm-bindgen"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_struct {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {
    $crate::__addr_ty_struct_base! {
      $(#[$attr])*
      #[$crate::__private::wasm_bindgen::prelude::wasm_bindgen(wasm_bindgen = $crate::__private::wasm_bindgen)]
      $name[$n]
    }
  };
}

#[cfg(all(not(feature = "pyo3"), not(feature = "wasm-bindgen")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __addr_ty_struct {
  (
    $(#[$attr:meta])*
    $name:ident[$n:expr]
  ) => {
    $crate::__addr_ty_struct_base! {
      $(#[$attr])*
      $name[$n]
    }
  };
}
