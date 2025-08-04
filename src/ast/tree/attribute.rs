use proc_macro2::TokenStream;
use pyo3::{Bound, PyAny, FromPyObject, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::{format_ident, quote};

use crate::{dump, CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
//#[pyo3(transparent)]
pub struct Attribute {
    value: Box<ExprType>,
    attr: String,
    ctx: String,
}

impl<'a> FromPyObject<'a> for Attribute {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let value = ob.getattr("value").expect("Attribute.value");
        let attr = ob.getattr("attr").expect("Attribute.attr");
        let ctx = ob
            .getattr("ctx")
            .expect("getting attribute context")
            .get_type()
            .name()
            .expect(
                ob.error_message(
                    "<unknown>",
                    format!("extracting type name {:?} in attribute", dump(ob, None)),
                )
                .as_str(),
            );
        Ok(Attribute {
            value: Box::new(value.extract().expect("Attribute.value")),
            attr: attr.extract().expect("Attribute.attr"),
            ctx: ctx.to_string(),
        })
    }
}

impl<'a> CodeGen for Attribute {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        _ctx: Self::Context,
        _options: Self::Options,
        _symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let name = self
            .value
            .to_rust(_ctx, _options, _symbols)
            .expect("Attribute.value");
        let attr = format_ident!("{}", self.attr);
        Ok(quote!(#name.#attr))
    }
}
