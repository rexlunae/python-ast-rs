use crate::tree::Arg;
//use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};

//use proc_macro2::TokenStream;
//use quote::{format_ident, quote};
use pyo3::{PyAny, FromPyObject, PyResult};
use log::{debug, trace};

/// The parameter list of a function.
#[derive(Clone, Debug, Default)]
pub struct ParameterList {
    pub posonlyargs: Vec<Arg>,
    pub args: Vec<Arg>,
    pub vararg: Vec<Arg>,
    pub kwonlyargs: Vec<Arg>,
    pub kw_defaults: Vec<String>,
    pub kwarg: Arg,
    pub defaults: Vec<String>,
}

impl<'a> FromPyObject<'a> for ParameterList {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        debug!("parsing arguments: {:?}", ob);
        trace!("{}", ob);

        let args = Self{
            ..Default::default()
        };
        Ok(args)
    }
}
