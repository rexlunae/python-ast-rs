use proc_macro2::TokenStream;
use pyo3::FromPyObject;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};

use crate::{Arg, CodeGen, CodeGenContext, PythonOptions, SymbolTableScopes};

/// A keyword argument, gnerally used in function calls.
#[derive(Clone, Debug, Default, FromPyObject, Serialize, Deserialize, PartialEq)]
pub struct Keyword {
    arg: String,
    value: Arg,
}

impl CodeGen for Keyword {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let arg = format_ident!("{}", self.arg);
        let value = self
            .value
            .clone()
            .to_rust(ctx, options, symbols)
            .expect(format!("parsing argument {:?}", self.value).as_str());
        Ok(quote!(#arg = #value))
    }
}
