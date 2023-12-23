use pyo3::{FromPyObject};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use log::debug;

use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};
use crate::symbols::SymbolTableScopes;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, FromPyObject, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub asname: Option<String>,
}

#[derive(Clone, Debug, FromPyObject, Serialize, Deserialize)]
pub struct Import {
    pub names: Vec<Alias>,
}

/// An Import (or FromImport) statement causes 2 things to occur:
/// 1. Declares the imported object within the existing scope.
/// 2. Causes the referenced module to be compiled into the program (only once).

impl CodeGen for Import {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, _symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut tokens = TokenStream::new();
        for alias in self.names.iter() {
            let names = format_ident!("{}", alias.name.replace(".", "::"));
            let code = match &alias.asname {
                None => {
                    //options.clone().import(names, name);
                    quote!{use #names;}
                },
                Some(n) => {
                    //options.clone().import(&full_mod_name, &String::from(n));

                    let name = format_ident!("{}", n);
                    quote!{use #names as #name;}
                },
            };
            tokens.extend(code);
        }
        debug!("context: {:?}", ctx);
        debug!("options: {:?}", options);
        debug!("tokens: {}", tokens);
        Ok(tokens)
    }
}

#[derive(Clone, Debug, FromPyObject, Serialize, Deserialize)]
pub struct ImportFrom {
    pub module: String,
    pub names: Vec<Alias>,
    pub level: usize,
}

impl CodeGen for ImportFrom {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, _options: Self::Options, _symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        debug!("ctx: {:?}", ctx);
        Ok(quote!{})
    }
}