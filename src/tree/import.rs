use pyo3::{PyAny, FromPyObject, PyResult};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};

#[derive(Clone, Debug, FromPyObject)]
pub struct Alias {
    pub name: String,
    pub asname: Option<String>,
}

#[derive(Clone, Debug, FromPyObject)]
pub struct Import {
    pub names: Vec<Alias>,
}

impl CodeGen for Import {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();
        for alias in self.names.iter() {
            let mod_path = format_ident!("{}", ctx.python_namespace);
            let names = alias.name.split(".");
            let full_mod_name_list: Vec<&str> = names.clone().collect();
            let full_mod_name = full_mod_name_list.join("::");
            let code = match &alias.asname {
                None => {
                    ctx.import(&full_mod_name, &full_mod_name);
                    let name = format_ident!("{}", alias.name);
                    quote!{use #(#names)::*:: ::#name}
                },
                Some(n) => {
                    ctx.import(&full_mod_name, &String::from(n));

                    let name = format_ident!("{}", alias.name);
                    let alias = format_ident!("{}", n);
                    quote!{use #(#names)::*:: ::#name as #alias}
                },
            };
            tokens.extend(code);
        }
        println!("ctx: {:?}", ctx);
        println!("tokens: {}", tokens);
        Ok(tokens)
    }
}

#[derive(Clone, Debug, FromPyObject)]
pub struct ImportFrom {
    pub module: String,
    pub names: Vec<Alias>,
    pub level: usize,
}

impl CodeGen for ImportFrom {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        println!("ctx: {:?}", ctx);
        Ok(quote!{})
    }
}