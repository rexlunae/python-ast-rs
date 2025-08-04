use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes,
    BinOps, FromPythonString, PyAttributeExtractor,
};

/// Augmented assignment statement (e.g., x += 1, y -= 2, etc.)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AugAssign {
    /// The target being assigned to (left side)
    pub target: ExprType,
    /// The operator (+=, -=, *=, etc.)
    pub op: BinOps,
    /// The value being assigned (right side)
    pub value: ExprType,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for AugAssign {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract target
        let target = ob.extract_attr_with_context("target", "augmented assignment target")?;
        let target: ExprType = target.extract()?;
        
        // Extract operator
        let op = ob.extract_attr_with_context("op", "augmented assignment operator")?;
        let op_type_str = op.extract_type_name("augmented assignment operator")?;
        let op = BinOps::parse_or_unknown(&op_type_str);
        
        // Extract value
        let value = ob.extract_attr_with_context("value", "augmented assignment value")?;
        let value: ExprType = value.extract()?;
        
        Ok(AugAssign {
            target,
            op,
            value,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl Node for AugAssign {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl CodeGen for AugAssign {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        // Process the value for symbols, but don't add new symbols for augmented assignment
        self.value.find_symbols(symbols)
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let target = self.target.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let value = self.value.to_rust(ctx, options, symbols)?;
        
        // Generate the appropriate augmented assignment operator
        match self.op {
            BinOps::Add => Ok(quote!(#target += #value)),
            BinOps::Sub => Ok(quote!(#target -= #value)),
            BinOps::Mult => Ok(quote!(#target *= #value)),
            BinOps::Div => Ok(quote!(#target /= #value)),
            BinOps::FloorDiv => Ok(quote!(#target /= #value)), // Rust doesn't have floor div assign
            BinOps::Mod => Ok(quote!(#target %= #value)),
            BinOps::BitAnd => Ok(quote!(#target &= #value)),
            BinOps::BitOr => Ok(quote!(#target |= #value)),
            BinOps::BitXor => Ok(quote!(#target ^= #value)),
            BinOps::LShift => Ok(quote!(#target <<= #value)),
            BinOps::RShift => Ok(quote!(#target >>= #value)),
            BinOps::Pow => {
                // Rust doesn't have **= operator, so we need to expand it
                Ok(quote!(#target = (#target).pow(#value)))
            },
            BinOps::MatMult => {
                // Matrix multiplication assignment - not directly supported in Rust
                // Would need specific matrix library support
                Err(format!("Matrix multiplication assignment not supported in Rust").into())
            },
            BinOps::Unknown => {
                Err(format!("Unknown augmented assignment operator").into())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_parse_test;

    create_parse_test!(test_add_assign, "x += 1", "test.py");
    create_parse_test!(test_sub_assign, "x -= 1", "test.py");
    create_parse_test!(test_mul_assign, "x *= 2", "test.py");
    create_parse_test!(test_div_assign, "x /= 3", "test.py");
    create_parse_test!(test_mod_assign, "x %= 4", "test.py");
    create_parse_test!(test_pow_assign, "x **= 2", "test.py");
    create_parse_test!(test_bitand_assign, "x &= 5", "test.py");
    create_parse_test!(test_bitor_assign, "x |= 6", "test.py");
    create_parse_test!(test_bitxor_assign, "x ^= 7", "test.py");
    create_parse_test!(test_lshift_assign, "x <<= 2", "test.py");
    create_parse_test!(test_rshift_assign, "x >>= 3", "test.py");
}