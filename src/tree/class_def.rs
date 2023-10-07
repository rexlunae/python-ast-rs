use pyo3::{FromPyObject};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};
use crate::tree::{Statement, Name, ExprType};

use log::debug;

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct ClassDef {
    pub name: String,
    pub bases: Vec<Name>,
    pub keywords: Vec<String>,
    pub body: Vec<Statement>,
}

impl CodeGen for ClassDef {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, _ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut streams = TokenStream::new();
        let class_name = format_ident!("{}", self.name);

        // The Python convention is that functions that begin with a single underscore,
        // it's private. Otherwise, it's public. We formalize that by default.
        let visibility = if self.name.starts_with("_") && !self.name.starts_with("__") {
            format_ident!("")
        } else if self.name.starts_with("__") && self.name.ends_with("__") {
            format_ident!("pub(crate)")
        } else {
            format_ident!("pub")
        };

        // bases will be empty if there are no base classes, which prevents any base traits
        // being added, and also prevents the : from being emitted.
        let mut bases = TokenStream::new();
        if self.bases.len() > 0 {
            bases.extend(quote!(:));
            let base_name = format_ident!("{}", self.bases[0].id);
            bases.extend(quote!(#base_name));
            for base in &self.bases[1..] {
                bases.extend(quote!(+));
                let base_name = format_ident!("{}", base.id);
                bases.extend(quote!(#base_name));
            }
        }
        debug!("bases: {:?}", bases);

        for s in self.body.clone() {
            streams.extend(s.clone().to_rust(CodeGenContext::Class, options.clone())?);
        }

        let docstring = if let Some(d) = self.get_docstring() {
            format!("/// {}", d)
        } else { "".to_string() };

        let class = quote!{
            #docstring
            #visibility trait #class_name #bases {
                #streams
            }
        };

        debug!("class: {}", class);
        Ok(class)
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
