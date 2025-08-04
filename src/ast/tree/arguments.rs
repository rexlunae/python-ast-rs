//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.
use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, Constant, Error, Node, PythonOptions, SymbolTableScopes,
};

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

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            Self::Constant(c) => {
                let v = c
                    .to_rust(ctx, options, symbols)
                    .expect(format!("Error generating constant argument.").as_str());
                println!("{:?}", v);
                Ok(quote!(#v))
            }
            _ => {
                let error = Error::UnknownType("Unknown argument type".to_string());
                Err(error.into())
            }
        }
    }
}

impl<'a> FromPyObject<'a> for Arg {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let ob_type = ob.get_type().name().expect(
            ob.error_message("<unknown>", "Could not extract argument type")
                .as_str(),
        );
        // FIXME: Hangle the rest of argument types.
        let ob_type_str: String = ob_type.extract()?;
        let r = match ob_type_str.as_ref() {
            "Constant" => {
                let err_msg = format!("parsing argument {:?} as a constant", ob);

                Self::Constant(
                    ob.extract()
                        .expect(ob.error_message("<unknown>", err_msg.as_str()).as_str()),
                )
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Argument {} is of unknown type {}",
                    ob, ob_type
                )))
            }
        };
        Ok(r)
    }
}
