use pyo3::{PyAny, FromPyObject, PyResult};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq)]
pub struct Constant {
    pub value: String,
}
