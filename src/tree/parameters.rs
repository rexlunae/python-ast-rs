use crate::tree::Arg;
//use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};

//use proc_macro2::TokenStream;
//use quote::{format_ident, quote};
use pyo3::{PyAny, FromPyObject, PyResult};
use log::{debug, trace};

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct Parameter {
    pub arg: String,
}

/// The parameter list of a function.
#[derive(Clone, Debug, Default, FromPyObject)]
pub struct ParameterList {
    pub posonlyargs: Vec<Parameter>,
    pub args: Vec<Parameter>,
    pub vararg: Option<Parameter>,
    pub kwonlyargs: Vec<Parameter>,
    pub kw_defaults: Vec<Arg>,
    pub kwarg: Option<Arg>,
    pub defaults: Vec<Arg>,
}
