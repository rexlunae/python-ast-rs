use pyo3::{FromPyObject};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
//use syn::Visibility;

use crate::codegen::{CodeGen, PythonContext, Result};
use crate::tree::{Arguments, Statement};

use log::info;

#[derive(Clone, Debug, FromPyObject)]
pub struct FunctionDef {
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Statement>,
}

impl CodeGen for FunctionDef {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        let mut streams = TokenStream::new();
        let fn_name = format_ident!("{}", self.name);

        // The Python convention is that functions that begin with a single underscore,
        // it's private. Otherwise, it's public. We formalize that by default.
        let visibility = if self.name.starts_with("_") && !self.name.starts_with("__") {
            format_ident!("")
        } else {
            format_ident!("pub")
        };

        for s in self.body.iter() {
            streams.extend(s.clone().to_rust(ctx)?);
        }

        let function = quote!{
            #visibility fn #fn_name() {
                #streams
            }
        };

        info!("function: {}", function);
        Ok(function)
    }
}