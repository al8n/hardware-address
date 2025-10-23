use pyo3::prelude::*;
use ::hardware_address::*;

#[pymodule]
fn hardware_address(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MacAddr>()?;
    m.add_class::<Eui64Addr>()?;
    m.add_class::<InfiniBandAddr>()?;
    Ok(())
}
