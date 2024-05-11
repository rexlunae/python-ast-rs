use proc_macro2::TokenStream;
use pyo3::{FromPyObject, PyAny, PyResult};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Name, Node, PythonOptions, SymbolTableNode,
    SymbolTableScopes,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Assign {
    pub targets: Vec<Name>,
    pub value: ExprType,
    pub type_comment: Option<String>,
}

impl<'a> FromPyObject<'a> for Assign {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let targets: Vec<Name> = ob
            .getattr("targets")
            .expect(
                ob.error_message("<unknown>", "error getting unary operator")
                    .as_str(),
            )
            .extract()
            .expect("1");

        let python_value = ob.getattr("value").expect(
            ob.error_message("<unknown>", "assignment statement value not found")
                .as_str(),
        );

        let value = ExprType::extract(python_value).expect(
            ob.error_message("<unknown>", "error getting value of assignment statement")
                .as_str(),
        );

        Ok(Assign {
            targets: targets,
            value: value,
            type_comment: None,
        })
    }
}

impl<'a> CodeGen for Assign {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        let mut position = 0;
        for target in self.targets {
            symbols.insert(
                target.id,
                SymbolTableNode::Assign {
                    position: position,
                    value: self.value.clone(),
                },
            );
            position += 1;
        }
        symbols
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut stream = TokenStream::new();
        for target in self.targets.into_iter().map(|n| n.id) {
            let ident = format_ident!("{}", target);
            stream.extend(quote!(#ident));
        }
        let value = self.value.to_rust(ctx, options, symbols)?;
        Ok(quote!(#stream = #value))
    }
}
