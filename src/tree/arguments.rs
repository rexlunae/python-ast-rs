//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.

//use crate::tree::Constant;
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, CodeGenContext, Node};
use crate::tree::Constant;
use crate::symbols::SymbolTableScopes;

use proc_macro2::TokenStream;
use quote::{quote};
use pyo3::{PyAny, FromPyObject, PyResult};

use serde::{Serialize, Deserialize};

/// An argument.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum Arg {
    #[default]
    Unknown,
    //Constant(Constant),
    Constant(Constant),
}

impl<'a> CodeGen for Arg {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            Self::Constant(c) => {
                let v = c.to_rust(ctx, options, symbols).expect(format!("Error generating constant argument.").as_str());
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
        let ob_type = ob.get_type().name().expect(
            ob.error_message("<unknown>", "Could not extract argument type").as_str()
        );
        // FIXME: Hangle the rest of argument types.
        let r = match ob_type {
            "Constant" => {
                let err_msg = format!("parsing argument {:?} as a constant", ob);

                Self::Constant(Constant::extract(ob).expect(
                ob.error_message("<unknown>", err_msg.as_str()).as_str()
            ))},
            _ => return Err(pyo3::exceptions::PyValueError::new_err(format!("Argument {} is of unknown type {}", ob, ob_type)))
        };
        Ok(r)
    }
}
