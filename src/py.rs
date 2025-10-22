
#[macro_export]
#[doc(hidden)]
macro_rules! __addr_ty_pyo3 {
  (
    $name:ident[$n:expr]
  ) => {
    #[cfg(feature = "pyo3")]
    #[allow(clippy::wrong_self_convention)]
    const _: () = {
      use $crate::__private::pyo3::prelude::*;
      use $crate::__private::pyo3::types::PyBytes;
      use $crate::__private::ToString;

      #[pymethods]
      impl $name {
        #[new]
        fn __py_new() -> Self {
          $name::new()
        }

        fn __str__(&self) -> $crate::__private::String {
          ::core::format_args!("{}", self).to_string()
        }

        fn __repr__(&self) -> $crate::__private::String {
          ::core::format_args!("{}(\"{}\")", ::core::stringify!($name), self).to_string()
        }

        fn __bytes__<'py>(&self, py: $crate::__private::pyo3::Python<'py>) -> Bound<'py, PyBytes> {
          PyBytes::new(py, &self.0)
        }

        fn __hash__(&self) -> u64 {
          use ::core::hash::{Hash, Hasher};
          let mut hasher = $crate::__private::DefaultHasher::new();
          self.hash(&mut hasher);
          hasher.finish()
        }

        fn __richcmp__(&self, other: &Self, op: $crate::__private::pyo3::pyclass::CompareOp) -> bool {
          use $crate::__private::pyo3::pyclass::CompareOp;
          match op {
            CompareOp::Lt => self < other,
            CompareOp::Le => self <= other,
            CompareOp::Eq => self == other,
            CompareOp::Ne => self != other,
            CompareOp::Gt => self > other,
            CompareOp::Ge => self >= other,
          }
        }

        /// Converts to colon-separated format bytes.
        #[pyo3(name = "to_colon_separated_bytes")]
        fn __to_colon_separated_array_py<'py>(&self, py: $crate::__private::pyo3::Python<'py>) -> Bound<'py, PyBytes> {
          let buf = self.to_colon_separated_array();
          PyBytes::new(py, &buf)
        }

        /// Converts to hyphen-separated format bytes.
        #[pyo3(name = "to_hyphen_separated_bytes")]
        fn __to_hyphen_separated_array_py<'py>(&self, py: $crate::__private::pyo3::Python<'py>) -> Bound<'py, PyBytes> {
          let buf = self.to_hyphen_separated_array();
          PyBytes::new(py, &buf)
        }

        /// Converts to dot-separated format bytes.
        #[pyo3(name = "to_dot_separated_bytes")]
        fn __to_dot_separated_array_py<'py>(&self, py: $crate::__private::pyo3::Python<'py>) -> Bound<'py, PyBytes> {
          let buf = self.to_dot_separated_array();
          PyBytes::new(py, &buf)
        }

        /// Converts to colon-separated string representation.
        #[pyo3(name = "to_colon_separated")]
        #[cfg(any(feature = "alloc", feature = "std"))]
        fn __to_colon_separated_py(&self) -> $crate::__private::String {
          self.to_colon_separated()
        }

        /// Converts to hyphen-separated string representation.
        #[pyo3(name = "to_hyphen_separated")]
        #[cfg(any(feature = "alloc", feature = "std"))]
        fn __to_hyphen_separated_py(&self) -> $crate::__private::String {
          self.to_hyphen_separated()
        }

        /// Converts to dot-separated string representation.
        #[pyo3(name = "to_dot_separated")]
        #[cfg(any(feature = "alloc", feature = "std"))]
        fn __to_dot_separated_py(&self) -> $crate::__private::String {
          self.to_dot_separated()
        }

        /// Parses an address from string.
        #[staticmethod]
        #[pyo3(name = "parse")]
        #[cfg_attr(docsrs, doc(hidden))]
        fn __parse_py(s: &::core::primitive::str) -> $crate::__private::pyo3::PyResult<Self> {
          <$name as ::core::str::FromStr>::from_str(s)
            .map_err(|e| $crate::__private::pyo3::exceptions::PyValueError::new_err(::core::format_args!("{}", e).to_string()))
        }

        /// Create an address from bytes.
        #[staticmethod]
        #[pyo3(name = "from_bytes")]
        fn __from_bytes(bytes: &Bound<'_, PyBytes>) -> $crate::__private::pyo3::PyResult<Self> {
          let data = bytes.as_bytes();
          if data.len() != $n {
            return Err($crate::__private::pyo3::exceptions::PyValueError::new_err(
              format!("Expected {} bytes, got {}", $n, data.len())
            ));
          }
          let mut arr = [0u8; $n];
          arr.copy_from_slice(data);
          Ok($name(arr))
        }
      }
    };
  }
}
