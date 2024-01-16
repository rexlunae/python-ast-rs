use pyo3::{FromPyObject};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::{
    dump,
    CodeGen, PythonOptions, CodeGenContext,
    ExprType,
    SymbolTableScopes, Node,
};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
//#[pyo3(transparent)]
pub struct Attribute {
    value: Box<ExprType>,
    attr: String,
    ctx: String,
}

impl<'a> FromPyObject<'a> for Attribute {
    fn extract(ob: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        let value = ob.getattr("value").expect("Attribute.value");
        let attr = ob.getattr("attr").expect("Attribute.attr");
        let ctx = ob.getattr("ctx").expect("getting attribute context")
            .get_type().name().expect(
                ob.error_message("<unknown>", format!("extracting type name {:?} in attribute", dump(ob, None)).as_str()).as_str()
        );
        Ok(Attribute {
            value: Box::new(ExprType::extract(&value).expect("Attribute.value")),
            attr: attr.extract().expect("Attribute.attr"),
            ctx: ctx.to_string(),
        })
    }
}

impl<'a> CodeGen for Attribute {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, _ctx: Self::Context, _options: Self::Options, _symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let name = self.value.to_rust(_ctx, _options, _symbols).expect("Attribute.value");
        let attr = format_ident!("{}", self.attr);
        Ok(quote!(#name.#attr))
    }
}
