use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, Node, PythonOptions, Statement, SymbolTableScopes,
    extract_list, WithItem,
};

/// Regular with statement (with context as var: ...)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct With {
    /// The with items (context managers)
    pub items: Vec<WithItem>,
    /// The body of the with statement
    pub body: Vec<Statement>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for With {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract items (list of withitem objects)
        let items: Vec<WithItem> = extract_list(ob, "items", "with items")?;
        
        // Extract body
        let body: Vec<Statement> = extract_list(ob, "body", "with body")?;
        
        Ok(With {
            items,
            body,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl Node for With {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl CodeGen for With {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        // Process items and body
        let symbols = self.items.into_iter().fold(symbols, |acc, item| {
            let acc = item.context_expr.find_symbols(acc);
            if let Some(vars) = item.optional_vars {
                vars.find_symbols(acc)
            } else {
                acc
            }
        });
        self.body.into_iter().fold(symbols, |acc, stmt| stmt.find_symbols(acc))
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // Generate body
        let body_tokens: Result<Vec<TokenStream>, Box<dyn std::error::Error>> = self.body.into_iter()
            .map(|stmt| stmt.to_rust(ctx.clone(), options.clone(), symbols.clone()))
            .collect();
        let body_tokens = body_tokens?;

        // For now, generate a simplified block
        // In practice, this would need proper context management
        Ok(quote! {
            {
                // With block - simplified translation
                // Python's with doesn't map directly to Rust patterns
                #(#body_tokens)*
            }
        })
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here - currently commented out as they need full AST infrastructure
    // create_parse_test!(test_simple_with, "with context:\n    pass", "test.py");
}