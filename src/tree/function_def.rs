use pyo3::{PyAny, FromPyObject, PyResult};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::codegen::{CodeGen, CodeGenError, Result};
use crate::tree::{Arguments, Statement};

#[derive(Clone, Debug, FromPyObject)]
pub struct FunctionDef {
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Statement>,
}

impl CodeGen for FunctionDef {
    fn to_rust(self) -> Result<TokenStream> {
        let mut streams = TokenStream::new();
        let fn_name = format_ident!("{}", self.name);

        for s in self.body.iter() {
            streams.extend(s.clone().to_rust()?);
        }

        let function = quote!{
            fn #fn_name() {
                #streams
            }
        };

        println!("function: {}", function);
        Ok(function)
    }
}