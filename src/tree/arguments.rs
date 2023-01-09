//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.


use pyo3::{PyAny, FromPyObject, PyResult};

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct Arg {
    pub name: String
}

/// An argument list. Represents all possible arguments to a function.
#[derive(Clone, Debug, Default)]
pub struct Arguments {
    pub posonlyargs: Vec<Arg>,
    pub args: Vec<Arg>,
    pub vararg: Vec<Arg>,
    pub kwonlyargs: Vec<Arg>,
    pub kw_defaults: Vec<String>,
    pub kwarg: Arg,
    pub defaults: Vec<String>,
}

impl<'a> FromPyObject<'a> for Arguments {
    fn extract(_ob: &'a PyAny) -> PyResult<Self> {
        Ok(Self{
            ..Default::default()
        })
    }
}


