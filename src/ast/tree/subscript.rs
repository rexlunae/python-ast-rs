use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, types::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, PythonOptions, SymbolTableScopes,
    Node, impl_node_with_positions, PyAttributeExtractor
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Subscript {
    pub value: Box<ExprType>,
    pub slice: Box<ExprType>,
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for Subscript {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let value = ob.extract_attr_with_context("value", "subscript value")?;
        let slice = ob.extract_attr_with_context("slice", "subscript slice")?;
        
        let value = value.extract().expect("getting subscript value");
        let slice = slice.extract().expect("getting subscript slice");
        
        Ok(Subscript {
            value: Box::new(value),
            slice: Box::new(slice),
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl_node_with_positions!(Subscript { lineno, col_offset, end_lineno, end_col_offset });

impl CodeGen for Subscript {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let value = self.value.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let slice = self.slice.to_rust(ctx, options, symbols)?;
        
        Ok(quote! {
            #value[#slice]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_parse_test;

    create_parse_test!(test_list_subscript, "a[0]", "subscript_test.py");
    create_parse_test!(test_dict_subscript, "d['key']", "subscript_test.py");
    create_parse_test!(test_nested_subscript, "matrix[i][j]", "subscript_test.py");
}