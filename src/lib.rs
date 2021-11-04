use pyo3::prelude::*;
use numpy::{ToPyArray, PyReadonlyArray1};

#[pyclass(name="FancyName")]
pub struct MyClass {
    a: f64
}

/// This is my class
#[pymethods]
impl MyClass {
    #[new]
    pub fn new(a: PyReadonlyArray1<f64>) -> Self {
        Self { a: 1.0 }
    }

   pub fn asd(&self) -> PyResult<()> { 
       println!("{}", self.a);
       Ok(())
   }
}

#[cfg(feature="python_module")]
#[pymodule]
pub fn pyo3_export_pyclass(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}
