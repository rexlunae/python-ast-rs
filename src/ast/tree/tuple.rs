use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, PythonOptions, SymbolTableScopes,
    Node, impl_node_with_positions, extract_list
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tuple {
    pub elts: Vec<ExprType>,
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for Tuple {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let elts: Vec<ExprType> = extract_list(ob, "elts", "tuple elements")?;
        
        Ok(Tuple {
            elts,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl_node_with_positions!(Tuple { lineno, col_offset, end_lineno, end_col_offset });

impl CodeGen for Tuple {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let elements: Result<Vec<_>, _> = self.elts
            .into_iter()
            .map(|elt| elt.to_rust(ctx.clone(), options.clone(), symbols.clone()))
            .collect();
        
        let elements = elements?;
        
        // Generate appropriate tuple syntax based on element count
        match elements.len() {
            0 => Ok(quote! { () }),
            1 => {
                let element = &elements[0];
                Ok(quote! { (#element,) }) // Single element tuple needs trailing comma
            },
            _ => Ok(quote! { (#(#elements),*) }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_parse_test;

    create_parse_test!(test_empty_tuple, "()", "tuple_test.py");
    create_parse_test!(test_single_tuple, "(1,)", "tuple_test.py");
    create_parse_test!(test_simple_tuple, "(1, 2, 3)", "tuple_test.py");
    create_parse_test!(test_tuple_with_variables, "(x, y, z)", "tuple_test.py");
}