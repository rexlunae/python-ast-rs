use pyo3::{FromPyObject, PyAny, PyResult};
use crate::codegen::Node;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use crate::tree::{ExprType, Name};
use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Assign {
    pub targets: Vec<Name>,
    pub value: ExprType,
    pub type_comment: Option<String>,
}

impl<'a> FromPyObject<'a> for Assign {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let targets: Vec<Name> = ob.getattr("targets").expect(
            ob.error_message("<unknown>", "error getting unary operator").as_str()
        ).extract().expect("1");

        let value = ob.extract().expect("3");

        return Ok(Assign{
            targets: targets,
            value: value,
            type_comment: None,
        });

    }
}

impl<'a> CodeGen for Assign {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut stream = TokenStream::new();
        for target in self.targets.into_iter().map(|n| n.id) {
            let ident = format_ident!("{}", target);
            stream.extend(quote!(#ident));
        };
        let value = self.value.to_rust(ctx, options)?;
        Ok(quote!(#stream = #value))
    }
}
