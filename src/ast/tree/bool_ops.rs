use pyo3::{FromPyObject, PyAny, PyResult};
use proc_macro2::TokenStream;
use quote::{quote};
use serde::{Serialize, Deserialize};

use crate::{
    dump,
    Node,
    ExprType,
    CodeGen, PythonOptions, CodeGenContext, CodeGenError,
    SymbolTableScopes,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BoolOps {
    And,
    Or,
    Unknown,
}


impl<'a> FromPyObject<'a> for BoolOps {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let err_msg = format!("Unimplemented unary op {}", dump(ob, None)?);
        Err(pyo3::exceptions::PyValueError::new_err(
            ob.error_message("<unknown>", err_msg.as_str())
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BoolOp {
    op: BoolOps,
    left: Box<ExprType>,
    right: Box<ExprType>,
}

impl<'a> FromPyObject<'a> for BoolOp {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        log::debug!("ob: {}", dump(ob, None)?);
        let op = ob.getattr("op").expect(
            ob.error_message("<unknown>", "error getting unary operator").as_str()
        );

        let op_type = op.get_type().name().expect(
            ob.error_message("<unknown>", format!("extracting type name {:?} for binary operator", op).as_str()).as_str()
        );

        let values = ob.getattr("values").expect(
            ob.error_message("<unknown>", "error getting binary operand").as_str()
        );

        println!("BoolOps values: {}", dump(values, None)?);

        let value: Vec<ExprType> = values.extract().expect("getting values from BoolOp");
        let left = value[0].clone();
        let right = value[1].clone();

        let op = match op_type {
            "And" => BoolOps::And,
            "Or" => BoolOps::Or,

            _ => {
                log::debug!("Found unknown BoolOp {:?}", op);
                BoolOps::Unknown
            }
        };

        log::debug!("left: {:?}, right: {:?}, op: {:?}/{:?}", left, right, op_type, op);

        return Ok(BoolOp{
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

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let left = self.left.clone().to_rust(ctx, options.clone(), symbols.clone())?;
        let right = self.right.clone().to_rust(ctx, options.clone(), symbols.clone())?;
        match self.op {
            BoolOps::Or => Ok(quote!((#left) || (#right))),
            BoolOps::And => Ok(quote!((#left) && (#right))),

            _ => {
                let error = CodeGenError::NotYetImplemented(format!("BoolOp not implemented {:?}", self));
                Err(error.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_and() {
        let options = PythonOptions::default();
        let result = crate::parse("1 and 2", "test_case").unwrap();
        log::info!("Python tree: {:?}", result);
        //log::info!("{}", result.to_rust().unwrap());

        let code = result.to_rust(CodeGenContext::Module, options, SymbolTableScopes::new()).unwrap();
        log::info!("module: {:?}", code);
    }

    #[test]
    fn test_or() {
        let options = PythonOptions::default();
        let result = crate::parse("1 or 2", "test_case").unwrap();
        log::info!("Python tree: {:?}", result);
        //log::info!("{}", result);

        let code = result.to_rust(CodeGenContext::Module, options, SymbolTableScopes::new()).unwrap();
        log::info!("module: {:?}", code);
    }
}
