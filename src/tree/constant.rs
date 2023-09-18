use pyo3::{PyAny, FromPyObject, PyResult};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq)]
//#[pyo3(transparent)]
pub struct Constant {
    pub value: String,
}
