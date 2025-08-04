use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    dump, CodeGen, CodeGenContext, Error, ExprType, Node, PythonOptions, SymbolTableScopes,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BinOps {
    Add,
    Sub,
    Mult,
    Div,
    FloorDiv,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    MatMult,

    Unknown,
}

impl<'a> FromPyObject<'a> for BinOps {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let err_msg = format!("Unimplemented unary op {}", dump(ob, None)?);
        Err(pyo3::exceptions::PyValueError::new_err(
            ob.error_message("<unknown>", err_msg),
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BinOp {
    op: BinOps,
    left: Box<ExprType>,
    right: Box<ExprType>,
}

impl<'a> FromPyObject<'a> for BinOp {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
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

        let left = ob.getattr("left").expect(
            ob.error_message("<unknown>", "error getting binary operand")
                .as_str(),
        );

        let right = ob.getattr("right").expect(
            ob.error_message("<unknown>", "error getting binary operand")
                .as_str(),
        );
        log::debug!("left: {}, right: {}", dump(&left, None)?, dump(&right, None)?);

        let op_type_str: String = op_type.extract()?;
        let op = match op_type_str.as_ref() {
            "Add" => BinOps::Add,
            "Sub" => BinOps::Sub,
            "Mult" => BinOps::Mult,
            "Div" => BinOps::Div,
            "FloorDiv" => BinOps::FloorDiv,
            "Mod" => BinOps::Mod,
            "Pow" => BinOps::Pow,
            "LShift" => BinOps::LShift,
            "RShift" => BinOps::RShift,
            "BitOr" => BinOps::BitOr,
            "BitXor" => BinOps::BitXor,
            "BitAnd" => BinOps::BitAnd,
            "MatMult" => BinOps::MatMult,

            _ => {
                log::debug!("Found unknown BinOp {:?}", op);
                BinOps::Unknown
            }
        };

        log::debug!(
            "left: {}, right: {}, op: {:?}/{:?}",
            dump(&left, None)?,
            dump(&right, None)?,
            op_type_str,
            op
        );

        let right = right.extract().expect("getting binary operator operand");
        let left = left.extract().expect("getting binary operator operand");

        return Ok(BinOp {
            op: op,
            left: Box::new(left),
            right: Box::new(right),
        });
    }
}

impl<'a> CodeGen for BinOp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        let left = self
            .left
            .clone()
            .to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let right = self
            .right
            .clone()
            .to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        match self.op {
            BinOps::Add => Ok(quote!((#left) + (#right))),
            BinOps::Sub => Ok(quote!((#left) - (#right))),
            BinOps::Mult => Ok(quote!((#left) * (#right))),
            BinOps::Div => Ok(quote!((#left) as f64 / (#right) as f64)),
            BinOps::FloorDiv => Ok(quote!((#left) / (#right))),
            BinOps::Mod => Ok(quote!((#left) % (#right))),
            BinOps::Pow => Ok(quote!((#left).pow(#right))),
            BinOps::LShift => Ok(quote!((#left) << (#right))),
            BinOps::RShift => Ok(quote!((#left) >> (#right))),
            BinOps::BitOr => Ok(quote!((#left) | (#right))),
            BinOps::BitXor => Ok(quote!((#left) ^ (#right))),
            BinOps::BitAnd => Ok(quote!((#left) & (#right))),
            //MatMult, XXX implement this
            _ => Err(Error::BinOpNotYetImplemented(self).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let options = PythonOptions::default();
        let result = crate::parse("1 + 2", "test_case.py").unwrap();
        log::info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(
            CodeGenContext::Module("test_case".to_string()),
            options,
            SymbolTableScopes::new(),
        );
        log::info!("module: {:?}", code);
    }

    #[test]
    fn test_subtract() {
        let options = PythonOptions::default();
        let result = crate::parse("1 - 2", "test_case.py").unwrap();
        log::info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(
            CodeGenContext::Module("test_case".to_string()),
            options,
            SymbolTableScopes::new(),
        );
        log::info!("module: {:?}", code);
    }
}
