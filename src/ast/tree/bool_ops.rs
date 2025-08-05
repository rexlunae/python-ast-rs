use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    dump, CodeGen, CodeGenContext, Error, ExprType, Node, PythonOptions, SymbolTableScopes,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BoolOps {
    And,
    Or,
    Unknown,
}

impl<'a> FromPyObject<'a> for BoolOps {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let op_type = ob.get_type().name().expect(
            ob.error_message(
                "<unknown>",
                format!("extracting type name {:?} for boolean operator", ob),
            )
            .as_str(),
        );

        let op_type_str: String = op_type.extract()?;
        let op = match op_type_str.as_str() {
            "And" => BoolOps::And,
            "Or" => BoolOps::Or,
            _ => {
                log::debug!("Found unknown BoolOp {:?}", op_type_str);
                BoolOps::Unknown
            }
        };

        Ok(op)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BoolOp {
    op: BoolOps,
    left: Box<ExprType>,
    right: Box<ExprType>,
}

impl<'a> FromPyObject<'a> for BoolOp {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        log::debug!("ob: {}", dump(ob, None)?);
        let op = ob.getattr("op").expect(
            ob.error_message("<unknown>", "error getting unary operator")
                .as_str(),
        );

        let op_type = op.get_type().name().expect(
            ob.error_message(
                "<unknown>",
                format!("extracting type name {:?} for binary operator", op),
            )
            .as_str(),
        );

        let values = ob.getattr("values").expect(
            ob.error_message("<unknown>", "error getting binary operand")
                .as_str(),
        );

        log::debug!("BoolOps values: {}", dump(&values, None)?);

        let value: Vec<ExprType> = values.extract().expect("getting values from BoolOp");
        let left = value[0].clone();
        let right = value[1].clone();

        let op_type_str: String = op_type.extract()?;
        let op = match op_type_str.as_str() {
            "And" => BoolOps::And,
            "Or" => BoolOps::Or,

            _ => {
                log::debug!("Found unknown BoolOp {:?}", op);
                BoolOps::Unknown
            }
        };

        log::debug!(
            "left: {:?}, right: {:?}, op: {:?}/{:?}",
            left,
            right,
            op_type,
            op
        );

        return Ok(BoolOp {
            op: op,
            left: Box::new(left),
            right: Box::new(right),
        });
    }
}

impl<'a> CodeGen for BoolOp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let left = self
            .left
            .clone()
            .to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let right = self
            .right
            .clone()
            .to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            
        // Python's boolean operators are different from Rust's - they return operands, not booleans
        // For now, we'll use a simplified approach that works for common cases
        let right_str = right.to_string();
        
        match self.op {
            BoolOps::Or => {
                if right_str.trim() == "None" {
                    // Special case for `x or None` - just return the left operand
                    // This avoids the type mismatch error with || None
                    Ok(quote!(#left))
                } else {
                    // Use simple boolean OR for other cases
                    // TODO: Implement proper Python `or` semantics
                    Ok(quote!((#left) || (#right)))
                }
            },
            BoolOps::And => {
                // Use simple boolean AND
                // TODO: Implement proper Python `and` semantics
                Ok(quote!((#left) && (#right)))
            },

            _ => Err(Error::BoolOpNotYetImplemented(self).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        let options = PythonOptions::default();
        let result = crate::parse("1 and 2", "test_case.py").unwrap();
        log::info!("Python tree: {:?}", result);
        //log::info!("{}", result.to_rust().unwrap());

        let code = result
            .to_rust(
                CodeGenContext::Module("test_case".to_string()),
                options,
                SymbolTableScopes::new(),
            )
            .unwrap();
        log::info!("module: {:?}", code);
    }

    #[test]
    fn test_or() {
        let options = PythonOptions::default();
        let result = crate::parse("1 or 2", "test_case.py").unwrap();
        log::info!("Python tree: {:?}", result);
        //log::info!("{}", result);

        let code = result
            .to_rust(
                CodeGenContext::Module("test_case".to_string()),
                options,
                SymbolTableScopes::new(),
            )
            .unwrap();
        log::info!("module: {:?}", code);
    }
}
