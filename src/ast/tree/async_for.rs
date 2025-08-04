use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Node, PythonOptions, Statement, SymbolTableScopes,
    extract_list,
};

/// Async for loop (async for target in iter: ...)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AsyncFor {
    /// The target variable(s)
    pub target: ExprType,
    /// The iterable expression
    pub iter: ExprType,
    /// The body of the loop
    pub body: Vec<Statement>,
    /// The else clause (executed when the loop completes normally)
    pub orelse: Vec<Statement>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for AsyncFor {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract target
        let target: ExprType = ob.getattr("target")?.extract()?;
        
        // Extract iter
        let iter: ExprType = ob.getattr("iter")?.extract()?;
        
        // Extract body
        let body: Vec<Statement> = extract_list(ob, "body", "async for body")?;
        
        // Extract orelse (optional)
        let orelse: Vec<Statement> = extract_list(ob, "orelse", "async for orelse").unwrap_or_default();
        
        Ok(AsyncFor {
            target,
            iter,
            body,
            orelse,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl Node for AsyncFor {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl CodeGen for AsyncFor {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        // Process target, iter, body, and orelse
        let symbols = self.target.find_symbols(symbols);
        let symbols = self.iter.find_symbols(symbols);
        let symbols = self.body.into_iter().fold(symbols, |acc, stmt| stmt.find_symbols(acc));
        self.orelse.into_iter().fold(symbols, |acc, stmt| stmt.find_symbols(acc))
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // Generate iter expression
        let _iter_expr = self.iter.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        
        // Generate body
        let body_tokens: Result<Vec<TokenStream>, Box<dyn std::error::Error>> = self.body.into_iter()
            .map(|stmt| stmt.to_rust(ctx.clone(), options.clone(), symbols.clone()))
            .collect();
        let body_tokens = body_tokens?;

        // Generate else clause if present
        let else_tokens = if !self.orelse.is_empty() {
            let else_body_tokens: Result<Vec<TokenStream>, Box<dyn std::error::Error>> = self.orelse.into_iter()
                .map(|stmt| stmt.to_rust(ctx.clone(), options.clone(), symbols.clone()))
                .collect();
            let else_body_tokens = else_body_tokens?;
            quote! {
                // Else clause (executed when loop completes normally)
                #(#else_body_tokens)*
            }
        } else {
            quote!()
        };

        // For now, generate a simplified async iteration
        // In practice, this would need proper async stream handling
        Ok(quote! {
            {
                // Async for loop - simplified translation  
                // Python's async for doesn't map directly to Rust's async streams
                // This would typically use futures::stream::StreamExt
                #(#body_tokens)*
                
                // Else clause
                #else_tokens
            }
        })
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here - currently commented out as they need full AST infrastructure
    // create_parse_test!(test_simple_async_for, "async for item in async_iter:\n    pass", "test.py");
}