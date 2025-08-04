use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, types::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, PythonOptions, SymbolTableScopes,
    Node, impl_node_with_positions, PyAttributeExtractor
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IfExp {
    pub test: Box<ExprType>,
    pub body: Box<ExprType>,
    pub orelse: Box<ExprType>,
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for IfExp {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let test = ob.extract_attr_with_context("test", "if expression test")?;
        let body = ob.extract_attr_with_context("body", "if expression body")?;
        let orelse = ob.extract_attr_with_context("orelse", "if expression orelse")?;
        
        let test = test.extract().expect("getting if expression test");
        let body = body.extract().expect("getting if expression body");
        let orelse = orelse.extract().expect("getting if expression orelse");
        
        Ok(IfExp {
            test: Box::new(test),
            body: Box::new(body),
            orelse: Box::new(orelse),
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl_node_with_positions!(IfExp { lineno, col_offset, end_lineno, end_col_offset });

impl CodeGen for IfExp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let test = self.test.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let body = self.body.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let orelse = self.orelse.to_rust(ctx, options, symbols)?;
        
        Ok(quote! {
            if #test { #body } else { #orelse }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_parse_test;

    create_parse_test!(test_if_expression, "x if condition else y", "if_exp_test.py");
    create_parse_test!(test_nested_if_expression, "a if b else c if d else e", "if_exp_test.py");
}