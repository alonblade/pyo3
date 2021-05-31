//! Test for [#220](https://github.com/PyO3/pyo3/issues/220)

use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct Subclassable {}

#[pymethods]
impl Subclassable {
    #[new]
    fn new() -> Self {
        Subclassable {}
    }
}

#[pyclass(subclass)]
pub struct SubclassableWithParameter {}

#[pymethods]
impl SubclassableWithParameter {
    #[new]
    fn new(foo: bool) -> Self {
        SubclassableWithParameter {}
    }
}


#[pyproto]
impl pyo3::PyObjectProtocol for Subclassable {
    fn __str__(&self) -> PyResult<&'static str> {
        Ok("Subclassable")
    }
}

#[pymodule]
fn subclassing(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Subclassable>()?;
    m.add_class::<SubclassableWithParameter>()?;
    Ok(())
}
