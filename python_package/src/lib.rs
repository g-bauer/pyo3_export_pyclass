use pyo3::prelude::*;
use pyo3_export_pyclass::MyClass;

pub fn python_package(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}
