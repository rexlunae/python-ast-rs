use pyo3::{PyAny, FromPyObject, PyResult};

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct Arg {
    pub name: String
}

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
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        Ok(Self{
            ..Default::default()
        })
    }
}


