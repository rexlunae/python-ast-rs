use pyo3::{FromPyObject};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
//use syn::Visibility;

use crate::codegen::{CodeGen, PythonContext};
use crate::tree::{ParameterList, Statement, ExprType};

use log::debug;

#[derive(Clone, Debug, FromPyObject)]
pub struct FunctionDef {
    pub name: String,
    pub args: ParameterList,
    pub body: Vec<Statement>,
    pub decorator_list: Vec<String>,
}

impl<'a> CodeGen for FunctionDef {
    fn to_rust(self, ctx: &mut PythonContext) ->Result<TokenStream, Box<dyn std::error::Error>> {
        let mut streams = TokenStream::new();
        let fn_name = format_ident!("{}", self.name);

        // The Python convention is that functions that begin with a single underscore,
        // it's private. Otherwise, it's public. We formalize that by default.
        let visibility = if self.name.starts_with("_") && !self.name.starts_with("__") {
            format_ident!("")
        } else if self.name.starts_with("__") && self.name.ends_with("__") {
            format_ident!("pub(crate)")
        } else {
            format_ident!("pub")
        };

        let parameters = self.args.clone().to_rust(ctx)?;

        for s in self.body.iter() {
            streams.extend(s.clone().to_rust(ctx)?);
            streams.extend(quote!(;));
        }

        let docstring = if let Some(d) = self.get_docstring() {
            format!("/// {}", d)
        } else { "".to_string() };


        let function = quote!{
            #docstring
            #visibility fn #fn_name(#parameters) {
                #streams
            }
        };

        debug!("function: {}", function);
        Ok(function)
    }

    // override the default to allow functions to be compiled as trait members.
    fn to_rust_trait_member(&self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        (*self).clone().to_rust(ctx)
    }

    fn get_docstring(&self) -> Option<String> {
        let expr = self.body[0].clone();
        match expr {
            Statement::Expr(e) => {
                match e.value {
                    ExprType::Constant(c) => Some(c.0.to_string()),
                    _ => None,
                }
            },
            _ => None,
        }
    }
}