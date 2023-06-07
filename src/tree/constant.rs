use pyo3::{PyAny, FromPyObject, PyResult};

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct Constant {
    value: String,
}
