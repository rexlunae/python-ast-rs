use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes,
    extract_list,
};

/// Joined string (f-string, e.g., f"Hello {name}")
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JoinedStr {
    /// The values that make up the f-string (mix of strings and expressions)
    pub values: Vec<ExprType>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// Formatted value within an f-string (e.g., the {name} part)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FormattedValue {
    /// The expression to be formatted
    pub value: Box<ExprType>,
    /// Conversion flag (None, 's', 'r', 'a') - represented as optional integer
    pub conversion: Option<i32>,
    /// Format specifier (optional)
    pub format_spec: Option<Box<ExprType>>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for JoinedStr {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract values
        let values: Vec<ExprType> = extract_list(ob, "values", "joined string values")?;
        
        Ok(JoinedStr {
            values,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl<'a> FromPyObject<'a> for FormattedValue {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract value
        let value: ExprType = ob.getattr("value")?.extract()?;
        
        // Extract conversion (optional)
        let conversion: Option<i32> = if let Ok(conv_attr) = ob.getattr("conversion") {
            let conv_val: i32 = conv_attr.extract()?;
            if conv_val == -1 {
                None // -1 means no conversion
            } else {
                Some(conv_val)
            }
        } else {
            None
        };
        
        // Extract format_spec (optional)
        let format_spec: Option<Box<ExprType>> = if let Ok(spec_attr) = ob.getattr("format_spec") {
            if spec_attr.is_none() {
                None
            } else {
                Some(Box::new(spec_attr.extract()?))
            }
        } else {
            None
        };
        
        Ok(FormattedValue {
            value: Box::new(value),
            conversion,
            format_spec,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl Node for JoinedStr {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl Node for FormattedValue {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl CodeGen for JoinedStr {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        self.values.into_iter().fold(symbols, |acc, val| val.find_symbols(acc))
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // Generate each part of the f-string
        let part_tokens: Result<Vec<TokenStream>, Box<dyn std::error::Error>> = self.values.into_iter()
            .map(|val| val.to_rust(ctx.clone(), options.clone(), symbols.clone()))
            .collect();
        let part_tokens = part_tokens?;

        // For now, generate a simple format! macro call
        // This is a simplified translation - real f-strings are more complex
        if part_tokens.is_empty() {
            Ok(quote! { String::new() })
        } else {
            Ok(quote! {
                format!("{}", #(#part_tokens)+*)
            })
        }
    }
}

impl CodeGen for FormattedValue {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let symbols = (*self.value).find_symbols(symbols);
        if let Some(format_spec) = self.format_spec {
            (*format_spec).find_symbols(symbols)
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
        let value_tokens = (*self.value).to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        
        if let Some(format_spec) = self.format_spec {
            let _spec_tokens = (*format_spec).to_rust(ctx, options, symbols)?;
            // For now, generate a simple format with the format specifier
            // TODO: Properly handle format specifications
            Ok(quote! {
                format!("{}", #value_tokens)
            })
        } else {
            // Simple case - just format with default formatting
            Ok(quote! {
                format!("{}", #value_tokens)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here - currently commented out as they need full AST infrastructure
    // create_parse_test!(test_simple_fstring, "f'Hello {name}'", "test.py");
}