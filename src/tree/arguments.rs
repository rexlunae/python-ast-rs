//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.


use crate::tree::Constant;
use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use pyo3::{PyAny, FromPyObject, PyResult};
use log::{debug, trace};

/// An argument.
#[derive(Clone, Debug, Default)]
pub enum Arg {
    #[default]
    Unknown,
    Constant(Constant),
}

impl CodeGen for Arg {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        match self {
            Self::Constant(c) => {
                let v = c.value;
                Ok(quote!(#v))
            },
            _ => Err(CodeGenError("Unknown argument type".to_string(), None)),
        }
    }
}

impl<'a> FromPyObject<'a> for Arg {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        debug!("parsing arg: {:?}", ob);
        trace!("{}", ob);

        // FIXME: We will need to figure out how to determine what type of argument this actually is.
        let args = Self::Constant(ob.extract()?);
        Ok(args)
    }
}

/// A function argument list.
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
        debug!("parsing arguments: {:?}", ob);
        trace!("{}", ob);

        let mut args = Self{
            ..Default::default()
        };



        /*
        match parts[0] {
            "Constant" => Constant()
        }*/
        Ok(args)
    }
}
