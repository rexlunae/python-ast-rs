use pyo3::{FromPyObject};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};
use crate::symbols::SymbolTableScopes;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq, Serialize, Deserialize)]
//#[pyo3(transparent)]
pub struct Name {
    pub id: String,
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name {
            id: s.to_string(),
        }
    }
}


impl CodeGen for Name {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, _ctx: Self::Context, _options: Self::Options, _symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let name = format_ident!("{}", self.id);
        Ok(quote!(#name))
    }
}
