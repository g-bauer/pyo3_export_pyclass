use pyo3::prelude::*;

#[pyclass(name="FancyName")]
pub struct MyClass {
    a: f64
}

#[cfg(feature="python_module")]
#[pymodule]
pub fn pyo3_export_pyclass(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}
