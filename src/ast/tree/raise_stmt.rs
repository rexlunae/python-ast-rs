use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes,
};

/// Raise statement (raise [exception [from cause]])
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Raise {
    /// The exception to raise (optional - bare raise re-raises current exception)
    pub exc: Option<ExprType>,
    /// The cause of the exception (optional - used with 'from' clause)
    pub cause: Option<ExprType>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for Raise {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract exc (optional)
        let exc: Option<ExprType> = if let Ok(exc_attr) = ob.getattr("exc") {
            if exc_attr.is_none() {
                None
            } else {
                Some(exc_attr.extract()?)
            }
        } else {
            None
        };
        
        // Extract cause (optional)
        let cause: Option<ExprType> = if let Ok(cause_attr) = ob.getattr("cause") {
            if cause_attr.is_none() {
                None
            } else {
                Some(cause_attr.extract()?)
            }
        } else {
            None
        };
        
        Ok(Raise {
            exc,
            cause,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl Node for Raise {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl CodeGen for Raise {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let symbols = if let Some(exc) = self.exc {
            exc.find_symbols(symbols)
        } else {
            symbols
        };
        
        if let Some(cause) = self.cause {
            cause.find_symbols(symbols)
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
        if let Some(exc) = self.exc {
            let exc_tokens = exc.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            
            if let Some(cause) = self.cause {
                let cause_tokens = cause.to_rust(ctx, options, symbols)?;
                // For now, generate a simple panic since Rust doesn't have the same exception model
                Ok(quote! {
                    panic!("Exception: {:?} caused by {:?}", #exc_tokens, #cause_tokens)
                })
            } else {
                Ok(quote! {
                    panic!("Exception: {:?}", #exc_tokens)
                })
            }
        } else {
            // Bare raise - re-raises current exception
            Ok(quote! {
                panic!("Re-raising current exception")
            })
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here - currently commented out as they need full AST infrastructure
    // create_parse_test!(test_simple_raise, "raise ValueError('error')", "test.py");
}