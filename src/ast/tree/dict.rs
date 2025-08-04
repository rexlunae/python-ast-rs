use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, PythonOptions, SymbolTableScopes,
    Node, impl_node_with_positions, extract_list
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Dict {
    pub keys: Vec<Option<ExprType>>,
    pub values: Vec<ExprType>,
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for Dict {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let keys: Vec<Option<ExprType>> = extract_list(ob, "keys", "dictionary keys")?;
        let values: Vec<ExprType> = extract_list(ob, "values", "dictionary values")?;
        
        Ok(Dict {
            keys,
            values,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl_node_with_positions!(Dict { lineno, col_offset, end_lineno, end_col_offset });

impl CodeGen for Dict {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut pairs = Vec::new();
        
        for (key, value) in self.keys.iter().zip(self.values.iter()) {
            match key {
                Some(k) => {
                    let key_tokens = k.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                    let value_tokens = value.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                    pairs.push(quote! { (#key_tokens, #value_tokens) });
                }
                None => {
                    // Handle dictionary unpacking (**dict)
                    let value_tokens = value.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                    pairs.push(quote! { ..#value_tokens });
                }
            }
        }
        
        Ok(quote! {
            std::collections::HashMap::from([#(#pairs),*])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_parse_test;

    create_parse_test!(test_empty_dict, "{}", "dict_test.py");
    create_parse_test!(test_simple_dict, "{'a': 1, 'b': 2}", "dict_test.py");
    create_parse_test!(test_dict_with_variables, "{x: y, z: w}", "dict_test.py");
}