use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, types::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, PythonOptions, SymbolTableScopes,
    Node, impl_node_with_positions, PyAttributeExtractor, extract_list
};

use super::Statement;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct While {
    pub test: ExprType,
    pub body: Vec<Statement>,
    pub orelse: Vec<Statement>,
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for While {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let test = ob.extract_attr_with_context("test", "while test condition")?;
        let test = test.extract().expect("getting while test");
        
        let body: Vec<Statement> = extract_list(ob, "body", "while body statements")?;
        let orelse: Vec<Statement> = extract_list(ob, "orelse", "while else statements")?;
        
        Ok(While {
            test,
            body,
            orelse,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl_node_with_positions!(While { lineno, col_offset, end_lineno, end_col_offset });

impl CodeGen for While {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let symbols = self.test.find_symbols(symbols);
        let symbols = self.body.into_iter().fold(symbols, |acc, stmt| stmt.find_symbols(acc));
        self.orelse.into_iter().fold(symbols, |acc, stmt| stmt.find_symbols(acc))
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let test = self.test.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        
        let body_stmts: Result<Vec<_>, _> = self.body
            .into_iter()
            .map(|stmt| stmt.to_rust(ctx.clone(), options.clone(), symbols.clone()))
            .collect();
        let body_stmts = body_stmts?;
        
        if self.orelse.is_empty() {
            Ok(quote! {
                while #test {
                    #(#body_stmts)*
                }
            })
        } else {
            // Note: Rust doesn't have while-else, so we need to track completion
            let else_stmts: Result<Vec<_>, _> = self.orelse
                .into_iter()
                .map(|stmt| stmt.to_rust(ctx.clone(), options.clone(), symbols.clone()))
                .collect();
            let else_stmts = else_stmts?;
            
            Ok(quote! {
                {
                    let mut broke = false;
                    while #test {
                        #(#body_stmts)*
                    }
                    if !broke {
                        #(#else_stmts)*
                    }
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_parse_test;

    create_parse_test!(test_simple_while, "while x > 0:\n    x -= 1", "while_test.py");
    create_parse_test!(test_while_else, "while x > 0:\n    x -= 1\nelse:\n    print('done')", "while_test.py");
    create_parse_test!(test_while_true, "while True:\n    break", "while_test.py");
}