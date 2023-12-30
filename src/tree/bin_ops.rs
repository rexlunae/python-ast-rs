use pyo3::{FromPyObject, PyAny, PyResult};
use crate::codegen::Node;
use proc_macro2::TokenStream;
use quote::{quote};

use crate::tree::{ExprType};
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, CodeGenContext};
use crate::symbols::SymbolTableScopes;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Ops {
    Add,
    Unknown,
}

impl<'a> FromPyObject<'a> for Ops {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let err_msg = format!("Unimplemented unary op {}", crate::ast_dump(ob, None)?);
        Err(pyo3::exceptions::PyValueError::new_err(
            ob.error_message("<unknown>", err_msg.as_str())
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BinOp {
    op: Ops,
    left: Box<ExprType>,
    right: Box<ExprType>,
}

impl<'a> FromPyObject<'a> for BinOp {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        log::debug!("ob: {}", crate::ast_dump(ob, None)?);
        let op = ob.getattr("op").expect(
            ob.error_message("<unknown>", "error getting unary operator").as_str()
        );

        let op_type = op.get_type().name().expect(
            ob.error_message("<unknown>", format!("extracting type name {:?} for binary operator", op).as_str()).as_str()
        );

        let left = ob.getattr("left").expect(
            ob.error_message("<unknown>", "error getting binary operand").as_str()
        );

        let right = ob.getattr("right").expect(
            ob.error_message("<unknown>", "error getting binary operand").as_str()
        );
        log::debug!("left: {}, right: {}", crate::ast_dump(left, None)?, crate::ast_dump(right, None)?);

        let op = match op_type {
            "Add" => Ops::Add,
            _ => {
                log::debug!("{:?}", op);
                Ops::Unknown
            }
        };

        log::debug!("left: {}, right: {}, op: {:?}/{:?}", crate::ast_dump(left, None)?, crate::ast_dump(right, None)?, op_type, op);

        let right = ExprType::extract(right).expect("getting binary operator operand");
        let left = ExprType::extract(left).expect("getting binary operator operand");


        return Ok(BinOp{
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

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let left = self.left.clone().to_rust(ctx, options.clone(), symbols.clone())?;
        let right = self.right.clone().to_rust(ctx, options.clone(), symbols.clone())?;
        match self.op {
            Ops::Add => {
                Ok(quote!((#left) + (#right)))
            },
            _ => {
                let error = CodeGenError(format!("BinOp not implemented {:?}", self), None);
                Err(Box::new(error))
            }
        }
    }
}
