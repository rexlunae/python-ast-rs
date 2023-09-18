//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.


use crate::tree::Constant;
use crate::codegen::{CodeGen, CodeGenError, PythonContext};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use pyo3::{PyAny, FromPyObject, PyResult};
use log::{debug, trace};

/// An argument.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Arg {
    #[default]
    Unknown,
    //Constant(Constant),
    Constant(String),
}

impl CodeGen for Arg {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            Self::Constant(c) => {
                //let v = c.value;
                //Ok(quote!(#v))
                Ok(quote!(#c))
            },
            _ => {
                let error = CodeGenError("Unknown argument type".to_string(), None);
                Err(Box::new(error))
            },
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
