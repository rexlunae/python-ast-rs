use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes,
};

/// Yield expression (yield value)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Yield {
    /// The value being yielded (optional)
    pub value: Option<Box<ExprType>>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// Yield from expression (yield from iterable)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct YieldFrom {
    /// The iterable being yielded from
    pub value: Box<ExprType>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for Yield {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract value (optional)
        let value: Option<Box<ExprType>> = if let Ok(value_attr) = ob.getattr("value") {
            if value_attr.is_none() {
                None
            } else {
                Some(Box::new(value_attr.extract()?))
            }
        } else {
            None
        };
        
        Ok(Yield {
            value,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl<'a> FromPyObject<'a> for YieldFrom {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract value
        let value: ExprType = ob.getattr("value")?.extract()?;
        
        Ok(YieldFrom {
            value: Box::new(value),
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl Node for Yield {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl Node for YieldFrom {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl CodeGen for Yield {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        if let Some(value) = self.value {
            (*value).find_symbols(symbols)
        } else {
            symbols
        }
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        if let Some(value) = self.value {
            let value_tokens = (*value).to_rust(ctx, options, symbols)?;
            // For now, generate a simple return since Rust doesn't have yield expressions
            // In practice, this would need to be part of an async generator or similar
            Ok(quote! {
                // Yield expression - simplified translation
                // Python's yield doesn't map directly to Rust
                #value_tokens
            })
        } else {
            Ok(quote! {
                // Bare yield - simplified translation
                ()
            })
        }
    }
}

impl CodeGen for YieldFrom {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        (*self.value).find_symbols(symbols)
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let value_tokens = (*self.value).to_rust(ctx, options, symbols)?;
        // For now, generate a simple expression since Rust doesn't have yield from
        Ok(quote! {
            // Yield from expression - simplified translation
            #value_tokens
        })
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here - currently commented out as they need full AST infrastructure
    // create_parse_test!(test_simple_yield, "def gen(): yield 42", "test.py");
}