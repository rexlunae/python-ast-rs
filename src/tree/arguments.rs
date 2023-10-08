//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.


//use crate::tree::Constant;
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, CodeGenContext};
use crate::tree::Constant;

use proc_macro2::TokenStream;
use quote::{quote};
use pyo3::{PyAny, FromPyObject, PyResult};

/// An argument.
#[derive(Clone, Debug, Default)]
pub enum Arg {
    #[default]
    Unknown,
    //Constant(Constant),
    Constant(Constant),
}

impl<'a> CodeGen for Arg {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            Self::Constant(c) => {
                let v = c.to_rust(ctx, options).expect(format!("Error generating constant argument.").as_str());
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
        let ob_type = ob.get_type().name().expect(format!("Could not extract argument type for {:?}", ob).as_str());
        // FIXME: Hangle the rest of argument types.
        let r = match ob_type {
            "Constant" => Self::Constant(Constant::extract(ob).expect(format!("parsing argument {:?} as a constant", ob).as_str())),
            _ => return Err(pyo3::exceptions::PyValueError::new_err(format!("Argument {} is of unknown type {}", ob, ob_type)))
        };
        Ok(r)
    }
}
