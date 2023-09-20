use pyo3::{FromPyObject};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
//use syn::Visibility;

use crate::codegen::{CodeGen, PythonContext};
use crate::tree::{ParameterList, Statement, Name};

use log::debug;

#[derive(Clone, Debug, FromPyObject)]
pub struct ClassDef {
    pub name: String,
    pub bases: Vec<Name>,
    pub keywords: Vec<String>,
    // This isn't right, obviously.
    pub body: Vec<Statement>,
}

impl CodeGen for ClassDef {
    fn to_rust(self, ctx: &mut PythonContext) ->Result<TokenStream, Box<dyn std::error::Error>> {
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

        for s in self.body.iter() {
            streams.extend(s.clone().to_rust(ctx)?);
            //streams.extend(quote!(;));
        }

        let class = quote!{
            #visibility trait #class_name #bases {
                #streams
            }
        };

        debug!("class: {}", class);
        Ok(class)
    }
}
