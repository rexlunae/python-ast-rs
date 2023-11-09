use pyo3::{FromPyObject};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq)]
//#[pyo3(transparent)]
pub struct Name {
    pub id: String,
}

impl<'a> CodeGen for Name {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, _ctx: Self::Context, _options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let name = format_ident!("{}", self.id);
        Ok(quote!(#name))
    }
}
