use log::debug;
use proc_macro2::TokenStream;
use pyo3::FromPyObject;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};

use crate::{CodeGen, CodeGenContext, PythonOptions, SymbolTableNode, SymbolTableScopes};

#[derive(Clone, Debug, FromPyObject, Serialize, Deserialize, PartialEq)]
pub struct Alias {
    pub name: String,
    pub asname: Option<String>,
}

#[derive(Clone, Debug, FromPyObject, Serialize, Deserialize, PartialEq)]
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

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        for alias in self.names.iter() {
            symbols.insert(alias.name.clone(), SymbolTableNode::Import(self.clone()));
            if let Some(a) = alias.asname.clone() {
                symbols.insert(a, SymbolTableNode::Alias(alias.name.clone()))
            }
        }
        symbols
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        _symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut tokens = TokenStream::new();
        for alias in self.names.iter() {
            // Check if this is a Python standard library module that needs special handling
            let rust_import = match alias.name.as_str() {
                // Python stdlib modules that don't have direct Rust equivalents
                "os" | "sys" | "subprocess" | "json" | "urllib" | "xml" | "asyncio" => {
                    // These will be provided by the stdpython runtime
                    // Generate a comment instead of a use statement
                    quote! {
                        // Python module '{}' will be provided by stdpython runtime
                    }
                }
                "os.path" => {
                    quote! {
                        // Python os.path module will be provided by stdpython runtime
                    }
                }
                _ => {
                    // Handle other imports normally
                    let names = if alias.name.contains('.') {
                        let parts: Vec<&str> = alias.name.split('.').collect();
                        let idents: Vec<_> = parts.iter().map(|part| format_ident!("{}", part)).collect();
                        quote!(#(#idents)::*)
                    } else {
                        let single_name = format_ident!("{}", alias.name);
                        quote!(#single_name)
                    };
                    
                    match &alias.asname {
                        None => {
                            quote! {use #names;}
                        }
                        Some(n) => {
                            let name = format_ident!("{}", n);
                            quote! {use #names as #name;}
                        }
                    }
                }
            };
            
            tokens.extend(rust_import);
        }
        debug!("context: {:?}", ctx);
        debug!("options: {:?}", options);
        debug!("tokens: {}", tokens);
        Ok(tokens)
    }
}

#[derive(Clone, Debug, FromPyObject, Serialize, Deserialize, PartialEq)]
pub struct ImportFrom {
    pub module: String,
    pub names: Vec<Alias>,
    pub level: usize,
}

impl CodeGen for ImportFrom {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        for alias in self.names.iter() {
            symbols.insert(
                alias.name.clone(),
                SymbolTableNode::ImportFrom(self.clone()),
            );
        }
        symbols
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        _options: Self::Options,
        _symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        debug!("ctx: {:?}", ctx);
        Ok(quote! {})
    }
}
