//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.


//use crate::tree::Constant;
use crate::codegen::{CodeGen, CodeGenError, PythonContext};
use crate::ast_dump;
use crate::tree::Constant;

use proc_macro2::TokenStream;
use quote::{quote};
use pyo3::{PyAny, FromPyObject, PyResult};
use log::{debug, trace};

/// An argument.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Arg {
    #[default]
    Unknown,
    //Constant(Constant),
    Constant(Constant),
}

impl CodeGen for Arg {
    fn to_rust(self, _ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            Self::Constant(c) => {
                let v = c.0;
                println!("{:?}", v);
                Ok(quote!(#v))
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
        // FIXME: We will need to figure out how to determine what type of argument this actually is. For now, it only supports Constants
        let value = ob.getattr("value")?;
        let args = Self::Constant(Constant(format!("{}", value)));
        Ok(args)
    }
}
