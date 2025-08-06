use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
// Keyword arguments are now handled by just passing values
use serde::{Deserialize, Serialize};

use crate::{CodeGen, CodeGenContext, ExprType, PythonOptions, SymbolTableScopes, Node};

/// A keyword argument in a function call.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Keyword {
    /// Keyword name (None for **kwargs unpacking)
    pub arg: Option<String>,
    /// Argument value
    pub value: ExprType,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for Keyword {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let arg = if let Ok(arg_attr) = ob.getattr("arg") {
            if arg_attr.is_none() {
                None
            } else {
                Some(arg_attr.extract()?)
            }
        } else {
            None
        };
        
        let value: ExprType = ob.getattr("value")?.extract()?;
        
        Ok(Self {
            arg,
            value,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl CodeGen for Keyword {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let value = self.value.to_rust(ctx, options, symbols)?;
        
        if let Some(_keyword) = self.arg {
            // Named keyword argument: In Python this is keyword=value,
            // but in Rust we typically pass just the value
            // For now, just pass the value - this could be enhanced to handle
            // struct initialization syntax or builder patterns in the future
            Ok(value)
        } else {
            // **kwargs unpacking: **dict_expr
            // This is complex in Rust and would need special handling
            // For now, just pass the value
            Ok(value)
        }
    }
}

impl Node for Keyword {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}
