use pyo3::{FromPyObject};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq)]
//#[pyo3(transparent)]
pub struct Constant {
    pub value: String,
}
