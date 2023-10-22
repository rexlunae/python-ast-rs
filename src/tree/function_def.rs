use pyo3::{FromPyObject};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
//use syn::Visibility;

use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};
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
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
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

        let parameters = self.args.clone().to_rust(ctx, options.clone())
            .expect(format!("parsing arguments {:?}", self.args).as_str());

        for s in self.body.iter() {
            streams.extend(s.clone().to_rust(ctx, options.clone()).expect(format!("parsing statement {:?}", s).as_str()));
            streams.extend(quote!(;));
        }

        let docstring = if let Some(d) = self.get_docstring() {
            format!("{}", d)
        } else { "No docstring\n".to_string() };


        // XXX: Figure out how to add docstrict.
        let function = quote!{
            #visibility fn #fn_name(#parameters) {
                #streams
            }
        };

        debug!("function: {}", function);
        Ok(function)
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